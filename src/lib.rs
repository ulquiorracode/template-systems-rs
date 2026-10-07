//! Core module template.

/// Returns greeting for demonstration.
#[must_use]
pub fn hello_systems() -> &'static str {
    "zero-overhead-systems-ready"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello_systems(), "zero-overhead-systems-ready");
    }
}
