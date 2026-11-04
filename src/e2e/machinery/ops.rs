pub struct Operation {
    pub name: &'static str,
    pub raw: &'static str,
    pub wrapped: &'static str,
}

impl Operation {
    #[must_use]
    const fn new(
        name: &'static str,
        raw: &'static str,
        wrapped: &'static str,
    ) -> Self {
        return Self { name, raw, wrapped };
    }
}

pub const U32_OPERATIONS: &[Operation] = &[
    Operation::new("add", "raw_add_u32", "wrapped_add_u32"),
    Operation::new("sub", "raw_sub_u32", "wrapped_sub_u32"),
    Operation::new("mul", "raw_mul_u32", "wrapped_mul_u32"),
    Operation::new("div", "raw_div_u32", "wrapped_div_u32"),
    Operation::new("rem", "raw_rem_u32", "wrapped_rem_u32"),
    Operation::new("bitand", "raw_bitand_u32", "wrapped_bitand_u32"),
    Operation::new("bitor", "raw_bitor_u32", "wrapped_bitor_u32"),
    Operation::new("bitxor", "raw_bitxor_u32", "wrapped_bitxor_u32"),
    Operation::new("not", "raw_not_u32", "wrapped_not_u32"),
    Operation::new("shl", "raw_shl_u32", "wrapped_shl_u32"),
    Operation::new("shr", "raw_shr_u32", "wrapped_shr_u32"),
];

pub const U32_MAP_OPERATIONS: &[Operation] = &[
    Operation::new("map_add", "raw_map_add_u32", "wrapped_map_add_u32"),
    Operation::new(
        "try_map_add",
        "raw_try_map_add_u32",
        "wrapped_try_map_add_u32",
    ),
];

pub const U64_CONST_OPERATIONS: &[Operation] = &[
    Operation::new("add", "raw_add_u64", "wrapped_add_u64"),
    Operation::new("sub", "raw_sub_u64", "wrapped_sub_u64"),
    Operation::new("mul", "raw_mul_u64", "wrapped_mul_u64"),
    Operation::new("div", "raw_div_u64", "wrapped_div_u64"),
    Operation::new("rem", "raw_rem_u64", "wrapped_rem_u64"),
    Operation::new("bitand", "raw_bitand_u64", "wrapped_bitand_u64"),
    Operation::new("bitor", "raw_bitor_u64", "wrapped_bitor_u64"),
    Operation::new("bitxor", "raw_bitxor_u64", "wrapped_bitxor_u64"),
    Operation::new("not", "raw_not_u64", "wrapped_not_u64"),
    Operation::new("shl", "raw_shl_u64", "wrapped_shl_u64"),
    Operation::new("shr", "raw_shr_u64", "wrapped_shr_u64"),
];

pub const SEQUENCE_OPERATIONS: &[Operation] = &[
    Operation::new(
        "circular subtraction with output conversion",
        "raw_sub_sequence",
        "wrapped_sub_sequence",
    ),
    Operation::new(
        "circular precedes",
        "raw_precedes_sequence",
        "wrapped_precedes_sequence",
    ),
];

pub const GENERIC_SEQUENCE_OPERATIONS: &[Operation] = &[
    Operation::new(
        "generic point subtraction with generic output",
        "raw_sub_generic_points",
        "wrapped_sub_generic_points",
    ),
    Operation::new(
        "generic point plus offset facet",
        "raw_add_generic_offset",
        "wrapped_add_generic_offset",
    ),
    Operation::new(
        "generic point minus offset facet",
        "raw_sub_generic_offset",
        "wrapped_sub_generic_offset",
    ),
    Operation::new(
        "generic circular precedes",
        "raw_precedes_sequence",
        "wrapped_precedes_generic_point",
    ),
];

