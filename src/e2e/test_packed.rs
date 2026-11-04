use crate::fx_packed::*;
use crate::*;

/// `make`, getters, and setters store each field in its own window.
#[cfg(test)]
mod accessors {
    use super::*;

    #[test]
    fn every_level_round_trips() {
        for raw in 1..=5u8 {
            let level = Level::try_make(raw).unwrap();
            let record = Gapped::make(GappedValues {
                level,
                offset: Offset::try_make(0).unwrap(),
                flag: false,
                tag: Tag::try_make(0).unwrap(),
            });

            assert_eq!(record.level(), level, "{raw}");
            assert_eq!(record.fields().level, level, "{raw}");
        }
    }

    #[test]
    fn every_offset_round_trips() {
        for raw in -8..=7i8 {
            let offset = Offset::try_make(raw).unwrap();
            let record = Gapped::make(GappedValues {
                level: Level::try_make(1).unwrap(),
                offset,
                flag: false,
                tag: Tag::try_make(0).unwrap(),
            });

            assert_eq!(record.offset(), offset, "{raw}");
            assert_eq!(record.fields().offset, offset, "{raw}");
        }
    }

    #[test]
    fn every_flag_round_trips() {
        for flag in [
            false,
            true,
        ] {
            let record = Gapped::make(GappedValues {
                level: Level::try_make(1).unwrap(),
                offset: Offset::try_make(0).unwrap(),
                flag,
                tag: Tag::try_make(0).unwrap(),
            });

            assert_eq!(record.flag(), flag, "{flag}");
            assert_eq!(record.fields().flag, flag, "{flag}");
        }
    }

    #[test]
    fn every_tag_round_trips() {
        for raw in 0..64u8 {
            let tag = Tag::try_make(raw).unwrap();
            let record = Gapped::make(GappedValues {
                level: Level::try_make(1).unwrap(),
                offset: Offset::try_make(0).unwrap(),
                flag: false,
                tag,
            });

            assert_eq!(record.tag(), tag, "{raw}");
            assert_eq!(record.fields().tag, tag, "{raw}");
        }
    }

    #[test]
    fn make_places_fields_at_their_bits() {
        let record = gapped_sample();

        assert_eq!(record.raw(), 42 << 10 | 1 << 7 | 0b1110 << 3 | 3);
    }

    #[test]
    fn with_level_changes_only_its_window() {
        let record = gapped_sample();
        for raw in 1..=5u8 {
            let level = Level::try_make(raw).unwrap();
            let changed = record.with_level(level);

            assert_eq!(changed.level(), level, "{raw}");
            assert_eq!(
                changed.raw() & !GAPPED_LEVEL_MASK,
                record.raw() & !GAPPED_LEVEL_MASK,
                "{raw}"
            );
        }
    }

    #[test]
    fn with_offset_changes_only_its_window() {
        let record = gapped_sample();
        for raw in -8..=7i8 {
            let offset = Offset::try_make(raw).unwrap();
            let changed = record.with_offset(offset);

            assert_eq!(changed.offset(), offset, "{raw}");
            assert_eq!(
                changed.raw() & !GAPPED_OFFSET_MASK,
                record.raw() & !GAPPED_OFFSET_MASK,
                "{raw}"
            );
        }
    }

    #[test]
    fn with_flag_changes_only_its_window() {
        let record = gapped_sample();
        for flag in [
            false,
            true,
        ] {
            let changed = record.with_flag(flag);

            assert_eq!(changed.flag(), flag, "{flag}");
            assert_eq!(
                changed.raw() & !GAPPED_FLAG_MASK,
                record.raw() & !GAPPED_FLAG_MASK,
                "{flag}"
            );
        }
    }

    #[test]
    fn with_tag_changes_only_its_window() {
        let record = gapped_sample();
        for raw in 0..64u8 {
            let tag = Tag::try_make(raw).unwrap();
            let changed = record.with_tag(tag);

            assert_eq!(changed.tag(), tag, "{raw}");
            assert_eq!(
                changed.raw() & !GAPPED_TAG_MASK,
                record.raw() & !GAPPED_TAG_MASK,
                "{raw}"
            );
        }
    }

    #[test]
    fn values_name_renames_the_values_struct() {
        let level = Level::try_make(4).unwrap();
        let flag = true;
        let record = Renamed::make(RenamedParts { level, flag });
        let RenamedParts {
            level: read_level,
            flag: read_flag,
        } = record.fields();

        assert_eq!(read_level, level);
        assert_eq!(read_flag, flag);
    }
}

