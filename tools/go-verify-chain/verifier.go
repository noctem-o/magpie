package main

import (
	"bytes"
	"crypto/sha256"
	"crypto/sha512"
	"encoding/binary"
	"encoding/hex"
	"errors"
	"fmt"
	"math/big"
	"strconv"
	"strings"
	"unicode/utf8"

	"filippo.io/edwards25519"
)

const (
	SignatureProfile = "magpie-ed25519-canonical-prime-subgroup-v1"
	CoreProfile      = "magpie-core-v1"
	SignatureDomain  = "magpie-sig-v1"
)

var (
	fieldPrime    = new(big.Int).Sub(new(big.Int).Lsh(big.NewInt(1), 255), big.NewInt(19))
	groupOrder, _ = new(big.Int).SetString("7237005577332262213973186563042994240857116359379907606001950938285454250989", 10)
	curveD        = new(big.Int)
	sqrtM1        = new(big.Int)
	sqrtExponent  = new(big.Int)
)

func init() {
	// d = -121665 / 121666 (mod p), and sqrtM1 is a square root of -1.
	numerator := new(big.Int).Neg(big.NewInt(121665))
	numerator.Mod(numerator, fieldPrime)
	denominatorInverse := new(big.Int).ModInverse(big.NewInt(121666), fieldPrime)
	curveD.Mul(numerator, denominatorInverse)
	curveD.Mod(curveD, fieldPrime)
	sqrtM1.Exp(big.NewInt(2), new(big.Int).Rsh(new(big.Int).Sub(fieldPrime, big.NewInt(1)), 2), fieldPrime)
	sqrtExponent.Div(new(big.Int).Add(fieldPrime, big.NewInt(3)), big.NewInt(8))
}

type result struct {
	Verdict                 string   `json:"verdict"`
	Class                   *string  `json:"class"`
	Line                    *uint64  `json:"line"`
	RecordIndex             *uint64  `json:"record_index"`
	EventCount              *uint64  `json:"event_count"`
	Tip                     *string  `json:"tip"`
	OrderedRecomputedHashes []string `json:"ordered_recomputed_hashes"`
}

func rejectResult(class string, line, index *uint64) result {
	return result{
		Verdict:                 "REJECT",
		Class:                   stringPtr(class),
		Line:                    line,
		RecordIndex:             index,
		EventCount:              nil,
		Tip:                     nil,
		OrderedRecomputedHashes: []string{},
	}
}

func acceptResult(count uint64, hashes [][32]byte) result {
	ordered := make([]string, len(hashes))
	for i := range hashes {
		ordered[i] = hex.EncodeToString(hashes[i][:])
	}
	tip := strings.Repeat("0", 64)
	if len(ordered) != 0 {
		tip = ordered[len(ordered)-1]
	}
	return result{
		Verdict:                 "ACCEPT",
		Class:                   nil,
		Line:                    nil,
		RecordIndex:             nil,
		EventCount:              uint64Ptr(count),
		Tip:                     stringPtr(tip),
		OrderedRecomputedHashes: ordered,
	}
}

func stringPtr(value string) *string { return &value }

func uint64Ptr(value uint64) *uint64 { return &value }

type verifier struct {
	keyBytes [32]byte
	public   *edwards25519.Point
}

// prepareVerifier performs the complete external-key gate. It returns a nil
// verifier for a governed ExternalKey rejection and an error only for an
// internal/configuration failure.
func prepareVerifier(profile, keyText string) (*verifier, bool, error) {
	if profile != SignatureProfile {
		return nil, false, fmt.Errorf("unsupported signature profile %q", profile)
	}
	key, ok := decodeLowerHex(keyText, 32)
	if !ok {
		return nil, true, nil
	}
	point, err := decodeCanonicalPoint(key)
	if err != nil {
		return nil, true, nil
	}
	if point.Equal(edwards25519.NewIdentityPoint()) != 0 {
		return nil, true, nil
	}
	if !isPrimeSubgroup(point) {
		return nil, true, nil
	}
	var keyBytes [32]byte
	copy(keyBytes[:], key)
	return &verifier{keyBytes: keyBytes, public: point}, false, nil
}

func verifyFile(profile, keyText, historyPath string) (result, error) {
	v, keyReject, err := prepareVerifier(profile, keyText)
	if err != nil {
		return result{}, err
	}
	if keyReject {
		return rejectResult("ExternalKey", nil, nil), nil
	}
	data, err := readHistory(historyPath)
	if err != nil {
		return result{}, err
	}
	return verifyRecords(v, data)
}

func readHistory(path string) ([]byte, error) {
	// Kept separate so callers can ensure external-key preflight occurs before
	// touching the history path.
	return osReadFile(path)
}

// osReadFile is assigned in main.go to keep the verifier package surface small
// and to make file access easy to audit in tests.
var osReadFile = func(path string) ([]byte, error) {
	return nil, errors.New("history file reader not initialized")
}

type historyState struct {
	count  uint64
	tip    [32]byte
	hashes [][32]byte
}