/// `code64` is compared only as written (`DOMAIN_INT_WRITTEN`): its domain
/// excludes zero through `% 3`, which LLVM does not see, so the safe
/// `NonZeroI64::new` keeps its zero check and panic, and a hand-written
/// panic always has another location (TODO.md).
pub const DOMAIN_INT_OPERATIONS: &[Operation] = &[
    Operation::new("domain code", "raw_code", "wrapped_code"),
    Operation::new(
        "domain even_or_band",
        "raw_even_or_band",
        "wrapped_even_or_band",
    ),
    Operation::new("domain bounded", "raw_bounded", "wrapped_bounded"),
    Operation::new("domain bits", "raw_bits", "wrapped_bits"),
    Operation::new("domain unsorted", "raw_unsorted", "wrapped_unsorted"),
    Operation::new(
        "domain overlapping",
        "raw_overlapping",
        "wrapped_overlapping",
    ),
];

/// `raw` is the condition as written: typekin must not be longer.
pub const DOMAIN_INT_WRITTEN: &[Operation] = &[
    Operation::new("domain code as written", "written_code", "wrapped_code"),
    Operation::new(
        "domain code64 as written",
        "written_code64",
        "wrapped_code64",
    ),
    Operation::new(
        "domain even_or_band as written",
        "written_even_or_band",
        "wrapped_even_or_band",
    ),
    Operation::new(
        "domain bounded as written",
        "written_bounded",
        "wrapped_bounded",
    ),
    Operation::new("domain bits as written", "written_bits", "wrapped_bits"),
    Operation::new(
        "domain unsorted as written",
        "written_unsorted",
        "wrapped_unsorted",
    ),
    Operation::new(
        "domain overlapping as written",
        "written_overlapping",
        "wrapped_overlapping",
    ),
];

/// Opt-levels 2 and 3: every operation equals the raw code.
pub const F64_OPERATIONS: &[Operation] = &[
    Operation::new("add", "raw_add_f64", "wrapped_add_f64"),
    Operation::new("sub", "raw_sub_f64", "wrapped_sub_f64"),
    Operation::new("mul", "raw_mul_f64", "wrapped_mul_f64"),
    Operation::new("div", "raw_div_f64", "wrapped_div_f64"),
    Operation::new("rem", "raw_rem_f64", "wrapped_rem_f64"),
    Operation::new(
        "add_assign",
        "raw_add_assign_f64",
        "wrapped_add_assign_f64",
    ),
    Operation::new(
        "mul_assign",
        "raw_mul_assign_f64",
        "wrapped_mul_assign_f64",
    ),
    Operation::new("neg", "raw_neg_f64", "wrapped_neg_f64"),
    Operation::new("add_prim", "raw_add_prim_f64", "wrapped_add_prim_f64"),
    Operation::new("eq", "raw_eq_f64", "wrapped_eq_f64"),
    Operation::new("ne", "raw_ne_f64", "wrapped_ne_f64"),
    Operation::new("lt", "raw_lt_f64", "wrapped_lt_f64"),
    Operation::new("le", "raw_le_f64", "wrapped_le_f64"),
    Operation::new("gt", "raw_gt_f64", "wrapped_gt_f64"),
    Operation::new("ge", "raw_ge_f64", "wrapped_ge_f64"),
    Operation::new(
        "partial_cmp",
        "raw_partial_cmp_f64",
        "wrapped_partial_cmp_f64",
    ),
    Operation::new("map", "raw_map_f64", "wrapped_map_f64"),
    Operation::new(
        "try_into_f32",
        "raw_try_into_f32_f64",
        "wrapped_try_into_f32_f64",
    ),
    Operation::new(
        "try_into_u8",
        "raw_try_into_u8_f64",
        "wrapped_try_into_u8_f64",
    ),
    Operation::new(
        "try_into_i32",
        "raw_try_into_i32_f64",
        "wrapped_try_into_i32_f64",
    ),
    Operation::new(
        "try_into_u32",
        "raw_try_into_u32_f64",
        "wrapped_try_into_u32_f64",
    ),
    Operation::new(
        "try_into_i64",
        "raw_try_into_i64_f64",
        "wrapped_try_into_i64_f64",
    ),
    Operation::new(
        "try_into_u64",
        "raw_try_into_u64_f64",
        "wrapped_try_into_u64_f64",
    ),
    Operation::new("abs", "raw_abs_f64", "wrapped_abs_f64"),
    Operation::new("signum", "raw_signum_f64", "wrapped_signum_f64"),
    Operation::new("copysign", "raw_copysign_f64", "wrapped_copysign_f64"),
    Operation::new("recip", "raw_recip_f64", "wrapped_recip_f64"),
    Operation::new("min", "raw_min_f64", "wrapped_min_f64"),
    Operation::new("max", "raw_max_f64", "wrapped_max_f64"),
    Operation::new("to_bits", "raw_to_bits_f64", "wrapped_to_bits_f64"),
    Operation::new("from_bits", "raw_from_bits_f64", "wrapped_from_bits_f64"),
    Operation::new("classify", "raw_classify_f64", "wrapped_classify_f64"),
    Operation::new("sum", "raw_sum_f64", "wrapped_sum_f64"),
    Operation::new("product", "raw_product_f64", "wrapped_product_f64"),
    Operation::new("dot", "raw_dot_f64", "wrapped_dot_f64"),
    Operation::new("floor", "raw_floor_f64", "wrapped_floor_f64"),
    Operation::new("sqrt", "raw_sqrt_f64", "wrapped_sqrt_f64"),
    Operation::new("mul_add", "raw_mul_add_f64", "wrapped_mul_add_f64"),
];