/// `try_make` accepts exactly the canonical raw values.
#[cfg(test)]
mod validation {
    use super::*;

    #[test]
    fn try_make_agrees_with_the_field_and_gap_predicate() {
        for raw in u16::MIN..=u16::MAX {
            let this = Gapped::try_make(raw);

            assert_eq!(this.is_ok(), gapped_is_valid(raw), "{raw:#06x}");
        }
    }

    #[test]
    fn try_make_returns_the_rejected_raw_value() {
        let raw = 0x0100 | 1;
        let this = Gapped::try_make(raw);

        assert_eq!(this.unwrap_err(), raw);
    }

    #[test]
    fn try_make_keeps_an_accepted_raw_value() {
        let raw = gapped_sample().raw();
        let this = Gapped::try_make(raw);

        assert!(this.is_ok(), "{raw:#06x}");
        assert_eq!(this.unwrap().raw(), raw);
    }
}

/// Signed fields store their low bits and sign-extend on read.
#[cfg(test)]
mod signed {
    use super::*;

    #[test]
    fn signed_wrapper_reads_its_window_minimum() {
        let offset = -8;
        let record = signed_of(offset, 0);

        assert_eq!(record.offset().raw(), offset);
        assert_eq!(record.raw() & 0xF, 0b1000);
    }

    #[test]
    fn signed_wrapper_reads_minus_one() {
        let offset = -1;
        let record = signed_of(offset, 0);

        assert_eq!(record.offset().raw(), offset);
        assert_eq!(record.raw() & 0xF, 0b1111);
    }

    #[test]
    fn signed_wrapper_reads_zero() {
        let offset = 0;
        let record = signed_of(offset, -1);

        assert_eq!(record.offset().raw(), offset);
        assert_eq!(record.raw() & 0xF, 0);
    }

    #[test]
    fn signed_wrapper_reads_its_window_maximum() {
        let offset = 7;
        let record = signed_of(offset, -1);

        assert_eq!(record.offset().raw(), offset);
        assert_eq!(record.raw() & 0xF, 0b0111);
    }

    #[test]
    fn signed_primitive_reads_its_minimum() {
        let byte = i8::MIN;
        let record = signed_of(0, byte);

        assert_eq!(record.byte(), byte);
        assert_eq!(record.raw(), 0x80 << 4);
    }

    #[test]
    fn signed_primitive_reads_minus_one() {
        let byte = -1;
        let record = signed_of(0, byte);

        assert_eq!(record.byte(), byte);
        assert_eq!(record.raw(), 0xFF << 4);
    }

    #[test]
    fn signed_primitive_reads_zero() {
        let byte = 0;
        let record = signed_of(-1, byte);

        assert_eq!(record.byte(), byte);
        assert_eq!(record.raw(), 0xF);
    }

    #[test]
    fn signed_primitive_reads_its_maximum() {
        let byte = i8::MAX;
        let record = signed_of(-1, byte);

        assert_eq!(record.byte(), byte);
        assert_eq!(record.raw(), 0x7F << 4 | 0xF);
    }

    #[test]
    fn signed_raw_bits_sign_extend_through_try_make() {
        let raw = 0x0FF8;
        let this = Signed::try_make(raw);

        assert!(this.is_ok(), "{raw:#06x}");
        let record = this.unwrap();
        assert_eq!(record.offset().raw(), -8);
        assert_eq!(record.byte(), -1);
    }
}

/// Typekin types as fields.
#[cfg(test)]
mod nesting {
    use super::*;

    #[test]
    fn nested_record_round_trips_in_a_full_width_window() {
        let inner = Inner::make(InnerValues {
            offset: Offset::try_make(-3).unwrap(),
            byte: -100,
            flag: true,
        });
        let outer = Outer::make(OuterValues {
            inner,
            access: AccessValue::empty(),
            low: 0xAB,
        });

        assert_eq!(outer.inner(), inner);
        assert_eq!(outer.raw() >> 16, u32::from(inner.raw()));
        assert_eq!(outer.low(), 0xAB);
    }

    #[test]
    fn nested_record_setter_keeps_other_fields() {
        let first = Inner::try_make(0x8001).unwrap();
        let second = Inner::try_make(0x0FF7).unwrap();
        let outer = Outer::make(OuterValues {
            inner: first,
            access: Access::Write.into_value(),
            low: 0x5A,
        });
        let changed = outer.with_inner(second);

        assert_eq!(changed.inner(), second);
        assert_eq!(changed.raw() & 0xFFFF, outer.raw() & 0xFFFF);
    }

