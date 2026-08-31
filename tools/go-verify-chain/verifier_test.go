package main

import (
	"os"
	"path/filepath"
	"testing"
)

const testManifestPath = "../../fixtures/verifier-language-v1/manifest.json"

func TestFrozenManifest(t *testing.T) {
	matched, err := runManifestCheck(testManifestPath)
	if err != nil {
		t.Fatal(err)
	}
	if !matched {
		t.Fatal("frozen manifest result mismatch")
	}
}

func TestExternalKeyGate(t *testing.T) {
	cases := []struct {
		name string
		key  string
		good bool
	}{
		{
			name: "ordinary prime subgroup key",
			key:  "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c",
			good: true,
		},
		{
			name: "identity",
			key:  "0100000000000000000000000000000000000000000000000000000000000000",
		},
		{
			name: "noncanonical y equals p",
			key:  "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		},
		{
			name: "noncanonical y p plus one",
			key:  "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		},
		{
			name: "zero x sign one",
			key:  "0100000000000000000000000000000000000000000000000000000000000080",
		},
		{
			name: "order two",
			key:  "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		},
	}
	for _, testCase := range cases {
		t.Run(testCase.name, func(t *testing.T) {
			prepared, keyReject, err := prepareVerifier(SignatureProfile, testCase.key)
			if err != nil {
				t.Fatal(err)
			}
			if testCase.good {
				if keyReject || prepared == nil {
					t.Fatal("ordinary key was rejected")
				}
			} else if !keyReject || prepared != nil {
				t.Fatal("inadmissible key was accepted")
			}
		})
	}
}

func TestInvalidKeyPrecedesHistoryIOMissing(t *testing.T) {
	got, err := verifyFile(SignatureProfile,
		"0100000000000000000000000000000000000000000000000000000000000000",
		"path-that-must-not-be-opened.jsonl")
	if err != nil {
		t.Fatal(err)
	}
	if got.Verdict != "REJECT" || got.Class == nil || *got.Class != "ExternalKey" {
		t.Fatalf("got %#v, want ExternalKey", got)
	}
}

func TestIdentityRAndFramingCoordinates(t *testing.T) {
	identityRPath := filepath.Join("..", "..", "fixtures", "verifier-language-v1", "cases", "a21-d2-identity-r-equation-true.jsonl")
	identityRBytes, err := os.ReadFile(identityRPath)
	if err != nil {
		t.Fatal(err)
	}
	prepared, keyReject, err := prepareVerifier(SignatureProfile,
		"ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c")
	if err != nil || keyReject {
		t.Fatalf("prepare key: %v", err)
	}
	accepted, err := verifyRecords(prepared, identityRBytes)
	if err != nil {
		t.Fatal(err)
	}
	if accepted.Verdict != "ACCEPT" || accepted.EventCount == nil || *accepted.EventCount != 1 {
		t.Fatalf("identity-R result: %#v", accepted)
	}

	withWhitespace := append(append([]byte(nil), identityRBytes...), []byte(" \t\n")...)
	framed, err := verifyRecords(prepared, withWhitespace)
	if err != nil {
		t.Fatal(err)
	}
	if framed.Verdict != "REJECT" || framed.Class == nil || *framed.Class != "Framing" ||
		framed.Line == nil || *framed.Line != 2 || framed.RecordIndex == nil || *framed.RecordIndex != 1 {
		t.Fatalf("whitespace framing result: %#v", framed)
	}
}

func TestZeroByteSnapshot(t *testing.T) {
	prepared, keyReject, err := prepareVerifier(SignatureProfile,
		"ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c")
	if err != nil || keyReject {
		t.Fatalf("prepare key: %v", err)
	}
	got, err := verifyRecords(prepared, nil)
	if err != nil {
		t.Fatal(err)
	}
	if got.Verdict != "ACCEPT" || got.EventCount == nil || *got.EventCount != 0 || got.Tip == nil || *got.Tip != zeroHash {
		t.Fatalf("empty result: %#v", got)
	}
}

const zeroHash = "0000000000000000000000000000000000000000000000000000000000000000"