/// Opt-levels 1, s, and z: checked conversions equal the same check behind
/// one `#[inline(always)]` layer (`written_*`), which is all typekin adds.
pub const F64_SIZE_OPERATIONS: &[Operation] = &[
    Operation::new("add", "raw_add_f64", "wrapped_add_f64"),
    Operation::new("sub", "raw_sub_f64", "wrapped_sub_f64"),
    Operation::new("mul", "raw_mul_f64", "wrapped_mul_f64"),
    Operation::new("div", "raw_div_f64", "wrapped_div_f64"),
    Operation::new("rem", "raw_rem_f64", "wrapped_rem_f64"),
    Operation::new(
        "add_assign",
        "raw_add_assign_f64",
        "wrapped_add_assign_f64",
    ),
    Operation::new(
        "mul_assign",
        "raw_mul_assign_f64",
        "wrapped_mul_assign_f64",
    ),
    Operation::new("neg", "raw_neg_f64", "wrapped_neg_f64"),
    Operation::new("add_prim", "raw_add_prim_f64", "wrapped_add_prim_f64"),
    Operation::new("eq", "raw_eq_f64", "wrapped_eq_f64"),
    Operation::new("ne", "raw_ne_f64", "wrapped_ne_f64"),
    Operation::new("lt", "raw_lt_f64", "wrapped_lt_f64"),
    Operation::new("le", "raw_le_f64", "wrapped_le_f64"),
    Operation::new("gt", "raw_gt_f64", "wrapped_gt_f64"),
    Operation::new("ge", "raw_ge_f64", "wrapped_ge_f64"),
    Operation::new(
        "partial_cmp",
        "raw_partial_cmp_f64",
        "wrapped_partial_cmp_f64",
    ),
    Operation::new("map", "raw_map_f64", "wrapped_map_f64"),
    Operation::new(
        "try_into_f32",
        "written_try_into_f32_f64",
        "wrapped_try_into_f32_f64",
    ),
    Operation::new(
        "try_into_u8",
        "written_try_into_u8_f64",
        "wrapped_try_into_u8_f64",
    ),
    Operation::new(
        "try_into_i32",
        "written_try_into_i32_f64",
        "wrapped_try_into_i32_f64",
    ),
    Operation::new(
        "try_into_u32",
        "written_try_into_u32_f64",
        "wrapped_try_into_u32_f64",
    ),
    Operation::new(
        "try_into_i64",
        "written_try_into_i64_f64",
        "wrapped_try_into_i64_f64",
    ),
    Operation::new(
        "try_into_u64",
        "written_try_into_u64_f64",
        "wrapped_try_into_u64_f64",
    ),
    Operation::new("abs", "raw_abs_f64", "wrapped_abs_f64"),
    Operation::new("signum", "raw_signum_f64", "wrapped_signum_f64"),
    Operation::new("copysign", "raw_copysign_f64", "wrapped_copysign_f64"),
    Operation::new("recip", "raw_recip_f64", "wrapped_recip_f64"),
    Operation::new("min", "raw_min_f64", "wrapped_min_f64"),
    Operation::new("max", "raw_max_f64", "wrapped_max_f64"),
    Operation::new("to_bits", "raw_to_bits_f64", "wrapped_to_bits_f64"),
    Operation::new("from_bits", "raw_from_bits_f64", "wrapped_from_bits_f64"),
    Operation::new("classify", "raw_classify_f64", "wrapped_classify_f64"),
    Operation::new("sum", "raw_sum_f64", "wrapped_sum_f64"),
    Operation::new("product", "raw_product_f64", "wrapped_product_f64"),
    Operation::new("dot", "raw_dot_f64", "wrapped_dot_f64"),
    Operation::new("floor", "raw_floor_f64", "wrapped_floor_f64"),
    Operation::new("sqrt", "raw_sqrt_f64", "wrapped_sqrt_f64"),
    Operation::new("mul_add", "raw_mul_add_f64", "wrapped_mul_add_f64"),
];

