use heck::{CamelCase, SnakeCase};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

fn main() {
    let raw = fs::read_to_string("openapi/spec3.json").unwrap();
    let spec: Value = serde_json::from_str(&raw).unwrap();

    // Compute additional metadata from spec.
    let mut objects = BTreeSet::new();
    let mut dependents: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    for (key, schema) in spec["components"]["schemas"].as_object().unwrap() {
        let schema_name = key.as_str();
        let fields = match schema["properties"].as_object() {
            Some(some) => some,
            None => continue,
        };
        if fields.contains_key("object") {
            objects.insert(schema_name);
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

    let mut requests: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    for (path, _) in spec["paths"].as_object().unwrap() {
        let seg = path.trim_start_matches("/v1/").split("/").into_iter().next().unwrap();
        let seg_like = &seg[0..seg.len() - 1];
        if seg == "account" {
            // This isn't documented in the API reference so let's skip it
            continue;
        }

        if objects.contains(seg) {
            requests.entry(seg).or_default().insert(path.as_str());
        } else if seg.ends_with("s") && objects.contains(seg_like) {
            requests.entry(seg_like).or_default().insert(path.as_str());
        }
    }

    let mut id_renames = BTreeMap::new();
    id_renames.insert("fee_refund", "application_fee_refund");
    id_renames.insert("invoiceitem", "invoice_item");
    id_renames.insert("line_item", "invoice_line_item");

    let mut schema_renames = BTreeMap::new();
    schema_renames.insert("account_business_profile", "business_profile");
    schema_renames.insert("account_branding_settings", "branding_settings");
    schema_renames.insert("account_card_payments_settings", "card_payments_settings");
    schema_renames.insert("account_dashboard_settings", "dashboard_settings");
    schema_renames.insert("account_decline_charge_on", "decline_charge_on");
    schema_renames.insert("account_external_account", "external_account");
    schema_renames.insert("account_payments_settings", "payments_settings");
    schema_renames.insert("account_payout_settings", "payout_settings");
    schema_renames.insert("account_tos_acceptance", "tos_acceptance");
    schema_renames.insert("charge_fraud_details", "fraud_details");
    schema_renames.insert("charge_transfer_data", "transfer_data");
    schema_renames.insert("fee_refund", "application_fee_refund");
    schema_renames.insert("issuing_authorization_merchant_data", "merchant_data");
    schema_renames.insert("issuing.authorization_wallet_provider", "wallet_provider");
    schema_renames.insert("invoiceitem", "invoice_item");
    schema_renames.insert("line_item", "invoice_line_item");
    schema_renames.insert("payment_method_card", "card_details");
    schema_renames.insert("payment_method_card_present", "card_present");
    schema_renames.insert("payment_method_card_wallet", "wallet_details");
    schema_renames
        .insert("payment_method_card_wallet_amex_express_checkout", "wallet_amex_express_checkout");
    schema_renames.insert("payment_method_card_wallet_apple_pay", "wallet_apple_pay");
    schema_renames.insert("payment_method_card_wallet_google_pay", "wallet_google_pay");
    schema_renames.insert("payment_method_card_wallet_masterpass", "wallet_masterpass");
    schema_renames.insert("payment_method_card_wallet_samsung_pay", "wallet_samsung_pay");
    schema_renames.insert("payment_method_card_wallet_visa_checkout", "wallet_visa_checkout");
    schema_renames.insert("payment_method_card_wallet_type", "wallet_type");

    let mut field_types = BTreeMap::new();
    field_types.insert(("account", "business_type"), ("BusinessType", "Option<BusinessType>"));
    field_types.insert(("account", "type"), ("AccountType", "AccountType"));
    field_types.insert(
        ("account_capabilities", "card_payments"),
        ("CapabilityStatus", "CapabilityStatus"),
    );
    field_types.insert(
        ("account_capabilities", "legacy_payments"),
        ("CapabilityStatus", "CapabilityStatus"),
    );
    field_types.insert(
        ("account_capabilities", "platform_payments"),
        ("CapabilityStatus", "CapabilityStatus"),
    );
    field_types.insert(
        ("balance_transaction", "status"),
        ("BalanceTransactionStatus", "BalanceTransactionStatus"),
    );
    field_types.insert(("charge", "source"), ("PaymentSource", "Option<PaymentSource>"));
    field_types.insert(("fee", "type"), ("FeeType", "FeeType"));
    field_types.insert(
        ("bank_account", "account_holder_type"),
        ("AccountHolderType", "Option<AccountHolderType>"),
    );
    field_types
        .insert(("bank_account", "status"), ("BankAccountStatus", "Option<BankAccountStatus>"));
    field_types.insert(("invoiceitem", "period"), ("Period", "Option<Period>"));
    field_types.insert(("line_item", "period"), ("Period", "Option<Period>"));
    field_types.insert(
        ("issuing.authorization", "authorization_method"),
        ("IssuingAuthorizationMethod", "IssuingAuthorizationMethod"),
    );
    field_types.insert(
        ("issuing_authorization_request", "reason"),
        ("IssuingAuthorizationReason", "IssuingAuthorizationReason"),
    );
    field_types.insert(
        ("issuing_authorization_verification_data", "address_line1_check"),
        ("IssuingAuthorizationCheck", "IssuingAuthorizationCheck"),
    );
    field_types.insert(
        ("issuing_authorization_verification_data", "address_zip_check"),
        ("IssuingAuthorizationCheck", "IssuingAuthorizationCheck"),
    );
    field_types.insert(
        ("issuing_authorization_verification_data", "cvc_check"),
        ("IssuingAuthorizationCheck", "IssuingAuthorizationCheck"),
    );
    field_types.insert(("issuing.card", "brand"), ("CardBrand", "CardBrand"));
    field_types.insert(
        ("issuing_card_authorization_controls", "allowed_categories"),
        ("MerchantCategory", "Option<Vec<MerchantCategory>>"),
    );
    field_types.insert(
        ("issuing_card_authorization_controls", "blocked_categories"),
        ("MerchantCategory", "Option<Vec<MerchantCategory>>"),
    );
    field_types.insert(
        ("issuing_card_authorization_controls", "spending_limits"),
        ("SpendingLimit", "Option<Vec<SpendingLimit>>"),
    );
    field_types.insert(
        ("issuing_cardholder_authorization_controls", "allowed_categories"),
        ("MerchantCategory", "Option<Vec<MerchantCategory>>"),
    );
    field_types.insert(
        ("issuing_cardholder_authorization_controls", "blocked_categories"),
        ("MerchantCategory", "Option<Vec<MerchantCategory>>"),
    );
    field_types.insert(
        ("issuing_cardholder_authorization_controls", "spending_limits"),
        ("SpendingLimit", "Option<Vec<SpendingLimit>>"),
    );
    field_types.insert(("payment_intent", "source"), ("PaymentSource", "Option<PaymentSource>"));
    field_types.insert(
        ("payment_intent_next_action", "use_stripe_sdk"),
        ("", "Option<serde_json::Value>"),
    );
    field_types.insert(("sku", "attributes"), ("Metadata", "Option<Metadata>"));
    field_types
        .insert(("subscription", "default_source"), ("PaymentSource", "Option<PaymentSource>"));

    // Generate files
    let meta = Metadata {
        spec: &spec,
        objects,
        dependents,
        requests,
        id_renames,
        schema_renames,
        field_types,
    };
    for object in &meta.objects {
        if object.starts_with("deleted_") {
            continue;
        }

        // Generate the types for the object
        let out = gen_impl_object(&meta, object);
        fs::write(
            &format!("openapi/out/{}.rs", object.replace('.', "_").to_snake_case()),
            out.as_bytes(),
        )
        .unwrap();
    }
}

struct Metadata<'a> {
    spec: &'a Value,
    /// The set of schemas which should implement `Object`.
    /// These have both an `id` property and on `object` property.
    objects: BTreeSet<&'a str>,
    /// A one to many map of schema to depending types.
    dependents: BTreeMap<&'a str, BTreeSet<&'a str>>,
    /// How a particular id type shouldd be renamed.
    id_renames: BTreeMap<&'a str, &'a str>,
    /// How a particular schema should be renamed.
    schema_renames: BTreeMap<&'a str, &'a str>,
    /// An override for the rust-type of a particular object/field pair.
    field_types: BTreeMap<(&'a str, &'a str), (&'a str, &'a str)>,
    /// A one to many map of _objects_ to requests which should be
    /// implemented for that object.
    ///
    /// This is typically determined by the first segment in the path.
    requests: BTreeMap<&'a str, BTreeSet<&'a str>>,
}

impl<'a> Metadata<'a> {
    fn id_to_rust_type(&self, id: &str) -> String {
        if let Some(rename) = self.id_renames.get(&id) {
            rename.to_camel_case() + "Id"
        } else {
            id.replace('.', "_").to_camel_case() + "Id"
        }
    }

    fn schema_to_rust_type(&self, schema: &str) -> String {
        if let Some(rename) = self.schema_renames.get(&schema) {
            rename.to_camel_case()
        } else {
            schema.replace('.', "_").to_camel_case()
        }
    }
}

#[derive(Default)]
struct Generated {
    /// The ids that must be imported in this file.
    use_ids: BTreeSet<String>,
    /// The config that must be imported in this file.
    use_config: BTreeSet<&'static str>,
    /// The params that must be imported in this file.
    use_params: BTreeSet<&'static str>,
    /// The resources that must be imported in this file.
    use_resources: BTreeSet<String>,
    /// Extra (simple) enums that were / will be generated in this file.
    inferred_enums: BTreeMap<String, InferredEnum>,
    /// Extra (complex) enums that were / will be generated in this file.
    inferred_unions: BTreeMap<String, InferredUnion>,
    /// Extra structs that were / will be generated in this file.
    inferred_structs: BTreeMap<String, InferredStruct>,
    /// The request parameter structs that were / will be generated in this file.
    inferred_parameters: BTreeMap<String, InferredParams>,
    /// The schemas that were / will be generated in this file.
    generated_schemas: BTreeMap<String, bool>,
}

impl Generated {
    fn insert_enum(&mut self, name: impl Into<String>, enum_: InferredEnum) {
        if let Err(other) = self.try_insert_enum(name, enum_.clone()) {
            panic!("conflicting enums are not compatible:\n\t{:?}\n\t!=\n\t{:?}", enum_, other);
        }
    }

    fn try_insert_enum(
        &mut self,
        name: impl Into<String>,
        enum_: InferredEnum,
    ) -> Result<(), &InferredEnum> {
        let name = name.into();
        let mut enum_ = enum_;
        enum_.options.sort();
        if !self.inferred_enums.contains_key(&name) {
            self.inferred_enums.insert(name, enum_);
            return Ok(());
        }
        if let Some(other) = self.inferred_enums.get(&name) {
            if enum_.options != other.options {
                return Err(other);
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct InferredEnum {
    parent: String,
    field: String,
    options: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct InferredUnion {
    field: String,
    schema_variants: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct InferredStruct {
    field: String,
    schema: Value,
}

#[derive(Clone, Debug, PartialEq)]
struct InferredParams {
    method: String,
    rust_type: String,
    parameters: Value,
}

fn gen_impl_object(meta: &Metadata, object: &str) -> String {
    let mut out = String::new();
    let mut state = Generated::default();
    let struct_name = meta.schema_to_rust_type(object);
    let schema = &meta.spec["components"]["schemas"][object];
    let schema_title = schema["title"].as_str().unwrap();
    let deleted_schema = &meta.spec["components"]["schemas"][format!("deleted_{}", object)];
    let id_type = if !schema["properties"]["id"].is_null() {
        schema["x-resourceId"].as_str().map(|id| meta.id_to_rust_type(id))
    } else {
        None
    };
    let fields = match schema["properties"].as_object() {
        Some(some) => some,
        None => return String::new(),
    };
    let object_literal = match fields["object"]["enum"][0].as_str() {
        Some(some) => some,
        None => return String::new(),
    };

    // Generate the struct type
    out.push_str("/// The resource representing a Stripe \"");
    out.push_str(schema_title);
    out.push_str("\".\n");
    if let Some(doc_url) = check_object_doc_url(object) {
        out.push_str("///\n");
        out.push_str("/// For more details see [");
        out.push_str(&doc_url);
        out.push_str("](");
        out.push_str(&doc_url);
        out.push_str(").\n");
    }
    out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
    out.push_str("pub struct ");
    out.push_str(&struct_name);
    out.push_str(" {\n");
    if let Some(id_type) = &id_type {
        state.use_ids.insert(id_type.clone());
        if let Some(doc) = schema["properties"]["id"]["description"].as_str() {
            print_doc_comment(&mut out, doc, 1);
        }
        out.push_str("    pub id: ");
        out.push_str(&id_type);
        out.push_str(",\n");
    }
    let mut did_emit_deleted = false;
    for (key, field) in fields {
        if key == "id" {
            continue;
        }
        if key == "object" {
            continue;
        }
        if !did_emit_deleted
            && !deleted_schema.is_null()
            && key.as_str().cmp(&"deleted") == std::cmp::Ordering::Greater
        {
            out.push('\n');
            out.push_str("    // Always true for a deleted object\n");
            out.push_str("    #[serde(default)]\n");
            out.push_str("    deleted: bool,\n");
            did_emit_deleted = true;
        }
        let required = schema["required"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|x| x.as_str()).any(|x| x == key))
            .unwrap_or(false);
        let force_optional = if deleted_schema["properties"].is_object() {
            deleted_schema["properties"][&key].is_null()
        } else {
            false
        };
        out.push('\n');
        out.push_str(&gen_field(
            &mut state,
            meta,
            object,
            &key,
            &field,
            required && !force_optional,
        ));
    }
    out.push_str("}\n");

    // Generate request methods
    out.push_str(&gen_impl_requests(
        &mut state,
        meta,
        object,
        id_type.as_ref().map(|x| x.as_str()),
    ));

    // Generate an `impl Object` block
    state.use_params.insert("Object");
    out.push('\n');
    out.push_str("impl Object for ");
    out.push_str(&struct_name);
    out.push_str(" {\n");
    out.push_str("    type Id = ");
    if let Some(id_type) = &id_type {
        out.push_str(&id_type);
        out.push_str(";\n");
        out.push_str("    fn id(&self) -> Self::Id {\n        self.id.clone()\n    }\n");
    } else {
        out.push_str("();\n");
        out.push_str("    fn id(&self) -> Self::Id {}\n");
    }
    out.push_str("    fn object(&self) -> &'static str {\n        \"");
    out.push_str(object_literal);
    out.push_str("\"\n    }\n");
    out.push_str("}\n");

    while let Some(schema_name) = state
        .generated_schemas
        .iter()
        .filter_map(|(k, &v)| if !v { Some(k) } else { None })
        .next()
        .cloned()
    {
        let struct_name = meta.schema_to_rust_type(&schema_name);
        out.push('\n');
        out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
        out.push_str("pub struct ");
        out.push_str(&struct_name);
        out.push_str(" {\n");
        for (key, field) in meta.spec["components"]["schemas"][&schema_name]["properties"]
            .as_object()
            .cloned()
            .unwrap_or_default()
        {
            let required = meta.spec["components"]["schemas"][&schema_name]["required"]
                .as_array()
                .map(|arr| arr.iter().filter_map(|x| x.as_str()).any(|x| x == key))
                .unwrap_or(false);
            out.push('\n');
            out.push_str(&gen_field(&mut state, meta, &schema_name, &key, &field, required));
        }
        out.push_str("}\n");

        // Set the schema to generated
        *state.generated_schemas.entry(schema_name).or_default() = true;
    }

    for (struct_name, struct_) in state.inferred_structs.clone() {
        let fields = match struct_.schema["properties"].as_object() {
            Some(some) => some,
            None => {
                // TODO: Handle these objects
                // eprintln!("warn: {} has no properties ({})", struct_name, struct_.schema);
                continue;
            }
        };
        out.push('\n');
        out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
        out.push_str("pub struct ");
        out.push_str(&struct_name.to_camel_case());
        out.push_str(" {\n");
        for (key, field) in fields {
            let required = struct_.schema["required"]
                .as_array()
                .map(|arr| arr.iter().filter_map(|x| x.as_str()).any(|x| x == key))
                .unwrap_or(false);
            out.push('\n');
            out.push_str(&gen_field(
                &mut state,
                meta,
                &struct_name.to_snake_case(),
                &key,
                &field,
                required,
            ));
        }
        out.push_str("}\n");
    }

    for (_, params) in state.inferred_parameters.clone() {
        let parameters = match params.parameters.as_array() {
            Some(some) => some.as_slice(),
            None => &[],
        };
        out.push('\n');
        out.push_str(&format!("/// The parameters for `{}::{}`.\n", struct_name, params.method));
        out.push_str("#[derive(Clone, Debug, Serialize)]\n");
        out.push_str("pub struct ");
        out.push_str(&params.rust_type);
        out.push_str("<'a> {\n");
        let mut initializers: Vec<(String, String, bool)> = Vec::new();
        for param in parameters {
            if param["in"].as_str() != Some("query") {
                continue;
            }

            let param_name = param["name"].as_str().unwrap();
            let param_rename = match param_name {
                "type" => "type_",
                other => other,
            };
            let print_doc = |out: &mut String| {
                out.push('\n');
                if let Some(doc) = param["description"].as_str() {
                    print_doc_comment(out, doc, 1);
                }
                if param_rename != param_name {
                    out.push_str(&format!("    #[serde(rename = \"{}\")]\n", param_name));
                }
            };
            let required = param["required"].as_bool() == Some(true);
            match param_name {
                "expand" => {
                    print_doc(&mut out);
                    initializers.push(("expand".into(), "&'a [&'a str]".into(), false));
                    state.use_params.insert("Expand");
                    out.push_str("    #[serde(skip_serializing_if = \"Expand::is_empty\")]\n");
                    out.push_str("    expand: &'a [&'a str],\n");
                }
                "limit" => {
                    print_doc(&mut out);
                    initializers.push(("limit".into(), "u64".into(), false));
                    if required {
                        out.push_str("    limit: u64,\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    limit: Option<u64>,\n");
                    }
                }
                "ending_before" => {
                    print_doc(&mut out);
                    let cursor_type = id_type.as_ref().map(|x| x.as_str()).unwrap_or("str");
                    initializers.push(("ending_before".into(), cursor_type.into(), false));
                    if required {
                        panic!("unexpected \"required\" `ending_before` parameter");
                    // out.push_str("    ending_before: &'a ");
                    // out.push_str(cursor_type);
                    // out.push_str(",\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    ending_before: Option<&'a ");
                        out.push_str(cursor_type);
                        out.push_str(">,\n");
                    }
                }
                "starting_after" => {
                    print_doc(&mut out);
                    let cursor_type = id_type.as_ref().map(|x| x.as_str()).unwrap_or("str");
                    initializers.push(("starting_after".into(), cursor_type.into(), false));
                    if required {
                        panic!("unexpected \"required\" `starting_after` parameter");
                    // out.push_str("    starting_after: &'a ");
                    // out.push_str(cursor_type);
                    // out.push_str(",\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    starting_after: Option<&'a ");
                        out.push_str(cursor_type);
                        out.push_str(">,\n");
                    }
                }
                "charge" => {
                    print_doc(&mut out);
                    initializers.push(("charge".into(), "ChargeId".into(), required));
                    state.use_ids.insert("ChargeId".into());
                    if required {
                        out.push_str("    charge: ChargeId,\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    charge: Option<ChargeId>,\n");
                    }
                }
                "customer" => {
                    print_doc(&mut out);
                    state.use_ids.insert("CustomerId".into());
                    initializers.push(("customer".into(), "CustomerId".into(), required));
                    if required {
                        out.push_str("    customer: CustomerId,\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    customer: Option<CustomerId>,\n");
                    }
                }
                "invoice" => {
                    print_doc(&mut out);
                    state.use_ids.insert("InvoiceId".into());
                    initializers.push(("invoice".into(), "InvoiceId".into(), required));
                    if required {
                        out.push_str("    invoice: InvoiceId,\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    invoice: Option<InvoiceId>,\n");
                    }
                }
                "plan" => {
                    print_doc(&mut out);
                    initializers.push(("plan".into(), "PlanId".into(), required));
                    state.use_ids.insert("PlanId".into());
                    if required {
                        out.push_str("    plan: PlanId,\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    plan: Option<PlanId>,\n");
                    }
                }
                "subscription" => {
                    print_doc(&mut out);
                    initializers.push(("subscription".into(), "SubscriptionId".into(), required));
                    state.use_ids.insert("SubscriptionId".into());
                    if required {
                        out.push_str("    subscription: SubscriptionId,\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    subscription: Option<SubscriptionId>,\n");
                    }
                }
                _ => {
                    if param["schema"]["type"].as_str() == Some("boolean") {
                        print_doc(&mut out);
                        initializers.push((param_rename.into(), "bool".into(), false));
                        if required {
                            out.push_str("    ");
                            out.push_str(param_rename);
                            out.push_str(": bool,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    ");
                            out.push_str(param_rename);
                            out.push_str(": Option<bool>,\n");
                        }
                    } else if param["schema"]["anyOf"][0]["title"].as_str()
                        == Some("range_query_specs")
                    {
                        print_doc(&mut out);
                        initializers.push((
                            param_rename.into(),
                            "RangeQuery<Timestamp>".into(),
                            required,
                        ));
                        state.use_params.insert("RangeQuery");
                        state.use_params.insert("Timestamp");
                        if required {
                            out.push_str("    ");
                            out.push_str(param_rename);
                            out.push_str(": RangeQuery<Timestamp>,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    ");
                            out.push_str(param_rename);
                            out.push_str(": Option<RangeQuery<Timestamp>>,\n");
                        }
                    } else if param["schema"]["type"].as_str() == Some("string")
                        && param["schema"]["enum"].is_array()
                    {
                        let enum_schema = format!("{}_{}", object, param_rename).to_snake_case();
                        let enum_name = meta.schema_to_rust_type(&enum_schema);
                        let enum_ = InferredEnum {
                            parent: params.rust_type.clone(),
                            field: param_rename.into(),
                            options: param["schema"]["enum"]
                                .as_array()
                                .unwrap()
                                .into_iter()
                                .map(|x| x.as_str().unwrap().into())
                                .collect(),
                        };
                        let inserted = state.try_insert_enum(enum_name.clone(), enum_.clone());
                        let enum_name = if inserted.is_err() {
                            let enum_schema =
                                format!("{}_{}_filter", object, param_rename).to_snake_case();
                            let enum_name = meta.schema_to_rust_type(&enum_schema);
                            state.insert_enum(enum_name.clone(), enum_);
                            enum_name
                        } else {
                            enum_name
                        };

                        print_doc(&mut out);
                        initializers.push((param_rename.into(), enum_name.clone(), required));
                        if required {
                            out.push_str("    ");
                            out.push_str(param_rename);
                            out.push_str(": ");
                            out.push_str(&enum_name);
                            out.push_str(",\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    ");
                            out.push_str(param_rename);
                            out.push_str(": Option<");
                            out.push_str(&enum_name);
                            out.push_str(">,\n");
                        }
                    } else if required {
                        panic!("error: skipped required parameter: {}", param_name);
                    } else {
                        eprintln!("warn: skipping optional parameter: {}", param_name);
                    }
                }
            }
        }
        out.push_str("}\n");
        out.push('\n');
        out.push_str("impl<'a> ");
        out.push_str(&params.rust_type);
        out.push_str("<'a> {\n");
        out.push_str("    pub fn new(");
        let mut required_count = 0;
        for (name, type_, required) in &initializers {
            if *required {
                if required_count > 0 {
                    out.push_str(", ");
                }
                out.push_str(&name);
                out.push_str(": ");
                out.push_str(&type_);
                required_count += 1;
            }
        }
        out.push_str(") -> Self {\n");
        out.push_str("        ");
        out.push_str(&params.rust_type);
        out.push_str(" {\n");
        for (name, _, required) in &initializers {
            out.push_str("            ");
            out.push_str(&name);
            if *required {
                out.push_str(",\n");
            } else {
                out.push_str(": Default::default(),\n");
            }
        }
        out.push_str("        }\n");
        out.push_str("    }\n");
        out.push_str("}\n");
    }

    for (union_name, union_) in state.inferred_unions.clone() {
        out.push('\n');
        out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
        out.push_str("#[serde(tag = \"object\", rename_all = \"snake_case\")]\n");
        out.push_str("pub enum ");
        out.push_str(&union_name.to_camel_case());
        out.push_str(" {\n");
        for variant_schema in &union_.schema_variants {
            let object_name = meta.spec["components"]["schemas"][&variant_schema]["properties"]
                ["object"]["enum"][0]
                .as_str()
                .unwrap();
            let variant_name = meta.schema_to_rust_type(object_name);
            let type_name = meta.schema_to_rust_type(&variant_schema);
            if variant_name.to_snake_case() != object_name {
                out.push_str("    #[serde(rename = \"");
                out.push_str(object_name);
                out.push_str("\")]\n");
            }
            out.push_str("    ");
            out.push_str(&variant_name);
            out.push_str("(");
            out.push_str(&type_name);
            out.push_str("),\n");
        }
        out.push_str("}\n");
    }

    for (enum_name, enum_) in state.inferred_enums.clone() {
        out.push('\n');
        out.push_str(&format!(
            "/// An enum representing the possible values of an `{}`'s `{}` field.\n",
            enum_.parent, enum_.field
        ));
        out.push_str("#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]\n");
        out.push_str("#[serde(rename_all = \"snake_case\")]\n");
        out.push_str("pub enum ");
        out.push_str(&enum_name.to_camel_case());
        out.push_str(" {\n");
        for wire_name in &enum_.options {
            let variant_name = wire_name.to_camel_case();
            if &variant_name.to_snake_case() != wire_name {
                out.push_str("    #[serde(rename = \"");
                out.push_str(wire_name);
                out.push_str("\")]\n");
            }
            out.push_str("    ");
            out.push_str(&variant_name);
            out.push_str(",\n");
        }
        out.push_str("}\n");
    }

    let mut prelude = String::new();
    prelude.push_str("// ======================================\n");
    prelude.push_str("// This file was automatically generated.\n");
    prelude.push_str("// ======================================\n\n");
    if state.use_config.len() > 0 {
        prelude.push_str("use crate::config::{");
        for (n, type_) in state.use_config.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(&type_);
        }
        prelude.push_str("};\n");
    }
    if state.use_ids.len() > 0 {
        prelude.push_str("use crate::ids::{");
        for (n, type_) in state.use_ids.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(&type_);
        }
        prelude.push_str("};\n");
    }
    if state.use_params.len() > 0 {
        prelude.push_str("use crate::params::{");
        for (n, type_) in state.use_params.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(&type_);
        }
        prelude.push_str("};\n");
    }
    if state.use_resources.len() > 0 {
        prelude.push_str("use crate::resources::{");
        for (n, type_) in state.use_resources.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(&type_);
        }
        prelude.push_str("};\n");
    }
    prelude.push_str("use serde_derive::{Deserialize, Serialize};\n");
    prelude.push('\n');

    // Done
    prelude + &out
}

fn gen_field(
    state: &mut Generated,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &Value,
    required: bool,
) -> String {
    let mut out = String::new();
    if let Some(doc) = field["description"].as_str() {
        print_doc_comment(&mut out, doc, 1);
    }
    let mut field_rename = field_name.to_snake_case();
    if field_rename == "type" {
        field_rename = "type_".into();
    }
    if &field_rename != field_name {
        out.push_str("    #[serde(rename = \"");
        out.push_str(field_name);
        out.push_str("\")]\n");
    }
    let rust_type = gen_field_rust_type(state, meta, object, &field_name, &field, required);
    if !required {
        if rust_type == "bool" || rust_type == "Metadata" || rust_type.starts_with("List<") {
            out.push_str("    #[serde(default)]\n");
        } else if rust_type.starts_with("Option<") {
            out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
        }
    }
    out.push_str("    pub ");
    out.push_str(&field_rename);
    out.push_str(": ");
    out.push_str(&rust_type);
    out.push_str(",\n");
    out
}

fn gen_field_rust_type(
    state: &mut Generated,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &Value,
    required: bool,
) -> String {
    if let Some(&(use_path, rust_type)) = meta.field_types.get(&(object, field_name)) {
        match use_path {
            "" | "String" => (),
            "Metadata" => {
                state.use_params.insert("Metadata");
            }
            _ => {
                state.use_resources.insert(use_path.into());
            }
        }
        return rust_type.into();
    }
    if field_name == "metadata" {
        state.use_params.insert("Metadata");
        return "Metadata".into();
    } else if field_name == "currency" || field_name.ends_with("_currency") {
        state.use_resources.insert("Currency".into());
        if !required || field["nullable"].as_bool() == Some(true) {
            return "Option<Currency>".into();
        } else {
            return "Currency".into();
        }
    } else if field_name == "created" {
        state.use_params.insert("Timestamp");
        if !required || field["nullable"].as_bool() == Some(true) {
            return "Option<Timestamp>".into();
        } else {
            return "Timestamp".into();
        }
    }

    let ty = match field["type"].as_str() {
        Some("boolean") => {
            // N.B. return immediately; we use `Default` for bool rather than `Option`
            return "bool".into();
        }
        Some("number") => "f64".into(),
        Some("integer") => {
            if field["format"].as_str() == Some("unix-time") || field_name.ends_with("_date") {
                state.use_params.insert("Timestamp");
                "Timestamp".into()
            } else if field_name == "monthly_anchor" {
                "u8".into()
            } else if field_name.contains("days") {
                "u32".into()
            } else if field_name.contains("count")
                || field_name.contains("size")
                || field_name.contains("quantity")
            {
                "u64".into()
            } else {
                "i64".into()
            }
        }
        Some("string") => {
            if let Some(variants) = field["enum"].as_array() {
                let enum_schema = format!("{}_{}", object, field_name).to_snake_case();
                let enum_name = meta.schema_to_rust_type(&enum_schema);
                let parent_type = meta.schema_to_rust_type(object);
                let enum_ = InferredEnum {
                    parent: parent_type,
                    field: field_name.into(),
                    options: variants.into_iter().map(|x| x.as_str().unwrap().into()).collect(),
                };
                state.insert_enum(enum_name.clone(), enum_);
                enum_name
            } else {
                "String".into()
            }
        }
        Some("array") => {
            let element = &field["items"];
            let element_type = gen_field_rust_type(state, meta, object, field_name, element, true);
            format!("Vec<{}>", element_type)
        }
        Some("object") => {
            if field["properties"]["object"]["enum"][0].as_str() == Some("list") {
                state.use_params.insert("List");
                let element = &field["properties"]["data"]["items"];
                let element_field_name = if field_name.ends_with("s") {
                    &field_name[0..field_name.len() - 1]
                } else {
                    field_name
                };
                let element_type =
                    gen_field_rust_type(state, meta, object, element_field_name, element, true);

                // N.B. return immediately; we use `Default` for list rather than `Option`
                return format!("List<{}>", element_type);
            } else {
                let struct_name =
                    format!("{}_{}", meta.schema_to_rust_type(object), field_name).to_camel_case();
                let struct_ = InferredStruct { field: field_name.into(), schema: field.clone() };
                state.inferred_structs.insert(struct_name.clone(), struct_);
                struct_name
            }
        }
        _ => {
            if let Some(path) = field["$ref"].as_str() {
                let schema_name = path.trim_start_matches("#/components/schemas/");
                let type_name = meta.schema_to_rust_type(schema_name);
                if schema_name != object {
                    if meta.objects.contains(schema_name)
                        || meta.dependents.get(schema_name).map(|x| x.len()).unwrap_or(0) > 1
                    {
                        state.use_resources.insert(type_name.clone());
                    } else if !state.generated_schemas.contains_key(schema_name) {
                        state.generated_schemas.insert(schema_name.into(), false);
                    }
                }
                type_name
            } else if let Some(any_of) = field["anyOf"].as_array().or(field["oneOf"].as_array()) {
                if any_of.len() == 1 {
                    gen_field_rust_type(state, meta, object, field_name, &any_of[0], true)
                } else if field["x-expansionResources"].is_object() {
                    let ty_ = gen_field_rust_type(
                        state,
                        meta,
                        object,
                        field_name,
                        &serde_json::json!({
                            "oneOf": Value::Array(field["x-expansionResources"]["oneOf"].as_array().unwrap()
                            .iter()
                            .filter(|v| !v["$ref"].as_str().unwrap().starts_with("#/components/schemas/deleted_"))
                            .cloned()
                            .collect())
                        }),
                        true,
                    );
                    state.use_params.insert("Expandable");
                    format!("Expandable<{}>", ty_)
                } else {
                    let union_schema = format!("{}_{}", object, field_name).to_snake_case();
                    let union_name = meta.schema_to_rust_type(&union_schema);
                    let union_ = InferredUnion {
                        field: field_name.into(),
                        schema_variants: any_of
                            .into_iter()
                            .map(|x| {
                                let schema_name = x["$ref"]
                                    .as_str()
                                    .unwrap()
                                    .trim_start_matches("#/components/schemas/");
                                let type_name = meta.schema_to_rust_type(schema_name);
                                if meta.objects.contains(schema_name)
                                    || meta
                                        .dependents
                                        .get(schema_name)
                                        .map(|x| x.len())
                                        .unwrap_or(0)
                                        > 1
                                {
                                    state.use_resources.insert(type_name.clone());
                                } else if !state.generated_schemas.contains_key(schema_name) {
                                    state.generated_schemas.insert(schema_name.into(), false);
                                }
                                schema_name.into()
                            })
                            .collect(),
                    };
                    state.inferred_unions.insert(union_name.clone(), union_);
                    union_name
                }
            } else {
                eprintln!("{}.{}: {}\n", object, field_name, field);
                unimplemented!()
            }
        }
    };
    if !required || field["nullable"].as_bool() == Some(true) {
        format!("Option<{}>", ty)
    } else {
        ty
    }
}

fn gen_impl_requests(
    state: &mut Generated,
    meta: &Metadata,
    object: &str,
    object_id: Option<&str>,
) -> String {
    let requests = match meta.requests.get(object) {
        Some(some) => some,
        None => return String::new(),
    };
    let rust_struct = meta.schema_to_rust_type(object);

    // Collect all methods we know how to auto-generate
    let mut methods = Vec::new();
    for path in requests {
        lazy_static! {
            static ref P_TAG: Regex = Regex::new("<p>|</p>").unwrap();
            static ref A_DOC_TAG: Regex =
                Regex::new("<a href=\"/docs/([^\"]+)\">([^<]+)</a>").unwrap();
        }

        let request = &meta.spec["paths"][path];
        let segments = path.trim_start_matches("/v1/").split("/").collect::<Vec<_>>();
        let get_request = &request["get"];
        if get_request.is_object() {
            let ok_schema =
                &get_request["responses"]["200"]["content"]["application/json"]["schema"];
            let err_schema =
                &get_request["responses"]["default"]["content"]["application/json"]["schema"];
            if ok_schema.is_null()
                || err_schema["$ref"].as_str() != Some("#/components/schemas/error")
            {
                continue; // skip generating this unusual request (for now...)
            }
            let doc_comment =
                P_TAG.replace_all(get_request["description"].as_str().unwrap_or_default(), "");
            let doc_comment =
                A_DOC_TAG.replace_all(&doc_comment, "[${2}](https://stripe.com/docs/${1})");
            if ok_schema["properties"]["object"]["enum"][0].as_str() == Some("list") {
                if segments.len() == 1 {
                    let params_name = format!("{}ListParams", rust_struct);
                    let params = InferredParams {
                        method: "list".into(),
                        rust_type: params_name.clone(),
                        parameters: get_request["parameters"].clone(),
                    };
                    state.inferred_parameters.insert(params_name.to_snake_case(), params);
                    state.use_params.insert("List");

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, &doc_comment, 1);
                    out.push_str("    pub fn list(client: &Client, params: ");
                    out.push_str(&params_name);
                    out.push_str("<'_>) -> Response<List<");
                    out.push_str(&rust_struct);
                    out.push_str(">> {\n");
                    out.push_str("        client.get_query(\"/");
                    out.push_str(&segments.join("/"));
                    out.push_str("\", &params)\n");
                    out.push_str("    }\n");
                    methods.push(out);
                } else {
                    // eprintln!("OTHER: {} {:?}", path, segments);
                }
            } else if segments.len() == 2 {
                let id_param = get_request["parameters"].as_array().and_then(|arr| {
                    arr.iter().find(|p| p["in"].as_str() == Some("path"))
                });
                let id_param = match id_param { Some(p) => p, None => continue };
                let expand_param = get_request["parameters"].as_array().and_then(|arr| {
                    arr.iter().find(|p| p["name"].as_str() == Some("expand"))
                });
                if let Some(id_type) = &object_id {
                    assert_eq!(id_param["required"].as_bool(), Some(true));
                    assert_eq!(id_param["style"].as_str(), Some("simple"));

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, &doc_comment, 1);
                    out.push_str("    pub fn retrieve(client: &Client, id: &");
                    out.push_str(&id_type);
                    if let Some(param) = expand_param {
                        state.use_params.insert("Expand");
                        assert_eq!(param["in"].as_str(), Some("query"));
                        out.push_str(", expand: &[&str]) -> Response<");
                        out.push_str(&rust_struct);
                        out.push_str("> {\n");
                        out.push_str("        client.get_query(");
                        out.push_str(&format!("&format!(\"/{}/{{}}\", id)", segments[0]));
                        out.push_str(", &Expand { expand })\n");
                    } else {
                        out.push_str(") -> Response<");
                        out.push_str(&rust_struct);
                        out.push_str("> {\n");
                        out.push_str("        client.get(/");
                        out.push_str(&format!("&format!(\"/{}/{{}}\", id)", segments[0]));
                        out.push_str(")\n");
                    }
                    out.push_str("    }\n");
                    methods.push(out);
                }
            } else {
                // eprintln!("OTHER: {} {:?}", path, segments);
            }
        }
        let delete_request = &request["delete"];
        if delete_request.is_object() {
            let ok_schema =
                &delete_request["responses"]["200"]["content"]["application/json"]["schema"];
            let err_schema =
                &delete_request["responses"]["default"]["content"]["application/json"]["schema"];
            if ok_schema.is_null()
                || err_schema["$ref"].as_str() != Some("#/components/schemas/error")
            {
                continue; // skip generating this unusual request (for now...)
            }

            let doc_comment =
                P_TAG.replace_all(get_request["description"].as_str().unwrap_or_default(), "");
            let doc_comment =
                A_DOC_TAG.replace_all(&doc_comment, "[${2}](https://stripe.com/docs/${1})");
            if segments.len() == 2 {
                let id_param = get_request["parameters"].as_array().and_then(|arr| {
                    arr.iter().find(|p| p["in"].as_str() == Some("path"))
                });
                let id_param = match id_param { Some(p) => p, None => continue };
                if let Some(id_type) = &object_id {
                    state.use_params.insert("Deleted");
                    assert_eq!(id_param["required"].as_bool(), Some(true));
                    assert_eq!(id_param["style"].as_str(), Some("simple"));

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, &doc_comment, 1);
                    out.push_str("    pub fn delete(client: &Client, id: &");
                    out.push_str(&id_type);
                    out.push_str(") -> Response<Deleted<");
                    out.push_str(&id_type);
                    out.push_str(">> {\n");
                    out.push_str("        client.delete(");
                    out.push_str(&format!("&format!(\"/{}/{{}}\", id)", segments[0]));
                    out.push_str(")\n");
                    out.push_str("    }\n");
                    methods.push(out);
                }
            }
        }
    }
    if methods.is_empty() {
        return String::new();
    }

    // Add imports
    state.use_config.insert("Client");
    state.use_config.insert("Response");

    // Output the impl block
    let mut out = String::new();
    out.push('\n');
    out.push_str("impl ");
    out.push_str(&rust_struct);
    out.push_str(" {\n");
    for method in methods {
        out.push_str(&method);
    }
    out.push_str("}\n");
    out
}

