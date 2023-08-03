use std::fmt::Write;

use indoc::formatdoc;

use crate::crate_inference::Crate;

const CORE_FEATURES: &[&str] = &["runtime-tokio-hyper", "runtime-tokio-hyper-rustls", "runtime-tokio-hyper-rustls-webpki", "runtime-blocking", "runtime-blocking-rustls", "runtime-blocking-rustls-webpki", "runtime-async-std-surf"];

pub fn gen_crate_toml(krate: Crate, crate_deps: Vec<Crate>) -> String {
    let crate_name = krate.name();
    let mut crate_dep_section = String::new();
    for dep in crate_deps {
        let dep_path = if dep == Crate::Types { "../../stripe_types".into() } else { format!("../../generated/{}", dep.name()) };
        let _ = writeln!(crate_dep_section, r#"{} = {{path = "{}"}}"#, dep.name(), dep_path);
    }
    let mut feature_section = String::new();
    for feature in CORE_FEATURES {
        let _ = writeln!(feature_section, r#"{feature} = ["async-stripe/{feature}"]"#);
    }
    formatdoc! {
        r#"
        [package]
        name = "{crate_name}"
        version.workspace = true
        description.workspace = true
        edition.workspace = true
        rust-version.workspace = true
        
        [lib]
        path = "src/mod.rs"
        
        [dependencies]
        serde.workspace = true
        http-types.workspace = true
        smol_str.workspace = true
        async-stripe = {{path = "../../async-stripe"}}
        {crate_dep_section}
        
        [features]
        {feature_section}
        "#
    }
}
