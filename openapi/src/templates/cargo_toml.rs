use std::fmt::Write;

use indoc::formatdoc;

use crate::crates::Crate;

fn get_serialization_feature(deps: &[Crate], kind: &str) -> String {
    let mut features = vec![format!("stripe_types/{kind}")];
    for dep in deps {
        features.push(format!("{}/{kind}", dep.name()));
    }
    features.into_iter().map(|f| format!(r#""{f}""#)).collect::<Vec<_>>().join(",")
}

pub fn gen_crate_toml(krate: Crate, crate_deps: Vec<Crate>, crate_features: Vec<String>) -> String {
    let mut crate_dep_section = String::new();

    for dep in &crate_deps {
        let dep_name = dep.name();
        let dep_path = format!("../../generated/{dep_name}");
        let _ = writeln!(crate_dep_section, r#"{dep_name} = {{path = "{dep_path}"}}"#);
    }

    // Dependencies only needed for libraries which implement Stripe requests
    let request_deps = if krate == Crate::SHARED {
        ""
    } else {
        r#"stripe_client_core = {path = "../../stripe_client_core"}"#
    };

    let ser_features = get_serialization_feature(&crate_deps, "serialize");
    let deser_features = get_serialization_feature(&crate_deps, "deserialize");

    let features =
        if krate == Crate::SHARED { "".into() } else { gen_feature_section(crate_features) };
    formatdoc! {
        r#"
        [package]
        name = "{krate}"
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
        
        [dependencies]
        serde.workspace = true
        serde_json = {{ workspace = true, optional = true }}
        smol_str.workspace = true
        miniserde.workspace = true
        stripe_types = {{path = "../../stripe_types"}}
        {request_deps}
        
        {crate_dep_section}

        [features]
        serialize = [{ser_features}]
        deserialize = [{deser_features}, "dep:serde_json"]
        {features}
        "#
    }
}

fn gen_feature_section(mut crate_features: Vec<String>) -> String {
    assert!(!crate_features.is_empty(), "expected crate to have features");
    let mut feature_section = String::new();

    crate_features.sort_unstable();
    for feature in &crate_features {
        let _ = writeln!(feature_section, "{feature} = []");
    }
    feature_section.push('\n');
    let _ = writeln!(
        feature_section,
        "full = [{}]",
        crate_features.iter().map(|feat| format!(r#""{feat}""#)).collect::<Vec<_>>().join(",\n")
    );

    formatdoc! {
        r#"
        {feature_section}
        
        [package.metadata.docs.rs]
        features = ["full"]
        "#
    }
}
