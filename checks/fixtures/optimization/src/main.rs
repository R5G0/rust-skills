//! Small real program for cold-path, LTO, and PGO build validation.

#[cold]
#[inline(never)]
fn invalid_input() -> Result<u64, &'static str> {
    Err("input must be nonzero")
}

fn compute(input: u64) -> Result<u64, &'static str> {
    if input == 0 {
        return invalid_input();
    }
    Ok((0..input).fold(0_u64, |sum, value| sum.wrapping_add(value)))
}

fn main() {
    let input = std::env::args()
        .nth(1)
        .map(|value| value.parse::<u64>().expect("integer input"))
        .unwrap_or(100);
    println!("{}", compute(input).expect("positive input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cold_error_and_hot_result_follow_the_same_contract() {
        assert_eq!(compute(0), Err("input must be nonzero"));
        assert_eq!(compute(1), Ok(0));
        assert_eq!(compute(100), Ok(4950));
    }
}