pub const F32_OPERATIONS: &[Operation] = &[
    Operation::new("add", "raw_add_f32", "wrapped_add_f32"),
    Operation::new("sub", "raw_sub_f32", "wrapped_sub_f32"),
    Operation::new("mul", "raw_mul_f32", "wrapped_mul_f32"),
    Operation::new("div", "raw_div_f32", "wrapped_div_f32"),
    Operation::new("rem", "raw_rem_f32", "wrapped_rem_f32"),
    Operation::new(
        "add_assign",
        "raw_add_assign_f32",
        "wrapped_add_assign_f32",
    ),
    Operation::new(
        "mul_assign",
        "raw_mul_assign_f32",
        "wrapped_mul_assign_f32",
    ),
    Operation::new("neg", "raw_neg_f32", "wrapped_neg_f32"),
    Operation::new("add_prim", "raw_add_prim_f32", "wrapped_add_prim_f32"),
    Operation::new("eq", "raw_eq_f32", "wrapped_eq_f32"),
    Operation::new("ne", "raw_ne_f32", "wrapped_ne_f32"),
    Operation::new("lt", "raw_lt_f32", "wrapped_lt_f32"),
    Operation::new("le", "raw_le_f32", "wrapped_le_f32"),
    Operation::new("gt", "raw_gt_f32", "wrapped_gt_f32"),
    Operation::new("ge", "raw_ge_f32", "wrapped_ge_f32"),
    Operation::new(
        "partial_cmp",
        "raw_partial_cmp_f32",
        "wrapped_partial_cmp_f32",
    ),
    Operation::new("map", "raw_map_f32", "wrapped_map_f32"),
    Operation::new("into_f64", "raw_into_f64_f32", "wrapped_into_f64_f32"),
    Operation::new(
        "try_into_u8",
        "raw_try_into_u8_f32",
        "wrapped_try_into_u8_f32",
    ),
    Operation::new(
        "try_into_i32",
        "raw_try_into_i32_f32",
        "wrapped_try_into_i32_f32",
    ),
    Operation::new(
        "try_into_u32",
        "raw_try_into_u32_f32",
        "wrapped_try_into_u32_f32",
    ),
    Operation::new(
        "try_into_i64",
        "raw_try_into_i64_f32",
        "wrapped_try_into_i64_f32",
    ),
    Operation::new(
        "try_into_u64",
        "raw_try_into_u64_f32",
        "wrapped_try_into_u64_f32",
    ),
];

