// Compatibility wrapper: keep the historical `proofyloops` binary name.
//
// We `include!` the main CLI implementation so that:
// - `proofloops` and `proofyloops` behave identically
// - we avoid Cargo warnings about the same source file belonging to multiple bins
include!("proofloops.rs");
