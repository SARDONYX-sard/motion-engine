pub mod generators;
pub mod hkxcmd_parser;

pub use generators::generate_classes;

#[cfg(test)]
mod tests {
    use crate::generators::generate_classes;

    #[ignore]
    #[test]
    pub fn should_generate_classes() {
        let _guard = hkx_serde_tracing::init_tracing(
            Some("should_generate_classes"),
            true,
            tracing::Level::ERROR,
        );

        let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("hkx_serde")
            .join("src")
            .join("classes")
            .join("generated");
        std::fs::create_dir_all(&output_dir).unwrap();

        let rpt_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("hkxcmd_help")
            .join("rpt");
        generate_classes(output_dir, rpt_dir)
    }
}
