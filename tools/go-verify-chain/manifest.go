package main

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"reflect"
	"strings"
)

const (
	manifestIdentity = "magpie-portable-verifier-corpus-v1"
	manifestSHA256   = "7d758d3f2dac1161fe15dd064b130ccbfcdaf0437ea8ab8d7772492194801a81"
)

type manifest struct {
	SchemaVersion    int    `json:"schema_version"`
	ManifestIdentity string `json:"manifest_identity"`
	ProducingContext struct {
		SignatureProfileIdentity string `json:"signature_profile_identity"`
	} `json:"producing_context"`
	Cases []manifestCase `json:"cases"`
}

type manifestCase struct {
	ID                      string           `json:"id"`
	InputPath               string           `json:"input_path"`
	InputSHA256             string           `json:"input_sha256"`
	ExternalVerifyingKeyHex string           `json:"external_verifying_key_hex"`
	Expected                manifestExpected `json:"expected"`
}

type manifestExpected struct {
	Verdict                 string   `json:"verdict"`
	Class                   *string  `json:"class"`
	Line                    *uint64  `json:"line"`
	RecordIndex             *uint64  `json:"record_index"`
	EventCount              *uint64  `json:"event_count"`
	Tip                     *string  `json:"tip"`
	OrderedRecomputedHashes []string `json:"ordered_recomputed_hashes"`
}

type batchResult struct {
	ID     string `json:"id"`
	Result result `json:"result"`
}

func runManifest(path string, check bool) error {
	matched, err := processManifest(path, true, check)
	if err != nil {
		return err
	}
	if check && !matched {
		return fmt.Errorf("one or more manifest results did not match expected fields")
	}
	return nil
}

func runManifestCheck(path string) (bool, error) {
	return processManifest(path, false, true)
}

func processManifest(path string, emit, compare bool) (bool, error) {
	manifestPath, err := filepath.Abs(path)
	if err != nil {
		return false, fmt.Errorf("resolve manifest path: %w", err)
	}
	manifestBytes, err := os.ReadFile(manifestPath)
	if err != nil {
		return false, fmt.Errorf("read manifest: %w", err)
	}
	if err := validateManifestCommitment(manifestPath, manifestBytes); err != nil {
		return false, err
	}
	var frozen manifest
	if err := json.Unmarshal(manifestBytes, &frozen); err != nil {
		return false, fmt.Errorf("decode manifest: %w", err)
	}
	if frozen.SchemaVersion != 1 || frozen.ManifestIdentity != manifestIdentity {
		return false, fmt.Errorf("unexpected frozen manifest identity or schema")
	}
	if frozen.ProducingContext.SignatureProfileIdentity != SignatureProfile {
		return false, fmt.Errorf("manifest selects unsupported signature profile")
	}
	if len(frozen.Cases) != 432 {
		return false, fmt.Errorf("manifest has %d cases, want 432", len(frozen.Cases))
	}
	if err := validateManifestCaseMetadata(frozen.Cases); err != nil {
		return false, err
	}
	repositoryRoot := filepath.Dir(filepath.Dir(filepath.Dir(manifestPath)))
	matched := true
	for _, frozenCase := range frozen.Cases {
		inputPath, err := manifestInputPath(repositoryRoot, frozenCase.InputPath)
		if err != nil {
			return false, fmt.Errorf("case %s: %w", frozenCase.ID, err)
		}
		// Preflight the external key before reading the history bytes. For a
		// governed key rejection, the bytes are read only for the independent
		// manifest hash commitment and are never parsed as history.
		prepared, keyReject, err := prepareVerifier(SignatureProfile, frozenCase.ExternalVerifyingKeyHex)
		if err != nil {
			return false, fmt.Errorf("case %s: %w", frozenCase.ID, err)
		}
		inputBytes, err := os.ReadFile(inputPath)
		if err != nil {
			return false, fmt.Errorf("case %s: read input: %w", frozenCase.ID, err)
		}
		inputDigest := sha256.Sum256(inputBytes)
		if hex.EncodeToString(inputDigest[:]) != frozenCase.InputSHA256 {
			return false, fmt.Errorf("case %s: input SHA-256 mismatch", frozenCase.ID)
		}
		var actual result
		if keyReject {
			actual = rejectResult("ExternalKey", nil, nil)
		} else {
			actual, err = verifyRecords(prepared, inputBytes)
			if err != nil {
				return false, fmt.Errorf("case %s: %w", frozenCase.ID, err)
			}
		}
		if compare && !resultsEqual(actual, frozenCase.Expected) {
			matched = false
			fmt.Fprintf(os.Stderr, "manifest case %s: result mismatch\n", frozenCase.ID)
		}
		if emit {
			if err := writeJSONLine(batchResult{ID: frozenCase.ID, Result: actual}); err != nil {
				return false, fmt.Errorf("write case %s result: %w", frozenCase.ID, err)
			}
		}
	}
	return matched, nil
}

