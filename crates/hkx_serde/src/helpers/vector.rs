//! Vector normalization

use super::number::cpp_to_rust_float_str;
use std::borrow::Cow;

/// Separate vectors classified by `()`, `,` space, etc.
///
/// # Note
/// C++ non-numbers and infinities are converted to Rust numeric strings.
pub fn normalize(str: &str) -> Vec<Cow<'_, str>> {
    str.split(&['(', ')', ',', ' ', '\n', '\r', '\t'][..])
        .filter(|&x| !x.is_empty())
        .map(|s| cpp_to_rust_float_str(s).into())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_normalize() {
        let input = "1.0 (2.0, 3.0) -1.#IND00";
        assert_eq!(normalize(input), vec!["1.0", "2.0", "3.0", "nan"]);
    }
}
