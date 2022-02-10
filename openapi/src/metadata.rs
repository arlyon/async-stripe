use std::collections::{BTreeMap, BTreeSet};

use heck::{CamelCase, SnakeCase};
use serde_json::Value;

use crate::{
    file_generator::FileGenerator,
    mappings::{self, FieldMap, ObjectMap},
    metadata,
    types::CopyOrClone,
};

/// Global metadata for the entire codegen process.
#[derive(Debug)]
pub struct Metadata<'a> {
    pub spec: &'a Value,
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
    pub fn from_spec(spec: &'a Value) -> Self {
        let id_renames = mappings::id_renames();
        let object_mappings = mappings::object_mappings();
        let field_mappings = mappings::field_mappings();
        let feature_groups = metadata::feature_groups();

        let mut objects = BTreeSet::new();
        let mut dependents: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
        let mut id_mappings = BTreeMap::new();

        for (key, schema) in spec["components"]["schemas"].as_object().unwrap() {
            let schema_name = key.as_str();
            let fields = match schema["properties"].as_object() {
                Some(some) => some,
                None => continue,
            };
            if fields.contains_key("object") {
                objects.insert(schema_name);
                if !schema["properties"]["id"].is_null() {
                    if let Some(id_src) = schema["x-resourceId"].as_str() {
                        let id_type = if let Some(rename) = id_renames.get(&id_src) {
                            rename.to_camel_case() + "Id"
                        } else {
                            id_src.replace('.', "_").to_camel_case() + "Id"
                        };
                        id_mappings.insert(
                            schema_name.replace('.', "_").to_owned(),
                            (id_type, CopyOrClone::Clone),
                        );
                    }
                }
            }
            for (_, field) in fields {
                if let Some(path) = field["$ref"].as_str() {
                    let dep = path.trim_start_matches("#/components/schemas/");
                    dependents.entry(dep).or_default().insert(schema_name);
                }
                if let Some(any_of) = field["anyOf"].as_array() {
                    for ty in any_of {
                        if let Some(path) = ty["$ref"].as_str() {
                            let dep = path.trim_start_matches("#/components/schemas/");
                            dependents.entry(dep).or_default().insert(schema_name);
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
    spec: &'a Value,
    objects: &BTreeSet<&'a str>,
) -> BTreeMap<String, BTreeSet<&'a str>> {
    let mut requests = BTreeMap::<String, BTreeSet<_>>::new();
    for (path, _) in spec["paths"].as_object().unwrap() {
        let mut seg_iterator = path.trim_start_matches("/v1/").split('/');
        let object = match (seg_iterator.next(), seg_iterator.next()) {
            // handle special case for sessions
            (Some(x), Some("sessions")) => format!("{}.session", x),
            (Some(x), _) => x.to_string(),
            _ => continue,
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
		("recipient", "connect"),
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
