use std::fmt::Write;

use indoc::formatdoc;

use crate::crates::Crate;

fn get_serialization_feature(deps: &[Crate], kind: &str) -> String {
    let mut features = vec![format!("async-stripe-types/{kind}")];
    for dep in deps {
        features.push(format!("{}/{kind}", dep.crate_name()));
    }
    features.into_iter().map(|f| format!(r#""{f}""#)).collect::<Vec<_>>().join(",")
}

pub fn gen_crate_toml(
    krate: Crate,
    crate_deps: Vec<Crate>,
    crate_features: Vec<String>,
    version: &str,
) -> String {
    let mut crate_dep_section = String::new();

    for dep in &crate_deps {
        let dep_name = dep.crate_name();
        let dep_path = format!("../../generated/{dep_name}");
        let _ = writeln!(
            crate_dep_section,
            // release-plz adds an extra space. let's add it here also to prevent churn
            r#"{dep_name} = {{path = "{dep_path}", version = "{version}" }}"#
        );
    }

    // Dependencies only needed for libraries which implement Stripe requests
    let request_deps = if krate == Crate::SHARED {
        None
    } else {
        Some(format!(
            // release-plz adds an extra space. let's add it here also to prevent churn
            r#"async-stripe-client-core = {{path = "../../async-stripe-client-core", version = "{version}" }}"#,
        ))
    };
    let request_deps = request_deps.as_deref().unwrap_or_default();

    let ser_features = get_serialization_feature(&crate_deps, "serialize");
    let deser_features = get_serialization_feature(&crate_deps, "deserialize");

    let features =
        if krate == Crate::SHARED { "".into() } else { gen_feature_section(crate_features) };

    let crate_name = krate.crate_name();
    let lib_name = krate.lib_name();
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
        name = "{lib_name}"

        [dependencies]
        serde.workspace = true
        serde_json = {{ workspace = true, optional = true }}
        smol_str.workspace = true
        miniserde.workspace = true
        async-stripe-types = {{path = "../../async-stripe-types", version = "{version}" }}
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
    // assert!(!crate_features.is_empty(), "expected crate to have features");
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