func verifyRecords(v *verifier, data []byte) (result, error) {
	if len(data) == 0 {
		return acceptResult(0, nil), nil
	}
	state := historyState{}
	start := 0
	line := uint64(1)
	recordIndex := uint64(0)
	for i, byteValue := range data {
		if byteValue != '\n' {
			continue
		}
		end := i
		if end > start && data[end-1] == '\r' {
			end--
		}
		candidate := data[start:end]
		if len(candidate) == 0 {
			return rejectResult("Framing", uint64Ptr(line), nil), nil
		}
		current, stop, err := verifyCandidate(v, &state, candidate, line, recordIndex)
		if err != nil {
			return result{}, err
		}
		if stop {
			return current, nil
		}
		recordIndex++
		start = i + 1
		line++
	}
	if start < len(data) {
		candidate := data[start:]
		current, stop, err := verifyCandidate(v, &state, candidate, line, recordIndex)
		if err != nil {
			return result{}, err
		}
		if stop {
			return current, nil
		}
		recordIndex++
	}
	return acceptResult(state.count, state.hashes), nil
}

func verifyCandidate(v *verifier, state *historyState, candidate []byte, line, recordIndex uint64) (result, bool, error) {
	linePtr := uint64Ptr(line)
	indexPtr := uint64Ptr(recordIndex)
	if len(candidate) >= 3 && candidate[0] == 0xef && candidate[1] == 0xbb && candidate[2] == 0xbf {
		return rejectResult("Framing", linePtr, indexPtr), true, nil
	}
	if bytes.IndexByte(candidate, '\r') >= 0 {
		return rejectResult("Framing", linePtr, indexPtr), true, nil
	}
	if allHorizontalWhitespace(candidate) {
		return rejectResult("Framing", linePtr, indexPtr), true, nil
	}
	if !utf8.Valid(candidate) {
		return rejectResult("Framing", linePtr, indexPtr), true, nil
	}
	parsed, schemaError, syntaxError := parseJSONRecord(candidate)
	if syntaxError {
		return rejectResult("JsonSyntax", linePtr, indexPtr), true, nil
	}
	if schemaError {
		return rejectResult("Schema", linePtr, indexPtr), true, nil
	}
	event, ok := decodeEvent(parsed)
	if !ok {
		return rejectResult("Schema", linePtr, indexPtr), true, nil
	}
	if event.seq != state.count {
		return rejectResult("Sequence", linePtr, indexPtr), true, nil
	}
	if event.prevHash != state.tip {
		return rejectResult("PreviousLink", linePtr, indexPtr), true, nil
	}
	canonical := encodeEventCore(event)
	recomputed := sha256.Sum256(canonical)
	if event.hash != recomputed {
		return rejectResult("ContentHash", linePtr, indexPtr), true, nil
	}
	if !v.verifySignature(event.signature, recomputed) {
		return rejectResult("Signature", linePtr, indexPtr), true, nil
	}
	if !validatePayload(event.payload) {
		return rejectResult("PayloadValidation", linePtr, indexPtr), true, nil
	}
	if !validateGenesis(event.payload, state.count, v.keyBytes) {
		return rejectResult("Genesis", linePtr, indexPtr), true, nil
	}
	if state.count == ^uint64(0) {
		return result{}, false, errors.New("event count overflow")
	}
	state.count++
	state.tip = recomputed
	state.hashes = append(state.hashes, recomputed)
	return result{}, false, nil
}

func allHorizontalWhitespace(candidate []byte) bool {
	if len(candidate) == 0 {
		return false
	}
	for _, byteValue := range candidate {
		if byteValue != ' ' && byteValue != '\t' {
			return false
		}
	}
	return true
}

func (v *verifier) verifySignature(signature [64]byte, contentHash [32]byte) bool {
	rPoint, err := decodeCanonicalPoint(signature[:32])
	if err != nil || !isPrimeSubgroup(rPoint) {
		return false
	}
	scalarS, err := new(edwards25519.Scalar).SetCanonicalBytes(signature[32:])
	if err != nil {
		return false
	}
	message := make([]byte, 0, len(SignatureDomain)+len(contentHash))
	message = append(message, SignatureDomain...)
	message = append(message, contentHash[:]...)
	challengeInput := make([]byte, 0, len(signature)+len(v.keyBytes)+len(message))
	challengeInput = append(challengeInput, signature[:32]...)
	challengeInput = append(challengeInput, v.keyBytes[:]...)
	challengeInput = append(challengeInput, message...)
	digest := sha512.Sum512(challengeInput)
	scalarK, err := new(edwards25519.Scalar).SetUniformBytes(digest[:])
	if err != nil {
		return false
	}
	lhs := new(edwards25519.Point).ScalarBaseMult(scalarS)
	kA := new(edwards25519.Point).ScalarMult(scalarK, v.public)
	rhs := new(edwards25519.Point).Add(rPoint, kA)
	return lhs.Equal(rhs) == 1
}

func validateGenesis(payload payload, count uint64, key [32]byte) bool {
	if count != 0 {
		return payload.kind != payloadGenesis
	}
	if payload.kind != payloadGenesis || payload.genesisProfile != CoreProfile {
		return false
	}
	declared, ok := decodeLowerHex(payload.genesisKey, 32)
	if !ok {
		return false
	}
	return bytes.Equal(declared, key[:])
}

