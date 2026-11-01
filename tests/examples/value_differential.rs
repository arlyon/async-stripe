//! Record both deserialization paths for every fixture and single-node corruption.
//! Run against each checkout with:
//! cargo run --locked -p async-stripe-tests --example value_differential > results.jsonl

#[path = "../tests/it/generated/mod.rs"]
mod generated;
#[path = "../value_differential.rs"]
mod value_differential;

mod deserialization_fixture {
    use std::io::Write;

    use serde_json::{Value, json};

    pub fn check_object<
        T: miniserde::Deserialize + serde::Serialize + stripe_types::miniserde_helpers::FromValueOpt,
    >(
        resources: &Value,
        key: &str,
    ) {
        let Some(fixture) = resources.get(key) else { return };
        let mut out = std::io::stdout().lock();
        for (path, input) in std::iter::once((None, fixture.clone())).chain(
            crate::value_differential::mutations(fixture).into_iter().map(|(p, v)| (Some(p), v)),
        ) {
            let (streaming, value) = crate::value_differential::outputs::<T>(&input);
            let record =
                json!({"fixture": key, "path": path, "streaming": streaming, "value": value});
            serde_json::to_writer(&mut out, &record).unwrap();
            writeln!(out).unwrap();
        }
    }
}

fn main() {
    let fixtures: serde_json::Value =
        serde_json::from_str(include_str!("../../openapi/fixtures.json")).unwrap();
    generated::check_fixtures(&fixtures["resources"]);
}