    #[test]
    fn every_bitflag_value_round_trips_in_a_narrow_window() {
        for bits in 0..=0b111u8 {
            let access = AccessValue::try_make(bits).unwrap();
            let outer = Outer::make(OuterValues {
                inner: Inner::try_make(0).unwrap(),
                access,
                low: 0xFF,
            });

            assert_eq!(outer.access(), access, "{bits}");
            assert_eq!(outer.raw(), 0xFF << 8 | u32::from(bits), "{bits}");
        }
    }

    #[test]
    fn invalid_nested_record_bits_are_rejected() {
        let raw = 0x1000 << 16;
        let this = Outer::try_make(raw);

        assert_eq!(this.unwrap_err(), raw);
    }

    #[test]
    fn invalid_bitflag_bits_are_rejected() {
        let raw = 0b1000;
        let this = Outer::try_make(raw);

        assert_eq!(this.unwrap_err(), raw);
    }
}

/// Storage size and niche.
#[cfg(test)]
mod layout {
    use super::*;

    #[test]
    fn default_record_has_the_size_of_its_repr() {
        assert_eq!(size_of::<Gapped>(), size_of::<u16>());
        assert_eq!(size_of::<Outer>(), size_of::<u32>());
    }

    #[test]
    fn non_zero_record_has_a_niche() {
        assert_eq!(
            size_of::<Option<NonZeroRecord>>(),
            size_of::<NonZeroRecord>()
        );
        assert_eq!(size_of::<NonZeroRecord>(), size_of::<u16>());
    }

    #[test]
    fn non_zero_record_rejects_zero() {
        let raw = 0;
        let this = NonZeroRecord::try_make(raw);

        assert_eq!(this.unwrap_err(), raw);
    }

    #[test]
    fn non_zero_record_round_trips() {
        let id = Id::try_make(1).unwrap();
        let data = 0;
        let record = NonZeroRecord::make(NonZeroRecordValues { id, data });

        assert_eq!(record.id(), id);
        assert_eq!(record.data(), data);
        assert_eq!(record.raw(), 1 << 8);
    }
}

/// Serde serializes the raw integer and revalidates it.
#[cfg(test)]
mod serialization {
    use serde_test::{
        Token,
        assert_de_tokens_error,
        assert_tokens,
    };

    use super::*;

    #[test]
    fn serde_round_trips_the_raw_value() {
        let record = SerdeRecord::make(SerdeRecordValues {
            level: Level::try_make(5).unwrap(),
            byte: 0xC3,
        });

        assert_tokens(&record, &[Token::U16(0xC305)]);
    }

    #[test]
    fn serde_rejects_a_set_gap_bit() {
        let raw = 1 << 3 | 1;

        assert_de_tokens_error::<SerdeRecord>(
            &[Token::U16(raw)],
            "invalid value",
        );
    }

    #[test]
    fn serde_rejects_invalid_field_bits() {
        let raw = 0xC306;

        assert_de_tokens_error::<SerdeRecord>(
            &[Token::U16(raw)],
            "invalid value",
        );
    }
}

/// Generated traits and their configuration.
#[cfg(test)]
mod traits {
    use super::*;
    use crate::{
        assert_no_associated_fn,
        assert_no_method,
        assert_not_impl,
    };

    #[test]
    fn records_with_equal_fields_are_equal() {
        let first = gapped_sample();
        let second = Gapped::make(first.fields());
        let changed = first.with_flag(false);

        assert_eq!(first, second);
        assert_ne!(first, changed);
        assert_eq!(changed.with_flag(true), first);
    }

    #[test]
    fn values_struct_compares_field_wise() {
        let first = gapped_sample().fields();
        let second = gapped_sample().fields();

        assert_eq!(first, second);
    }

    #[test]
    fn debug_prints_fields_through_their_getters() {
        let record = gapped_sample();

        assert_eq!(
            format!("{record:?}"),
            "Gapped { level: Level(3), offset: Offset(-2), flag: true, tag: Tag(42) }"
        );
    }

    #[test]
    fn values_struct_debug_prints_its_fields() {
        let values = gapped_sample().fields();

        assert_eq!(
            format!("{values:?}"),
            "GappedValues { level: Level(3), offset: Offset(-2), flag: true, tag: Tag(42) }"
        );
    }

    #[test]
    fn without_values_struct_removes_make_and_fields() {
        let level = Level::try_make(2).unwrap();
        let byte = 0x7E;
        let record = NoValues::try_make(1)
            .unwrap()
            .with_level(level)
            .with_byte(byte);

        assert_no_associated_fn!(NoValues, make);
        assert_no_method!(NoValues, fields);
        assert_eq!(record.level(), level);
        assert_eq!(record.byte(), byte);
    }

