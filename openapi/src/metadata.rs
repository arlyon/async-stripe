use std::collections::{BTreeMap, BTreeSet};
use std::fs::write;
use std::path::Path;

use heck::{CamelCase, SnakeCase};
use openapiv3::{ReferenceOr, SchemaKind};

use crate::spec::{as_object_properties, Spec};
use crate::{
    file_generator::FileGenerator,
    mappings::{self, FieldMap, ObjectMap},
    metadata,
    types::CopyOrClone,
};

/// Global metadata for the entire codegen process.
#[derive(Debug)]
pub struct Metadata<'a> {
    pub spec: &'a Spec,
    /// A map of `objects` to their rust id type
    pub id_mappings: BTreeMap<String, (String, CopyOrClone)>,

    pub feature_groups: BTreeMap<&'a str, &'a str>,

    /// The set of schemas which should implement `Object`.
    /// These have both an `id` property and on `object` property.
    pub objects: BTreeSet<&'a str>,
    /// A one to many map of schema to depending types.
    pub dependents: BTreeMap<&'a str, BTreeSet<&'a str>>,
    /// How a particular schema should be renamed.
    pub object_mappings: ObjectMap,
    /// An override for the rust-type of a particular object/field pair.
    pub field_mappings: FieldMap,
    /// A one to many map of _objects_ to requests which should be
    /// implemented for that object.
    ///
    /// This is typically determined by the first segment in the path.
    pub requests: BTreeMap<String, BTreeSet<&'a str>>,
}

impl<'a> Metadata<'a> {
    pub fn from_spec(spec: &'a Spec) -> Self {
        let id_renames = mappings::id_renames();
        let object_mappings = mappings::object_mappings();
        let field_mappings = mappings::field_mappings();
        let feature_groups = metadata::feature_groups();

        let mut objects = BTreeSet::new();
        let mut dependents: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
        let mut id_mappings = BTreeMap::new();

        for (key, ref_or_schema) in spec.component_schemas() {
            let schema_name = key.as_str();
            let properties = match ref_or_schema.as_item().and_then(as_object_properties) {
                Some(props) => props,
                None => continue,
            };
            if properties.contains_key("object") {
                objects.insert(schema_name);
                if properties.contains_key("id") {
                    let id_type = id_renames
                        .get(&schema_name)
                        .unwrap_or(&schema_name)
                        .replace('.', "_")
                        .to_camel_case()
                        + "Id";

                    id_mappings.insert(
                        schema_name.replace('.', "_").to_owned(),
                        (id_type, CopyOrClone::Clone),
                    );
                }
            }
            for schema_or_ref in properties.values() {
                match schema_or_ref {
                    ReferenceOr::Reference { reference } => {
                        let dep = reference.trim_start_matches("#/components/schemas/");
                        dependents.entry(dep).or_default().insert(schema_name);
                    }
                    ReferenceOr::Item(schema) => {
                        if let SchemaKind::AnyOf { any_of } = &schema.schema_kind {
                            for ty in any_of {
                                if let ReferenceOr::Reference { reference } = ty {
                                    let dep = reference.trim_start_matches("#/components/schemas/");
                                    dependents.entry(dep).or_default().insert(schema_name);
                                }
                            }
                        }
                    }
                }
            }
        }

        Self {
            spec,
            requests: metadata_requests(spec, &objects),
            objects,
            dependents,
            id_mappings,
            object_mappings,
            field_mappings,
            feature_groups,
        }
    }

    /// generate placeholder types with stubs for potentially missing features
    pub fn write_placeholders<T>(&self, out_path: T)
    where
        T: AsRef<Path>,
    {
        let mut out = String::new();
        out.push_str("use crate::ids::*;\n");
        out.push_str("use crate::params::Object;\n");
        out.push_str("use serde::{Deserialize, Serialize};\n");

        for (schema, feature) in self.feature_groups.iter() {
            out.push('\n');
            let (id_type, c_c) =
                self.schema_to_id_type(schema).unwrap_or_else(|| ("()".into(), CopyOrClone::Copy));
            let struct_type = self.schema_to_rust_type(schema);
            out.push_str(&format!("#[cfg(not(feature = \"{}\"))]\n", feature));
            out.push_str("#[derive(Clone, Debug, Default, Deserialize, Serialize)]\n");
            out.push_str(&format!("pub struct {} {{\n", struct_type));
            out.push_str(&format!("\tpub id: {},\n", id_type));
            out.push_str("}\n\n");
            out.push_str(&format!("#[cfg(not(feature = \"{}\"))]\n", feature));
            out.push_str(&format!("impl Object for {} {{\n", struct_type));
            out.push_str(&format!("\ttype Id = {};\n", id_type));
            out.push_str(&format!(
                "\tfn id(&self) -> Self::Id {{ self.id{} }}\n",
                match c_c {
                    CopyOrClone::Clone => ".clone()",
                    CopyOrClone::Copy => "",
                }
            ));
            out.push_str(&format!("\tfn object(&self) -> &'static str {{ \"{}\" }}\n", schema));
            out.push_str("}\n");
        }

        write(out_path.as_ref().join("placeholders.rs"), out.as_bytes()).unwrap();
    }

