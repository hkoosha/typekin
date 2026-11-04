use std::alloc::GlobalAlloc;
use std::cell::Cell;
use std::rc::Rc;

use crate::*;

//noinspection SpellCheckingInspection
pub(crate) const MY_WORD: &str = "safeé";

pub(crate) fn valid_owned_text(value: &str) -> bool {
    if value == "panic" {
        panic!("validator panic");
    }
    return value == "safe" || value == MY_WORD;
}

#[typekin::text(valid = valid_owned_text)]
pub(crate) struct OwnedText(String);

pub(crate) fn tracked_text(value: &str) -> (OwnedText, AllocationWatch) {
    let mut backing = String::with_capacity(64);
    backing.push_str(value);
    let text = OwnedText::try_make(backing).unwrap();
    assert!(text.capacity() >= 64);
    let watch = AllocationWatch::watch(text.as_str().as_ptr());
    return (text, watch);
}

pub(crate) struct EmptySetSource(pub(crate) String);
pub(crate) struct BlindTrustEmptySetSource(pub(crate) String);

pub(crate) fn empty_set_text(source: EmptySetSource) -> String {
    source.0
}

pub(crate) fn blind_trust_empty_set_text(
    source: BlindTrustEmptySetSource
) -> String {
    source.0
}

#[typekin::text(
    domain = {},
    friend = [
        empty_set_text(EmptySetSource) -> Make,
        blind_trust_empty_set_text(BlindTrustEmptySetSource) -> [Make, unsafe],
    ],
)]
pub(crate) struct EmptySetText(String);

#[typekin::text(domain = {"draft"}, with = with_blind_trust)]
pub(crate) struct BlindTrustTag(String);

pub(crate) fn blind_trust_tag(value: String) -> BlindTrustTag {
    BlindTrustTag::_blind_trust(value)
}

pub(crate) struct DropProbe(pub(crate) Rc<Cell<usize>>);

impl DropProbe {
    pub(crate) fn assert_live(&self) {
        assert_eq!(self.0.get(), 0);
    }
}

impl Drop for DropProbe {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

// =============================================================================

pub(crate) fn is_slug(value: &str) -> bool {
    return !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-');
}

pub(crate) fn is_not_reserved(value: &str) -> bool {
    return value != "reserved";
}

pub(crate) struct SlugSource(pub(crate) String);
pub(crate) struct BlindTrustSlugSource(pub(crate) String);

impl SlugSource {
    pub(crate) fn into_string(self) -> String {
        return self.0;
    }
}

impl BlindTrustSlugSource {
    pub(crate) fn into_string(self) -> String {
        return self.0;
    }
}

#[typekin::text(
    domain = {"draft", "published"},
    friend = [
        SlugSource::into_string(SlugSource) -> Make,
        BlindTrustSlugSource::into_string(BlindTrustSlugSource) -> unsafe,
    ],
)]
pub(crate) struct Slug(String);

#[typekin::text]
pub(crate) struct FullyGeneratedText(String);

#[typekin::text(without = [clone, repr])]
#[repr(transparent)]
#[derive(Clone)]
pub(crate) struct ManuallyDerivedText(String);

#[typekin::text(
    friend = SlugSource::into_string(SlugSource) -> Make,
    valid = is_slug,
)]
pub(crate) struct CallbackText(String);

#[typekin::text(valid = [is_slug, is_not_reserved])]
pub(crate) struct CallbackChainText(String);

#[typekin::text(domain = {"", "é"})]
pub(crate) struct LiteralText(String);

#[typekin::text(friend = self(Self) -> [Rel])]
pub(crate) struct DisplayText(String);

#[typekin::text(without = display)]
pub(crate) struct NoDisplayText(String);

impl core::fmt::Display for NoDisplayText {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        return formatter.write_str(self.as_str());
    }
}

thread_local! {
    static MUTATION_CALLS: Cell<usize> = const { Cell::new(0) };
}

pub(crate) fn reset() {
    MUTATION_CALLS.set(0);
}
pub(crate) fn inc() {
    MUTATION_CALLS.update(|it| it + 1);
}
pub(crate) fn require(it: usize) {
    MUTATION_CALLS.with(|v| assert_eq!(v.get(), it));
}