    #[test]
    fn without_eq_hash_debug_removes_those_traits() {
        assert_not_impl!(Bare: PartialEq);
        assert_not_impl!(Bare: Eq);
        assert_not_impl!(Bare: core::hash::Hash);
        assert_not_impl!(Bare: core::fmt::Debug);
        assert_not_impl!(BareValues: PartialEq);
        assert_not_impl!(BareValues: core::hash::Hash);
        assert_not_impl!(BareValues: core::fmt::Debug);
    }

    #[test]
    fn with_ord_orders_by_raw_value() {
        let high = Ordered::make(OrderedValues { high: 1, low: 0 });
        let low = Ordered::make(OrderedValues {
            high: 0,
            low: u8::MAX,
        });

        assert!(low < high);
        assert_eq!(high.cmp(&low), core::cmp::Ordering::Greater);
    }

    #[test]
    fn default_record_has_no_order() {
        assert_not_impl!(Gapped: PartialOrd);
        assert_not_impl!(Gapped: Ord);
    }

    #[test]
    fn record_has_no_operators() {
        assert_not_impl!(Gapped: core::ops::Add<Gapped>);
        assert_not_impl!(Gapped: core::ops::Sub<Gapped>);
        assert_not_impl!(Gapped: core::ops::BitOr<Gapped>);
        assert_not_impl!(Gapped: core::ops::BitAnd<Gapped>);
        assert_not_impl!(Gapped: core::ops::Not);
        assert_not_impl!(Gapped: core::ops::Shl<u32>);
        assert_not_impl!(Gapped: core::ops::Add<u16>);
        assert_not_impl!(Gapped: core::ops::BitOr<u16>);
    }

    #[test]
    fn record_has_no_conversions_or_display() {
        assert_not_impl!(Gapped: From<u16>);
        assert_not_impl!(u16: From<Gapped>);
        assert_not_impl!(Gapped: core::fmt::Display);
    }
}

/// Visibility and hiding.
#[cfg(test)]
mod hiding {
    use super::*;

    #[test]
    fn hide_in_none_keeps_private_accessors_usable_in_defining_module() {
        let first = 0x12;
        let second = 0x34;

        assert_eq!(visible_secret_after(first, second), (first, second));
    }

    #[test]
    fn default_hiding_keeps_private_accessors_usable_in_defining_module() {
        let first = 0x56;
        let second = 0x78;

        assert_eq!(hidden_secret_after(first, second), (first, second));
    }
}

/// Every accessor is `const` under the default `konst`.
#[cfg(test)]
mod constness {
    use super::*;

    #[test]
    fn accessors_evaluate_in_const_items() {
        const LEVEL: Level = match Level::try_make(4) {
            Ok(level) => level,
            Err(_) => panic!("valid level rejected"),
        };
        const RECORD: Gapped = Gapped::make(GappedValues {
            level: LEVEL,
            offset: match Offset::try_make(-1) {
                Ok(offset) => offset,
                Err(_) => panic!("valid offset rejected"),
            },
            flag: false,
            tag: match Tag::try_make(9) {
                Ok(tag) => tag,
                Err(_) => panic!("valid tag rejected"),
            },
        });
        const READ: Level = RECORD.level();
        const FLAGGED: Gapped = RECORD.with_flag(true);
        const FIELDS: GappedValues = FLAGGED.fields();
        const RAW: u16 = FLAGGED.raw();
        const PARSED: Result<Gapped, u16> = Gapped::try_make(RAW);

        assert_eq!(READ, LEVEL);
        assert!(FIELDS.flag);
        assert_eq!(PARSED, Ok(FLAGGED));
    }

    #[test]
    fn nightly_konst_compares_in_const_items() {
        const LEVEL: NightlyLevel = match NightlyLevel::try_make(2) {
            Ok(level) => level,
            Err(_) => panic!("valid level rejected"),
        };
        const FIRST: NightlyRecord = NightlyRecord::make(NightlyRecordValues {
            level: LEVEL,
            flag: true,
        });
        const SECOND: NightlyRecord = FIRST.with_flag(false).with_flag(true);
        const OTHER: NightlyRecord = FIRST.with_flag(false);
        const EQUAL: bool = FIRST == SECOND;
        const DIFFERENT: bool = FIRST != OTHER;

        assert!(EQUAL);
        assert!(DIFFERENT);
    }
}
