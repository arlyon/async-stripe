use std::fmt::Write;

use indoc::formatdoc;

use crate::crate_inference::Crate;

const CORE_FEATURES: &[&str] = &[
    "runtime-tokio-hyper",
    "runtime-tokio-hyper-rustls",
    "runtime-tokio-hyper-rustls-webpki",
    "runtime-blocking",
    "runtime-blocking-rustls",
    "runtime-blocking-rustls-webpki",
    "runtime-async-std-surf",
];

pub fn gen_crate_toml(
    krate: Crate,
    crate_deps: Vec<Crate>,
    mut crate_features: Vec<String>,
) -> String {
    let crate_name = krate.name();
    let mut crate_dep_section = String::new();
    for dep in crate_deps {
        let dep_path = if dep == Crate::TYPES {
            "../../stripe_types".into()
        } else {
            format!("../../generated/{}", dep.name())
        };
        let _ = writeln!(crate_dep_section, r#"{} = {{path = "{}"}}"#, dep.name(), dep_path);
    }
    let mut feature_section = String::new();
    for feature in CORE_FEATURES {
        let _ = writeln!(feature_section, r#"{feature} = ["async-stripe/{feature}"]"#);
    }
    feature_section.push('\n');

    crate_features.sort_unstable();
    for feature in &crate_features {
        let _ = writeln!(feature_section, "{feature} = []");
    }
    if !crate_features.is_empty() {
        feature_section.push('\n');
        let _ = writeln!(
            feature_section,
            "full = [{}]",
            crate_features
                .iter()
                .map(|feat| format!(r#""{feat}""#))
                .collect::<Vec<_>>()
                .join(",\n")
        );
    }

    let mut docs_rs_features = String::from(r#"["runtime-tokio-hyper""#);
    if !crate_features.is_empty() {
        docs_rs_features.push_str(r#", "full""#);
    }
    docs_rs_features.push(']');
    formatdoc! {
        r#"
        [package]
        name = "{crate_name}"
        version.workspace = true
        description.workspace = true
        edition.workspace = true
        rust-version.workspace = true
        authors.workspace = true
        license.workspace = true
        homepage.workspace = true
        repository.workspace = true
        keywords.workspace = true
        categories.workspace = true
        
        [lib]
        path = "src/mod.rs"
        
        [package.metadata.docs.rs]
        features = {docs_rs_features}
        
        [dependencies]
        serde.workspace = true
        http-types.workspace = true
        smol_str.workspace = true
        serde_json.workspace = true
        async-stripe = {{path = "../../async-stripe"}}
        
        {crate_dep_section}
        [features]
        {feature_section}
        "#
    }
}