pub(crate) fn validate_mutation(value: &str) -> bool {
    inc();
    return !value.is_empty() && value != "b" && !value.contains('!');
}

#[typekin::text(valid = validate_mutation)]
pub(crate) struct CheckedMutationText(String);

pub(crate) fn validate_empty_mutation(value: &str) -> bool {
    inc();
    return value.is_empty();
}

#[typekin::text(valid = validate_empty_mutation)]
pub(crate) struct CheckedEmptyText(String);

#[typekin::text(domain = {"a", "ab", "abc"})]
pub(crate) struct MemberMutationText(String);

#[typekin::text(valid = is_not_reserved)]
pub(crate) struct MutationAllocationText(String);

pub(crate) fn is_serde_text_valid(value: &str) -> bool {
    return value == "valid";
}

#[typekin::text(
    valid = is_serde_text_valid,
    with = serde,
)]
pub(crate) struct SerdeText(String);

#[typekin::text(
    valid = is_slug,
    friend = Self::into_inner(Self) -> [Rel, unsafe],
)]
pub(crate) struct RelatedText(String);

pub(crate) struct CheckedSource(pub(crate) String);
pub(crate) struct BlindTrustSource(pub(crate) String);

pub(crate) fn checked_text(source: CheckedSource) -> String {
    source.0
}

pub(crate) fn blind_trust_text(source: BlindTrustSource) -> String {
    source.0
}

#[typekin::text(
    domain = {},
    friend = [
        checked_text(CheckedSource) -> Make,
        blind_trust_text(BlindTrustSource) -> [unsafe],
    ],
)]
pub(crate) struct EmptyDomain(String);

#[typekin::text(domain = {})]
pub(crate) struct CallbackEmptyDomain(String);

#[typekin::text(domain = {""})]
pub(crate) struct EmptyStringOnly(String);

#[typekin::text(konst = none, valid = is_slug)]
pub(crate) struct ValidatedSlug(String);

pub(crate) struct UnlistedSource(pub(crate) String);

pub(crate) fn unlisted_text(source: UnlistedSource) -> String {
    return source.0;
}

#[typekin::text(friend = unlisted_text(UnlistedSource))]
pub(crate) struct CapabilityFreeFriendText(String);

#[typekin::text(friend = [], valid = is_slug)]
pub(crate) struct NoFriendText(String);

#[typekin::text(friend = [_(Self) -> Rel])]
pub(crate) struct SelfRelText(String);

pub(crate) fn self_make_text(text: SelfMakeText) -> String {
    return text.into_inner();
}

#[typekin::text(
    valid = is_slug,
    friend = [self_make_text(Self,) -> [Rel, unsafe]],
)]
pub(crate) struct SelfMakeText(String);

#[typekin::text(error = (), valid = is_slug)]
pub(crate) struct UnitErrorText(String);

#[typekin::text(without = [serde])]
pub(crate) struct NoSerdeText(String);

#[typekin::text(with = display)]
pub(crate) struct SingleDisplayFlagText(String);

#[typekin::text(with = [clone, debug, hash, repr])]
pub(crate) struct ExplicitTraitsText(String);

#[typekin::text(without = [clone, debug, hash, repr])]
pub(crate) struct NoTraitsText(String);

#[typekin::text(konst = stable)]
pub(crate) struct StableKonstText(String);

#[typekin::text(konst = nightly)]
pub(crate) struct NightlyKonstText(String);

#[typekin::text(
    konst = nightly,
    with = [konst_text_access, konst_text_from],
    without = konst_text,
)]
pub(crate) struct SpecificKonstText(String);

#[typekin::text(without = [fn_try_push, fn_len])]
pub(crate) struct WithoutSomeTextFns(String);

#[typekin::text(
    with = serde,
    without = [
        fn_try_push,
        fn_try_push_str,
        fn_try_insert,
        fn_try_insert_str,
        fn_try_replace_range,
        fn_try_truncate,
        fn_try_clear,
        fn_try_retain,
        fn_try_remove,
        fn_try_pop,
        fn_into_bytes,
        fn_as_str,
        fn_as_bytes,
        fn_len,
        fn_is_empty,
        fn_capacity,
    ],
)]
pub(crate) struct WithoutTextFns(String);
