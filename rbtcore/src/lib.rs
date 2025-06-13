//! Core codec algorithms (α-Flow, β-Hierarchy, γ-Controller)

pub mod alpha_flow {
    //! Placeholder for α-Flow entropy coding algorithms.

    /// Return library version string.
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn version_not_empty() {
            assert!(!version().is_empty());
        }
    }
} 