var (
	actorClasses = map[string]struct{}{
		"HumanRoot": {}, "AgentProposer": {}, "AutomatedVerifier": {},
		"DeadboltAnchorer": {}, "LensWitness": {}, "SourceImporter": {},
	}
	evidenceKinds = map[string]struct{}{
		"DeterministicVerification": {}, "HumanRatification": {}, "DeadboltAnchor": {},
		"ExecutionEvidence": {}, "BehavioralEvaluation": {}, "ExternalSource": {},
		"ModelSelfReport": {}, "LensReadout": {},
	}
	edgeKinds = map[string]struct{}{
		"supports": {}, "derived_from": {}, "contradicts": {}, "supersedes": {},
		"invalidates": {}, "ratifies": {},
	}
)

func validatePayload(payload payload) bool {
	switch payload.kind {
	case payloadSegmentAnchored:
		return isLowerHex(payload.witnessRoot, 64)
	case payloadClaimAssertedV2:
		return payload.claimID != "" && payload.statement != "" && payload.scopeRef != "" &&
			contains(actorClasses, payload.actorClass) && validOptionalHash(payload.contentHash)
	case payloadEvidenceRegistered:
		return payload.evidenceID != "" && payload.summary != "" && payload.scopeRef != "" &&
			contains(evidenceKinds, payload.evidenceKind) && contains(actorClasses, payload.actorClass) &&
			validOptionalHash(payload.contentHash)
	case payloadJustificationEdgeRecorded:
		return payload.edgeID != "" && payload.sourceID != "" && payload.targetID != "" &&
			payload.scopeRef != "" && payload.rationale != "" && contains(edgeKinds, payload.edgeKind) &&
			contains(actorClasses, payload.actorClass)
	default:
		return true
	}
}

func contains(values map[string]struct{}, value string) bool {
	_, ok := values[value]
	return ok
}

func validOptionalHash(value string) bool {
	return value == "" || isLowerHex(value, 64)
}