/// See `F64_SIZE_OPERATIONS`.
pub const F32_SIZE_OPERATIONS: &[Operation] = &[
    Operation::new("add", "raw_add_f32", "wrapped_add_f32"),
    Operation::new("sub", "raw_sub_f32", "wrapped_sub_f32"),
    Operation::new("mul", "raw_mul_f32", "wrapped_mul_f32"),
    Operation::new("div", "raw_div_f32", "wrapped_div_f32"),
    Operation::new("rem", "raw_rem_f32", "wrapped_rem_f32"),
    Operation::new(
        "add_assign",
        "raw_add_assign_f32",
        "wrapped_add_assign_f32",
    ),
    Operation::new(
        "mul_assign",
        "raw_mul_assign_f32",
        "wrapped_mul_assign_f32",
    ),
    Operation::new("neg", "raw_neg_f32", "wrapped_neg_f32"),
    Operation::new("add_prim", "raw_add_prim_f32", "wrapped_add_prim_f32"),
    Operation::new("eq", "raw_eq_f32", "wrapped_eq_f32"),
    Operation::new("ne", "raw_ne_f32", "wrapped_ne_f32"),
    Operation::new("lt", "raw_lt_f32", "wrapped_lt_f32"),
    Operation::new("le", "raw_le_f32", "wrapped_le_f32"),
    Operation::new("gt", "raw_gt_f32", "wrapped_gt_f32"),
    Operation::new("ge", "raw_ge_f32", "wrapped_ge_f32"),
    Operation::new(
        "partial_cmp",
        "raw_partial_cmp_f32",
        "wrapped_partial_cmp_f32",
    ),
    Operation::new("map", "raw_map_f32", "wrapped_map_f32"),
    Operation::new("into_f64", "raw_into_f64_f32", "wrapped_into_f64_f32"),
    Operation::new(
        "try_into_u8",
        "written_try_into_u8_f32",
        "wrapped_try_into_u8_f32",
    ),
    Operation::new(
        "try_into_i32",
        "written_try_into_i32_f32",
        "wrapped_try_into_i32_f32",
    ),
    Operation::new(
        "try_into_u32",
        "written_try_into_u32_f32",
        "wrapped_try_into_u32_f32",
    ),
    Operation::new(
        "try_into_i64",
        "written_try_into_i64_f32",
        "wrapped_try_into_i64_f32",
    ),
    Operation::new(
        "try_into_u64",
        "written_try_into_u64_f32",
        "wrapped_try_into_u64_f32",
    ),
];

/// NaN-free totality, validated construction, and queries the domain
/// decides against the best hand-written raw code.
pub const UNIT_OPERATIONS: &[Operation] = &[
    Operation::new("try_make", "raw_try_make_unit", "wrapped_try_make_unit"),
    Operation::new("eq", "raw_eq_unit", "wrapped_eq_unit"),
    Operation::new("lt", "raw_lt_unit", "wrapped_lt_unit"),
    Operation::new("cmp", "raw_cmp_unit", "wrapped_cmp_unit"),
    Operation::new("max", "raw_max_unit", "wrapped_max_unit"),
    Operation::new("min", "raw_min_unit", "wrapped_min_unit"),
    Operation::new("hash", "raw_hash_unit", "wrapped_hash_unit"),
    Operation::new(
        "hash without zero",
        "raw_hash_at_least_one",
        "wrapped_hash_at_least_one",
    ),
    Operation::new("neg closed", "raw_neg_signed", "wrapped_neg_signed"),
    Operation::new("abs closed", "raw_abs_unit", "wrapped_abs_unit"),
    Operation::new("is_nan decided", "raw_is_nan_unit", "wrapped_is_nan_unit"),
    Operation::new(
        "is_finite decided",
        "raw_is_finite_unit",
        "wrapped_is_finite_unit",
    ),
];

/// `Ord::clamp` asserts `min <= max`; the panic location is fixture data
/// that differs between the two sides, so these compare instruction counts.
pub const UNIT_ASSERTING_OPERATIONS: &[Operation] =
    &[Operation::new("clamp", "raw_clamp_unit", "wrapped_clamp_unit")];

/// Signed `Neg` and unchecked `Neg`/`abs` on closed domains.
pub const I32_OPERATIONS: &[Operation] = &[
    Operation::new("neg_i32", "raw_neg_i32", "wrapped_neg_i32"),
    Operation::new(
        "neg_symmetric",
        "raw_neg_symmetric",
        "wrapped_neg_symmetric",
    ),
    Operation::new(
        "abs_symmetric",
        "raw_abs_symmetric",
        "wrapped_abs_symmetric",
    ),
    Operation::new(
        "abs_non_negative",
        "raw_abs_non_negative",
        "wrapped_abs_non_negative",
    ),
    Operation::new("into_f64_i32", "raw_into_f64_i32", "wrapped_into_f64_i32"),
];

