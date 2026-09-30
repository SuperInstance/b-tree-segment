//! The fleet polyformalism canary.
//!
//! `b_tree_segment::fnv1a64` is FNV-1a 64, the same digest every other substrate in the
//! fleet agrees on. Asserting it here means a change to the hash — a wrong prime, an
//! offset, a signed multiply — fails this crate's own test suite rather than producing
//! digests that quietly disagree with the rest of the fleet.

#[test]
fn canary_matches_the_rest_of_the_fleet() {
    assert_eq!(
        b_tree_segment::fnv1a64("café Δ 日本語".as_bytes()),
        0x024a555471370b18d
    );
}

#[test]
fn runtime_canary_check_agrees() {
    assert!(b_tree_segment::canary_holds());
}
