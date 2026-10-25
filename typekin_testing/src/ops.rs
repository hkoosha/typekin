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
