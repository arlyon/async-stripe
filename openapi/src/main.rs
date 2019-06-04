use heck::{CamelCase, SnakeCase};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::{json, Value as Json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

fn main() {
    let raw = fs::read_to_string("openapi/spec3.json").unwrap();
    let spec: Json = serde_json::from_str(&raw).unwrap();

    // Rename a few id types
    let mut id_renames = BTreeMap::new();
    id_renames.insert("fee_refund", "application_fee_refund");
    id_renames.insert("invoiceitem", "invoice_item");
    id_renames.insert("line_item", "invoice_line_item");
    id_renames.insert("source_transaction", "charge");

    // Compute additional metadata from spec.
    let mut ids = BTreeMap::new();
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
            if !schema["properties"]["id"].is_null() {
                if let Some(id_src) = schema["x-resourceId"].as_str() {
                    let id_type = if let Some(rename) = id_renames.get(&id_src) {
                        rename.to_camel_case() + "Id"
                    } else {
                        id_src.replace('.', "_").to_camel_case() + "Id"
                    };
                    ids.insert(schema_name, id_type);
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

    let mut schema_renames = BTreeMap::new();
    schema_renames.extend(vec![
        ("account_business_profile", "business_profile"),
        ("account_business_type", "business_type"),
        ("account_capabilities_card_payments", "capability_status"),
        ("account_capabilities_legacy_payments", "capability_status"),
        ("account_capabilities_platform_payments", "capability_status"),
        ("account_branding_settings", "branding_settings"),
        ("account_card_payments_settings", "card_payments_settings"),
        ("account_dashboard_settings", "dashboard_settings"),
        ("account_decline_charge_on", "decline_charge_on"),
        ("account_external_account", "external_account"),
        ("account_payments_settings", "payments_settings"),
        ("account_payout_settings", "payout_settings"),
        ("account_tos_acceptance", "tos_acceptance"),
        ("charge_fraud_details", "fraud_details"),
        ("charge_transfer_data", "transfer_data"),
        ("fee_refund", "application_fee_refund"),
        ("issuing_authorization_merchant_data", "merchant_data"),
        ("issuing.authorization_wallet_provider", "wallet_provider"),
        ("invoiceitem", "invoice_item"),
        ("legal_entity_company", "company"),
        ("legal_entity_japan_address", "address"),
        ("legal_entity_person_verification", "person_verification"),
        ("legal_entity_person_verification_document", "person_verification_document"),
        ("line_item", "invoice_line_item"),
        ("payment_method_card", "card_details"),
        ("payment_method_card_present", "card_present"),
        ("payment_method_card_wallet", "wallet_details"),
        ("payment_method_card_wallet_amex_express_checkout", "wallet_amex_express_checkout"),
        ("payment_method_card_wallet_apple_pay", "wallet_apple_pay"),
        ("payment_method_card_wallet_google_pay", "wallet_google_pay"),
        ("payment_method_card_wallet_masterpass", "wallet_masterpass"),
        ("payment_method_card_wallet_samsung_pay", "wallet_samsung_pay"),
        ("payment_method_card_wallet_visa_checkout", "wallet_visa_checkout"),
        ("payment_method_card_wallet_type", "wallet_type"),
    ]);

    let mut field_overrides = BTreeMap::new();
    field_overrides.extend(vec![
        (
            ("bank_account", "account_holder_type"),
            ("AccountHolderType", "Option<AccountHolderType>"),
        ),
        (("charge", "source"), ("PaymentSource", "Option<PaymentSource>")),
        (
            ("charge_fraud_details", "stripe_report"),
            ("FraudDetailsReport", "Option<FraudDetailsReport>"),
        ),
        (("customer", "default_source"), ("PaymentSource", "Option<Expandable<PaymentSource>>")),
        (("customer", "sources"), ("PaymentSource", "List<PaymentSource>")),
        (("invoice", "default_source"), ("PaymentSource", "Option<Expandable<PaymentSource>>")),
        (("invoiceitem", "period"), ("Period", "Option<Period>")),
        (("line_item", "period"), ("Period", "Option<Period>")),
        (
            ("issuing.authorization", "authorization_method"),
            ("IssuingAuthorizationMethod", "IssuingAuthorizationMethod"),
        ),
        (
            ("issuing_authorization_request", "reason"),
            ("IssuingAuthorizationReason", "IssuingAuthorizationReason"),
        ),
        (
            ("issuing_authorization_verification_data", "address_line1_check"),
            ("IssuingAuthorizationCheck", "IssuingAuthorizationCheck"),
        ),
        (
            ("issuing_authorization_verification_data", "address_zip_check"),
            ("IssuingAuthorizationCheck", "IssuingAuthorizationCheck"),
        ),
        (
            ("issuing_authorization_verification_data", "cvc_check"),
            ("IssuingAuthorizationCheck", "IssuingAuthorizationCheck"),
        ),
        (("issuing.card", "brand"), ("CardBrand", "CardBrand")),
        (
            ("issuing_card_authorization_controls", "allowed_categories"),
            ("MerchantCategory", "Option<Vec<MerchantCategory>>"),
        ),
        (
            ("issuing_card_authorization_controls", "blocked_categories"),
            ("MerchantCategory", "Option<Vec<MerchantCategory>>"),
        ),
        (
            ("issuing_card_authorization_controls", "spending_limits"),
            ("SpendingLimit", "Option<Vec<SpendingLimit>>"),
        ),
        (
            ("issuing_cardholder_authorization_controls", "allowed_categories"),
            ("MerchantCategory", "Option<Vec<MerchantCategory>>"),
        ),
        (
            ("issuing_cardholder_authorization_controls", "blocked_categories"),
            ("MerchantCategory", "Option<Vec<MerchantCategory>>"),
        ),
        (
            ("issuing_cardholder_authorization_controls", "spending_limits"),
            ("SpendingLimit", "Option<Vec<SpendingLimit>>"),
        ),
        (("payment_intent", "source"), ("PaymentSource", "Option<Expandable<PaymentSource>>")),
        (("payment_intent_next_action", "use_stripe_sdk"), ("", "Option<serde_json::Value>")),
        (("person", "dob"), ("Dob", "Option<Dob>")),
        (("sku", "attributes"), ("Metadata", "Option<Metadata>")),
        (
            ("subscription", "default_source"),
            ("PaymentSource", "Option<Expandable<PaymentSource>>"),
        ),
        (("transfer_schedule", "weekly_anchor"), ("Weekday", "Option<Weekday>")),
    ]);

    // Renames for `account` params
    schema_renames.extend(vec![
        ("create_account_company", "company_params"),
        ("update_account_company", "company_params"),
        ("create_account_individual", "person_params"),
        ("update_account_individual", "person_params"),
        ("create_account_requested_capabilities", "requested_capability"),
        ("update_account_requested_capabilities", "requested_capability"),
        ("create_account_settings", "account_settings_params"),
        ("update_account_settings", "account_settings_params"),
        ("account_settings_params_branding", "branding_settings_params"),
        ("account_settings_params_card_payments", "card_payments_settings_params"),
        ("account_settings_params_payments", "payments_settings_params"),
        ("account_settings_params_dashboard_settings", "dashboard_settings_params"),
        ("account_settings_params_payouts", "payout_settings_params"),
        ("create_account_tos_acceptance", "accept_tos"),
        ("update_account_tos_acceptance", "accept_tos"),
        ("card_payments_settings_params_decline_on", "decline_charge_on_params"),
        ("payout_settings_params_schedule", "transfer_schedule_params"),
        ("person_params_verification", "person_verification_params"),
        ("person_verification_params_document", "person_verification_document_params"),
        ("transfer_schedule_params_interval", "transfer_schedule_interval"),
    ]);
    field_overrides.extend(vec![
        (("create_account", "business_profile"), ("BusinessProfile", "Option<BusinessProfile>")),
        (("update_account", "business_profile"), ("BusinessProfile", "Option<BusinessProfile>")),
        (("company_params", "address"), ("Address", "Option<Address>")),
        (("company_params", "address_kana"), ("Address", "Option<Address>")),
        (("company_params", "address_kanji"), ("Address", "Option<Address>")),
        (("person_params", "address"), ("Address", "Option<Address>")),
        (("person_params", "address_kana"), ("Address", "Option<Address>")),
        (("person_params", "address_kanji"), ("Address", "Option<Address>")),
        (("person_params", "dob"), ("Dob", "Option<Dob>")),
        (
            ("create_payment_method", "billing_details"),
            ("BillingDetails", "Option<BillingDetails>"),
        ),
        (("transfer_schedule_params", "delay_days"), ("DelayDays", "Option<DelayDays>")),
        (("transfer_schedule_params", "weekly_anchor"), ("Weekday", "Option<Weekday>")),
    ]);

    // Renames for `charge` params
    schema_renames.extend(vec![
        ("create_charge_transfer_data", "transfer_data_params"),
        ("update_charge_fraud_details", "fraud_details_params"),
    ]);
    field_overrides.extend(vec![
        (("create_charge", "shipping"), ("Shipping", "Option<Shipping>")),
        (("create_charge", "source"), ("PaymentSourceParams", "Option<PaymentSourceParams<'a>>")),
        (("update_charge", "shipping"), ("Shipping", "Option<Shipping>")),
        (("fraud_details_params", "user_report"), ("FraudDetailsReport", "FraudDetailsReport")),
    ]);

    // Renames for `customer` params
    schema_renames.extend(vec![
        ("create_customer_invoice_settings", "customer_invoice_settings"),
        ("update_customer_invoice_settings", "customer_invoice_settings"),
        ("create_customer_tax_id_data", "tax_id_data"),
        ("create_customer_tax_info", "tax_info_params"),
        ("update_customer_tax_info", "tax_info_params"),
        ("tax_info_params_type", "tax_info_type"),
    ]);
    field_overrides.extend(vec![
        (("create_customer", "address"), ("Address", "Option<Address>")),
        (("update_customer", "address"), ("Address", "Option<Address>")),
        (("update_customer", "default_alipay_account"), ("AlipayAccountId", "Option<AlipayAccountId>")),
        (("update_customer", "default_bank_account"), ("BankAccountId", "Option<BankAccountId>")),
        (("update_customer", "default_card"), ("CardId", "Option<CardId>")),
        (("create_customer", "default_source"), ("PaymentSourceId", "Option<PaymentSourceId>")),
        (("update_customer", "default_source"), ("PaymentSourceId", "Option<PaymentSourceId>")),
        (("create_customer", "shipping"), ("ShippingParams", "Option<ShippingParams>")),
        (("update_customer", "shipping"), ("ShippingParams", "Option<ShippingParams>")),
        (("create_customer", "source"), ("PaymentSourceParams", "Option<PaymentSourceParams<'a>>")),
        (("update_customer", "source"), ("PaymentSourceParams", "Option<PaymentSourceParams<'a>>")),
        (("update_customer", "trial_end"), ("Scheduled", "Option<Scheduled>")),
        (
            ("customer_invoice_settings", "custom_fields"),
            ("CustomField", "Option<Vec<CustomField>>"),
        ),
    ]);
    // Renames for `invoice` params
    field_overrides.extend(vec![(
        ("create_invoice", "custom_fields"),
        ("CustomField", "Option<Vec<CustomField>>"),
    )]);

    // Renames for `invoiceitem` params
    schema_renames.extend(vec![
        ("create_invoiceitem", "create_invoice_item"),
        ("update_invoiceitem", "update_invoice_item"),
    ]);
    field_overrides.extend(vec![
        (("create_invoice_item", "period"), ("Period", "Option<Period>")),
        (("update_invoice_item", "period"), ("Period", "Option<Period>")),
    ]);

    // Renames for `order` params
    field_overrides.extend(vec![
        (("create_order", "shipping"), ("ShippingParams", "Option<ShippingParams>")),
        (("update_order", "shipping"), ("ShippingParams", "Option<ShippingParams>")),
    ]);

    // Renames for `payment_intent` params
    schema_renames.extend(vec![
        ("create_order_items", "order_item_params"),
        ("order_items_params_type", "order_item_type"),
    ]);
    field_overrides.extend(vec![
        (("create_payment_intent", "shipping"), ("ShippingParams", "Option<ShippingParams>")),
        (("update_payment_intent", "shipping"), ("ShippingParams", "Option<ShippingParams>")),
    ]);

    // Renames for `sku` params
    field_overrides.extend(vec![
        (("list_skus", "attributes"), ("Metadata", "Option<Metadata>")),
        (("create_sku", "attributes"), ("Metadata", "Option<Metadata>")),
        (("update_sku", "attributes"), ("Metadata", "Option<Metadata>")),
        (("create_sku", "inventory"), ("Inventory", "Option<Inventory>")),
        (("update_sku", "inventory"), ("Inventory", "Option<Inventory>")),
        (("create_sku", "package_dimensions"), ("PackageDimensions", "Option<PackageDimensions>")),
        (("update_sku", "package_dimensions"), ("PackageDimensions", "Option<PackageDimensions>")),
    ]);

    // Renames for `source` params
    schema_renames.extend(vec![
        ("create_source_mandate", "source_mandate_params"),
        ("update_source_mandate", "source_mandate_params"),
        ("source_mandate_params_acceptance", "source_acceptance_params"),
        ("source_mandate_params_interval", "source_mandate_interval"),
        ("source_mandate_params_notification_method", "source_mandate_notification_method"),
        ("source_acceptance_params_offline", "source_acceptance_offline_params"),
        ("source_acceptance_params_online", "source_acceptance_online_params"),
        ("create_source_receiver_refund_attributes_method", "source_refund_notification_method"),
    ]);
    field_overrides.extend(vec![
        (("create_source", "owner"), ("BillingDetails", "Option<BillingDetails>")),
        (("update_source", "owner"), ("BillingDetails", "Option<BillingDetails>")),
    ]);

    // Renames for `subscription` params
    field_overrides.extend(vec![
        (
            ("create_subscription", "billing_thresholds"),
            ("SubscriptionBillingThresholds", "Option<SubscriptionBillingThresholds>"),
        ),
        (
            ("update_subscription", "billing_thresholds"),
            ("SubscriptionBillingThresholds", "Option<SubscriptionBillingThresholds>"),
        ),
        (
            ("create_subscription_items", "billing_thresholds"),
            ("SubscriptionItemBillingThresholds", "Option<SubscriptionItemBillingThresholds>"),
        ),
        (
            ("update_subscription_item", "billing_thresholds"),
            ("SubscriptionItemBillingThresholds", "Option<SubscriptionItemBillingThresholds>"),
        ),
        (
            ("update_subscription_items", "billing_thresholds"),
            ("SubscriptionItemBillingThresholds", "Option<SubscriptionItemBillingThresholds>"),
        ),
        (("create_subscription", "trial_end"), ("Scheduled", "Option<Scheduled>")),
        (("update_subscription", "trial_end"), ("Scheduled", "Option<Scheduled>")),
    ]);

    // Renames for `subscription_schedule` params
    field_overrides.extend(vec![
        (("create_subscription_schedule", "start_date"), ("Scheduled", "Option<Scheduled>")),
        (("update_subscription_schedule", "start_date"), ("Scheduled", "Option<Scheduled>")),
        (("create_subscription_schedule_phases", "start_date"), ("Scheduled", "Option<Scheduled>")),
        (("update_subscription_schedule_phases", "start_date"), ("Scheduled", "Option<Scheduled>")),
        (("create_subscription_schedule_phases", "end_date"), ("Scheduled", "Option<Scheduled>")),
        (("update_subscription_schedule_phases", "end_date"), ("Scheduled", "Option<Scheduled>")),
        (("create_subscription_schedule_phases", "trial_end"), ("Scheduled", "Option<Scheduled>")),
        (("update_subscription_schedule_phases", "trial_end"), ("Scheduled", "Option<Scheduled>")),
    ]);

    // Renames for misc
    schema_renames.insert("webhook_endpoint_enabled_events", "event_filter");
    schema_renames.insert("webhook_endpoint_api_version", "api_version");
    schema_renames.insert("create_webhook_endpoint_enabled_events", "event_filter");
    schema_renames.insert("update_webhook_endpoint_enabled_events", "event_filter");
    field_overrides.insert(
        ("update_payment_method", "billing_details"),
        ("BillingDetails", "Option<BillingDetails>"),
    );
    field_overrides.insert(
        ("create_product", "package_dimensions"),
        ("PackageDimensions", "Option<PackageDimensions>"),
    );
    field_overrides.insert(
        ("update_product", "package_dimensions"),
        ("PackageDimensions", "Option<PackageDimensions>"),
    );
    field_overrides.insert(("create_plan_tiers", "up_to"), ("UpTo", "Option<UpTo>"));
    field_overrides.insert(("update_file_link", "expires_at"), ("Scheduled", "Option<Scheduled>"));
    field_overrides.insert(
        ("create_token_account", "business_type"),
        ("BusinessType", "Option<BusinessType>"),
    );
    field_overrides
        .insert(("create_token_account", "company"), ("CompanyParams", "Option<CompanyParams>"));
    field_overrides
        .insert(("create_token_account", "individual"), ("PersonParams", "Option<PersonParams>"));

    // Generate files
    let meta = Metadata {
        spec: &spec,
        ids,
        objects,
        dependents,
        requests,
        schema_renames,
        field_overrides,
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
    spec: &'a Json,
    /// A map of `objects` to their rust id type
    ids: BTreeMap<&'a str, String>,
    /// The set of schemas which should implement `Object`.
    /// These have both an `id` property and on `object` property.
    objects: BTreeSet<&'a str>,
    /// A one to many map of schema to depending types.
    dependents: BTreeMap<&'a str, BTreeSet<&'a str>>,
    /// How a particular schema should be renamed.
    schema_renames: BTreeMap<&'a str, &'a str>,
    /// An override for the rust-type of a particular object/field pair.
    field_overrides: BTreeMap<(&'a str, &'a str), (&'a str, &'a str)>,
    /// A one to many map of _objects_ to requests which should be
    /// implemented for that object.
    ///
    /// This is typically determined by the first segment in the path.
    requests: BTreeMap<&'a str, BTreeSet<&'a str>>,
}

impl<'a> Metadata<'a> {
    fn schema_to_rust_type(&self, schema: &str) -> String {
        if let Some(rename) = self.schema_renames.get(&schema) {
            rename.to_camel_case()
        } else {
            schema.replace('.', "_").to_camel_case()
        }
    }

    fn schema_field(&self, parent: &str, field: &str) -> String {
        let parent_type = self.schema_to_rust_type(parent);
        format!("{}_{}", parent_type, field).to_snake_case()
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

    fn insert_struct(&mut self, name: impl Into<String>, struct_: InferredStruct) {
        if let Err(other) = self.try_insert_struct(name, struct_.clone()) {
            panic!("conflicting structs are not compatible:\n\t{:?}\n\t!=\n\t{:?}", struct_, other);
        }
    }

    fn try_insert_struct(
        &mut self,
        name: impl Into<String>,
        struct_: InferredStruct,
    ) -> Result<(), &InferredStruct> {
        let name = name.into();
        if !self.inferred_structs.contains_key(&name) {
            self.inferred_structs.insert(name, struct_);
            return Ok(());
        }
        if let Some(other) = self.inferred_structs.get(&name) {
            if struct_.schema != other.schema {
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
    schema: Json,
}

#[derive(Clone, Debug, PartialEq)]
struct InferredParams {
    method: String,
    rust_type: String,
    parameters: Json,
}

fn gen_impl_object(meta: &Metadata, object: &str) -> String {
    let mut out = String::new();
    let mut state = Generated::default();
    let id_type = meta.ids.get(object);
    let struct_name = meta.schema_to_rust_type(object);
    let schema = &meta.spec["components"]["schemas"][object];
    let schema_title = schema["title"].as_str().unwrap();
    let deleted_schema = &meta.spec["components"]["schemas"][format!("deleted_{}", object)];
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
    if let Some(id_type) = id_type {
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
            out.push_str("    pub deleted: bool,\n");
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
            true,
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
            out.push_str(&gen_field(&mut state, meta, &schema_name, &key, &field, required, true));
        }
        out.push_str("}\n");

        // Set the schema to generated
        *state.generated_schemas.entry(schema_name).or_default() = true;
    }

    for (_, params) in state.inferred_parameters.clone() {
        let params_schema = params.rust_type.to_snake_case();
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
            match param["in"].as_str() {
                Some("query") | Some("form") => (),
                _ => continue,
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
                // TODO: Handle these unusual params
                "bank_account" | "card" | "destination" | "product" => continue,

                "metadata" => {
                    print_doc(&mut out);
                    initializers.push(("metadata".into(), "Metadata".into(), required));
                    state.use_params.insert("Metadata");
                    if required {
                        out.push_str("    pub metadata: Metadata,\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    pub metadata: Option<Metadata>,\n");
                    }
                }
                "expand" => {
                    print_doc(&mut out);
                    initializers.push(("expand".into(), "&'a [&'a str]".into(), false));
                    state.use_params.insert("Expand");
                    out.push_str("    #[serde(skip_serializing_if = \"Expand::is_empty\")]\n");
                    out.push_str("    pub expand: &'a [&'a str],\n");
                }
                "limit" => {
                    print_doc(&mut out);
                    initializers.push(("limit".into(), "u64".into(), false));
                    if required {
                        out.push_str("    pub limit: u64,\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    pub limit: Option<u64>,\n");
                    }
                }
                "ending_before" => {
                    print_doc(&mut out);
                    let cursor_type = id_type.as_ref().map(|x| x.as_str()).unwrap_or("str");
                    initializers.push(("ending_before".into(), cursor_type.into(), false));
                    if required {
                        panic!("unexpected \"required\" `ending_before` parameter");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    pub ending_before: Option<&'a ");
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
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    pub starting_after: Option<");
                        out.push_str(cursor_type);
                        out.push_str(">,\n");
                    }
                }
                _ => {
                    if let Some((use_path, rust_type)) =
                        meta.field_overrides.get(&(params_schema.as_str(), param_name))
                    {
                        print_doc(&mut out);
                        initializers.push((param_rename.into(), rust_type.to_string(), required));
                        match *use_path {
                            "" | "String" => (),
                            "Metadata" => {
                                state.use_params.insert("Metadata");
                            }
                            path if path.ends_with("Id") && path != "TaxId" => {
                                state.use_ids.insert(path.into());
                            }
                            path => {
                                state.use_resources.insert(path.into());
                            }
                        }
                        if rust_type.starts_with("Option<") {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                        }
                        out.push_str("    pub ");
                        out.push_str(&param_rename);
                        out.push_str(": ");
                        out.push_str(&rust_type);
                        out.push_str(",\n");
                    } else if meta.ids.contains_key(param_name)
                        && param["schema"]["type"].as_str() == Some("string")
                        && param_name != "tax_id"
                    {
                        let id_type = &meta.ids[param_name];
                        print_doc(&mut out);
                        initializers.push((param_name.into(), id_type.clone(), required));
                        state.use_ids.insert(id_type.clone());
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_name);
                            out.push_str(": ");
                            out.push_str(&id_type);
                            out.push_str(",\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_name);
                            out.push_str(": Option<");
                            out.push_str(&id_type);
                            out.push_str(">,\n");
                        }
                    } else if param["schema"]["type"].as_str() == Some("boolean") {
                        print_doc(&mut out);
                        initializers.push((param_rename.into(), "bool".into(), false));
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": bool,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<bool>,\n");
                        }
                    } else if param["schema"]["type"].as_str() == Some("integer") {
                        let rust_type = if param["schema"]["format"].as_str() == Some("unix-time")
                            || param_name.ends_with("_date")
                        {
                            state.use_params.insert("Timestamp");
                            "Timestamp"
                        } else if param_name == "monthly_anchor" {
                            "u8"
                        } else if param_name.contains("days") {
                            "u32"
                        } else if param_name.contains("count")
                            || param_name.contains("size")
                            || param_name.contains("quantity")
                        {
                            "u64"
                        } else {
                            "i64"
                        };

                        print_doc(&mut out);
                        initializers.push((param_rename.into(), rust_type.into(), required));
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": ");
                            out.push_str(&rust_type);
                            out.push_str(",\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<");
                            out.push_str(&rust_type);
                            out.push_str(">,\n");
                        }
                    } else if param["schema"]["type"].as_str() == Some("number") {
                        print_doc(&mut out);
                        initializers.push((param_rename.into(), "f64".into(), required));
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": f64,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<f64>,\n");
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
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": RangeQuery<Timestamp>,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<RangeQuery<Timestamp>>,\n");
                        }
                    } else if param["schema"]["type"].as_str() == Some("string")
                        && param["schema"]["enum"].is_array()
                    {
                        let enum_schema = meta.schema_field(object, param_rename);
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
                            let enum_schema = format!("{}_filter", enum_schema);
                            let enum_name = meta.schema_to_rust_type(&enum_schema);
                            state.insert_enum(enum_name.clone(), enum_);
                            enum_name
                        } else {
                            enum_name
                        };

                        print_doc(&mut out);
                        initializers.push((param_rename.into(), enum_name.clone(), required));
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": ");
                            out.push_str(&enum_name);
                            out.push_str(",\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<");
                            out.push_str(&enum_name);
                            out.push_str(">,\n");
                        }
                    } else if (param_name == "currency" || param_name.ends_with("_currency"))
                        && param["schema"]["type"].as_str() == Some("string")
                    {
                        print_doc(&mut out);
                        initializers.push((param_rename.into(), "Currency".into(), required));
                        state.use_resources.insert("Currency".into());
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Currency,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<Currency>,\n");
                        }
                    } else if param["schema"]["type"].as_str() == Some("string") {
                        print_doc(&mut out);
                        initializers.push((param_rename.into(), "&'a str".into(), required));
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": &'a str,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<&'a str>,\n");
                        }
                    } else if param["schema"]["type"].as_str() == Some("object")
                        || param["schema"]["type"].as_str() == Some("array")
                        || param["schema"]["$ref"].is_string()
                        || param["schema"]["anyOf"].is_array()
                    {
                        let rust_type = gen_field_rust_type(
                            &mut state,
                            meta,
                            &params.rust_type.to_snake_case(),
                            &param_rename,
                            &param["schema"],
                            required,
                            false,
                        );
                        initializers.push((param_rename.into(), rust_type.clone(), required));

                        print_doc(&mut out);
                        if !required {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                        }
                        out.push_str("    pub ");
                        out.push_str(param_rename);
                        out.push_str(": ");
                        out.push_str(&rust_type);
                        out.push_str(",\n");
                    } else if required {
                        panic!(
                            "error: skipped required parameter: {} => {:?}",
                            param_name, param["schema"]
                        );
                    } else {
                        eprintln!("warning: skipping optional parameter: {}", param_name);
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

    let mut emitted_structs = BTreeSet::new();
    while state
        .inferred_structs
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
        .difference(&emitted_structs)
        .any(|_| true)
    {
        for (struct_name, struct_) in state.inferred_structs.clone() {
            if emitted_structs.contains(&struct_name) {
                continue;
            } else {
                emitted_structs.insert(struct_name.clone());
                println!("struct {} {{ ... }}", struct_name);
            }

            let fields = match struct_.schema["properties"].as_object() {
                Some(some) => some,
                None => {
                    // TODO: Handle these objects
                    // eprintln!("warning: {} has no properties ({})", struct_name, struct_.schema);
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
                    false,
                ));
            }
            out.push_str("}\n");
        }
    }

    for (union_name, union_) in state.inferred_unions.clone() {
        println!("union {} {{ ... }}", union_name);

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
        println!("enum {} {{ ... }}", enum_name);

        out.push('\n');
        out.push_str(&format!(
            "/// An enum representing the possible values of an `{}`'s `{}` field.\n",
            enum_.parent, enum_.field
        ));
        out.push_str("#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]\n");
        out.push_str("#[serde(rename_all = \"snake_case\")]\n");
        out.push_str("pub enum ");
        out.push_str(&enum_name);
        out.push_str(" {\n");
        for wire_name in &enum_.options {
            if wire_name.trim().is_empty() {
                continue;
            }
            let variant_name = match wire_name.as_str() {
                "*" => "All".to_string(),
                n => {
                    if n.chars().next().unwrap().is_digit(10) {
                        format!("V{}", n.to_string().replace('-', "_"))
                    } else {
                        meta.schema_to_rust_type(wire_name)
                    }
                }
            };
            if variant_name.trim().is_empty() {
                panic!("unhandled enum variant: {:?}", wire_name)
            }
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
        for (n, type_) in state
            .use_resources
            .iter()
            .filter(|&x| {
                state.generated_schemas.keys().all(|sch| x != &meta.schema_to_rust_type(sch))
                    && !state.inferred_parameters.contains_key(x)
                    && !state.inferred_structs.contains_key(x)
                    && !state.inferred_unions.contains_key(x)
                    && !state.inferred_enums.contains_key(x)
            })
            .enumerate()
        {
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
    field: &Json,
    required: bool,
    default: bool,
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
    let rust_type =
        gen_field_rust_type(state, meta, object, &field_name, &field, required, default);
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
    field: &Json,
    required: bool,
    default: bool,
) -> String {
    if let Some(&(use_path, rust_type)) = meta.field_overrides.get(&(object, field_name)) {
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
            if default {
                // N.B. return immediately; if we want to use `Default` for bool rather than `Option`
                return "bool".into();
            } else {
                "bool".into()
            }
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
                let enum_schema = meta.schema_field(object, field_name);
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
            let element_type =
                gen_field_rust_type(state, meta, object, field_name, element, true, false);
            format!("Vec<{}>", element_type)
        }
        Some("object") => {
            if field["properties"]["object"]["enum"][0].as_str() == Some("list") {
                state.use_params.insert("List");
                let element = &field["properties"]["data"]["items"];
                let element_field_name = if field_name.ends_with("s") {
                    field_name[0..field_name.len() - 1].into()
                } else if field_name.ends_with("ies") {
                    format!("{}y", &field_name[0..field_name.len() - 3])
                } else {
                    field_name.into()
                };
                let element_type = gen_field_rust_type(
                    state,
                    meta,
                    object,
                    &element_field_name,
                    element,
                    true,
                    false,
                );

                // N.B. return immediately; we use `Default` for list rather than `Option`
                return format!("List<{}>", element_type);
            } else {
                let struct_schema = meta.schema_field(object, field_name);
                let struct_name = meta.schema_to_rust_type(&struct_schema);
                let struct_ = InferredStruct { field: field_name.into(), schema: field.clone() };
                state.insert_struct(struct_name.clone(), struct_);
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
                    gen_field_rust_type(state, meta, object, field_name, &any_of[0], true, false)
                } else if any_of.len() == 2 && any_of[1]["enum"][0].as_str() == Some("") {
                    gen_field_rust_type(state, meta, object, field_name, &any_of[0], true, false)
                } else if field["x-expansionResources"].is_object() {
                    let ty_ = gen_field_rust_type(
                        state,
                        meta,
                        object,
                        field_name,
                        &json!({
                            "oneOf": Json::Array(field["x-expansionResources"]["oneOf"].as_array().unwrap()
                            .iter()
                            .filter(|v| !v["$ref"].as_str().unwrap().starts_with("#/components/schemas/deleted_"))
                            .cloned()
                            .collect())
                        }),
                        true,
                        false,
                    );
                    state.use_params.insert("Expandable");
                    format!("Expandable<{}>", ty_)
                } else if any_of[0]["title"].as_str() == Some("range_query_specs") {
                    state.use_params.insert("RangeQuery");
                    state.use_params.insert("Timestamp");
                    "RangeQuery<Timestamp>".into()
                } else {
                    let union_schema = meta.schema_field(object, field_name);
                    let union_name = meta.schema_to_rust_type(&union_schema);
                    let union_ = InferredUnion {
                        field: field_name.into(),
                        schema_variants: any_of
                            .into_iter()
                            .map(|x| {
                                let schema_name = x["$ref"]
                                    .as_str()
                                    .expect(&format!(
                                        "invalid union for `{}.{}`:  {:?}",
                                        object, field_name, field
                                    ))
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
                panic!("unhandled field type for `{}.{}`: {}\n", object, field_name, field)
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
            let doc_comment = get_request["description"].as_str().unwrap_or_default();
            if ok_schema["properties"]["object"]["enum"][0].as_str() == Some("list") {
                if segments.len() == 1 {
                    let params_name = if rust_struct.ends_with("y") {
                        format!("List{}ies", &rust_struct[0..rust_struct.len() - 1])
                    } else {
                        format!("List{}s", rust_struct)
                    };
                    let params = InferredParams {
                        method: "list".into(),
                        rust_type: params_name.clone(),
                        parameters: get_request["parameters"].clone(),
                    };
                    state.inferred_parameters.insert(params_name.to_snake_case(), params);
                    state.use_params.insert("List");

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, doc_comment, 1);
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
                }
            } else if segments.len() == 2 {
                let id_param = get_request["parameters"]
                    .as_array()
                    .and_then(|arr| arr.iter().find(|p| p["in"].as_str() == Some("path")));
                let id_param = match id_param {
                    Some(p) => p,
                    None => continue,
                };
                let expand_param = get_request["parameters"]
                    .as_array()
                    .and_then(|arr| arr.iter().find(|p| p["name"].as_str() == Some("expand")));
                if let Some(id_type) = &object_id {
                    assert_eq!(id_param["required"].as_bool(), Some(true));
                    assert_eq!(id_param["style"].as_str(), Some("simple"));

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, doc_comment, 1);
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
            }
        }
        let post_request = &request["post"];
        if post_request.is_object() {
            let ok_schema =
                &post_request["responses"]["200"]["content"]["application/json"]["schema"];
            let err_schema =
                &post_request["responses"]["default"]["content"]["application/json"]["schema"];
            if ok_schema.is_null()
                || err_schema["$ref"].as_str() != Some("#/components/schemas/error")
            {
                continue; // skip generating this unusual request (for now...)
            }
            let return_type = if let Some(path) = ok_schema["$ref"].as_str() {
                assert!(ok_schema["nullable"].is_null());
                let schema = path.trim_start_matches("#/components/schemas/");
                meta.schema_to_rust_type(schema)
            } else {
                continue;
            };

            let doc_comment = post_request["description"].as_str().unwrap_or_default();
            if segments.len() == 1 {
                if !doc_comment.contains("Create") && !doc_comment.contains("create") {
                    continue; // skip requests which don't appear to be `create` for now
                }

                // Just make sure I don't miss anything unexpected
                let query_params: &[_] =
                    post_request["parameters"].as_array().map(|x| x.as_ref()).unwrap_or_default();
                assert!(query_params.is_empty());

                // Construct `parameters` from the request body schema
                let create_schema = &post_request["requestBody"]["content"]
                    ["application/x-www-form-urlencoded"]["schema"];
                let mut create_parameters = Vec::new();
                for (key, value) in create_schema["properties"].as_object().unwrap() {
                    create_parameters.push(json!({
                        "in": "form",
                        "name": key,
                        "description": value["description"],
                        "required": create_schema["required"].as_array().map(|arr| {
                            arr.iter().any(|v| v.as_str() == Some(&key))
                        }),
                        "schema": value,
                        "style": "deepObject"
                    }));
                }
                let params_name = format!("Create{}", rust_struct);
                let params = InferredParams {
                    method: "create".into(),
                    rust_type: params_name.clone(),
                    parameters: Json::Array(create_parameters),
                };
                state.inferred_parameters.insert(params_name.to_snake_case(), params);

                let mut out = String::new();
                out.push('\n');
                print_doc_comment(&mut out, doc_comment, 1);
                out.push_str("    pub fn create(client: &Client, params: ");
                out.push_str(&params_name);
                out.push_str("<'_>) -> Response<");
                out.push_str(&return_type);
                out.push_str("> {\n");
                out.push_str("        client.post_form(\"/");
                out.push_str(&segments.join("/"));
                out.push_str("\", &params)\n");
                out.push_str("    }\n");
                methods.push(out);
            } else if segments.len() == 2 {
                if !doc_comment.contains("Update") && !doc_comment.contains("update") {
                    continue; // skip requests which don't appear to be `update` for now
                }

                // Just make sure I don't miss anything unexpected
                let query_params: &[_] =
                    post_request["parameters"].as_array().map(|x| x.as_ref()).unwrap_or_default();
                assert_eq!(query_params.len(), 1);

                // Get the id parameter
                let id_param = post_request["parameters"]
                    .as_array()
                    .and_then(|arr| arr.iter().find(|p| p["in"].as_str() == Some("path")));
                let id_param = match id_param {
                    Some(p) => p,
                    None => continue,
                };

                // Construct `parameters` from the request body schema
                let update_schema = &post_request["requestBody"]["content"]
                    ["application/x-www-form-urlencoded"]["schema"];
                let mut update_parameters = Vec::new();
                for (key, value) in update_schema["properties"].as_object().unwrap() {
                    update_parameters.push(json!({
                        "in": "form",
                        "name": key,
                        "description": value["description"],
                        "required": update_schema["required"].as_array().map(|arr| {
                            arr.iter().any(|v| v.as_str() == Some(&key))
                        }),
                        "schema": value,
                        "style": "deepObject"
                    }));
                }
                let params_name = format!("Update{}", rust_struct);
                let params = InferredParams {
                    method: "update".into(),
                    rust_type: params_name.clone(),
                    parameters: Json::Array(update_parameters),
                };
                state.inferred_parameters.insert(params_name.to_snake_case(), params);

                if let Some(id_type) = &object_id {
                    assert_eq!(id_param["required"].as_bool(), Some(true));
                    assert_eq!(id_param["style"].as_str(), Some("simple"));

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, doc_comment, 1);
                    out.push_str("    pub fn update(client: &Client, id: &");
                    out.push_str(&id_type);
                    out.push_str(", params: ");
                    out.push_str(&params_name);
                    out.push_str("<'_>) -> Response<");
                    out.push_str(&return_type);
                    out.push_str("> {\n");
                    out.push_str("        client.post_form(");
                    out.push_str(&format!("&format!(\"/{}/{{}}\", id)", segments[0]));
                    out.push_str(", &params)\n");
                    out.push_str("    }\n");
                    methods.push(out);
                }
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

            let doc_comment = delete_request["description"].as_str().unwrap_or_default();
            if segments.len() == 2 {
                let id_param = delete_request["parameters"]
                    .as_array()
                    .and_then(|arr| arr.iter().find(|p| p["in"].as_str() == Some("path")));
                let id_param = match id_param {
                    Some(p) => p,
                    None => continue,
                };
                if let Some(id_type) = &object_id {
                    state.use_params.insert("Deleted");
                    assert_eq!(id_param["required"].as_bool(), Some(true));
                    assert_eq!(id_param["style"].as_str(), Some("simple"));

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, doc_comment, 1);
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

fn print_doc_comment(out: &mut String, description: &str, depth: u8) {
    if description.trim().is_empty() {
        return;
    }

    let doc = format_doc_comment(description);
    let mut doc_parts = doc.splitn(2, ". ");
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
            out.push_str(&part.replace('\n', " ").trim());
            if !part.ends_with(".") {
                out.push('.');
            }
            out.push('\n');
        }
    }
}

fn format_doc_comment(doc: &str) -> String {
    lazy_static! {
        static ref P_TAG: Regex = Regex::new("<p>|</p>").unwrap();
        static ref BR_TAG: Regex = Regex::new("<br ?/?>").unwrap();
        static ref A_DOC_TAG: Regex = Regex::new("<a href=\"/docs/([^\"]+)\">([^<]+)</a>").unwrap();
        static ref A_HASH_TAG: Regex = Regex::new("<a href=\"#([^\"]+)\">([^<]+)</a>").unwrap();
        static ref A_HTTP_TAG: Regex =
            Regex::new("<a href=\"(https?://[^\"]+)\">([^<]+)</a>").unwrap();
        static ref CODE_TAG: Regex = Regex::new("<code>|</code>").unwrap();
        static ref EM_TAG: Regex = Regex::new("<em>|</em>|<i>|</i>").unwrap();
        static ref STRONG_TAG: Regex = Regex::new("<strong>|</strong>|<b>|</b>").unwrap();
        static ref AMOUNT_OPEN_TAG: Regex = Regex::new("<amount>").unwrap();
        static ref AMOUNT_CLOSE_TAG: Regex = Regex::new("</amount>").unwrap();
        static ref CURRENCY_OPEN_TAG: Regex = Regex::new("<currency>").unwrap();
        static ref CURRENCY_CLOSE_TAG: Regex = Regex::new("</currency>").unwrap();
    }
    let doc = P_TAG.replace_all(&doc, "");
    let doc = BR_TAG.replace_all(&doc, "\n");
    let doc = A_DOC_TAG.replace_all(&doc, "[${2}](https://stripe.com/docs/${1})");
    let doc = A_HASH_TAG.replace_all(&doc, "[${2}](https://stripe.com/docs/api#${1})");
    let doc = A_HTTP_TAG.replace_all(&doc, "[$2]($1)");
    let doc = CODE_TAG.replace_all(&doc, "`");
    let doc = EM_TAG.replace_all(&doc, "_");
    let doc = STRONG_TAG.replace_all(&doc, "**");
    let doc = AMOUNT_OPEN_TAG.replace_all(&doc, "");
    let doc = AMOUNT_CLOSE_TAG.replace_all(&doc, "00"); // add cents to get correct "integer" argument
    let doc = CURRENCY_OPEN_TAG.replace_all(&doc, "$"); // add locale formatting (we can only support one easily in our rust docs...)
    let doc = CURRENCY_CLOSE_TAG.replace_all(&doc, "");
    doc.trim().into()
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