fn print_indent(out: &mut String, depth: u8) {
    for _ in 0..depth {
        out.push_str("    ");
    }
}

fn print_doc_comment(out: &mut String, comment: &str, depth: u8) {
    if comment.trim().is_empty() {
        return;
    }

    let mut doc_parts = comment.splitn(2, ". ");
    let head = doc_parts.next().unwrap().trim();
    for (i, line) in head.split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
        }
        print_indent(out, depth);
        if !line.is_empty() {
            out.push_str("/// ");
            out.push_str(line);
        } else {
            out.push_str("///");
        }
    }
    if !head.ends_with(".") {
        out.push('.');
    }
    out.push('\n');
    if let Some(tail) = doc_parts.next() {
        print_indent(out, depth);
        out.push_str("///\n");
        for part in tail.split(". ") {
            print_indent(out, depth);
            out.push_str("/// ");
            out.push_str(&part.replace('\n', " "));
            if !part.ends_with(".") {
                out.push('.');
            }
            out.push('\n');
        }
    }
}

fn check_object_doc_url(object: &str) -> Option<String> {
    let doc_url =
        format!("https://stripe.com/docs/api/{}s/object", object.replace('.', "_").to_snake_case());
    let cache_file =
        format!("openapi/cache/{}_object.html", object.replace('.', "_").to_snake_case());
    if let Some(cached) = fs::read_to_string(&cache_file).ok() {
        return if cached.is_empty() { None } else { Some(doc_url) };
    }

    // Make a request to the stripe docs to see if the path exists
    if let Ok(mut resp) = reqwest::get(&doc_url) {
        if resp.status().is_success() {
            let text = resp.text().unwrap();
            if text.contains("<title>Stripe API Reference") && text.contains("object</title>") {
                fs::write(&cache_file, text.as_bytes()).unwrap();
                return Some(doc_url);
            } else {
                panic!("fatal: documentation response didn't match the expected format.");
            }
        }
    }
    eprintln!("warning: could not determine doc_url for object `{}`", object);
    fs::write(&cache_file, "").unwrap();
    None
}
