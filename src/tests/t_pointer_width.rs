#[cfg(test)]
mod tests {
    use crate::zz_value_type::N;

    #[test]
    fn u16_fits_a_16_bit_usize() {
        let source = N::U016;
        let target = N::USIZ;
        let width = 16;
        let safe = source.can_safe_cast_to(target, width);

        assert!(safe, "{source:?} -> {target:?} on {width} bits");
    }

    #[test]
    fn u32_does_not_fit_a_16_bit_usize() {
        let source = N::U032;
        let target = N::USIZ;
        let width = 16;
        let safe = source.can_safe_cast_to(target, width);

        assert!(!safe, "{source:?} -> {target:?} on {width} bits");
    }

    #[test]
    fn u16_does_not_fit_a_16_bit_isize() {
        let source = N::U016;
        let target = N::ISIZ;
        let width = 16;
        let safe = source.can_safe_cast_to(target, width);

        assert!(!safe, "{source:?} -> {target:?} on {width} bits");
    }

    #[test]
    fn i16_fits_a_16_bit_isize() {
        let source = N::I016;
        let target = N::ISIZ;
        let width = 16;
        let safe = source.can_safe_cast_to(target, width);

        assert!(safe, "{source:?} -> {target:?} on {width} bits");
    }
}
