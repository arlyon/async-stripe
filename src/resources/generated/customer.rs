// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{CouponId, CustomerId, PaymentMethodId, PaymentSourceId, PromotionCodeId};
use crate::params::{
    Deleted, Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp,
};
use crate::resources::{
    Address, CashBalance, Currency, Discount, InvoiceSettingRenderingOptions, PaymentMethod,
    PaymentSource, PaymentSourceParams, Shipping, Subscription, TaxId, TestHelpersTestClock,
};

/// The resource representing a Stripe "Customer".
///
/// For more details see <https://stripe.com/docs/api/customers/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Customer {
    /// Unique identifier for the object.
    pub id: CustomerId,

    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// Current balance, if any, being stored on the customer.
    ///
    /// If negative, the customer has credit to apply to their next invoice.
    /// If positive, the customer has an amount owed that will be added to their next invoice.
    /// The balance does not refer to any unpaid invoices; it solely takes into account amounts that have yet to be successfully applied to any invoice.
    /// This balance is only taken into account as invoices are finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,

    /// The current funds being held by Stripe on behalf of the customer.
    ///
    /// These funds can be applied towards payment intents with source "cash_balance".
    /// The settings[reconciliation_mode] field describes whether these funds are applied to such payment intents manually or automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<CashBalance>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) the customer can be charged in for recurring billing purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// ID of the default payment source for the customer.
    ///
    /// If you are using payment methods created via the PaymentMethods API, see the [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) field instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<Expandable<PaymentSource>>,

    /// Always true for a deleted object.
    #[serde(default)]
    pub deleted: bool,

    /// When the customer's latest invoice is billed by charging automatically, `delinquent` is `true` if the invoice's latest charge failed.
    ///
    /// When the customer's latest invoice is billed by sending an invoice, `delinquent` is `true` if the invoice isn't paid by its due date.  If an invoice is marked uncollectible by [dunning](https://stripe.com/docs/billing/automatic-collection), `delinquent` doesn't get reset to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delinquent: Option<bool>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Describes the current discount active on the customer, if there is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,

    /// The customer's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The current multi-currency balances, if any, being stored on the customer.
    ///
    /// If positive in a currency, the customer has a credit to apply to their next invoice denominated in that currency.
    /// If negative, the customer has an amount owed that will be added to their next invoice denominated in that currency.
    /// These balances do not refer to any unpaid invoices.
    /// They solely track amounts that have yet to be successfully applied to any invoice.
    /// A balance in a particular currency is only applied to any invoice as an invoice in that currency is finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_credit_balance: Option<i64>,

    /// The prefix for the customer used to generate unique invoice numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<InvoiceSettingCustomerSetting>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The suffix of the customer's next invoice number, e.g., 0001.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,

    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The customer's preferred locales (languages), ordered by preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,

    /// Mailing and shipping address for the customer.
    ///
    /// Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,

    /// The customer's payment sources, if any.
    #[serde(default)]
    pub sources: List<PaymentSource>,

    /// The customer's current subscriptions, if any.
    #[serde(default)]
    pub subscriptions: List<Subscription>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<CustomerTax>,

    /// Describes the customer's tax exemption status.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    /// When set to `reverse`, invoice and receipt PDFs include the text **"Reverse charge"**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<CustomerTaxExempt>,

    /// The customer's tax IDs.
    #[serde(default)]
    pub tax_ids: List<TaxId>,

    /// ID of the test clock this customer belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<Expandable<TestHelpersTestClock>>,
}

impl Customer {
    /// Returns a list of your customers.
    ///
    /// The customers are returned sorted by creation date, with the most recent customers appearing first.
    pub fn list(client: &Client, params: &ListCustomers<'_>) -> Response<List<Customer>> {
        client.get_query("/customers", &params)
    }

    /// Creates a new customer object.
    pub fn create(client: &Client, params: CreateCustomer<'_>) -> Response<Customer> {
        client.post_form("/customers", &params)
    }

    /// Retrieves a Customer object.
    pub fn retrieve(client: &Client, id: &CustomerId, expand: &[&str]) -> Response<Customer> {
        client.get_query(&format!("/customers/{}", id), &Expand { expand })
    }

