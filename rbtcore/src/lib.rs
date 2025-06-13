//! Core codec algorithms (α-Flow, β-Context, γ-Controller)

pub mod alpha_flow;
pub mod beta_context;
pub mod gamma_control;

/// Returns rbtcore crate version.
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