/// Opt-levels 1, s, and z, with checked integer-to-float conversions against
/// the same check behind one `#[inline(always)]` layer.
pub const I32_SIZE_OPERATIONS: &[Operation] = &[
    Operation::new("neg_i32", "raw_neg_i32", "wrapped_neg_i32"),
    Operation::new(
        "neg_symmetric",
        "raw_neg_symmetric",
        "wrapped_neg_symmetric",
    ),
    Operation::new(
        "abs_symmetric",
        "raw_abs_symmetric",
        "wrapped_abs_symmetric",
    ),
    Operation::new(
        "abs_non_negative",
        "raw_abs_non_negative",
        "wrapped_abs_non_negative",
    ),
    Operation::new("into_f64_i32", "raw_into_f64_i32", "wrapped_into_f64_i32"),
    Operation::new(
        "try_into_f32_i32",
        "written_try_into_f32_i32",
        "wrapped_try_into_f32_i32",
    ),
    Operation::new(
        "try_into_f64_u64",
        "written_try_into_f64_u64",
        "wrapped_try_into_f64_u64",
    ),
];

/// Checked integer-to-float conversions against the hand-written check at
/// opt-levels 2 and 3: LLVM orders the operands of the round-trip `==`
/// differently once the value comes from a newtype, so these compare
/// instruction counts.
pub const I32_CONVERSION_OPERATIONS: &[Operation] = &[
    Operation::new(
        "try_into_f32_i32",
        "raw_try_into_f32_i32",
        "wrapped_try_into_f32_i32",
    ),
    Operation::new(
        "try_into_f64_u64",
        "raw_try_into_f64_u64",
        "wrapped_try_into_f64_u64",
    ),
];

/// Operations proven closed over their domain, and a literal macro: the
/// wrapper must compile to the bare primitive.
pub const CLOSURE_OPERATIONS: &[Operation] = &[
    Operation::new("percent div", "raw_percent_div", "wrapped_percent_div"),
    Operation::new("percent rem", "raw_percent_rem", "wrapped_percent_rem"),
    Operation::new(
        "percent bitand",
        "raw_percent_bitand",
        "wrapped_percent_bitand",
    ),
    Operation::new("percent shr", "raw_percent_shr", "wrapped_percent_shr"),
    Operation::new(
        "percent midpoint",
        "raw_percent_midpoint",
        "wrapped_percent_midpoint",
    ),
    Operation::new("literal port", "raw_lit_port", "wrapped_lit_port"),
];

/// Not provably closed: the validation stays.
pub const CLOSURE_CONTROL_OPERATIONS: &[Operation] =
    &[Operation::new("percent add", "raw_percent_add", "wrapped_percent_add")];

/// Proven `Within` friends: `of` must compile to the bare cast.
pub const WITHIN_OPERATIONS: &[Operation] = &[
    Operation::new("within score", "raw_within_score", "wrapped_within_score"),
    Operation::new(
        "within positive",
        "raw_within_positive",
        "wrapped_within_positive",
    ),
    Operation::new(
        "within signed",
        "raw_within_signed",
        "wrapped_within_signed",
    ),
];

/// A validated friend of the same pair: the check stays.
pub const WITHIN_CONTROL_OPERATIONS: &[Operation] = &[
    Operation::new(
        "validated score",
        "raw_within_score",
        "wrapped_within_validated",
    ),
];

/// Packed record accessors: `make`, `with_*`, and proven getters must compile
/// to the hand-written shifts and masks.
pub const PACKED_OPERATIONS: &[Operation] = &[
    Operation::new("packed make", "raw_packed_make", "wrapped_packed_make"),
    Operation::new(
        "packed with_state",
        "raw_packed_with_state",
        "wrapped_packed_with_state",
    ),
    Operation::new(
        "packed version",
        "raw_packed_version",
        "wrapped_packed_version",
    ),
    Operation::new("packed data", "raw_packed_data", "wrapped_packed_data"),
    Operation::new(
        "packed sealed",
        "raw_packed_sealed",
        "wrapped_packed_sealed",
    ),
];

/// A getter that re-checks its field's domain: the check stays.
pub const PACKED_CONTROL_OPERATIONS: &[Operation] =
    &[Operation::new("packed id", "raw_packed_id", "wrapped_packed_id")];
