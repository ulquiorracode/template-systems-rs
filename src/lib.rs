//! Systems-grade crate template enforcing the canonical 3-Tier API Paradigm.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

// --- Tier 1: Core Imperative API (Default, #![no_std], Zero-Alloc) ---
pub mod core {
    /// Returns greeting for demonstration.
    #[must_use]
    pub const fn hello_systems() -> &'static str {
        "zero-overhead-systems-ready"
    }
}
pub use crate::core::*;

// --- Tier 2: Fluent / Builder API (Additive, feature = "builder") ---
#[cfg(feature = "builder")]
pub mod builder {
    /// Example fluent builder scaffolding for Tier 2 API.
    #[derive(Debug, Default)]
    pub struct SystemsBuilder {
        enabled: bool,
    }

    impl SystemsBuilder {
        /// Creates a new builder instance.
        #[must_use]
        pub const fn new() -> Self {
            Self { enabled: false }
        }

        /// Enables systems optimizations.
        #[must_use]
        pub const fn enable(mut self) -> Self {
            self.enabled = true;
            self
        }

        /// Finalizes the configuration.
        #[must_use]
        pub const fn build(self) -> bool {
            self.enabled
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello_systems(), "zero-overhead-systems-ready");
    }

    #[cfg(feature = "builder")]
    #[test]
    fn test_builder() {
        let b = builder::SystemsBuilder::new().enable();
        assert!(b.build());
    }
}