    /// Updates the specified customer by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    /// For example, if you pass the **source** parameter, that becomes the customer’s active source (e.g., a card) to be used for all charges in the future.
    /// When you update a customer to a new valid card source by passing the **source** parameter: for each of the customer’s current subscriptions, if the subscription bills automatically and is in the `past_due` state, then the latest open invoice for the subscription with automatic collection enabled will be retried.
    /// This retry will not count as an automatic retry, and will not affect the next regularly scheduled payment for the invoice.
    /// Changing the **default_source** for a customer will not trigger this behavior.  This request accepts mostly the same arguments as the customer creation call.
    pub fn update(
        client: &Client,
        id: &CustomerId,
        params: UpdateCustomer<'_>,
    ) -> Response<Customer> {
        client.post_form(&format!("/customers/{}", id), &params)
    }

    /// Permanently deletes a customer.
    ///
    /// It cannot be undone.
    /// Also immediately cancels any active subscriptions on the customer.
    pub fn delete(client: &Client, id: &CustomerId) -> Response<Deleted<CustomerId>> {
        client.delete(&format!("/customers/{}", id))
    }
}

impl Object for Customer {
    type Id = CustomerId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "customer"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerTax {
    /// Surfaces if automatic tax computation is possible given the current customer location information.
    pub automatic_tax: CustomerTaxAutomaticTax,

    /// A recent IP address of the customer used for tax reporting and tax location inference.
    pub ip_address: Option<String>,

    /// The customer's location as identified by Stripe Tax.
    pub location: Option<CustomerTaxLocation>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerTaxLocation {
    /// The customer's country as identified by Stripe Tax.
    pub country: String,

    /// The data source used to infer the customer's location.
    pub source: CustomerTaxLocationSource,

    /// The customer's state, county, province, or region as identified by Stripe Tax.
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceSettingCustomerSetting {
    /// Default custom fields to be displayed on invoices for this customer.
    pub custom_fields: Option<Vec<InvoiceSettingCustomField>>,

    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    pub default_payment_method: Option<Expandable<PaymentMethod>>,

    /// Default footer to be displayed on invoices for this customer.
    pub footer: Option<String>,

    /// Default options for invoice PDF rendering for this customer.
    pub rendering_options: Option<InvoiceSettingRenderingOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceSettingCustomField {
    /// The name of the custom field.
    pub name: String,

    /// The value of the custom field.
    pub value: String,
}

/// The parameters for `Customer::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateCustomer<'a> {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// An integer amount in cents (or local equivalent) that represents the customer's current balance, which affect the customer's future invoices.
    ///
    /// A negative amount represents a credit that decreases the amount due on an invoice; a positive amount increases the amount due on an invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,

    /// Balance information and default balance settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<CreateCustomerCashBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,

    /// An arbitrary string that you can attach to a customer object.
    ///
    /// It is displayed alongside the customer in the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Customer's email address.
    ///
    /// It's displayed alongside the customer in your dashboard and can be useful for searching and tracking.
    /// This may be up to *512 characters*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The prefix for the customer used to generate unique invoice numbers.
    ///
    /// Must be 3–12 uppercase letters or numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<&'a str>,

    /// Default invoice settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CustomerInvoiceSettings>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,

    /// The sequence to be used on the customer's next invoice.
    ///
    /// Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodId>,

    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,

    /// Customer's preferred languages, ordered by preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,

    /// The API ID of a promotion code to apply to the customer.
    ///
    /// The customer will have a discount applied on all recurring payments.
    /// Charges you create through the API will not have the discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<PromotionCodeId>,

    /// The customer's shipping information.
    ///
    /// Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateCustomerShipping>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentSourceParams>,

    /// Tax details about the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<CreateCustomerTax>,

    /// The customer's tax exemption.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<CustomerTaxExemptFilter>,

    /// The customer's tax IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_data: Option<Vec<TaxIdData>>,

    /// ID of the test clock to attach to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}

impl<'a> CreateCustomer<'a> {
    pub fn new() -> Self {
        CreateCustomer {
            address: Default::default(),
            balance: Default::default(),
            cash_balance: Default::default(),
            coupon: Default::default(),
            description: Default::default(),
            email: Default::default(),
            expand: Default::default(),
            invoice_prefix: Default::default(),
            invoice_settings: Default::default(),
            metadata: Default::default(),
            name: Default::default(),
            next_invoice_sequence: Default::default(),
            payment_method: Default::default(),
            phone: Default::default(),
            preferred_locales: Default::default(),
            promotion_code: Default::default(),
            shipping: Default::default(),
            source: Default::default(),
            tax: Default::default(),
            tax_exempt: Default::default(),
            tax_id_data: Default::default(),
            test_clock: Default::default(),
            validate: Default::default(),
        }
    }
}

/// The parameters for `Customer::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListCustomers<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A case-sensitive filter on the list based on the customer's `email` field.
    ///
    /// The value must be a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<CustomerId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<CustomerId>,

    /// Provides a list of customers that are associated with the specified test clock.
    ///
    /// The response will not include customers with test clocks if this parameter is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<&'a str>,
}

impl<'a> ListCustomers<'a> {
    pub fn new() -> Self {
        ListCustomers {
            created: Default::default(),
            email: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            test_clock: Default::default(),
        }
    }
}

/// The parameters for `Customer::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateCustomer<'a> {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// An integer amount in cents (or local equivalent) that represents the customer's current balance, which affect the customer's future invoices.
    ///
    /// A negative amount represents a credit that decreases the amount due on an invoice; a positive amount increases the amount due on an invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,

    /// Balance information and default balance settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<UpdateCustomerCashBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,

    /// If you are using payment methods created via the PaymentMethods API, see the [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/update#update_customer-invoice_settings-default_payment_method) parameter.
    ///
    /// Provide the ID of a payment source already attached to this customer to make it this customer's default payment source.
    ///
    /// If you want to add a new payment source and make it the default, see the [source](https://stripe.com/docs/api/customers/update#update_customer-source) property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<PaymentSourceId>,

    /// An arbitrary string that you can attach to a customer object.
    ///
    /// It is displayed alongside the customer in the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Customer's email address.
    ///
    /// It's displayed alongside the customer in your dashboard and can be useful for searching and tracking.
    /// This may be up to *512 characters*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The prefix for the customer used to generate unique invoice numbers.
    ///
    /// Must be 3–12 uppercase letters or numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<&'a str>,

    /// Default invoice settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CustomerInvoiceSettings>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,

    /// The sequence to be used on the customer's next invoice.
    ///
    /// Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,

    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,

    /// Customer's preferred languages, ordered by preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,

    /// The API ID of a promotion code to apply to the customer.
    ///
    /// The customer will have a discount applied on all recurring payments.
    /// Charges you create through the API will not have the discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<PromotionCodeId>,

    /// The customer's shipping information.
    ///
    /// Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<UpdateCustomerShipping>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentSourceParams>,

    /// Tax details about the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<UpdateCustomerTax>,

    /// The customer's tax exemption.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<CustomerTaxExemptFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}

impl<'a> UpdateCustomer<'a> {
    pub fn new() -> Self {
        UpdateCustomer {
            address: Default::default(),
            balance: Default::default(),
            cash_balance: Default::default(),
            coupon: Default::default(),
            default_source: Default::default(),
            description: Default::default(),
            email: Default::default(),
            expand: Default::default(),
            invoice_prefix: Default::default(),
            invoice_settings: Default::default(),
            metadata: Default::default(),
            name: Default::default(),
            next_invoice_sequence: Default::default(),
            phone: Default::default(),
            preferred_locales: Default::default(),
            promotion_code: Default::default(),
            shipping: Default::default(),
            source: Default::default(),
            tax: Default::default(),
            tax_exempt: Default::default(),
            validate: Default::default(),
        }
    }
}

impl Paginable for ListCustomers<'_> {
    type O = Customer;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCustomerCashBalance {
    /// Settings controlling the behavior of the customer's cash balance,
    /// such as reconciliation of funds received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CreateCustomerCashBalanceSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCustomerShipping {
    /// Customer shipping address.
    pub address: CreateCustomerShippingAddress,

    /// Customer name.
    pub name: String,

    /// Customer phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCustomerTax {
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    ///
    /// Stripe recommends updating the IP address when a new PaymentMethod is attached or the address field on the customer is updated.
    /// We recommend against updating this field more frequently since it could result in unexpected tax location/reporting outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerInvoiceSettings {
    /// Default custom fields to be displayed on invoices for this customer.
    ///
    /// When updating, pass an empty string to remove previously-defined fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomerInvoiceSettingsCustomFields>>,

    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,

    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CustomerInvoiceSettingsRenderingOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxIdData {
    /// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: TaxIdType,

    /// Value of the tax ID.
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateCustomerCashBalance {
    /// Settings controlling the behavior of the customer's cash balance,
    /// such as reconciliation of funds received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<UpdateCustomerCashBalanceSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateCustomerShipping {
    /// Customer shipping address.
    pub address: UpdateCustomerShippingAddress,

    /// Customer name.
    pub name: String,

    /// Customer phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateCustomerTax {
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    ///
    /// Stripe recommends updating the IP address when a new PaymentMethod is attached or the address field on the customer is updated.
    /// We recommend against updating this field more frequently since it could result in unexpected tax location/reporting outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCustomerCashBalanceSettings {
    /// Controls how funds transferred by the customer are applied to payment intents and invoices.
    ///
    /// Valid options are `automatic` or `manual`.
    /// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconciliation_mode: Option<CreateCustomerCashBalanceSettingsReconciliationMode>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCustomerShippingAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerInvoiceSettingsCustomFields {
    /// The name of the custom field.
    ///
    /// This may be up to 30 characters.
    pub name: String,

    /// The value of the custom field.
    ///
    /// This may be up to 30 characters.
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerInvoiceSettingsRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<CustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateCustomerCashBalanceSettings {
    /// Controls how funds transferred by the customer are applied to payment intents and invoices.
    ///
    /// Valid options are `automatic` or `manual`.
    /// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconciliation_mode: Option<UpdateCustomerCashBalanceSettingsReconciliationMode>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateCustomerShippingAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// An enum representing the possible values of an `CreateCustomerCashBalanceSettings`'s `reconciliation_mode` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCustomerCashBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
}

impl CreateCustomerCashBalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCustomerCashBalanceSettingsReconciliationMode::Automatic => "automatic",
            CreateCustomerCashBalanceSettingsReconciliationMode::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreateCustomerCashBalanceSettingsReconciliationMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCustomerCashBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCustomerCashBalanceSettingsReconciliationMode {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `CustomerInvoiceSettingsRenderingOptions`'s `amount_tax_display` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl CustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            CustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay::ExcludeTax => "exclude_tax",
            CustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay::IncludeInclusiveTax => {
                "include_inclusive_tax"
            }
        }
    }
}

impl AsRef<str> for CustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn default() -> Self {
        Self::ExcludeTax
    }
}

/// An enum representing the possible values of an `CustomerTax`'s `automatic_tax` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerTaxAutomaticTax {
    Failed,
    NotCollecting,
    Supported,
    UnrecognizedLocation,
}

impl CustomerTaxAutomaticTax {
    pub fn as_str(self) -> &'static str {
        match self {
            CustomerTaxAutomaticTax::Failed => "failed",
            CustomerTaxAutomaticTax::NotCollecting => "not_collecting",
            CustomerTaxAutomaticTax::Supported => "supported",
            CustomerTaxAutomaticTax::UnrecognizedLocation => "unrecognized_location",
        }
    }
}

impl AsRef<str> for CustomerTaxAutomaticTax {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerTaxAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CustomerTaxAutomaticTax {
    fn default() -> Self {
        Self::Failed
    }
}

/// An enum representing the possible values of an `Customer`'s `tax_exempt` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl CustomerTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            CustomerTaxExempt::Exempt => "exempt",
            CustomerTaxExempt::None => "none",
            CustomerTaxExempt::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for CustomerTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CustomerTaxExempt {
    fn default() -> Self {
        Self::Exempt
    }
}

/// An enum representing the possible values of an `CreateCustomer`'s `tax_exempt` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerTaxExemptFilter {
    Exempt,
    None,
    Reverse,
}

impl CustomerTaxExemptFilter {
    pub fn as_str(self) -> &'static str {
        match self {
            CustomerTaxExemptFilter::Exempt => "exempt",
            CustomerTaxExemptFilter::None => "none",
            CustomerTaxExemptFilter::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for CustomerTaxExemptFilter {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerTaxExemptFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CustomerTaxExemptFilter {
    fn default() -> Self {
        Self::Exempt
    }
}

/// An enum representing the possible values of an `CustomerTaxLocation`'s `source` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerTaxLocationSource {
    BillingAddress,
    IpAddress,
    PaymentMethod,
    ShippingDestination,
}

impl CustomerTaxLocationSource {
    pub fn as_str(self) -> &'static str {
        match self {
            CustomerTaxLocationSource::BillingAddress => "billing_address",
            CustomerTaxLocationSource::IpAddress => "ip_address",
            CustomerTaxLocationSource::PaymentMethod => "payment_method",
            CustomerTaxLocationSource::ShippingDestination => "shipping_destination",
        }
    }
}

impl AsRef<str> for CustomerTaxLocationSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerTaxLocationSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CustomerTaxLocationSource {
    fn default() -> Self {
        Self::BillingAddress
    }
}

/// An enum representing the possible values of an `TaxIdData`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxIdType {
    AeTrn,
    AuAbn,
    AuArn,
    BgUic,
    BrCnpj,
    BrCpf,
    CaBn,
    CaGstHst,
    CaPstBc,
    CaPstMb,
    CaPstSk,
    CaQst,
    ChVat,
    ClTin,
    EgTin,
    EsCif,
    EuOssVat,
    EuVat,
    GbVat,
    GeVat,
    HkBr,
    HuTin,
    IdNpwp,
    IlVat,
    InGst,
    IsVat,
    JpCn,
    JpRn,
    JpTrn,
    KePin,
    KrBrn,
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NoVat,
    NzGst,
    PhTin,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    ThVat,
    TrTin,
    TwVat,
    UaVat,
    UsEin,
    ZaVat,
}

impl TaxIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxIdType::AeTrn => "ae_trn",
            TaxIdType::AuAbn => "au_abn",
            TaxIdType::AuArn => "au_arn",
            TaxIdType::BgUic => "bg_uic",
            TaxIdType::BrCnpj => "br_cnpj",
            TaxIdType::BrCpf => "br_cpf",
            TaxIdType::CaBn => "ca_bn",
            TaxIdType::CaGstHst => "ca_gst_hst",
            TaxIdType::CaPstBc => "ca_pst_bc",
            TaxIdType::CaPstMb => "ca_pst_mb",
            TaxIdType::CaPstSk => "ca_pst_sk",
            TaxIdType::CaQst => "ca_qst",
            TaxIdType::ChVat => "ch_vat",
            TaxIdType::ClTin => "cl_tin",
            TaxIdType::EgTin => "eg_tin",
            TaxIdType::EsCif => "es_cif",
            TaxIdType::EuOssVat => "eu_oss_vat",
            TaxIdType::EuVat => "eu_vat",
            TaxIdType::GbVat => "gb_vat",
            TaxIdType::GeVat => "ge_vat",
            TaxIdType::HkBr => "hk_br",
            TaxIdType::HuTin => "hu_tin",
            TaxIdType::IdNpwp => "id_npwp",
            TaxIdType::IlVat => "il_vat",
            TaxIdType::InGst => "in_gst",
            TaxIdType::IsVat => "is_vat",
            TaxIdType::JpCn => "jp_cn",
            TaxIdType::JpRn => "jp_rn",
            TaxIdType::JpTrn => "jp_trn",
            TaxIdType::KePin => "ke_pin",
            TaxIdType::KrBrn => "kr_brn",
            TaxIdType::LiUid => "li_uid",
            TaxIdType::MxRfc => "mx_rfc",
            TaxIdType::MyFrp => "my_frp",
            TaxIdType::MyItn => "my_itn",
            TaxIdType::MySst => "my_sst",
            TaxIdType::NoVat => "no_vat",
            TaxIdType::NzGst => "nz_gst",
            TaxIdType::PhTin => "ph_tin",
            TaxIdType::RuInn => "ru_inn",
            TaxIdType::RuKpp => "ru_kpp",
            TaxIdType::SaVat => "sa_vat",
            TaxIdType::SgGst => "sg_gst",
            TaxIdType::SgUen => "sg_uen",
            TaxIdType::SiTin => "si_tin",
            TaxIdType::ThVat => "th_vat",
            TaxIdType::TrTin => "tr_tin",
            TaxIdType::TwVat => "tw_vat",
            TaxIdType::UaVat => "ua_vat",
            TaxIdType::UsEin => "us_ein",
            TaxIdType::ZaVat => "za_vat",
        }
    }
}

impl AsRef<str> for TaxIdType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxIdType {
    fn default() -> Self {
        Self::AeTrn
    }
}

/// An enum representing the possible values of an `UpdateCustomerCashBalanceSettings`'s `reconciliation_mode` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateCustomerCashBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
}

impl UpdateCustomerCashBalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateCustomerCashBalanceSettingsReconciliationMode::Automatic => "automatic",
            UpdateCustomerCashBalanceSettingsReconciliationMode::Manual => "manual",
        }
    }
}

impl AsRef<str> for UpdateCustomerCashBalanceSettingsReconciliationMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateCustomerCashBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateCustomerCashBalanceSettingsReconciliationMode {
    fn default() -> Self {
        Self::Automatic
    }
}
