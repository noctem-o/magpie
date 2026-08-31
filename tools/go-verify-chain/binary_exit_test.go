package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"testing"
)

func TestBuiltBinaryPreservesExitCodeContract(t *testing.T) {
	moduleRoot, err := os.Getwd()
	if err != nil {
		t.Fatal(err)
	}
	repositoryRoot := filepath.Clean(filepath.Join(moduleRoot, "..", ".."))
	executableSuffix := ""
	if runtime.GOOS == "windows" {
		executableSuffix = ".exe"
	}
	goExecutable := filepath.Join(runtime.GOROOT(), "bin", "go"+executableSuffix)
	binaryPath := filepath.Join(t.TempDir(), "go-verify-chain"+executableSuffix)

	build := exec.Command(goExecutable, "build", "-mod=vendor", "-o", binaryPath, ".")
	build.Dir = moduleRoot
	build.Env = offlineGoEnvironment(os.Environ())
	if output, buildErr := build.CombinedOutput(); buildErr != nil {
		t.Fatalf("offline vendored binary build failed: %v\n%s", buildErr, output)
	}

	manifestBytes, err := os.ReadFile(testManifestPath)
	if err != nil {
		t.Fatal(err)
	}
	var frozen manifest
	if err := json.Unmarshal(manifestBytes, &frozen); err != nil {
		t.Fatal(err)
	}
	acceptCase, rejectCase := selectExitCodeCases(t, frozen.Cases)

	tests := []struct {
		name        string
		args        []string
		wantCode    int
		wantVerdict string
	}{
		{
			name: "accept exit 0",
			args: []string{
				SignatureProfile,
				acceptCase.ExternalVerifyingKeyHex,
				filepath.Join(repositoryRoot, filepath.FromSlash(acceptCase.InputPath)),
			},
			wantCode:    0,
			wantVerdict: "ACCEPT",
		},
		{
			name: "governed reject exit 1",
			args: []string{
				SignatureProfile,
				rejectCase.ExternalVerifyingKeyHex,
				filepath.Join(repositoryRoot, filepath.FromSlash(rejectCase.InputPath)),
			},
			wantCode:    1,
			wantVerdict: "REJECT",
		},
		{
			name:     "usage failure exit 2",
			wantCode: 2,
		},
	}

	for _, testCase := range tests {
		t.Run(testCase.name, func(t *testing.T) {
			code, stdout, stderr := runBuiltBinary(t, binaryPath, repositoryRoot, testCase.args)
			if code != testCase.wantCode {
				t.Fatalf("exit code = %d, want %d; stdout=%q stderr=%q", code, testCase.wantCode, stdout, stderr)
			}
			if testCase.wantVerdict == "" {
				if !bytes.Contains(stderr, []byte("usage: go-verify-chain")) {
					t.Fatalf("usage failure stderr = %q", stderr)
				}
				return
			}
			var got result
			if err := json.Unmarshal(stdout, &got); err != nil {
				t.Fatalf("decode binary result: %v; stdout=%q", err, stdout)
			}
			if got.Verdict != testCase.wantVerdict {
				t.Fatalf("verdict = %q, want %q", got.Verdict, testCase.wantVerdict)
			}
		})
	}
}

func selectExitCodeCases(t *testing.T, cases []manifestCase) (manifestCase, manifestCase) {
	t.Helper()
	var acceptCase manifestCase
	var rejectCase manifestCase
	for _, candidate := range cases {
		switch candidate.Expected.Verdict {
		case "ACCEPT":
			if acceptCase.ID == "" {
				acceptCase = candidate
			}
		case "REJECT":
			if rejectCase.ID == "" {
				rejectCase = candidate
			}
		}
	}
	if acceptCase.ID == "" || rejectCase.ID == "" {
		t.Fatal("frozen manifest lacks an ACCEPT or REJECT case")
	}
	return acceptCase, rejectCase
}

func runBuiltBinary(t *testing.T, binaryPath, workingDirectory string, args []string) (int, []byte, []byte) {
	t.Helper()
	command := exec.Command(binaryPath, args...)
	command.Dir = workingDirectory
	var stdout bytes.Buffer
	var stderr bytes.Buffer
	command.Stdout = &stdout
	command.Stderr = &stderr
	err := command.Run()
	if err == nil {
		return 0, stdout.Bytes(), stderr.Bytes()
	}
	var exitError *exec.ExitError
	if !errors.As(err, &exitError) {
		t.Fatalf("execute built binary: %v", err)
	}
	return exitError.ExitCode(), stdout.Bytes(), stderr.Bytes()
}

func offlineGoEnvironment(base []string) []string {
	blocked := map[string]struct{}{
		"GOENV":       {},
		"GOFLAGS":     {},
		"GONOPROXY":   {},
		"GONOSUMDB":   {},
		"GOPRIVATE":   {},
		"GOPROXY":     {},
		"GOSUMDB":     {},
		"GOTOOLCHAIN": {},
		"GOVCS":       {},
		"GOWORK":      {},
	}
	environment := make([]string, 0, len(base)+10)
	for _, entry := range base {
		name, _, found := strings.Cut(entry, "=")
		if !found {
			continue
		}
		shouldKeep := true
		for blockedName := range blocked {
			if strings.EqualFold(name, blockedName) {
				shouldKeep = false
				break
			}
		}
		if shouldKeep {
			environment = append(environment, entry)
		}
	}
	return append(
		environment,
		"GOENV=off",
		"GOFLAGS=",
		"GONOPROXY=",
		"GONOSUMDB=",
		"GOPRIVATE=",
		"GOPROXY=off",
		"GOSUMDB=off",
		"GOTOOLCHAIN=local",
		"GOVCS=*:off",
		"GOWORK=off",
	)
}
