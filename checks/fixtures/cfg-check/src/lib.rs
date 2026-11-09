//! Exercise real bodies under a declared feature and a declared custom cfg.
//!
//! `check-cfg` registers permitted names; the verifier separately enables
//! `validation_mode` and executes the tests behind it.

/// Double any `u32` without overflow by widening the result first.
pub fn double(value: u32) -> u64 {
    u64::from(value) * 2
}

/// Add values with checked arithmetic when the optional capability is enabled.
#[cfg(feature = "checked")]
pub fn checked_total(values: &[u32]) -> Option<u32> {
    values
        .iter()
        .try_fold(0_u32, |total, &value| total.checked_add(value))
}

/// Compute a wider sum for the verifier's small bounded input fixtures.
#[cfg(validation_mode)]
pub fn validation_total(values: &[u32]) -> u64 {
    values.iter().map(|&value| u64::from(value)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_transform_is_available() {
        assert_eq!(double(21), 42);
        assert_eq!(double(u32::MAX), 8_589_934_590);
    }

    #[cfg(feature = "checked")]
    #[test]
    fn checked_feature_body_executes() {
        assert_eq!(checked_total(&[]), Some(0));
        assert_eq!(checked_total(&[20, 22]), Some(42));
        assert_eq!(checked_total(&[u32::MAX, 1]), None);
    }

    #[cfg(validation_mode)]
    #[test]
    fn custom_cfg_body_executes() {
        assert_eq!(validation_total(&[]), 0);
        assert_eq!(validation_total(&[20, 22]), 42);
        assert_eq!(validation_total(&[u32::MAX, 1]), 4_294_967_296);
    }
}
