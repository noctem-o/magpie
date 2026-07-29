//! Claim-inline deterministic direct refutation under standing policy v4.
//!
//! Policy v4 inherits one complete policy-v3 resolution from the same verified
//! replay context and immutable content closure. Its only new authority is the
//! closed `sha256_claim_inline_bytes_direct_refutation_v0` rule. Public audit
//! values are one-way outputs and are never accepted by the resolver.
//!
//! The three authority-bearing audit structs cannot be constructed:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ClaimInlinePredicateEdgeLaneIneligibleReasonV0,
//!     StandingTraceClassificationV4, StandingTraceEntryV4,
//! };
//! let _ = StandingTraceEntryV4 {
//!     edge_id: String::new(),
//!     evidence_id: String::new(),
//!     classification: StandingTraceClassificationV4::Ineligible,
//!     ineligible_reason: None::<ClaimInlinePredicateEdgeLaneIneligibleReasonV0>,
//!     failure: None,
//! };
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     StandingPolicyApplicationV4, StandingPolicyContextV4,
//!     StandingPolicyRuleV4,
//! };
//! use magpie_log::Status;
//! let _ = StandingPolicyApplicationV4 {
//!     rule: StandingPolicyRuleV4::Sha256ClaimInlineBytesDirectRefutationV0,
//!     context: StandingPolicyContextV4::UncontestedRefutationEligible,
//!     achieved_standing: Some(Status::Refuted),
//! };
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ResolutionContentClosureIdentityV0, StandingCurrentness,
//!     StandingResolutionV3, StandingResolutionV4,
//!     VerifiedLogPrefixIdentityV0,
//! };
//! fn fabricate(
//!     prefix: VerifiedLogPrefixIdentityV0,
//!     closure: ResolutionContentClosureIdentityV0,
//!     inherited: StandingResolutionV3,
//! ) {
//!     let _ = StandingResolutionV4 {
//!         claim_id: String::new(),
//!         governed_standing: None,
//!         legacy_raw_standing: None,
//!         currentness: StandingCurrentness::Unknown,
//!         policy_id: "foreign",
//!         verified_prefix_identity: prefix,
//!         closure_identity: closure,
//!         inherited_v3: inherited,
//!         resolution_failure: None,
//!         candidate_trace: Vec::new(),
//!         applications: Vec::new(),
//!         blockers: Vec::new(),
//!     };
//! }
//! ```
//!
//! They expose no public constructors:
//!
//! ```compile_fail
//! use magpie_claims::StandingTraceEntryV4;
//! let _ = StandingTraceEntryV4::new();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyApplicationV4;
//! let _ = StandingPolicyApplicationV4::new();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionV4;
//! let _ = StandingResolutionV4::new();
//! ```
//!
//! They cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::StandingTraceEntryV4;
//! let _: StandingTraceEntryV4 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyApplicationV4;
//! let _: StandingPolicyApplicationV4 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionV4;
//! let _: StandingResolutionV4 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! They cannot be defaulted:
//!
//! ```compile_fail
//! use magpie_claims::StandingTraceEntryV4;
//! let _ = StandingTraceEntryV4::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyApplicationV4;
//! let _ = StandingPolicyApplicationV4::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionV4;
//! let _ = StandingResolutionV4::default();
//! ```
//!
//! Their fields cannot be mutated:
//!
//! ```compile_fail
//! use magpie_claims::StandingTraceEntryV4;
//! fn mutate(value: &mut StandingTraceEntryV4) {
//!     value.edge_id = String::new();
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyApplicationV4;
//! use magpie_log::Status;
//! fn mutate(value: &mut StandingPolicyApplicationV4) {
//!     value.achieved_standing = Some(Status::Settled);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionV4;
//! use magpie_log::Status;
//! fn mutate(value: &mut StandingResolutionV4) {
//!     value.governed_standing = Some(Status::Settled);
//! }
//! ```
//!
//! They expose no consuming field extraction:
//!
//! ```compile_fail
//! use magpie_claims::StandingTraceEntryV4;
//! fn consume(value: StandingTraceEntryV4) {
//!     let _ = value.into_edge_id();
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyApplicationV4;
//! fn consume(value: StandingPolicyApplicationV4) {
//!     let _ = value.into_rule();
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionV4;
//! fn consume(value: StandingResolutionV4) {
//!     let _ = value.into_inherited_v3();
//! }
//! ```
//!
//! No `From` or `TryFrom` authority conversion exists:
//!
//! ```compile_fail
//! use magpie_claims::StandingTraceEntryV4;
//! let _: StandingTraceEntryV4 = ().into();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyApplicationV4;
//! let _: StandingPolicyApplicationV4 = ().into();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionV4;
//! let _: StandingResolutionV4 = ().into();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingTraceEntryV4;
//! let _ = StandingTraceEntryV4::try_from(());
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyApplicationV4;
//! let _ = StandingPolicyApplicationV4::try_from(());
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionV4;
//! let _ = StandingResolutionV4::try_from(());
//! ```
//!
//! They have no `Debug` surface:
//!
//! ```compile_fail
//! use magpie_claims::StandingTraceEntryV4;
//! fn inspect(value: &StandingTraceEntryV4) {
//!     let _ = format!("{value:?}");
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyApplicationV4;
//! fn inspect(value: &StandingPolicyApplicationV4) {
//!     let _ = format!("{value:?}");
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionV4;
//! fn inspect(value: &StandingResolutionV4) {
//!     let _ = format!("{value:?}");
//! }
//! ```
//!
//! They have no `Display` surface:
//!
//! ```compile_fail
//! use magpie_claims::StandingTraceEntryV4;
//! fn inspect(value: &StandingTraceEntryV4) {
//!     let _ = format!("{value}");
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyApplicationV4;
//! fn inspect(value: &StandingPolicyApplicationV4) {
//!     let _ = format!("{value}");
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionV4;
//! fn inspect(value: &StandingResolutionV4) {
//!     let _ = format!("{value}");
//! }
//! ```
//!
//! They cannot be treated as errors:
//!
//! ```compile_fail
//! use magpie_claims::StandingTraceEntryV4;
//! fn inspect(value: &StandingTraceEntryV4) {
//!     let _: &(dyn std::error::Error + 'static) = value;
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyApplicationV4;
//! fn inspect(value: &StandingPolicyApplicationV4) {
//!     let _: &(dyn std::error::Error + 'static) = value;
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionV4;
//! fn inspect(value: &StandingResolutionV4) {
//!     let _: &(dyn std::error::Error + 'static) = value;
//! }
//! ```
//!
//! They cannot be compared for equality:
//!
//! ```compile_fail
//! use magpie_claims::StandingTraceEntryV4;
//! fn compare(left: &StandingTraceEntryV4, right: &StandingTraceEntryV4) {
//!     let _ = left == right;
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyApplicationV4;
//! fn compare(
//!     left: &StandingPolicyApplicationV4,
//!     right: &StandingPolicyApplicationV4,
//! ) {
//!     let _ = left == right;
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionV4;
//! fn compare(left: &StandingResolutionV4, right: &StandingResolutionV4) {
//!     let _ = left == right;
//! }
//! ```
//!
//! They cannot be ordered:
//!
//! ```compile_fail
//! use magpie_claims::StandingTraceEntryV4;
//! fn compare(left: &StandingTraceEntryV4, right: &StandingTraceEntryV4) {
//!     let _ = left < right;
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyApplicationV4;
//! fn compare(
//!     left: &StandingPolicyApplicationV4,
//!     right: &StandingPolicyApplicationV4,
//! ) {
//!     let _ = left < right;
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionV4;
//! fn compare(left: &StandingResolutionV4, right: &StandingResolutionV4) {
//!     let _ = left < right;
//! }
//! ```
//!
//! They cannot be hashed:
//!
//! ```compile_fail
//! use std::hash::{Hash, Hasher};
//! use magpie_claims::StandingTraceEntryV4;
//! fn hash<H: Hasher>(value: &StandingTraceEntryV4, state: &mut H) {
//!     value.hash(state);
//! }
//! ```
//!
//! ```compile_fail
//! use std::hash::{Hash, Hasher};
//! use magpie_claims::StandingPolicyApplicationV4;
//! fn hash<H: Hasher>(value: &StandingPolicyApplicationV4, state: &mut H) {
//!     value.hash(state);
//! }
//! ```
//!
//! ```compile_fail
//! use std::hash::{Hash, Hasher};
//! use magpie_claims::StandingResolutionV4;
//! fn hash<H: Hasher>(value: &StandingResolutionV4, state: &mut H) {
//!     value.hash(state);
//! }
//! ```
//!
//! Constructible inspection enums remain one-way and cannot be deserialized or
//! defaulted:
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyRuleV4;
//! let _: StandingPolicyRuleV4 = serde_json::from_str(
//!     r#""sha256_claim_inline_bytes_direct_refutation_v0""#,
//! ).unwrap();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyRuleV4;
//! let _ = StandingPolicyRuleV4::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyContextV4;
//! let _: StandingPolicyContextV4 = serde_json::from_str(
//!     r#"{"kind":"uncontested_refutation_eligible"}"#,
//! ).unwrap();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyContextV4;
//! let _ = StandingPolicyContextV4::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingTraceClassificationV4;
//! let _: StandingTraceClassificationV4 =
//!     serde_json::from_str(r#""refutation_eligible""#).unwrap();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingTraceClassificationV4;
//! let _ = StandingTraceClassificationV4::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingBlockerV4;
//! let _: StandingBlockerV4 =
//!     serde_json::from_str(r#"{"kind":"inherited_v3_resolution_failed"}"#).unwrap();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingBlockerV4;
//! let _ = StandingBlockerV4::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionFailureV4;
//! let _: StandingResolutionFailureV4 =
//!     serde_json::from_str(r#"{"kind":"inherited_claim_mismatch"}"#).unwrap();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionFailureV4;
//! let _ = StandingResolutionFailureV4::default();
//! ```
//!
//! The v4 resolver rejects every caller-supplied authority substitute by
//! signature. Inherited v3 audit cannot be injected:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     StandingResolutionV3,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     value: &StandingResolutionV3,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, value);
//! }
//! ```
//!
//! V4 trace and application audit cannot be injected:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     StandingTraceEntryV4,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     value: &StandingTraceEntryV4,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, value);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     StandingPolicyApplicationV4,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     value: &StandingPolicyApplicationV4,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, value);
//! }
//! ```
//!
//! V4 blocker and failure tokens cannot be injected:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     StandingBlockerV4,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     value: &StandingBlockerV4,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, value);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     StandingResolutionFailureV4,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     value: StandingResolutionFailureV4,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, value);
//! }
//! ```
//!
//! Predicate outcomes and receipts cannot be injected:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     Sha256ClaimInlineBytesPredicateOutcomeV0,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     value: &Sha256ClaimInlineBytesPredicateOutcomeV0,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, value);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     Sha256ClaimInlineBytesPredicateReceiptV0,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     value: &Sha256ClaimInlineBytesPredicateReceiptV0,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, value);
//! }
//! ```
//!
//! Attestation outcomes and receipts cannot be injected:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     InlinePredicateAttestationBindingOutcomeV0,
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     value: &InlinePredicateAttestationBindingOutcomeV0,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, value);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     InlinePredicateAttestationBindingReceiptV0,
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     value: &InlinePredicateAttestationBindingReceiptV0,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, value);
//! }
//! ```
//!
//! Edge-lane outcomes and receipts cannot be injected:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ClaimInlinePredicateEdgeLaneOutcomeV0,
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     value: &ClaimInlinePredicateEdgeLaneOutcomeV0,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, value);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ClaimInlinePredicateEdgeLaneReceiptV0,
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     value: &ClaimInlinePredicateEdgeLaneReceiptV0,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, value);
//! }
//! ```
//!
//! A relation token cannot be selected by the caller:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ClaimInlinePredicateEdgeLaneRelationV0,
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     value: ClaimInlinePredicateEdgeLaneRelationV0,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, value);
//! }
//! ```
//!
//! Candidate lists cannot be supplied:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     candidates: &[String],
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4(
//!         "claim", closure, candidates,
//!     );
//! }
//! ```
//!
//! Replayed edge and evidence objects cannot be supplied:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     TypedJustificationEdge,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     edge: &TypedJustificationEdge,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, edge);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     TypedEvidenceNode,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     evidence: &TypedEvidenceNode,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4(
//!         "claim", closure, evidence,
//!     );
//! }
//! ```
//!
//! Status, ceiling, threshold, policy ID, and precedence cannot be selected:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! use magpie_log::Status;
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     status: Status,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, status);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! use magpie_log::Status;
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     ceiling: Option<Status>,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4(
//!         "claim", closure, ceiling,
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     threshold: usize,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4(
//!         "claim", closure, threshold,
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     policy_id: &str,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4(
//!         "claim", closure, policy_id,
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! use magpie_log::Status;
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     precedence: &[Status],
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4(
//!         "claim", closure, precedence,
//!     );
//! }
//! ```
//!
//! Callbacks and serialized bytes cannot be supplied:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     callback: fn(),
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4(
//!         "claim", closure, callback,
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     bytes: &[u8],
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, bytes);
//! }
//! ```
//!
//! Alternate snapshots and prefix identities cannot be supplied:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     StandingReplaySnapshot,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     snapshot: &StandingReplaySnapshot,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4(
//!         "claim", closure, snapshot,
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     VerifiedLogPrefixIdentityV0,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     prefix: &VerifiedLogPrefixIdentityV0,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4(
//!         "claim", closure, prefix,
//!     );
//! }
//! ```
//!
//! Existing audit objects cannot be reinjected:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     SupportContributionAuditV0,
//! };
//! fn inject(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     audit: &SupportContributionAuditV0,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v4("claim", closure, audit);
//! }
//! ```
//!
//! There is no free resolver, no `StandingView` or snapshot method, no latest
//! alias, and no public composer or raw-candidate resolver:
//!
//! ```compile_fail
//! use magpie_claims::ResolutionContentClosureV0;
//! fn resolve(closure: &ResolutionContentClosureV0) {
//!     let _ = magpie_claims::resolved_standing_with_trace_v4("claim", closure);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{ResolutionContentClosureV0, StandingView};
//! fn resolve(view: &StandingView, closure: &ResolutionContentClosureV0) {
//!     let _ = view.resolved_standing_with_trace_v4("claim", closure);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ResolutionContentClosureV0, StandingReplaySnapshot,
//! };
//! fn resolve(
//!     snapshot: &StandingReplaySnapshot,
//!     closure: &ResolutionContentClosureV0,
//! ) {
//!     let _ = snapshot.resolved_standing_with_trace_v4("claim", closure);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//! ) {
//!     let _ = context.resolved_standing_latest("claim", closure);
//! }
//! ```
//!
//! ```compile_fail
//! let _ = magpie_claims::compose_standing_resolution_v4;
//! ```
//!
//! ```compile_fail
//! use magpie_claims::OriginAdmissionReplayContextV0;
//! fn resolve(context: &OriginAdmissionReplayContextV0) {
//!     let _ = context.resolve_claim_inline_predicate_negative_candidate_v4(
//!         "claim", "evidence", "edge",
//!     );
//! }
//! ```
//!
//! No production test-staging API is public:
//!
//! ```compile_fail
//! let _ = magpie_claims::stage_standing_resolution_v4_for_tests;
//! ```

use magpie_log::Status;
use serde::Serialize;

use crate::claim_inline_sha256_predicate::{
    evaluate_resolved_claim_inline_sha256_relation_v0,
    resolve_claim_inline_predicate_negative_candidate_v4, resolve_claim_inline_subject_v0,
    ResolvedClaimInlinePredicateNegativeCandidateV4,
};
use crate::origin_admission_replay::{OriginAdmissionReplayContextV0, VerifiedLogPrefixIdentityV0};
use crate::resolution_content_closure::{
    ResolutionContentClosureIdentityV0, ResolutionContentClosureV0,
};
use crate::standing::StandingCurrentness;
use crate::standing_v3::{StandingBlockerV3, StandingResolutionV3, MAGPIE_CLAIMS_POLICY_V3_ID};
use crate::{
    ClaimInlinePredicateEdgeLaneFailureV0, ClaimInlinePredicateEdgeLaneIneligibleReasonV0,
    ClaimInlineSubjectResolutionFailureV0, InlinePredicateAttestationBindingFailureV0,
    StandingReplaySnapshot,
};

pub const MAGPIE_CLAIMS_POLICY_V4_ID: &str = "magpie-claims-standing-v4";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StandingPolicyRuleV4 {
    Sha256ClaimInlineBytesDirectRefutationV0,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StandingPolicyContextV4 {
    UncontestedRefutationEligible,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StandingTraceClassificationV4 {
    RefutationEligible,
    Ineligible,
    ResolutionFailed,
}

#[derive(Clone, Serialize)]
pub struct StandingTraceEntryV4 {
    edge_id: String,
    evidence_id: String,
    classification: StandingTraceClassificationV4,
    ineligible_reason: Option<ClaimInlinePredicateEdgeLaneIneligibleReasonV0>,
    failure: Option<ClaimInlinePredicateEdgeLaneFailureV0>,
}

impl StandingTraceEntryV4 {
    pub fn edge_id(&self) -> &str {
        &self.edge_id
    }

    pub fn evidence_id(&self) -> &str {
        &self.evidence_id
    }

    pub fn classification(&self) -> StandingTraceClassificationV4 {
        self.classification
    }

    pub fn ineligible_reason(&self) -> Option<ClaimInlinePredicateEdgeLaneIneligibleReasonV0> {
        self.ineligible_reason
    }

    pub fn failure(&self) -> Option<&ClaimInlinePredicateEdgeLaneFailureV0> {
        self.failure.as_ref()
    }

    fn refutation_eligible(candidate: &StandingNegativeCandidateV4) -> Self {
        Self {
            edge_id: candidate.edge_id.clone(),
            evidence_id: candidate.evidence_id.clone(),
            classification: StandingTraceClassificationV4::RefutationEligible,
            ineligible_reason: None,
            failure: None,
        }
    }

    fn ineligible(
        candidate: &StandingNegativeCandidateV4,
        reason: ClaimInlinePredicateEdgeLaneIneligibleReasonV0,
    ) -> Self {
        debug_assert_eq!(
            reason,
            ClaimInlinePredicateEdgeLaneIneligibleReasonV0::DigestEqualDoesNotRefute
        );
        Self {
            edge_id: candidate.edge_id.clone(),
            evidence_id: candidate.evidence_id.clone(),
            classification: StandingTraceClassificationV4::Ineligible,
            ineligible_reason: Some(reason),
            failure: None,
        }
    }

    fn resolution_failed(
        candidate: &StandingNegativeCandidateV4,
        failure: ClaimInlinePredicateEdgeLaneFailureV0,
    ) -> Self {
        Self {
            edge_id: candidate.edge_id.clone(),
            evidence_id: candidate.evidence_id.clone(),
            classification: StandingTraceClassificationV4::ResolutionFailed,
            ineligible_reason: None,
            failure: Some(failure),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct StandingPolicyApplicationV4 {
    rule: StandingPolicyRuleV4,
    context: StandingPolicyContextV4,
    achieved_standing: Option<Status>,
}

impl StandingPolicyApplicationV4 {
    pub fn rule(&self) -> &StandingPolicyRuleV4 {
        &self.rule
    }

    pub fn context(&self) -> &StandingPolicyContextV4 {
        &self.context
    }

    pub fn achieved_standing(&self) -> Option<Status> {
        self.achieved_standing
    }

    fn direct_refutation() -> Self {
        Self {
            rule: StandingPolicyRuleV4::Sha256ClaimInlineBytesDirectRefutationV0,
            context: StandingPolicyContextV4::UncontestedRefutationEligible,
            achieved_standing: Some(Status::Refuted),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StandingBlockerV4 {
    InheritedV3ResolutionFailed,
    InheritedV3NotExtensionSafe { blocker: StandingBlockerV3 },
    InheritedPositiveStandingRequiresContradictionPolicy { inherited_status: Status },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StandingResolutionFailureV4 {
    InheritedClaimMismatch,
    InheritedPolicyMismatch,
    VerifiedPrefixIdentityMismatch,
    ClosureIdentityMismatch,
}

#[derive(Clone, Serialize)]
pub struct StandingResolutionV4 {
    claim_id: String,
    governed_standing: Option<Status>,
    legacy_raw_standing: Option<Status>,
    currentness: StandingCurrentness,
    policy_id: &'static str,
    verified_prefix_identity: VerifiedLogPrefixIdentityV0,
    closure_identity: ResolutionContentClosureIdentityV0,
    inherited_v3: StandingResolutionV3,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolution_failure: Option<StandingResolutionFailureV4>,
    candidate_trace: Vec<StandingTraceEntryV4>,
    applications: Vec<StandingPolicyApplicationV4>,
    blockers: Vec<StandingBlockerV4>,
}

impl StandingResolutionV4 {
    pub fn claim_id(&self) -> &str {
        &self.claim_id
    }

    pub fn governed_standing(&self) -> Option<Status> {
        self.governed_standing
    }

    pub fn legacy_raw_standing(&self) -> Option<Status> {
        self.legacy_raw_standing
    }

    pub fn currentness(&self) -> StandingCurrentness {
        self.currentness
    }

    pub fn policy_id(&self) -> &str {
        self.policy_id
    }

    pub fn verified_prefix_identity(&self) -> &VerifiedLogPrefixIdentityV0 {
        &self.verified_prefix_identity
    }

    pub fn closure_identity(&self) -> &ResolutionContentClosureIdentityV0 {
        &self.closure_identity
    }

    pub fn inherited_v3(&self) -> &StandingResolutionV3 {
        &self.inherited_v3
    }

    pub fn resolution_failure(&self) -> Option<&StandingResolutionFailureV4> {
        self.resolution_failure.as_ref()
    }

    pub fn candidate_trace(&self) -> &[StandingTraceEntryV4] {
        &self.candidate_trace
    }

    pub fn applications(&self) -> &[StandingPolicyApplicationV4] {
        &self.applications
    }

    pub fn blockers(&self) -> &[StandingBlockerV4] {
        &self.blockers
    }

    /// Deterministic one-way audit bytes under
    /// `magpie-claims-standing-v4-resolution-json-v0`.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("StandingResolutionV4 is always serializable")
    }
}

#[derive(Clone)]
struct StandingNegativeCandidateV4 {
    edge_id: String,
    evidence_id: String,
}

pub(crate) struct StandingCandidateDerivationV4 {
    trace: Vec<StandingTraceEntryV4>,
    has_eligible_candidate: bool,
}

pub(crate) struct StandingV4CompositionInputV0 {
    requested_claim_id: String,
    expected_verified_prefix_identity: VerifiedLogPrefixIdentityV0,
    expected_closure_identity: ResolutionContentClosureIdentityV0,
    inherited_v3: StandingResolutionV3,
}

impl StandingV4CompositionInputV0 {
    pub(crate) fn new(
        requested_claim_id: &str,
        expected_verified_prefix_identity: VerifiedLogPrefixIdentityV0,
        expected_closure_identity: ResolutionContentClosureIdentityV0,
        inherited_v3: StandingResolutionV3,
    ) -> Self {
        Self {
            requested_claim_id: requested_claim_id.to_owned(),
            expected_verified_prefix_identity,
            expected_closure_identity,
            inherited_v3,
        }
    }
}

pub(crate) fn compose_standing_resolution_v4<DeriveCandidates>(
    input: StandingV4CompositionInputV0,
    derive_candidates: DeriveCandidates,
) -> StandingResolutionV4
where
    DeriveCandidates: FnOnce() -> StandingCandidateDerivationV4,
{
    if input.inherited_v3.claim_id().as_bytes() != input.requested_claim_id.as_bytes() {
        return failed_resolution(input, StandingResolutionFailureV4::InheritedClaimMismatch);
    }
    if input.inherited_v3.policy_id() != MAGPIE_CLAIMS_POLICY_V3_ID {
        return failed_resolution(input, StandingResolutionFailureV4::InheritedPolicyMismatch);
    }
    if input.inherited_v3.verified_prefix_identity() != &input.expected_verified_prefix_identity {
        return failed_resolution(
            input,
            StandingResolutionFailureV4::VerifiedPrefixIdentityMismatch,
        );
    }
    if input.inherited_v3.closure_identity() != &input.expected_closure_identity {
        return failed_resolution(input, StandingResolutionFailureV4::ClosureIdentityMismatch);
    }

    if input.inherited_v3.resolution_failure().is_some() {
        return preserved_resolution(input, vec![StandingBlockerV4::InheritedV3ResolutionFailed]);
    }
    if let Some(blocker) = input
        .inherited_v3
        .blockers()
        .iter()
        .find(|blocker| is_extension_unsafe_v3_blocker(blocker))
        .cloned()
    {
        return preserved_resolution(
            input,
            vec![StandingBlockerV4::InheritedV3NotExtensionSafe { blocker }],
        );
    }

    let candidate_derivation = derive_candidates();
    let has_eligible_candidate = candidate_derivation.has_eligible_candidate;
    let inherited_standing = input.inherited_v3.governed_standing();
    // Precedence preserves inherited governed Refuted without amplification.
    // Supported and Settled also remain preserved, with the explicit
    // contradiction-policy blocker when an eligible negative candidate exists;
    // only None, Open, and Conjectured may be promoted by this rule. These
    // exhaustive explicit arms deliberately avoid a wildcard so a future
    // Status variant requires an explicit policy decision at compile time.
    let (governed_standing, applications, blockers) = match inherited_standing {
        Some(Status::Refuted) => (Some(Status::Refuted), Vec::new(), Vec::new()),
        Some(inherited_status @ (Status::Supported | Status::Settled))
            if has_eligible_candidate =>
        {
            (
                Some(inherited_status),
                Vec::new(),
                vec![
                    StandingBlockerV4::InheritedPositiveStandingRequiresContradictionPolicy {
                        inherited_status,
                    },
                ],
            )
        }
        Some(inherited_status @ (Status::Supported | Status::Settled)) => {
            (Some(inherited_status), Vec::new(), Vec::new())
        }
        _inherited @ (None | Some(Status::Open | Status::Conjectured))
            if has_eligible_candidate =>
        {
            (
                Some(Status::Refuted),
                vec![StandingPolicyApplicationV4::direct_refutation()],
                Vec::new(),
            )
        }
        inherited @ (None | Some(Status::Open | Status::Conjectured)) => {
            (inherited, Vec::new(), Vec::new())
        }
    };

    StandingResolutionV4 {
        claim_id: input.requested_claim_id,
        governed_standing,
        legacy_raw_standing: input.inherited_v3.legacy_raw_standing(),
        currentness: input.inherited_v3.currentness(),
        policy_id: MAGPIE_CLAIMS_POLICY_V4_ID,
        verified_prefix_identity: input.expected_verified_prefix_identity,
        closure_identity: input.expected_closure_identity,
        inherited_v3: input.inherited_v3,
        resolution_failure: None,
        candidate_trace: candidate_derivation.trace,
        applications,
        blockers,
    }
}

fn is_extension_unsafe_v3_blocker(blocker: &StandingBlockerV3) -> bool {
    matches!(
        blocker,
        StandingBlockerV3::SupportAuditIncomplete
            | StandingBlockerV3::SupportAuditRejected
            | StandingBlockerV3::InheritedV2ResolutionFailed
    )
}

fn failed_resolution(
    input: StandingV4CompositionInputV0,
    resolution_failure: StandingResolutionFailureV4,
) -> StandingResolutionV4 {
    StandingResolutionV4 {
        claim_id: input.requested_claim_id,
        governed_standing: None,
        legacy_raw_standing: None,
        currentness: StandingCurrentness::Unknown,
        policy_id: MAGPIE_CLAIMS_POLICY_V4_ID,
        verified_prefix_identity: input.expected_verified_prefix_identity,
        closure_identity: input.expected_closure_identity,
        inherited_v3: input.inherited_v3,
        resolution_failure: Some(resolution_failure),
        candidate_trace: Vec::new(),
        applications: Vec::new(),
        blockers: Vec::new(),
    }
}

fn preserved_resolution(
    input: StandingV4CompositionInputV0,
    blockers: Vec<StandingBlockerV4>,
) -> StandingResolutionV4 {
    StandingResolutionV4 {
        claim_id: input.requested_claim_id,
        governed_standing: input.inherited_v3.governed_standing(),
        legacy_raw_standing: input.inherited_v3.legacy_raw_standing(),
        currentness: input.inherited_v3.currentness(),
        policy_id: MAGPIE_CLAIMS_POLICY_V4_ID,
        verified_prefix_identity: input.expected_verified_prefix_identity,
        closure_identity: input.expected_closure_identity,
        inherited_v3: input.inherited_v3,
        resolution_failure: None,
        candidate_trace: Vec::new(),
        applications: Vec::new(),
        blockers,
    }
}

fn enumerate_negative_candidates_v4(
    snapshot: &StandingReplaySnapshot,
    claim_id: &str,
) -> Vec<StandingNegativeCandidateV4> {
    snapshot
        .standing()
        .justification_edges
        .iter()
        .filter(|(_edge_id, edge)| {
            edge.edge_kind.as_bytes() == b"contradicts"
                && edge.target_id.as_bytes() == claim_id.as_bytes()
        })
        .map(|(edge_id, edge)| StandingNegativeCandidateV4 {
            edge_id: edge_id.clone(),
            evidence_id: edge.source_id.clone(),
        })
        .collect()
}

fn derive_candidate_trace_v4(
    snapshot: &StandingReplaySnapshot,
    claim_id: &str,
    candidates: &[StandingNegativeCandidateV4],
) -> StandingCandidateDerivationV4 {
    derive_candidate_trace_with_v4(
        candidates,
        || resolve_claim_inline_subject_v0(snapshot, claim_id),
        evaluate_resolved_claim_inline_sha256_relation_v0,
        |resolved_claim, relation, candidate| {
            resolve_claim_inline_predicate_negative_candidate_v4(
                snapshot,
                resolved_claim,
                relation,
                claim_id,
                &candidate.evidence_id,
                &candidate.edge_id,
            )
        },
    )
}

fn derive_candidate_trace_with_v4<
    ResolvedClaim,
    Relation,
    ResolveClaim,
    EvaluateRelation,
    Classify,
>(
    candidates: &[StandingNegativeCandidateV4],
    resolve_claim: ResolveClaim,
    evaluate_relation: EvaluateRelation,
    mut classify: Classify,
) -> StandingCandidateDerivationV4
where
    Relation: Copy,
    ResolveClaim: FnOnce() -> Result<ResolvedClaim, ClaimInlineSubjectResolutionFailureV0>,
    EvaluateRelation: FnOnce(&ResolvedClaim) -> Relation,
    Classify: FnMut(
        &ResolvedClaim,
        Relation,
        &StandingNegativeCandidateV4,
    ) -> Result<
        ResolvedClaimInlinePredicateNegativeCandidateV4,
        ClaimInlinePredicateEdgeLaneFailureV0,
    >,
{
    if candidates.is_empty() {
        return StandingCandidateDerivationV4 {
            trace: Vec::new(),
            has_eligible_candidate: false,
        };
    }

    let resolved_claim = match resolve_claim() {
        Ok(resolved_claim) => resolved_claim,
        Err(failure) => {
            let failure = ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
                InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(failure),
            );
            let trace = candidates
                .iter()
                .map(|candidate| StandingTraceEntryV4::resolution_failed(candidate, failure))
                .collect();
            return StandingCandidateDerivationV4 {
                trace,
                has_eligible_candidate: false,
            };
        }
    };
    let relation = evaluate_relation(&resolved_claim);
    let mut has_eligible_candidate = false;
    let trace = candidates
        .iter()
        .map(
            |candidate| match classify(&resolved_claim, relation, candidate) {
                Ok(ResolvedClaimInlinePredicateNegativeCandidateV4::RefutationEligible) => {
                    has_eligible_candidate = true;
                    StandingTraceEntryV4::refutation_eligible(candidate)
                }
                Ok(ResolvedClaimInlinePredicateNegativeCandidateV4::Ineligible(reason)) => {
                    StandingTraceEntryV4::ineligible(candidate, reason)
                }
                Err(failure) => StandingTraceEntryV4::resolution_failed(candidate, failure),
            },
        )
        .collect();
    StandingCandidateDerivationV4 {
        trace,
        has_eligible_candidate,
    }
}

impl OriginAdmissionReplayContextV0 {
    /// Resolve one claim under the explicitly selected standing policy v4.
    pub fn resolved_standing_v4(
        &self,
        claim_id: &str,
        closure: &ResolutionContentClosureV0,
    ) -> Option<Status> {
        self.resolved_standing_with_trace_v4(claim_id, closure)
            .governed_standing()
    }

    /// Resolve one claim with the complete deterministic policy-v4 trace.
    pub fn resolved_standing_with_trace_v4(
        &self,
        claim_id: &str,
        closure: &ResolutionContentClosureV0,
    ) -> StandingResolutionV4 {
        let inherited_v3 = self.resolved_standing_with_trace_v3(claim_id, closure);
        let input = StandingV4CompositionInputV0::new(
            claim_id,
            self.verified_prefix_identity().clone(),
            closure.identity().clone(),
            inherited_v3,
        );
        compose_standing_resolution_v4(input, || {
            let candidates = enumerate_negative_candidates_v4(self.snapshot(), claim_id);
            derive_candidate_trace_v4(self.snapshot(), claim_id, &candidates)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use sha2::{Digest, Sha256};

    use super::*;
    use crate::admitted_contribution_audit::ADMITTED_CONTRIBUTION_POLICY_ID_V0;
    use crate::origin_admission_audit::ORIGIN_ADMISSION_POLICY_ID_V0;
    use crate::origin_admission_replay::stage_verified_log_prefix_identity_v0_for_tests;
    use crate::resolution_content_closure::stage_resolution_content_closure_identity_v0_for_tests;
    use crate::standing_v2::{StandingResolutionFailureV2, MAGPIE_CLAIMS_POLICY_V2_ID};
    use crate::standing_v3::{
        compose_standing_resolution_v3, stage_standing_resolution_v3_policy_id_for_tests,
        StandingResolutionFailureV3, StandingV3CompositionInputV0,
    };
    use crate::support_contribution_audit::{
        stage_support_contribution_audit_v0_for_tests, SupportContributionAuditCompletionV0,
        SupportContributionCompositionFailureV0,
        SUPPORT_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0,
        SUPPORT_CONTRIBUTION_AUDIT_SCHEMA_V0, SUPPORT_CONTRIBUTION_POLICY_ID_V0,
    };
    use crate::{StandingResolutionV2, StandingTraceReason};

    fn prefix(digit: char) -> VerifiedLogPrefixIdentityV0 {
        stage_verified_log_prefix_identity_v0_for_tests(7, &digit.to_string().repeat(64))
    }

    fn closure(digit: char) -> ResolutionContentClosureIdentityV0 {
        stage_resolution_content_closure_identity_v0_for_tests(&digit.to_string().repeat(64))
    }

    fn staged_v2(
        claim_id: &str,
        governed_standing: Option<Status>,
        legacy_raw_standing: Option<Status>,
        currentness: StandingCurrentness,
        resolution_failure: Option<StandingResolutionFailureV2>,
    ) -> StandingResolutionV2 {
        StandingResolutionV2 {
            claim_id: claim_id.to_owned(),
            governed_standing,
            legacy_raw_standing,
            currentness,
            policy_id: MAGPIE_CLAIMS_POLICY_V2_ID,
            resolution_failure,
            trace: Vec::new(),
            blockers: Vec::<StandingTraceReason>::new(),
        }
    }

    fn staged_v3_with(
        requested_claim_id: &str,
        inherited_v2: StandingResolutionV2,
        expected_prefix: VerifiedLogPrefixIdentityV0,
        expected_closure: ResolutionContentClosureIdentityV0,
        completion: SupportContributionAuditCompletionV0,
    ) -> StandingResolutionV3 {
        let audit = stage_support_contribution_audit_v0_for_tests(
            SUPPORT_CONTRIBUTION_AUDIT_SCHEMA_V0,
            SUPPORT_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0,
            SUPPORT_CONTRIBUTION_POLICY_ID_V0,
            ADMITTED_CONTRIBUTION_POLICY_ID_V0,
            ORIGIN_ADMISSION_POLICY_ID_V0,
            expected_prefix.clone(),
            expected_closure.clone(),
            completion,
            Vec::new(),
        );
        compose_standing_resolution_v3(StandingV3CompositionInputV0::new(
            requested_claim_id,
            None,
            expected_prefix,
            expected_closure,
            inherited_v2,
            audit,
        ))
    }

    fn complete_v3(
        claim_id: &str,
        governed_standing: Option<Status>,
        legacy_raw_standing: Option<Status>,
        currentness: StandingCurrentness,
        expected_prefix: VerifiedLogPrefixIdentityV0,
        expected_closure: ResolutionContentClosureIdentityV0,
    ) -> StandingResolutionV3 {
        staged_v3_with(
            claim_id,
            staged_v2(
                claim_id,
                governed_standing,
                legacy_raw_standing,
                currentness,
                None,
            ),
            expected_prefix,
            expected_closure,
            SupportContributionAuditCompletionV0::Complete,
        )
    }

    fn candidate(edge_id: &str, evidence_id: &str) -> StandingNegativeCandidateV4 {
        StandingNegativeCandidateV4 {
            edge_id: edge_id.to_owned(),
            evidence_id: evidence_id.to_owned(),
        }
    }

    fn empty_derivation() -> StandingCandidateDerivationV4 {
        StandingCandidateDerivationV4 {
            trace: Vec::new(),
            has_eligible_candidate: false,
        }
    }

    fn eligible_derivation(candidates: &[(&str, &str)]) -> StandingCandidateDerivationV4 {
        StandingCandidateDerivationV4 {
            trace: candidates
                .iter()
                .map(|(edge_id, evidence_id)| {
                    StandingTraceEntryV4::refutation_eligible(&candidate(edge_id, evidence_id))
                })
                .collect(),
            has_eligible_candidate: true,
        }
    }

    fn ineligible_derivation() -> StandingCandidateDerivationV4 {
        StandingCandidateDerivationV4 {
            trace: vec![StandingTraceEntryV4::ineligible(
                &candidate("edge-a", "evidence-a"),
                ClaimInlinePredicateEdgeLaneIneligibleReasonV0::DigestEqualDoesNotRefute,
            )],
            has_eligible_candidate: false,
        }
    }

    fn failed_derivation(
        failure: ClaimInlinePredicateEdgeLaneFailureV0,
    ) -> StandingCandidateDerivationV4 {
        StandingCandidateDerivationV4 {
            trace: vec![StandingTraceEntryV4::resolution_failed(
                &candidate("edge-a", "evidence-a"),
                failure,
            )],
            has_eligible_candidate: false,
        }
    }

    fn compose_complete(
        claim_id: &str,
        governed_standing: Option<Status>,
        legacy_raw_standing: Option<Status>,
        currentness: StandingCurrentness,
        derivation: StandingCandidateDerivationV4,
    ) -> StandingResolutionV4 {
        let expected_prefix = prefix('1');
        let expected_closure = closure('2');
        let inherited_v3 = complete_v3(
            claim_id,
            governed_standing,
            legacy_raw_standing,
            currentness,
            expected_prefix.clone(),
            expected_closure.clone(),
        );
        compose_standing_resolution_v4(
            StandingV4CompositionInputV0::new(
                claim_id,
                expected_prefix,
                expected_closure,
                inherited_v3,
            ),
            || derivation,
        )
    }

    #[test]
    fn public_vocabulary_serialization_is_exact() {
        assert_eq!(MAGPIE_CLAIMS_POLICY_V4_ID, "magpie-claims-standing-v4");
        assert_eq!(
            serde_json::to_string(&StandingPolicyRuleV4::Sha256ClaimInlineBytesDirectRefutationV0)
                .unwrap(),
            r#""sha256_claim_inline_bytes_direct_refutation_v0""#
        );
        assert_eq!(
            serde_json::to_string(&StandingPolicyContextV4::UncontestedRefutationEligible).unwrap(),
            r#"{"kind":"uncontested_refutation_eligible"}"#
        );
        for (classification, expected) in [
            (
                StandingTraceClassificationV4::RefutationEligible,
                r#""refutation_eligible""#,
            ),
            (StandingTraceClassificationV4::Ineligible, r#""ineligible""#),
            (
                StandingTraceClassificationV4::ResolutionFailed,
                r#""resolution_failed""#,
            ),
        ] {
            assert_eq!(serde_json::to_string(&classification).unwrap(), expected);
        }
        assert_eq!(
            serde_json::to_string(&StandingBlockerV4::InheritedV3ResolutionFailed).unwrap(),
            r#"{"kind":"inherited_v3_resolution_failed"}"#
        );
        assert_eq!(
            serde_json::to_string(&StandingBlockerV4::InheritedV3NotExtensionSafe {
                blocker: StandingBlockerV3::SupportAuditRejected,
            })
            .unwrap(),
            r#"{"kind":"inherited_v3_not_extension_safe","blocker":{"kind":"support_audit_rejected"}}"#
        );
        assert_eq!(
            serde_json::to_string(
                &StandingBlockerV4::InheritedPositiveStandingRequiresContradictionPolicy {
                    inherited_status: Status::Supported,
                },
            )
            .unwrap(),
            r#"{"kind":"inherited_positive_standing_requires_contradiction_policy","inherited_status":"Supported"}"#
        );
        for (failure, expected) in [
            (
                StandingResolutionFailureV4::InheritedClaimMismatch,
                r#"{"kind":"inherited_claim_mismatch"}"#,
            ),
            (
                StandingResolutionFailureV4::InheritedPolicyMismatch,
                r#"{"kind":"inherited_policy_mismatch"}"#,
            ),
            (
                StandingResolutionFailureV4::VerifiedPrefixIdentityMismatch,
                r#"{"kind":"verified_prefix_identity_mismatch"}"#,
            ),
            (
                StandingResolutionFailureV4::ClosureIdentityMismatch,
                r#"{"kind":"closure_identity_mismatch"}"#,
            ),
        ] {
            assert_eq!(serde_json::to_string(&failure).unwrap(), expected);
        }
    }

    #[test]
    fn trace_invariants_and_zero_or_one_application_are_exact() {
        let eligible = compose_complete(
            "claim-c",
            Some(Status::Open),
            Some(Status::Open),
            StandingCurrentness::Unknown,
            eligible_derivation(&[("edge-a", "evidence-a"), ("edge-z", "evidence-z")]),
        );
        assert_eq!(eligible.governed_standing(), Some(Status::Refuted));
        assert_eq!(eligible.applications().len(), 1);
        assert_eq!(
            eligible.applications()[0].rule(),
            &StandingPolicyRuleV4::Sha256ClaimInlineBytesDirectRefutationV0
        );
        assert_eq!(
            eligible.applications()[0].context(),
            &StandingPolicyContextV4::UncontestedRefutationEligible
        );
        assert_eq!(
            eligible.applications()[0].achieved_standing(),
            Some(Status::Refuted)
        );
        assert!(eligible.candidate_trace().iter().all(|entry| {
            entry.classification() == StandingTraceClassificationV4::RefutationEligible
                && entry.ineligible_reason().is_none()
                && entry.failure().is_none()
        }));

        let ineligible = compose_complete(
            "claim-c",
            Some(Status::Open),
            Some(Status::Open),
            StandingCurrentness::Unknown,
            ineligible_derivation(),
        );
        assert_eq!(ineligible.governed_standing(), Some(Status::Open));
        assert!(ineligible.applications().is_empty());
        let entry = &ineligible.candidate_trace()[0];
        assert_eq!(
            entry.ineligible_reason(),
            Some(ClaimInlinePredicateEdgeLaneIneligibleReasonV0::DigestEqualDoesNotRefute)
        );
        assert!(entry.failure().is_none());

        let failed = compose_complete(
            "claim-c",
            Some(Status::Open),
            Some(Status::Open),
            StandingCurrentness::Unknown,
            failed_derivation(ClaimInlinePredicateEdgeLaneFailureV0::MissingEdge),
        );
        assert!(failed.applications().is_empty());
        let entry = &failed.candidate_trace()[0];
        assert_eq!(
            entry.classification(),
            StandingTraceClassificationV4::ResolutionFailed
        );
        assert!(entry.ineligible_reason().is_none());
        assert_eq!(
            entry.failure(),
            Some(&ClaimInlinePredicateEdgeLaneFailureV0::MissingEdge)
        );
    }

    #[test]
    fn candidate_derivation_resolves_once_evaluates_once_and_binds_each_candidate() {
        let candidates = vec![
            candidate("edge-a", "evidence-a"),
            candidate("edge-b", "evidence-b"),
            candidate("edge-c", "evidence-c"),
        ];
        let claim_calls = Cell::new(0);
        let relation_calls = Cell::new(0);
        let binding_calls = Cell::new(0);
        let derivation = derive_candidate_trace_with_v4(
            &candidates,
            || {
                claim_calls.set(claim_calls.get() + 1);
                Ok(())
            },
            |_| {
                relation_calls.set(relation_calls.get() + 1);
            },
            |_, _, _| {
                binding_calls.set(binding_calls.get() + 1);
                Ok(ResolvedClaimInlinePredicateNegativeCandidateV4::RefutationEligible)
            },
        );
        assert_eq!(claim_calls.get(), 1);
        assert_eq!(relation_calls.get(), 1);
        assert_eq!(binding_calls.get(), 3);
        assert!(derivation.has_eligible_candidate);
        assert_eq!(derivation.trace.len(), 3);

        let empty: Vec<StandingNegativeCandidateV4> = Vec::new();
        let empty_claim_calls = Cell::new(0);
        let empty_relation_calls = Cell::new(0);
        let empty_derivation = derive_candidate_trace_with_v4(
            &empty,
            || {
                empty_claim_calls.set(empty_claim_calls.get() + 1);
                Ok(())
            },
            |_| {
                empty_relation_calls.set(empty_relation_calls.get() + 1);
            },
            |_,
             _,
             _|
             -> Result<
                ResolvedClaimInlinePredicateNegativeCandidateV4,
                ClaimInlinePredicateEdgeLaneFailureV0,
            > { unreachable!() },
        );
        assert_eq!(empty_claim_calls.get(), 0);
        assert_eq!(empty_relation_calls.get(), 0);
        assert!(empty_derivation.trace.is_empty());
        assert!(!empty_derivation.has_eligible_candidate);

        let failed_claim_calls = Cell::new(0);
        let failed_relation_calls = Cell::new(0);
        let failed_binding_calls = Cell::new(0);
        let failed = derive_candidate_trace_with_v4(
            &candidates,
            || {
                failed_claim_calls.set(failed_claim_calls.get() + 1);
                Err::<(), _>(ClaimInlineSubjectResolutionFailureV0::MissingClaim)
            },
            |_| {
                failed_relation_calls.set(failed_relation_calls.get() + 1);
            },
            |_, _, _| {
                failed_binding_calls.set(failed_binding_calls.get() + 1);
                Ok(ResolvedClaimInlinePredicateNegativeCandidateV4::RefutationEligible)
            },
        );
        assert_eq!(failed_claim_calls.get(), 1);
        assert_eq!(failed_relation_calls.get(), 0);
        assert_eq!(failed_binding_calls.get(), 0);
        assert_eq!(failed.trace.len(), 3);
        assert!(failed.trace.iter().all(|entry| {
            entry.failure()
                == Some(
                    &ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
                        InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(
                            ClaimInlineSubjectResolutionFailureV0::MissingClaim,
                        ),
                    ),
                )
        }));
    }

    #[test]
    fn independent_candidate_failure_does_not_veto_eligible_sibling() {
        let candidates = vec![
            candidate("edge-a", "evidence-missing"),
            candidate("edge-b", "evidence-valid"),
        ];
        let derivation = derive_candidate_trace_with_v4(
            &candidates,
            || Ok(()),
            |_| (),
            |_, _, candidate| {
                if candidate.edge_id == "edge-a" {
                    Err(
                        ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
                            InlinePredicateAttestationBindingFailureV0::MissingEvidence,
                        ),
                    )
                } else {
                    Ok(ResolvedClaimInlinePredicateNegativeCandidateV4::RefutationEligible)
                }
            },
        );
        assert!(derivation.has_eligible_candidate);
        assert_eq!(
            derivation.trace[0].classification(),
            StandingTraceClassificationV4::ResolutionFailed
        );
        assert_eq!(
            derivation.trace[1].classification(),
            StandingTraceClassificationV4::RefutationEligible
        );

        let resolution = compose_complete(
            "claim-c",
            Some(Status::Conjectured),
            Some(Status::Open),
            StandingCurrentness::Unknown,
            derivation,
        );
        assert_eq!(resolution.governed_standing(), Some(Status::Refuted));
        assert_eq!(resolution.applications().len(), 1);
    }

    #[test]
    fn complete_standing_precedence_and_legacy_raw_quarantine_are_exact() {
        for inherited in [None, Some(Status::Open), Some(Status::Conjectured)] {
            let resolution = compose_complete(
                "claim-c",
                inherited,
                Some(Status::Refuted),
                StandingCurrentness::Current,
                eligible_derivation(&[("edge-a", "evidence-a")]),
            );
            assert_eq!(resolution.governed_standing(), Some(Status::Refuted));
            assert_eq!(resolution.legacy_raw_standing(), Some(Status::Refuted));
            assert_eq!(resolution.currentness(), StandingCurrentness::Current);
            assert_eq!(resolution.applications().len(), 1);
            assert!(resolution.blockers().is_empty());
        }

        for inherited in [Status::Supported, Status::Settled] {
            let resolution = compose_complete(
                "claim-c",
                Some(inherited),
                Some(Status::Refuted),
                StandingCurrentness::Unknown,
                eligible_derivation(&[("edge-a", "evidence-a")]),
            );
            assert_eq!(resolution.governed_standing(), Some(inherited));
            assert!(resolution.applications().is_empty());
            assert_eq!(
                resolution.blockers(),
                [
                    StandingBlockerV4::InheritedPositiveStandingRequiresContradictionPolicy {
                        inherited_status: inherited,
                    }
                ]
            );
        }

        let inherited_refuted = compose_complete(
            "claim-c",
            Some(Status::Refuted),
            Some(Status::Open),
            StandingCurrentness::Unknown,
            eligible_derivation(&[("edge-a", "evidence-a"), ("edge-b", "evidence-b")]),
        );
        assert_eq!(inherited_refuted.governed_standing(), Some(Status::Refuted));
        assert!(inherited_refuted.applications().is_empty());
        assert!(inherited_refuted.blockers().is_empty());
        assert_eq!(inherited_refuted.candidate_trace().len(), 2);

        for inherited in [
            None,
            Some(Status::Open),
            Some(Status::Conjectured),
            Some(Status::Supported),
            Some(Status::Settled),
            Some(Status::Refuted),
        ] {
            let resolution = compose_complete(
                "claim-c",
                inherited,
                Some(Status::Refuted),
                StandingCurrentness::Unknown,
                empty_derivation(),
            );
            assert_eq!(resolution.governed_standing(), inherited);
            assert!(resolution.applications().is_empty());
            assert!(resolution.blockers().is_empty());
        }
    }

    #[test]
    fn v3_resolution_failures_and_unsafe_blockers_stop_extension() {
        let expected_prefix = prefix('1');
        let expected_closure = closure('2');
        let failed_v3 = staged_v3_with(
            "claim-c",
            staged_v2(
                "claim-other",
                Some(Status::Conjectured),
                Some(Status::Refuted),
                StandingCurrentness::Current,
                None,
            ),
            expected_prefix.clone(),
            expected_closure.clone(),
            SupportContributionAuditCompletionV0::Complete,
        );
        assert!(failed_v3.resolution_failure().is_some());
        let candidate_calls = Cell::new(0);
        let resolution = compose_standing_resolution_v4(
            StandingV4CompositionInputV0::new(
                "claim-c",
                expected_prefix.clone(),
                expected_closure.clone(),
                failed_v3,
            ),
            || {
                candidate_calls.set(candidate_calls.get() + 1);
                empty_derivation()
            },
        );
        assert_eq!(candidate_calls.get(), 0);
        assert_eq!(
            resolution.blockers(),
            [StandingBlockerV4::InheritedV3ResolutionFailed]
        );
        assert!(resolution.candidate_trace().is_empty());

        for (completion, expected_blocker) in [
            (
                SupportContributionAuditCompletionV0::AdmittedContributionIncomplete {
                    unavailable_binding_selectors: Vec::new(),
                },
                StandingBlockerV3::SupportAuditIncomplete,
            ),
            (
                SupportContributionAuditCompletionV0::UpstreamCompositionRejected {
                    failure: SupportContributionCompositionFailureV0::UpstreamSchemaMismatch,
                },
                StandingBlockerV3::SupportAuditRejected,
            ),
        ] {
            let inherited = staged_v3_with(
                "claim-c",
                staged_v2(
                    "claim-c",
                    Some(Status::Open),
                    Some(Status::Refuted),
                    StandingCurrentness::Unknown,
                    None,
                ),
                expected_prefix.clone(),
                expected_closure.clone(),
                completion,
            );
            let resolution = compose_standing_resolution_v4(
                StandingV4CompositionInputV0::new(
                    "claim-c",
                    expected_prefix.clone(),
                    expected_closure.clone(),
                    inherited,
                ),
                || panic!("unsafe v3 blockers must stop candidate derivation"),
            );
            assert_eq!(
                resolution.blockers(),
                [StandingBlockerV4::InheritedV3NotExtensionSafe {
                    blocker: expected_blocker,
                }]
            );
        }

        let inherited_v2_failure = staged_v3_with(
            "claim-c",
            staged_v2(
                "claim-c",
                Some(Status::Conjectured),
                Some(Status::Open),
                StandingCurrentness::Unknown,
                Some(StandingResolutionFailureV2::InheritedV1CandidateMismatch { index: 0 }),
            ),
            expected_prefix.clone(),
            expected_closure.clone(),
            SupportContributionAuditCompletionV0::Complete,
        );
        assert_eq!(
            inherited_v2_failure.blockers(),
            [
                StandingBlockerV3::InsufficientDistinctOriginGroups {
                    distinct_group_count: 0,
                },
                StandingBlockerV3::InheritedV2ResolutionFailed,
            ]
        );
        let resolution = compose_standing_resolution_v4(
            StandingV4CompositionInputV0::new(
                "claim-c",
                expected_prefix.clone(),
                expected_closure.clone(),
                inherited_v2_failure,
            ),
            || panic!("inherited v2 failure must stop candidate derivation"),
        );
        assert_eq!(
            resolution.blockers(),
            [StandingBlockerV4::InheritedV3NotExtensionSafe {
                blocker: StandingBlockerV3::InheritedV2ResolutionFailed,
            }]
        );

        let first_unsafe = staged_v3_with(
            "claim-c",
            staged_v2(
                "claim-c",
                Some(Status::Conjectured),
                Some(Status::Open),
                StandingCurrentness::Unknown,
                Some(StandingResolutionFailureV2::InheritedV1CandidateMismatch { index: 0 }),
            ),
            expected_prefix.clone(),
            expected_closure.clone(),
            SupportContributionAuditCompletionV0::AdmittedContributionIncomplete {
                unavailable_binding_selectors: Vec::new(),
            },
        );
        assert_eq!(
            first_unsafe.blockers(),
            [
                StandingBlockerV3::SupportAuditIncomplete,
                StandingBlockerV3::InheritedV2ResolutionFailed,
            ]
        );
        let retained = first_unsafe.clone();
        let resolution = compose_standing_resolution_v4(
            StandingV4CompositionInputV0::new(
                "claim-c",
                expected_prefix.clone(),
                expected_closure.clone(),
                first_unsafe,
            ),
            || panic!("first unsafe blocker must stop candidate derivation"),
        );
        assert_eq!(
            resolution.blockers(),
            [StandingBlockerV4::InheritedV3NotExtensionSafe {
                blocker: StandingBlockerV3::SupportAuditIncomplete,
            }]
        );
        assert_eq!(resolution.inherited_v3().blockers(), retained.blockers());

        let extension_safe = complete_v3(
            "claim-c",
            Some(Status::Open),
            Some(Status::Open),
            StandingCurrentness::Unknown,
            expected_prefix.clone(),
            expected_closure.clone(),
        );
        assert_eq!(
            extension_safe.blockers(),
            [StandingBlockerV3::InsufficientDistinctOriginGroups {
                distinct_group_count: 0,
            }]
        );
        let resolution = compose_standing_resolution_v4(
            StandingV4CompositionInputV0::new(
                "claim-c",
                expected_prefix,
                expected_closure,
                extension_safe,
            ),
            || eligible_derivation(&[("edge-a", "evidence-a")]),
        );
        assert_eq!(resolution.governed_standing(), Some(Status::Refuted));
        assert!(resolution.blockers().is_empty());
    }

    #[test]
    fn outer_identity_first_failure_order_clears_top_level_authority() {
        let expected_prefix = prefix('1');
        let expected_closure = closure('2');

        let inherited_claim_mismatch = complete_v3(
            "claim-other",
            Some(Status::Open),
            Some(Status::Refuted),
            StandingCurrentness::Current,
            prefix('3'),
            closure('4'),
        );
        let resolution = compose_standing_resolution_v4(
            StandingV4CompositionInputV0::new(
                "claim-c",
                expected_prefix.clone(),
                expected_closure.clone(),
                inherited_claim_mismatch,
            ),
            || panic!("outer failure must stop candidate derivation"),
        );
        assert_outer_failure(
            &resolution,
            StandingResolutionFailureV4::InheritedClaimMismatch,
        );
        assert_eq!(resolution.inherited_v3().claim_id(), "claim-other");

        let mut inherited_policy_mismatch = complete_v3(
            "claim-c",
            Some(Status::Open),
            Some(Status::Refuted),
            StandingCurrentness::Current,
            prefix('3'),
            closure('4'),
        );
        stage_standing_resolution_v3_policy_id_for_tests(
            &mut inherited_policy_mismatch,
            "foreign-policy-v3",
        );
        let resolution = compose_standing_resolution_v4(
            StandingV4CompositionInputV0::new(
                "claim-c",
                expected_prefix.clone(),
                expected_closure.clone(),
                inherited_policy_mismatch,
            ),
            || panic!("outer failure must stop candidate derivation"),
        );
        assert_outer_failure(
            &resolution,
            StandingResolutionFailureV4::InheritedPolicyMismatch,
        );

        let prefix_mismatch = complete_v3(
            "claim-c",
            Some(Status::Open),
            Some(Status::Refuted),
            StandingCurrentness::Current,
            prefix('3'),
            closure('4'),
        );
        let resolution = compose_standing_resolution_v4(
            StandingV4CompositionInputV0::new(
                "claim-c",
                expected_prefix.clone(),
                expected_closure.clone(),
                prefix_mismatch,
            ),
            || panic!("outer failure must stop candidate derivation"),
        );
        assert_outer_failure(
            &resolution,
            StandingResolutionFailureV4::VerifiedPrefixIdentityMismatch,
        );

        let closure_mismatch = complete_v3(
            "claim-c",
            Some(Status::Open),
            Some(Status::Refuted),
            StandingCurrentness::Current,
            expected_prefix.clone(),
            closure('4'),
        );
        let resolution = compose_standing_resolution_v4(
            StandingV4CompositionInputV0::new(
                "claim-c",
                expected_prefix.clone(),
                expected_closure.clone(),
                closure_mismatch,
            ),
            || panic!("outer failure must stop candidate derivation"),
        );
        assert_outer_failure(
            &resolution,
            StandingResolutionFailureV4::ClosureIdentityMismatch,
        );
        assert_eq!(resolution.verified_prefix_identity(), &expected_prefix);
        assert_eq!(resolution.closure_identity(), &expected_closure);
    }

    fn assert_outer_failure(
        resolution: &StandingResolutionV4,
        expected: StandingResolutionFailureV4,
    ) {
        assert_eq!(resolution.claim_id(), "claim-c");
        assert_eq!(resolution.governed_standing(), None);
        assert_eq!(resolution.legacy_raw_standing(), None);
        assert_eq!(resolution.currentness(), StandingCurrentness::Unknown);
        assert_eq!(resolution.policy_id(), MAGPIE_CLAIMS_POLICY_V4_ID);
        assert_eq!(resolution.resolution_failure(), Some(&expected));
        assert!(resolution.candidate_trace().is_empty());
        assert!(resolution.applications().is_empty());
        assert!(resolution.blockers().is_empty());
    }

    #[test]
    fn all_claim_resolution_failures_survive_nested_in_order() {
        let failures = all_claim_failures();
        assert_eq!(failures.len(), 33);
        let spellings = [
            "missing_claim",
            "missing_typed_claim",
            "malformed_metadata",
            "missing_claim_domain",
            "wrong_type_claim_domain",
            "duplicate_claim_metadata_key",
            "unknown_claim_domain",
            "claim_domain_mismatch",
            "unknown_claim_metadata_key",
            "missing_inline_subject_descriptor",
            "wrong_type_inline_subject_descriptor",
            "duplicate_descriptor_key",
            "unknown_descriptor_key",
            "missing_schema",
            "wrong_type_schema",
            "unknown_schema",
            "missing_predicate_id",
            "wrong_type_predicate_id",
            "empty_predicate_id",
            "predicate_id_too_long",
            "invalid_predicate_id",
            "unknown_predicate_id",
            "missing_expected_sha256",
            "wrong_type_expected_sha256",
            "invalid_expected_sha256",
            "missing_subject_hex",
            "wrong_type_subject_hex",
            "subject_hex_too_long",
            "invalid_subject_hex",
            "statement_binding_mismatch",
            "missing_claim_content_hash",
            "claim_content_hash_mismatch",
            "decoded_subject_too_large",
        ];
        let candidate = candidate("edge-a", "evidence-a");
        for (index, (failure, spelling)) in failures.into_iter().zip(spellings).enumerate() {
            let derivation = derive_candidate_trace_with_v4(
                std::slice::from_ref(&candidate),
                || Err::<(), _>(failure),
                |_| unreachable!(),
                |_,
                 _,
                 _|
                 -> Result<
                    ResolvedClaimInlinePredicateNegativeCandidateV4,
                    ClaimInlinePredicateEdgeLaneFailureV0,
                > { unreachable!() },
            );
            assert!(!derivation.has_eligible_candidate);
            assert_eq!(derivation.trace.len(), 1, "failure index {index}");
            assert_eq!(
                derivation.trace[0].failure(),
                Some(
                    &ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
                        InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(
                            failure,
                        ),
                    ),
                ),
                "failure index {index}"
            );
            assert_eq!(
                serde_json::to_string(&derivation.trace[0]).unwrap(),
                format!(
                    concat!(
                        r#"{{"edge_id":"edge-a","evidence_id":"evidence-a","#,
                        r#""classification":"resolution_failed","ineligible_reason":null,"#,
                        r#""failure":{{"attestation_binding_failed":"#,
                        r#"{{"claim_predicate_resolution_failed":"{}"}}}}}}"#
                    ),
                    spelling
                ),
                "failure spelling index {index}"
            );
        }
    }

    #[test]
    fn all_attestation_binding_failures_survive_nested_in_order() {
        let failures = all_binding_failures();
        assert_eq!(failures.len(), 28);
        let spellings = [
            "claim_predicate_resolution_failed",
            "missing_evidence",
            "wrong_evidence_kind",
            "malformed_attestation_metadata",
            "duplicate_attestation_metadata_key",
            "unknown_attestation_metadata_key",
            "missing_inline_predicate_attestation",
            "wrong_type_inline_predicate_attestation",
            "duplicate_attestation_field",
            "unknown_attestation_field",
            "missing_schema",
            "wrong_type_schema",
            "unknown_schema",
            "missing_predicate_id",
            "wrong_type_predicate_id",
            "empty_predicate_id",
            "predicate_id_too_long",
            "invalid_predicate_id",
            "missing_subject_claim_id",
            "wrong_type_subject_claim_id",
            "empty_subject_claim_id",
            "missing_scope_ref",
            "wrong_type_scope_ref",
            "empty_scope_ref",
            "predicate_binding_mismatch",
            "claim_binding_mismatch",
            "evidence_scope_binding_mismatch",
            "claim_scope_binding_mismatch",
        ];
        let candidate = candidate("edge-a", "evidence-a");
        for (index, (failure, spelling)) in failures.into_iter().zip(spellings).enumerate() {
            let derivation = derive_candidate_trace_with_v4(
                std::slice::from_ref(&candidate),
                || Ok(()),
                |_| (),
                |_, _, _| {
                    Err(ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(failure))
                },
            );
            assert!(!derivation.has_eligible_candidate);
            assert_eq!(
                derivation.trace[0].failure(),
                Some(&ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(failure)),
                "failure index {index}"
            );
            let serialized_failure = serde_json::to_string(
                &ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(failure),
            )
            .unwrap();
            if index == 0 {
                assert_eq!(
                    serialized_failure,
                    r#"{"attestation_binding_failed":{"claim_predicate_resolution_failed":"missing_claim"}}"#
                );
            } else {
                assert_eq!(
                    serialized_failure,
                    format!(r#"{{"attestation_binding_failed":"{spelling}"}}"#),
                    "failure spelling index {index}"
                );
            }
        }
    }

    fn all_claim_failures() -> Vec<ClaimInlineSubjectResolutionFailureV0> {
        use ClaimInlineSubjectResolutionFailureV0 as Failure;
        vec![
            Failure::MissingClaim,
            Failure::MissingTypedClaim,
            Failure::MalformedMetadata,
            Failure::MissingClaimDomain,
            Failure::WrongTypeClaimDomain,
            Failure::DuplicateClaimMetadataKey,
            Failure::UnknownClaimDomain,
            Failure::ClaimDomainMismatch,
            Failure::UnknownClaimMetadataKey,
            Failure::MissingInlineSubjectDescriptor,
            Failure::WrongTypeInlineSubjectDescriptor,
            Failure::DuplicateDescriptorKey,
            Failure::UnknownDescriptorKey,
            Failure::MissingSchema,
            Failure::WrongTypeSchema,
            Failure::UnknownSchema,
            Failure::MissingPredicateId,
            Failure::WrongTypePredicateId,
            Failure::EmptyPredicateId,
            Failure::PredicateIdTooLong,
            Failure::InvalidPredicateId,
            Failure::UnknownPredicateId,
            Failure::MissingExpectedSha256,
            Failure::WrongTypeExpectedSha256,
            Failure::InvalidExpectedSha256,
            Failure::MissingSubjectHex,
            Failure::WrongTypeSubjectHex,
            Failure::SubjectHexTooLong,
            Failure::InvalidSubjectHex,
            Failure::StatementBindingMismatch,
            Failure::MissingClaimContentHash,
            Failure::ClaimContentHashMismatch,
            Failure::DecodedSubjectTooLarge,
        ]
    }

    fn all_binding_failures() -> Vec<InlinePredicateAttestationBindingFailureV0> {
        use InlinePredicateAttestationBindingFailureV0 as Failure;
        vec![
            Failure::ClaimPredicateResolutionFailed(
                ClaimInlineSubjectResolutionFailureV0::MissingClaim,
            ),
            Failure::MissingEvidence,
            Failure::WrongEvidenceKind,
            Failure::MalformedAttestationMetadata,
            Failure::DuplicateAttestationMetadataKey,
            Failure::UnknownAttestationMetadataKey,
            Failure::MissingInlinePredicateAttestation,
            Failure::WrongTypeInlinePredicateAttestation,
            Failure::DuplicateAttestationField,
            Failure::UnknownAttestationField,
            Failure::MissingSchema,
            Failure::WrongTypeSchema,
            Failure::UnknownSchema,
            Failure::MissingPredicateId,
            Failure::WrongTypePredicateId,
            Failure::EmptyPredicateId,
            Failure::PredicateIdTooLong,
            Failure::InvalidPredicateId,
            Failure::MissingSubjectClaimId,
            Failure::WrongTypeSubjectClaimId,
            Failure::EmptySubjectClaimId,
            Failure::MissingScopeRef,
            Failure::WrongTypeScopeRef,
            Failure::EmptyScopeRef,
            Failure::PredicateBindingMismatch,
            Failure::ClaimBindingMismatch,
            Failure::EvidenceScopeBindingMismatch,
            Failure::ClaimScopeBindingMismatch,
        ]
    }

    #[test]
    fn canonical_bytes_are_compact_ordered_and_repeatable() {
        let resolution = compose_complete(
            "claim/\"unicode-\u{e9}",
            Some(Status::Open),
            Some(Status::Open),
            StandingCurrentness::Unknown,
            empty_derivation(),
        );
        let bytes = resolution.canonical_bytes();
        assert_eq!(bytes, resolution.clone().canonical_bytes());
        assert_eq!(bytes.first(), Some(&b'{'));
        assert_eq!(bytes.last(), Some(&b'}'));
        assert!(!bytes.starts_with(&[0xef, 0xbb, 0xbf]));
        assert!(!bytes.ends_with(b"\n"));
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.contains(r#""claim_id":"claim/\"unicode-é""#));
        assert!(!text.contains("\"resolution_failure\""));
        assert!(text.contains(r#""candidate_trace":[],"applications":[],"blockers":[]}"#));
        let fields = [
            "\"claim_id\"",
            "\"governed_standing\"",
            "\"legacy_raw_standing\"",
            "\"currentness\"",
            "\"policy_id\"",
            "\"verified_prefix_identity\"",
            "\"closure_identity\"",
            "\"inherited_v3\"",
            "\"candidate_trace\"",
            "\"applications\"",
            "\"blockers\"",
        ];
        let mut cursor = 0;
        for field in fields {
            let relative = text[cursor..].find(field).unwrap();
            cursor += relative + field.len();
        }
    }

    #[test]
    fn staged_v3_failure_vocabulary_remains_complete_for_outer_gate() {
        let _ = [
            StandingResolutionFailureV3::InheritedClaimMismatch,
            StandingResolutionFailureV3::InheritedPolicyMismatch,
            StandingResolutionFailureV3::SupportAuditSchemaMismatch,
            StandingResolutionFailureV3::SupportAuditCanonicalizationProfileMismatch,
            StandingResolutionFailureV3::SupportAuditPolicyMismatch,
            StandingResolutionFailureV3::SupportAuditAdmittedPolicyMismatch,
            StandingResolutionFailureV3::SupportAuditOriginPolicyMismatch,
            StandingResolutionFailureV3::VerifiedPrefixIdentityMismatch,
            StandingResolutionFailureV3::ClosureIdentityMismatch,
        ];
    }

    const VECTOR_A: &str = r#"{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[],"applications":[],"blockers":[]}"#;
    const VECTOR_B: &str = r#"{"claim_id":"claim-c","governed_standing":"Refuted","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[{"edge_id":"edge-a","evidence_id":"evidence-a","classification":"refutation_eligible","ineligible_reason":null,"failure":null}],"applications":[{"rule":"sha256_claim_inline_bytes_direct_refutation_v0","context":{"kind":"uncontested_refutation_eligible"},"achieved_standing":"Refuted"}],"blockers":[]}"#;
    const VECTOR_C: &str = r#"{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[{"edge_id":"edge-a","evidence_id":"evidence-a","classification":"ineligible","ineligible_reason":"digest_equal_does_not_refute","failure":null}],"applications":[],"blockers":[]}"#;
    const VECTOR_D: &str = r#"{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[{"edge_id":"edge-a","evidence_id":"evidence-a","classification":"resolution_failed","ineligible_reason":null,"failure":{"attestation_binding_failed":"missing_evidence"}}],"applications":[],"blockers":[]}"#;
    const VECTOR_E: &str = r#"{"claim_id":"claim-c","governed_standing":"Refuted","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[{"edge_id":"edge-a","evidence_id":"evidence-a","classification":"refutation_eligible","ineligible_reason":null,"failure":null},{"edge_id":"edge-z","evidence_id":"evidence-z","classification":"refutation_eligible","ineligible_reason":null,"failure":null}],"applications":[{"rule":"sha256_claim_inline_bytes_direct_refutation_v0","context":{"kind":"uncontested_refutation_eligible"},"achieved_standing":"Refuted"}],"blockers":[]}"#;
    const VECTOR_F: &str = r#"{"claim_id":"claim-c","governed_standing":"Supported","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Supported","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Supported","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[{"edge_id":"edge-a","evidence_id":"evidence-a","classification":"refutation_eligible","ineligible_reason":null,"failure":null}],"applications":[],"blockers":[{"kind":"inherited_positive_standing_requires_contradiction_policy","inherited_status":"Supported"}]}"#;
    const VECTOR_G: &str = r#"{"claim_id":"claim-c","governed_standing":"Settled","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Settled","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Settled","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[{"edge_id":"edge-a","evidence_id":"evidence-a","classification":"refutation_eligible","ineligible_reason":null,"failure":null}],"applications":[],"blockers":[{"kind":"inherited_positive_standing_requires_contradiction_policy","inherited_status":"Settled"}]}"#;
    const VECTOR_H: &str = r#"{"claim_id":"claim-c","governed_standing":"Refuted","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-c","governed_standing":"Refuted","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-c","governed_standing":"Refuted","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"candidate_trace":[{"edge_id":"edge-a","evidence_id":"evidence-a","classification":"refutation_eligible","ineligible_reason":null,"failure":null}],"applications":[],"blockers":[]}"#;
    const VECTOR_I: &str = r#"{"claim_id":"claim-c","governed_standing":null,"legacy_raw_standing":null,"currentness":"unknown","policy_id":"magpie-claims-standing-v4","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v3":{"claim_id":"claim-other","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v3","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"inherited_v2":{"claim_id":"claim-other","governed_standing":"Open","legacy_raw_standing":"Open","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"support_contribution_audit":{"schema":"magpie-support-contribution-audit-v0","canonicalization_profile":"magpie-support-contribution-audit-json-v0","policy_id":"magpie-support-contribution-v0","admitted_contribution_policy_id":"magpie-admitted-contribution-v0","origin_admission_policy_id":"magpie-origin-admission-v0","verified_prefix_identity":{"event_count":7,"tip_sha256":"1111111111111111111111111111111111111111111111111111111111111111"},"closure_identity":{"schema":"magpie-resolution-content-closure-v0","canonicalization_profile":"magpie-resolution-content-closure-json-v0","digest_algorithm":"sha256","manifest_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"completion":{"outcome":"complete"},"support_contributions":[]},"lane":null,"application":null,"ignored_contributions":[],"blockers":[{"kind":"insufficient_distinct_origin_groups","distinct_group_count":0}]},"resolution_failure":{"kind":"inherited_claim_mismatch"},"candidate_trace":[],"applications":[],"blockers":[]}"#;

    fn canonical_vector_resolutions() -> Vec<(
        char,
        StandingResolutionV4,
        &'static str,
        usize,
        &'static str,
    )> {
        let outer_prefix = prefix('1');
        let outer_closure = closure('2');
        let inherited_mismatch = complete_v3(
            "claim-other",
            Some(Status::Open),
            Some(Status::Open),
            StandingCurrentness::Unknown,
            outer_prefix.clone(),
            outer_closure.clone(),
        );
        vec![
            (
                'A',
                compose_complete(
                    "claim-c",
                    Some(Status::Open),
                    Some(Status::Open),
                    StandingCurrentness::Unknown,
                    empty_derivation(),
                ),
                VECTOR_A,
                2_197,
                "3292f6ceafc1dbd16666cce56f2bdac6de7d8de5faeb36cc4495fe272e243d9a",
            ),
            (
                'B',
                compose_complete(
                    "claim-c",
                    Some(Status::Open),
                    Some(Status::Open),
                    StandingCurrentness::Unknown,
                    eligible_derivation(&[("edge-a", "evidence-a")]),
                ),
                VECTOR_B,
                2_466,
                "755aa9761363346847d54a8250cf9c82a5d0c917cf40fe250e611cc42d8e57c5",
            ),
            (
                'C',
                compose_complete(
                    "claim-c",
                    Some(Status::Open),
                    Some(Status::Open),
                    StandingCurrentness::Unknown,
                    ineligible_derivation(),
                ),
                VECTOR_C,
                2_340,
                "02b8aec71af0b4e21370ad8ba01b1c5796600e6702df1881bbf36ddd58f7e5be",
            ),
            (
                'D',
                compose_complete(
                    "claim-c",
                    Some(Status::Open),
                    Some(Status::Open),
                    StandingCurrentness::Unknown,
                    failed_derivation(
                        ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
                            InlinePredicateAttestationBindingFailureV0::MissingEvidence,
                        ),
                    ),
                ),
                VECTOR_D,
                2_366,
                "8ae9ce1ca675a954e8b01e315a2be6f94f393db4defa88aae87bbd410c816637",
            ),
            (
                'E',
                compose_complete(
                    "claim-c",
                    Some(Status::Open),
                    Some(Status::Open),
                    StandingCurrentness::Unknown,
                    eligible_derivation(&[("edge-a", "evidence-a"), ("edge-z", "evidence-z")]),
                ),
                VECTOR_E,
                2_593,
                "b1a77cb96b628a30e5e8b1d70a075835d69abbe9f07e6275b6cf43e30dc79db5",
            ),
            (
                'F',
                compose_complete(
                    "claim-c",
                    Some(Status::Supported),
                    Some(Status::Open),
                    StandingCurrentness::Unknown,
                    eligible_derivation(&[("edge-a", "evidence-a")]),
                ),
                VECTOR_F,
                2_437,
                "cab622ea9e552e14e49a5b302d0cbfca0cae778644841c64fd42343f8b049e09",
            ),
            (
                'G',
                compose_complete(
                    "claim-c",
                    Some(Status::Settled),
                    Some(Status::Open),
                    StandingCurrentness::Unknown,
                    eligible_derivation(&[("edge-a", "evidence-a")]),
                ),
                VECTOR_G,
                2_429,
                "40b633838e23754522e51a3e31f9f915537eb109b4c14087772ebceec09dfadd",
            ),
            (
                'H',
                compose_complete(
                    "claim-c",
                    Some(Status::Refuted),
                    Some(Status::Open),
                    StandingCurrentness::Unknown,
                    eligible_derivation(&[("edge-a", "evidence-a")]),
                ),
                VECTOR_H,
                2_332,
                "59e179505b97022dd4e85b49f5a24d922f39fcbfa91bc56feca861389e50f71b",
            ),
            (
                'I',
                compose_standing_resolution_v4(
                    StandingV4CompositionInputV0::new(
                        "claim-c",
                        outer_prefix,
                        outer_closure,
                        inherited_mismatch,
                    ),
                    || panic!("the outer claim mismatch must stop candidate derivation"),
                ),
                VECTOR_I,
                2_258,
                "ac1dd8a308aaf4f93dec01152409350d47f8b5008174d998de52962f4a52c800",
            ),
        ]
    }

    #[test]
    fn all_nine_ratified_canonical_vectors_match_actual_v4_values() {
        for (label, resolution, literal, expected_len, expected_sha256) in
            canonical_vector_resolutions()
        {
            let bytes = resolution.canonical_bytes();
            assert_eq!(bytes, literal.as_bytes(), "vector {label} literal");
            assert_eq!(bytes.len(), expected_len, "vector {label} length");
            assert_eq!(
                hex::encode(Sha256::digest(&bytes)),
                expected_sha256,
                "vector {label} SHA-256"
            );
            assert_eq!(bytes.first(), Some(&b'{'), "vector {label} first byte");
            assert_eq!(bytes.last(), Some(&b'}'), "vector {label} last byte");
            assert!(
                !bytes.starts_with(&[0xef, 0xbb, 0xbf]),
                "vector {label} BOM"
            );
            assert!(!bytes.ends_with(b"\n"), "vector {label} newline");
            let text = std::str::from_utf8(&bytes).unwrap();
            assert!(text.contains("\"candidate_trace\":"));
            assert!(text.contains("\"applications\":"));
            assert!(text.contains("\"blockers\":"));
            if label == 'I' {
                assert!(text.contains("\"resolution_failure\":"));
            } else {
                assert!(!text.contains("\"resolution_failure\":"));
            }
        }
    }

    #[test]
    fn emit_ratified_vectors_for_independent_hashlib_check() {
        if std::env::var_os("MAGPIE_EMIT_STANDING_V4_VECTORS").as_deref()
            != Some(std::ffi::OsStr::new("1"))
        {
            return;
        }
        for (label, resolution, _literal, _expected_len, _expected_sha256) in
            canonical_vector_resolutions()
        {
            println!(
                "MAGPIE_STANDING_V4_VECTOR_{label}_HEX={}",
                hex::encode(resolution.canonical_bytes())
            );
        }
    }

    #[test]
    fn sha256_helper_used_by_vector_tests_is_lowercase_and_exact() {
        assert_eq!(
            hex::encode(Sha256::digest(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