    pub fn write_version<T>(&self, out_path: T)
    where
        T: AsRef<Path>,
    {
        let mut out = String::new();
        out.push_str("use crate::ApiVersion;\n\n");
        out.push_str(&format!(
            "pub const VERSION: ApiVersion = ApiVersion::V{};",
            self.spec.version().replace('-', "_")
        ));

        write(out_path.as_ref().join("version.rs"), out.as_bytes()).unwrap();
    }

    #[tracing::instrument(skip_all)]
    pub fn get_files(&self) -> Vec<FileGenerator> {
        self.objects
            .iter()
            .filter(|o| !o.starts_with("deleted_"))
            .map(|o| FileGenerator::new(o.to_string()))
            .collect()
    }

    pub fn schema_to_id_type(&self, schema: &str) -> Option<(String, CopyOrClone)> {
        let schema = schema.replace('.', "_");
        self.id_mappings.get(schema.as_str()).map(ToOwned::to_owned)
    }

    #[tracing::instrument(skip(self))]
    pub fn schema_to_rust_type(&self, schema: &str) -> String {
        let schema = schema.replace('.', "_");
        if let Some(rename) = self.object_mappings.get(schema.as_str()) {
            rename.to_camel_case()
        } else {
            schema.to_camel_case()
        }
    }

    pub fn field_to_rust_type(
        &self,
        schema: &str,
        field: &str,
    ) -> Option<(&'static str, &'static str)> {
        let schema = schema.replace('.', "_");
        self.field_mappings.get(&(schema.as_str(), field)).copied()
    }

    pub fn schema_field(&self, parent: &str, field: &str) -> String {
        let parent_type = self.schema_to_rust_type(parent);
        format!("{}_{}", parent_type, field).to_snake_case()
    }
}

/// given a spec and a set of objects in that spec, metadatas a
/// map with the requests to implement for each of the types in the spec
pub fn metadata_requests<'a>(
    spec: &'a Spec,
    objects: &BTreeSet<&'a str>,
) -> BTreeMap<String, BTreeSet<&'a str>> {
    let mut requests = BTreeMap::<String, BTreeSet<_>>::new();
    for path in spec.paths() {
        let mut seg_iterator = path.trim_start_matches("/v1/").split('/');
        let object = match (seg_iterator.next(), seg_iterator.next(), seg_iterator.next()) {
            // handle special case for sessions
            (Some(x), Some("sessions"), _) => format!("{}.session", x),

            // special case for usage_records
            (_, _, Some("usage_records")) => "usage_records".to_string(),

            // special case: terminal is a resource, not an object
            (Some("terminal"), Some(x), _) => format!("terminal.{}", x),

            (Some(x), _, _) => x.to_string(),
            _ => {
                // this should never happen
                tracing::error!("path ignored: {path}");
                continue;
            }
        };

        // This isn't documented in the API reference so let's skip it
        if object == "account" {
            continue;
        }

        let seg_like = &object[0..object.len() - 1];
        if objects.contains(object.as_str()) {
            requests.entry(object).or_default().insert(path.as_str());
        } else if object.ends_with('s') && objects.contains(seg_like) {
            requests.entry(seg_like.to_string()).or_default().insert(path.as_str());
        }
    }
    requests
}

#[rustfmt::skip]
pub fn feature_groups() -> BTreeMap<&'static str, &'static str> {
   [
		// N.B. For now both `core` and `payment-methods` are always enabled.
		/*
		// Core Resources
		("balance", "core"),
		("balance_transaction", "core"),
		("charge", "core"),
		("customer", "core"),
		("dispute", "core"),
		("file", "core"),
		("file_link", "core"),
		("setup_intent", "core"),
		("payout", "core"),
		("platform_tax_fee", "core"),
		("product", "core"),
		("refund", "core"),
		("reserve_transaction", "core"),
		("token", "core"),
		// Payment Methods
                ("alipay_account", "payment-methods"),
		("bank_account", "payment-methods"),
		("payment_method", "payment-methods"),
		("source", "payment-methods"),
		*/

		// Checkout
		("checkout_session", "checkout"),

		// Billing (aka. Subscriptions)
		("coupon", "billing"),
		("discount", "billing"),
		("invoice", "billing"),
		("invoiceitem", "billing"),
        ("line_item", "billing"),
		("plan", "billing"),
		("subscription", "billing"),
		("subscription_item", "billing"),
		("subscription_schedule", "billing"),
 		("subscription_schedule_revision", "billing"),
        ("tax_id", "billing"),
		("tax_rate", "billing"),

		// Connect
		("account", "connect"),
		("application", "connect"),
		("application_fee", "connect"),
		("connect_collection_transfer", "connect"),
		("fee_refund", "connect"),
		("person", "connect"),
		("topup", "connect"),
		("transfer", "connect"),
		("transfer_reversal", "connect"),

		// Fraud
		("review", "fraud"),

		// Issuing
		("issuing.authorization", "issuing"),
		("issuing.card", "issuing"),
		("issuing.cardholder", "issuing"),
		("issuing.dispute", "issuing"),
		("issuing.transaction", "issuing"),

		// Orders
		("order", "orders"),
		("order_item", "orders"),
		("order_return", "orders"),
		("sku", "orders"),

		// Sigma
		("scheduled_query_run", "sigma"),

		// Webhooks Endpoints
		("webhook_endpoint", "webhook-endpoints"),
	]
	.iter()
	.copied()
	.collect()
}