func isLowerHex(value string, length int) bool {
	if len(value) != length {
		return false
	}
	for i := 0; i < len(value); i++ {
		c := value[i]
		if !((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')) {
			return false
		}
	}
	return true
}

func decodeLowerHex(value string, octets int) ([]byte, bool) {
	if !isLowerHex(value, octets*2) {
		return nil, false
	}
	decoded, err := hex.DecodeString(value)
	if err != nil || len(decoded) != octets {
		return nil, false
	}
	return decoded, true
}

type payloadKind uint8

const (
	payloadGenesis payloadKind = iota
	payloadClaimAsserted
	payloadEvidenceRecorded
	payloadClaimStatusChanged
	payloadNote
	payloadSegmentAnchored
	payloadClaimAssertedV2
	payloadEvidenceRegistered
	payloadJustificationEdgeRecorded
)

type payload struct {
	kind payloadKind

	genesisProfile   string
	genesisKey       string
	claimID          string
	statement        string
	status           uint8
	summary          string
	from             uint8
	to               uint8
	reason           string
	text             string
	bundleKind       string
	witnessRoot      string
	witnessAlgorithm string
	runID            string
	scopeRef         string
	actorClass       string
	contentHash      string
	metadataJSON     string
	evidenceID       string
	evidenceKind     string
	edgeID           string
	edgeKind         string
	sourceID         string
	targetID         string
	rationale        string
}

type event struct {
	seq            uint64
	timestampNanos uint64
	prevHash       [32]byte
	agent          string
	source         string
	payload        payload
	hash           [32]byte
	signature      [64]byte
}

func encodeEventCore(event event) []byte {
	var out bytes.Buffer
	out.WriteString(CoreProfile)
	writeU64(&out, event.seq)
	writeU64(&out, event.timestampNanos)
	out.Write(event.prevHash[:])
	writeString(&out, event.agent)
	writeString(&out, event.source)
	out.WriteByte(byte(event.payload.kind))
	switch event.payload.kind {
	case payloadGenesis:
		writeString(&out, event.payload.genesisProfile)
		writeString(&out, event.payload.genesisKey)
	case payloadClaimAsserted:
		writeString(&out, event.payload.claimID)
		writeString(&out, event.payload.statement)
		out.WriteByte(event.payload.status)
	case payloadEvidenceRecorded:
		writeString(&out, event.payload.claimID)
		writeString(&out, event.payload.summary)
	case payloadClaimStatusChanged:
		writeString(&out, event.payload.claimID)
		out.WriteByte(event.payload.from)
		out.WriteByte(event.payload.to)
		writeString(&out, event.payload.reason)
	case payloadNote:
		writeString(&out, event.payload.text)
	case payloadSegmentAnchored:
		writeString(&out, event.payload.bundleKind)
		writeString(&out, event.payload.witnessRoot)
		writeString(&out, event.payload.witnessAlgorithm)
		writeString(&out, event.payload.genesisProfile)
		writeString(&out, event.payload.runID)
	case payloadClaimAssertedV2:
		writeString(&out, event.payload.claimID)
		writeString(&out, event.payload.statement)
		writeString(&out, event.payload.scopeRef)
		writeString(&out, event.payload.actorClass)
		writeString(&out, event.payload.contentHash)
		writeString(&out, event.payload.metadataJSON)
	case payloadEvidenceRegistered:
		writeString(&out, event.payload.evidenceID)
		writeString(&out, event.payload.evidenceKind)
		writeString(&out, event.payload.summary)
		writeString(&out, event.payload.scopeRef)
		writeString(&out, event.payload.actorClass)
		writeString(&out, event.payload.contentHash)
		writeString(&out, event.payload.metadataJSON)
	case payloadJustificationEdgeRecorded:
		writeString(&out, event.payload.edgeID)
		writeString(&out, event.payload.edgeKind)
		writeString(&out, event.payload.sourceID)
		writeString(&out, event.payload.targetID)
		writeString(&out, event.payload.scopeRef)
		writeString(&out, event.payload.actorClass)
		writeString(&out, event.payload.rationale)
		writeString(&out, event.payload.metadataJSON)
	}
	return out.Bytes()
}

func writeU64(out *bytes.Buffer, value uint64) {
	var encoded [8]byte
	binary.BigEndian.PutUint64(encoded[:], value)
	out.Write(encoded[:])
}

func writeString(out *bytes.Buffer, value string) {
	writeU64(out, uint64(len(value)))
	out.WriteString(value)
}

func parseU64Node(node *jsonNode) (uint64, bool) {
	if node == nil || node.kind != jsonNumber {
		return 0, false
	}
	if node.raw != "0" && (len(node.raw) == 0 || node.raw[0] < '1' || node.raw[0] > '9') {
		return 0, false
	}
	for i := 0; i < len(node.raw); i++ {
		if node.raw[i] < '0' || node.raw[i] > '9' {
			return 0, false
		}
	}
	value, err := strconv.ParseUint(node.raw, 10, 64)
	if err != nil {
		return 0, false
	}
	return value, true
}

func parseStatusNode(node *jsonNode) (uint8, bool) {
	if node == nil || node.kind != jsonString {
		return 0, false
	}
	statuses := map[string]uint8{
		"Open": 0, "Conjectured": 1, "Supported": 2, "Settled": 3, "Refuted": 4,
	}
	value, ok := statuses[node.str]
	return value, ok
}

func requireString(fields map[string]*jsonNode, name string) (string, bool) {
	node, ok := fields[name]
	if !ok || node == nil || node.kind != jsonString {
		return "", false
	}
	return node.str, true
}

func exactObject(node *jsonNode, names []string) (map[string]*jsonNode, bool) {
	if node == nil || node.kind != jsonObject {
		return nil, false
	}
	wanted := make(map[string]struct{}, len(names))
	for _, name := range names {
		wanted[name] = struct{}{}
	}
	fields := make(map[string]*jsonNode, len(node.members))
	for _, member := range node.members {
		if _, ok := wanted[member.name]; !ok {
			return nil, false
		}
		if _, duplicate := fields[member.name]; duplicate {
			return nil, false
		}
		fields[member.name] = member.value
	}
	if len(fields) != len(wanted) {
		return nil, false
	}
	return fields, true
}

func decodeEvent(node *jsonNode) (event, bool) {
	var event event
	signedEvent, ok := exactObject(node, []string{"core", "hash", "signature"})
	if !ok {
		return event, false
	}
	coreFields, ok := exactObject(signedEvent["core"], []string{"seq", "timestamp_nanos", "prev_hash", "provenance", "payload"})
	if !ok {
		return event, false
	}
	event.seq, ok = parseU64Node(coreFields["seq"])
	if !ok {
		return event, false
	}
	event.timestampNanos, ok = parseU64Node(coreFields["timestamp_nanos"])
	if !ok {
		return event, false
	}
	prevText, ok := requireString(coreFields, "prev_hash")
	if !ok {
		return event, false
	}
	prevBytes, ok := decodeLowerHex(prevText, 32)
	if !ok {
		return event, false
	}
	copy(event.prevHash[:], prevBytes)
	provenanceFields, ok := exactObject(coreFields["provenance"], []string{"agent", "source"})
	if !ok {
		return event, false
	}
	event.agent, ok = requireString(provenanceFields, "agent")
	if !ok {
		return event, false
	}
	event.source, ok = requireString(provenanceFields, "source")
	if !ok {
		return event, false
	}
	event.payload, ok = decodePayload(coreFields["payload"])
	if !ok {
		return event, false
	}
	hashText, ok := requireString(signedEvent, "hash")
	if !ok {
		return event, false
	}
	hashBytes, ok := decodeLowerHex(hashText, 32)
	if !ok {
		return event, false
	}
	copy(event.hash[:], hashBytes)
	signatureText, ok := requireString(signedEvent, "signature")
	if !ok {
		return event, false
	}
	signatureBytes, ok := decodeLowerHex(signatureText, 64)
	if !ok {
		return event, false
	}
	copy(event.signature[:], signatureBytes)
	return event, true
}

func decodePayload(node *jsonNode) (payload, bool) {
	var payload payload
	if node == nil || node.kind != jsonObject {
		return payload, false
	}
	kindNode := findMember(node, "kind")
	if kindNode == nil || kindNode.kind != jsonString {
		return payload, false
	}
	switch kindNode.str {
	case "Genesis":
		fields, ok := exactObject(node, []string{"kind", "canonicalization_profile", "verifying_key"})
		if !ok {
			return payload, false
		}
		payload.kind = payloadGenesis
		payload.genesisProfile, ok = requireString(fields, "canonicalization_profile")
		if !ok {
			return payload, false
		}
		payload.genesisKey, ok = requireString(fields, "verifying_key")
		return payload, ok
	case "ClaimAsserted":
		fields, ok := exactObject(node, []string{"kind", "claim_id", "statement", "status"})
		if !ok {
			return payload, false
		}
		payload.kind = payloadClaimAsserted
		payload.claimID, ok = requireString(fields, "claim_id")
		if !ok {
			return payload, false
		}
		payload.statement, ok = requireString(fields, "statement")
		if !ok {
			return payload, false
		}
		payload.status, ok = parseStatusNode(fields["status"])
		return payload, ok
	case "EvidenceRecorded":
		fields, ok := exactObject(node, []string{"kind", "claim_id", "summary"})
		if !ok {
			return payload, false
		}
		payload.kind = payloadEvidenceRecorded
		payload.claimID, ok = requireString(fields, "claim_id")
		if !ok {
			return payload, false
		}
		payload.summary, ok = requireString(fields, "summary")
		return payload, ok
	case "ClaimStatusChanged":
		fields, ok := exactObject(node, []string{"kind", "claim_id", "from", "to", "reason"})
		if !ok {
			return payload, false
		}
		payload.kind = payloadClaimStatusChanged
		payload.claimID, ok = requireString(fields, "claim_id")
		if !ok {
			return payload, false
		}
		payload.from, ok = parseStatusNode(fields["from"])
		if !ok {
			return payload, false
		}
		payload.to, ok = parseStatusNode(fields["to"])
		if !ok {
			return payload, false
		}
		payload.reason, ok = requireString(fields, "reason")
		return payload, ok
	case "Note":
		fields, ok := exactObject(node, []string{"kind", "text"})
		if !ok {
			return payload, false
		}
		payload.kind = payloadNote
		payload.text, ok = requireString(fields, "text")
		return payload, ok
	case "SegmentAnchored":
		fields, ok := exactObject(node, []string{"kind", "bundle_kind", "witness_root", "witness_algorithm", "canonicalization_profile", "run_id"})
		if !ok {
			return payload, false
		}
		payload.kind = payloadSegmentAnchored
		payload.bundleKind, ok = requireString(fields, "bundle_kind")
		if !ok {
			return payload, false
		}
		payload.witnessRoot, ok = requireString(fields, "witness_root")
		if !ok {
			return payload, false
		}
		payload.witnessAlgorithm, ok = requireString(fields, "witness_algorithm")
		if !ok {
			return payload, false
		}
		payload.genesisProfile, ok = requireString(fields, "canonicalization_profile")
		if !ok {
			return payload, false
		}
		payload.runID, ok = requireString(fields, "run_id")
		return payload, ok
	case "ClaimAssertedV2":
		fields, ok := exactObject(node, []string{"kind", "claim_id", "statement", "scope_ref", "actor_class", "content_hash", "metadata_json"})
		if !ok {
			return payload, false
		}
		payload.kind = payloadClaimAssertedV2
		payload.claimID, ok = requireString(fields, "claim_id")
		if !ok {
			return payload, false
		}
		payload.statement, ok = requireString(fields, "statement")
		if !ok {
			return payload, false
		}
		payload.scopeRef, ok = requireString(fields, "scope_ref")
		if !ok {
			return payload, false
		}
		payload.actorClass, ok = requireString(fields, "actor_class")
		if !ok {
			return payload, false
		}
		payload.contentHash, ok = requireString(fields, "content_hash")
		if !ok {
			return payload, false
		}
		payload.metadataJSON, ok = requireString(fields, "metadata_json")
		return payload, ok
	case "EvidenceRegistered":
		fields, ok := exactObject(node, []string{"kind", "evidence_id", "evidence_kind", "summary", "scope_ref", "actor_class", "content_hash", "metadata_json"})
		if !ok {
			return payload, false
		}
		payload.kind = payloadEvidenceRegistered
		payload.evidenceID, ok = requireString(fields, "evidence_id")
		if !ok {
			return payload, false
		}
		payload.evidenceKind, ok = requireString(fields, "evidence_kind")
		if !ok {
			return payload, false
		}
		payload.summary, ok = requireString(fields, "summary")
		if !ok {
			return payload, false
		}
		payload.scopeRef, ok = requireString(fields, "scope_ref")
		if !ok {
			return payload, false
		}
		payload.actorClass, ok = requireString(fields, "actor_class")
		if !ok {
			return payload, false
		}
		payload.contentHash, ok = requireString(fields, "content_hash")
		if !ok {
			return payload, false
		}
		payload.metadataJSON, ok = requireString(fields, "metadata_json")
		return payload, ok
	case "JustificationEdgeRecorded":
		fields, ok := exactObject(node, []string{"kind", "edge_id", "edge_kind", "source_id", "target_id", "scope_ref", "actor_class", "rationale", "metadata_json"})
		if !ok {
			return payload, false
		}
		payload.kind = payloadJustificationEdgeRecorded
		payload.edgeID, ok = requireString(fields, "edge_id")
		if !ok {
			return payload, false
		}
		payload.edgeKind, ok = requireString(fields, "edge_kind")
		if !ok {
			return payload, false
		}
		payload.sourceID, ok = requireString(fields, "source_id")
		if !ok {
			return payload, false
		}
		payload.targetID, ok = requireString(fields, "target_id")
		if !ok {
			return payload, false
		}
		payload.scopeRef, ok = requireString(fields, "scope_ref")
		if !ok {
			return payload, false
		}
		payload.actorClass, ok = requireString(fields, "actor_class")
		if !ok {
			return payload, false
		}
		payload.rationale, ok = requireString(fields, "rationale")
		if !ok {
			return payload, false
		}
		payload.metadataJSON, ok = requireString(fields, "metadata_json")
		return payload, ok
	default:
		return payload, false
	}
}

func findMember(node *jsonNode, name string) *jsonNode {
	for _, member := range node.members {
		if member.name == name {
			return member.value
		}
	}
	return nil
}

func littleEndianBig(value []byte) *big.Int {
	reversed := make([]byte, len(value))
	for i := range value {
		reversed[len(value)-1-i] = value[i]
	}
	return new(big.Int).SetBytes(reversed)
}

func littleEndianBytes(value *big.Int, size int) []byte {
	bigEndian := value.Bytes()
	out := make([]byte, size)
	for i := 0; i < len(bigEndian) && i < size; i++ {
		out[i] = bigEndian[len(bigEndian)-1-i]
	}
	return out
}

func decodeCanonicalPoint(encoded []byte) (*edwards25519.Point, error) {
	if len(encoded) != 32 {
		return nil, errors.New("point length")
	}
	yBytes := append([]byte(nil), encoded...)
	sign := uint(yBytes[31] >> 7)
	yBytes[31] &= 0x7f
	y := littleEndianBig(yBytes)
	if y.Cmp(fieldPrime) >= 0 {
		return nil, errors.New("non-canonical y")
	}
	// RFC 8032 section 5.1.3: x^2 = (y^2 - 1)/(d*y^2 + 1),
	// followed by the p == 5 (mod 8) square-root procedure.
	ySquared := new(big.Int).Mul(y, y)
	ySquared.Mod(ySquared, fieldPrime)
	numerator := new(big.Int).Sub(ySquared, big.NewInt(1))
	numerator.Mod(numerator, fieldPrime)
	denominator := new(big.Int).Mul(curveD, ySquared)
	denominator.Add(denominator, big.NewInt(1))
	denominator.Mod(denominator, fieldPrime)
	inverse := new(big.Int).ModInverse(denominator, fieldPrime)
	if inverse == nil {
		return nil, errors.New("non-invertible point denominator")
	}
	xSquared := new(big.Int).Mul(numerator, inverse)
	xSquared.Mod(xSquared, fieldPrime)
	x := new(big.Int).Exp(xSquared, sqrtExponent, fieldPrime)
	check := new(big.Int).Mul(x, x)
	check.Mod(check, fieldPrime)
	if check.Cmp(xSquared) != 0 {
		x.Mul(x, sqrtM1)
		x.Mod(x, fieldPrime)
		check.Mul(x, x)
		check.Mod(check, fieldPrime)
		if check.Cmp(xSquared) != 0 {
			return nil, errors.New("point is not on curve")
		}
	}
	if x.Sign() == 0 && sign != 0 {
		return nil, errors.New("invalid sign for zero x")
	}
	if uint(x.Bit(0)) != sign {
		x.Sub(fieldPrime, x)
		x.Mod(x, fieldPrime)
	}
	canonical := littleEndianBytes(y, 32)
	canonical[31] |= byte(sign << 7)
	if !bytes.Equal(canonical, encoded) {
		return nil, errors.New("non-canonical point encoding")
	}
	point, err := new(edwards25519.Point).SetBytes(encoded)
	if err != nil {
		return nil, err
	}
	if !bytes.Equal(point.Bytes(), encoded) {
		return nil, errors.New("point library re-encoding mismatch")
	}
	return point, nil
}

func isPrimeSubgroup(point *edwards25519.Point) bool {
	return multiplyByInteger(point, groupOrder).Equal(edwards25519.NewIdentityPoint()) == 1
}

func multiplyByInteger(point *edwards25519.Point, scalar *big.Int) *edwards25519.Point {
	accumulator := edwards25519.NewIdentityPoint()
	base := new(edwards25519.Point).Set(point)
	for bit := 0; bit < scalar.BitLen(); bit++ {
		if scalar.Bit(bit) != 0 {
			accumulator = new(edwards25519.Point).Add(accumulator, base)
		}
		base = new(edwards25519.Point).Double(base)
	}
	return accumulator
}

// The small JSON representation below retains decoded names and raw number
// tokens. It deliberately does not use encoding/json for governed records:
// encoding/json accepts duplicate names, replaces invalid UTF-8, and exposes
// numbers through a lossy/default numeric path.
type jsonKind uint8

const (
	jsonNull jsonKind = iota
	jsonBoolean
	jsonNumber
	jsonString
	jsonArray
	jsonObject
)

type jsonNode struct {
	kind    jsonKind
	boolean bool
	raw     string
	str     string
	array   []*jsonNode
	members []jsonMember
}

type jsonMember struct {
	name  string
	value *jsonNode
}

var (
	errJSONSyntax = errors.New("json syntax")
)

type jsonParser struct {
	data      []byte
	position  int
	schemaErr bool
}

func parseJSONRecord(data []byte) (*jsonNode, bool, bool) {
	parser := &jsonParser{data: data}
	parser.skipWhitespace()
	root, err := parser.parseValue()
	if err != nil {
		return nil, false, true
	}
	parser.skipWhitespace()
	if parser.position != len(parser.data) {
		return nil, false, true
	}
	return root, parser.schemaErr, false
}

func (parser *jsonParser) skipWhitespace() {
	for parser.position < len(parser.data) && (parser.data[parser.position] == ' ' || parser.data[parser.position] == '\t') {
		parser.position++
	}
}

func (parser *jsonParser) parseValue() (*jsonNode, error) {
	if parser.position >= len(parser.data) {
		return nil, errJSONSyntax
	}
	switch parser.data[parser.position] {
	case 'n':
		if !parser.consumeLiteral("null") {
			return nil, errJSONSyntax
		}
		return &jsonNode{kind: jsonNull}, nil
	case 't':
		if !parser.consumeLiteral("true") {
			return nil, errJSONSyntax
		}
		return &jsonNode{kind: jsonBoolean, boolean: true}, nil
	case 'f':
		if !parser.consumeLiteral("false") {
			return nil, errJSONSyntax
		}
		return &jsonNode{kind: jsonBoolean, boolean: false}, nil
	case '"':
		value, err := parser.parseString()
		if err != nil {
			return nil, err
		}
		return &jsonNode{kind: jsonString, str: value}, nil
	case '[':
		return parser.parseArray()
	case '{':
		return parser.parseObject()
	case '-':
		return parser.parseNumber()
	default:
		if parser.data[parser.position] >= '0' && parser.data[parser.position] <= '9' {
			return parser.parseNumber()
		}
		return nil, errJSONSyntax
	}
}

func (parser *jsonParser) consumeLiteral(literal string) bool {
	if len(parser.data)-parser.position < len(literal) || string(parser.data[parser.position:parser.position+len(literal)]) != literal {
		return false
	}
	parser.position += len(literal)
	return true
}

func (parser *jsonParser) parseArray() (*jsonNode, error) {
	parser.position++ // [
	array := &jsonNode{kind: jsonArray}
	parser.skipWhitespace()
	if parser.position < len(parser.data) && parser.data[parser.position] == ']' {
		parser.position++
		return array, nil
	}
	for {
		value, err := parser.parseValue()
		if err != nil {
			return nil, err
		}
		array.array = append(array.array, value)
		parser.skipWhitespace()
		if parser.position >= len(parser.data) {
			return nil, errJSONSyntax
		}
		if parser.data[parser.position] == ']' {
			parser.position++
			return array, nil
		}
		if parser.data[parser.position] != ',' {
			return nil, errJSONSyntax
		}
		parser.position++
		parser.skipWhitespace()
	}
}

func (parser *jsonParser) parseObject() (*jsonNode, error) {
	parser.position++ // {
	object := &jsonNode{kind: jsonObject}
	seen := make(map[string]struct{})
	parser.skipWhitespace()
	if parser.position < len(parser.data) && parser.data[parser.position] == '}' {
		parser.position++
		return object, nil
	}
	for {
		if parser.position >= len(parser.data) || parser.data[parser.position] != '"' {
			return nil, errJSONSyntax
		}
		name, err := parser.parseString()
		if err != nil {
			return nil, err
		}
		if _, duplicate := seen[name]; duplicate {
			parser.schemaErr = true
		}
		seen[name] = struct{}{}
		parser.skipWhitespace()
		if parser.position >= len(parser.data) || parser.data[parser.position] != ':' {
			return nil, errJSONSyntax
		}
		parser.position++
		parser.skipWhitespace()
		value, err := parser.parseValue()
		if err != nil {
			return nil, err
		}
		object.members = append(object.members, jsonMember{name: name, value: value})
		parser.skipWhitespace()
		if parser.position >= len(parser.data) {
			return nil, errJSONSyntax
		}
		if parser.data[parser.position] == '}' {
			parser.position++
			return object, nil
		}
		if parser.data[parser.position] != ',' {
			return nil, errJSONSyntax
		}
		parser.position++
		parser.skipWhitespace()
	}
}

func (parser *jsonParser) parseString() (string, error) {
	if parser.position >= len(parser.data) || parser.data[parser.position] != '"' {
		return "", errJSONSyntax
	}
	parser.position++
	var value strings.Builder
	for parser.position < len(parser.data) {
		byteValue := parser.data[parser.position]
		switch {
		case byteValue == '"':
			parser.position++
			return value.String(), nil
		case byteValue == '\\':
			parser.position++
			if parser.position >= len(parser.data) {
				return "", errJSONSyntax
			}
			escape := parser.data[parser.position]
			parser.position++
			switch escape {
			case '"', '\\', '/':
				value.WriteByte(escape)
			case 'b':
				value.WriteByte('\b')
			case 'f':
				value.WriteByte('\f')
			case 'n':
				value.WriteByte('\n')
			case 'r':
				value.WriteByte('\r')
			case 't':
				value.WriteByte('\t')
			case 'u':
				runeValue, err := parser.parseUnicodeEscape()
				if err != nil {
					return "", err
				}
				value.WriteRune(runeValue)
			default:
				return "", errJSONSyntax
			}
		case byteValue < 0x20:
			return "", errJSONSyntax
		default:
			runeValue, size := utf8.DecodeRune(parser.data[parser.position:])
			if runeValue == utf8.RuneError && size == 1 {
				return "", errJSONSyntax
			}
			value.WriteRune(runeValue)
			parser.position += size
		}
	}
	return "", errJSONSyntax
}

func (parser *jsonParser) parseUnicodeEscape() (rune, error) {
	if len(parser.data)-parser.position < 4 {
		return 0, errJSONSyntax
	}
	value, ok := parseHex4(parser.data[parser.position : parser.position+4])
	if !ok {
		return 0, errJSONSyntax
	}
	parser.position += 4
	if value >= 0xd800 && value <= 0xdbff {
		if len(parser.data)-parser.position >= 6 && parser.data[parser.position] == '\\' && parser.data[parser.position+1] == 'u' {
			low, lowOK := parseHex4(parser.data[parser.position+2 : parser.position+6])
			if lowOK && low >= 0xdc00 && low <= 0xdfff {
				parser.position += 6
				codePoint := 0x10000 + (int(value-0xd800)<<10 | int(low-0xdc00))
				return rune(codePoint), nil
			}
		}
		parser.schemaErr = true
		return utf8.RuneError, nil
	}
	if value >= 0xdc00 && value <= 0xdfff {
		parser.schemaErr = true
		return utf8.RuneError, nil
	}
	return rune(value), nil
}

func parseHex4(value []byte) (uint16, bool) {
	if len(value) != 4 {
		return 0, false
	}
	var out uint16
	for _, byteValue := range value {
		var digit byte
		switch {
		case byteValue >= '0' && byteValue <= '9':
			digit = byteValue - '0'
		case byteValue >= 'a' && byteValue <= 'f':
			digit = byteValue - 'a' + 10
		case byteValue >= 'A' && byteValue <= 'F':
			digit = byteValue - 'A' + 10
		default:
			return 0, false
		}
		out = (out << 4) | uint16(digit)
	}
	return out, true
}

func (parser *jsonParser) parseNumber() (*jsonNode, error) {
	start := parser.position
	if parser.data[parser.position] == '-' {
		parser.position++
		if parser.position >= len(parser.data) {
			return nil, errJSONSyntax
		}
	}
	if parser.position >= len(parser.data) {
		return nil, errJSONSyntax
	}
	if parser.data[parser.position] == '0' {
		parser.position++
		if parser.position < len(parser.data) && parser.data[parser.position] >= '0' && parser.data[parser.position] <= '9' {
			return nil, errJSONSyntax
		}
	} else if parser.data[parser.position] >= '1' && parser.data[parser.position] <= '9' {
		for parser.position < len(parser.data) && parser.data[parser.position] >= '0' && parser.data[parser.position] <= '9' {
			parser.position++
		}
	} else {
		return nil, errJSONSyntax
	}
	if parser.position < len(parser.data) && parser.data[parser.position] == '.' {
		parser.position++
		fractionStart := parser.position
		for parser.position < len(parser.data) && parser.data[parser.position] >= '0' && parser.data[parser.position] <= '9' {
			parser.position++
		}
		if parser.position == fractionStart {
			return nil, errJSONSyntax
		}
	}
	if parser.position < len(parser.data) && (parser.data[parser.position] == 'e' || parser.data[parser.position] == 'E') {
		parser.position++
		if parser.position < len(parser.data) && (parser.data[parser.position] == '+' || parser.data[parser.position] == '-') {
			parser.position++
		}
		exponentStart := parser.position
		for parser.position < len(parser.data) && parser.data[parser.position] >= '0' && parser.data[parser.position] <= '9' {
			parser.position++
		}
		if parser.position == exponentStart {
			return nil, errJSONSyntax
		}
	}
	return &jsonNode{kind: jsonNumber, raw: string(parser.data[start:parser.position])}, nil
}
