package main

import (
	"encoding/json"
	"fmt"
	"os"
)

func init() {
	osReadFile = os.ReadFile
}

func main() {
	code := command(os.Args[1:])
	os.Exit(code)
}

func command(args []string) int {
	if len(args) == 2 && args[0] == "--manifest" {
		if err := runManifest(args[1], false); err != nil {
			fmt.Fprintln(os.Stderr, err)
			return 2
		}
		return 0
	}
	if len(args) == 2 && args[0] == "--check-manifest" {
		matched, err := runManifestCheck(args[1])
		if err != nil {
			fmt.Fprintln(os.Stderr, err)
			return 2
		}
		if !matched {
			return 1
		}
		return 0
	}
	if len(args) != 3 {
		fmt.Fprintln(os.Stderr, "usage: go-verify-chain <signature-profile> <lowercase-external-key-hex> <history-path>")
		fmt.Fprintln(os.Stderr, "   or: go-verify-chain --manifest <fixtures/verifier-language-v1/manifest.json>")
		fmt.Fprintln(os.Stderr, "   or: go-verify-chain --check-manifest <fixtures/verifier-language-v1/manifest.json>")
		return 2
	}
	got, err := verifyFile(args[0], args[1], args[2])
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		return 2
	}
	if err := writeJSONLine(got); err != nil {
		fmt.Fprintln(os.Stderr, err)
		return 2
	}
	if got.Verdict == "REJECT" {
		return 1
	}
	return 0
}

func writeJSONLine(value any) error {
	encoded, err := json.Marshal(value)
	if err != nil {
		return err
	}
	_, err = os.Stdout.Write(append(encoded, '\n'))
	return err
}