func validateManifestCommitment(manifestPath string, manifestBytes []byte) error {
	digest := sha256.Sum256(manifestBytes)
	if hex.EncodeToString(digest[:]) != manifestSHA256 {
		return fmt.Errorf("manifest SHA-256 does not match frozen v1 commitment")
	}
	sidecarPath := filepath.Join(filepath.Dir(manifestPath), "manifest.sha256")
	sidecarBytes, err := os.ReadFile(sidecarPath)
	if err != nil {
		return fmt.Errorf("read manifest commitment: %w", err)
	}
	lines := strings.Split(strings.ReplaceAll(string(sidecarBytes), "\r\n", "\n"), "\n")
	if len(lines) < 2 || lines[0] != "# "+manifestIdentity {
		return fmt.Errorf("manifest commitment identity mismatch")
	}
	fields := strings.Fields(lines[1])
	if len(fields) != 2 || fields[0] != manifestSHA256 || fields[1] != "manifest.json" {
		return fmt.Errorf("manifest commitment digest mismatch")
	}
	return nil
}

func validateManifestCaseMetadata(cases []manifestCase) error {
	seen := make(map[string]struct{}, len(cases))
	for _, frozenCase := range cases {
		if frozenCase.ID == "" {
			return fmt.Errorf("manifest contains empty case ID")
		}
		if _, duplicate := seen[frozenCase.ID]; duplicate {
			return fmt.Errorf("manifest contains duplicate case ID %q", frozenCase.ID)
		}
		seen[frozenCase.ID] = struct{}{}
		if !isLowerHex(frozenCase.InputSHA256, 64) {
			return fmt.Errorf("case %s has invalid input SHA-256 spelling", frozenCase.ID)
		}
	}
	return nil
}

func manifestInputPath(root, relative string) (string, error) {
	if relative == "" || filepath.IsAbs(relative) {
		return "", fmt.Errorf("input path must be repository-relative")
	}
	clean := filepath.Clean(filepath.FromSlash(relative))
	if clean == "." || clean == ".." || strings.HasPrefix(clean, ".."+string(os.PathSeparator)) {
		return "", fmt.Errorf("input path escapes repository root")
	}
	path := filepath.Join(root, clean)
	back, err := filepath.Rel(root, path)
	if err != nil || back == ".." || strings.HasPrefix(back, ".."+string(os.PathSeparator)) {
		return "", fmt.Errorf("input path escapes repository root")
	}
	return path, nil
}

func resultsEqual(actual result, expected manifestExpected) bool {
	return reflect.DeepEqual(actual.Verdict, expected.Verdict) &&
		reflect.DeepEqual(actual.Class, expected.Class) &&
		reflect.DeepEqual(actual.Line, expected.Line) &&
		reflect.DeepEqual(actual.RecordIndex, expected.RecordIndex) &&
		reflect.DeepEqual(actual.EventCount, expected.EventCount) &&
		reflect.DeepEqual(actual.Tip, expected.Tip) &&
		reflect.DeepEqual(actual.OrderedRecomputedHashes, expected.OrderedRecomputedHashes)
}
