// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{CouponId, CustomerId, PaymentMethodId, SourceId};
use crate::params::{Deleted, Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    Address, Currency, CustomField, Discount, PaymentMethod, PaymentSource, Scheduled, Shipping,
    ShippingParams, Subscription, TaxId,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Customer".
///
/// For more details see [https://stripe.com/docs/api/customers/object](https://stripe.com/docs/api/customers/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Customer {
    /// Unique identifier for the object.
    pub id: CustomerId,

    /// Current balance, if any, being stored on the customer's account.
    ///
    /// If negative, the customer has credit to apply to the next invoice.
    /// If positive, the customer has an amount owed that will be added to the next invoice.
    /// The balance does not refer to any unpaid invoices; it solely takes into account amounts that have yet to be successfully applied to any invoice.
    /// This balance is only taken into account as invoices are finalized.
    /// Note that the balance does not include unpaid invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<u64>,

    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) the customer can be charged in for recurring billing purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// ID of the default payment source for the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<Expandable<PaymentSource>>,

    // Always true for a deleted object
    #[serde(default)]
    deleted: bool,

    /// When the customer's latest invoice is billed by charging automatically, delinquent is true if the invoice's latest charge is failed.
    ///
    /// When the customer's latest invoice is billed by sending an invoice, delinquent is true if the invoice is not paid by its due date.
    #[serde(default)]
    pub delinquent: bool,

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

    /// The prefix for the customer used to generate unique invoice numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<InvoiceSettingCustomerSetting>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(default)]
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

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

    /// Describes the customer's tax exemption status.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    /// When set to `reverse`, invoice and receipt PDFs include the text **"Reverse charge"**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<CustomerTaxExempt>,

    /// The customer's tax IDs.
    #[serde(default)]
    pub tax_ids: List<TaxId>,

    /// The customer's tax information.
    ///
    /// Appears on invoices emailed to this customer.
    /// This field has been deprecated and will be removed in a future API version, for further information view the [migration guide](https://stripe.com/docs/billing/migration/taxes#moving-from-taxinfo-to-customer-tax-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_info: Option<TaxInfo>,

    /// Describes the status of looking up the tax ID provided in `tax_info`.
    ///
    /// This field has been deprecated and will be removed in a future API version, for further information view the [migration guide](https://stripe.com/docs/billing/migration/taxes#moving-from-taxinfo-to-customer-tax-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_info_verification: Option<TaxInfoVerification>,
}

impl Customer {
    /// Returns a list of your customers.
    ///
    /// The customers are returned sorted by creation date, with the most recent customers appearing first.
    pub fn list(client: &Client, params: ListCustomers<'_>) -> Response<List<Customer>> {
        client.get_query("/customers", &params)
    }

    /// Creates a new customer object.
    pub fn create(client: &Client, params: CreateCustomer<'_>) -> Response<Customer> {
        client.post_form("/customers", &params)
    }

    /// Retrieves the details of an existing customer.
    ///
    /// You need only supply the unique customer identifier that was returned upon customer creation.
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceSettingCustomerSetting {
    /// Default custom fields to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<InvoiceSettingCustomField>>,

    /// ID of the default payment method used for subscriptions and invoices for the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Expandable<PaymentMethod>>,

    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceSettingCustomField {
    /// The name of the custom field.
    pub name: String,

    /// The value of the custom field.
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxInfo {
    /// The customer's tax ID number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,

    /// The type of ID number.
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxInfoVerification {
    /// The state of verification for this customer.
    ///
    /// Possible values are `unverified`, `pending`, or `verified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The official name associated with the tax ID returned from the external provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,
}

/// The parameters for `Customer::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateCustomer<'a> {
    /// An integer amount in %s that represents the account balance for your customer.
    ///
    /// Account balances only affect invoices.
    /// A negative amount represents a credit that decreases the amount due on an invoice; a positive amount increases the amount due on an invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    account_balance: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coupon: Option<CouponId>,

    /// An arbitrary string that you can attach to a customer object.
    ///
    /// It is displayed alongside the customer in the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,

    /// Customer's email address.
    ///
    /// It's displayed alongside the customer in your dashboard and can be useful for searching and tracking.
    /// This may be up to *512 characters*.
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// The prefix for the customer used to generate unique invoice numbers.
    ///
    /// Must be 3–12 uppercase letters or numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_prefix: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_settings: Option<CustomerInvoiceSettings>,

    /// A set of key-value pairs that you can attach to a customer object.
    ///
    /// It can be useful for storing additional information about the customer in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Metadata>,

    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<PaymentMethodId>,

    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_locales: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping: Option<ShippingParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<SourceId>,

    /// The customer's tax exemption.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_exempt: Option<CustomerTaxExemptFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_id_data: Option<Vec<TaxIdData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_info: Option<TaxInfoParams>,
}

impl<'a> CreateCustomer<'a> {
    pub fn new() -> Self {
        CreateCustomer {
            account_balance: Default::default(),
            address: Default::default(),
            coupon: Default::default(),
            description: Default::default(),
            email: Default::default(),
            expand: Default::default(),
            invoice_prefix: Default::default(),
            invoice_settings: Default::default(),
            metadata: Default::default(),
            name: Default::default(),
            payment_method: Default::default(),
            phone: Default::default(),
            preferred_locales: Default::default(),
            shipping: Default::default(),
            source: Default::default(),
            tax_exempt: Default::default(),
            tax_id_data: Default::default(),
            tax_info: Default::default(),
        }
    }
}

/// The parameters for `Customer::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListCustomers<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<RangeQuery<Timestamp>>,

    /// A filter on the list based on the customer's `email` field.
    ///
    /// The value must be a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a CustomerId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a CustomerId>,
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
        }
    }
}

/// The parameters for `Customer::update`.
#[derive(Clone, Debug, Serialize)]
pub struct UpdateCustomer<'a> {
    /// An integer amount in %s that represents the account balance for your customer.
    ///
    /// Account balances only affect invoices.
    /// A negative amount represents a credit that decreases the amount due on an invoice; a positive amount increases the amount due on an invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    account_balance: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coupon: Option<CouponId>,

    /// ID of Alipay account to make the customer's new default for invoice payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    default_alipay_account: Option<&'a str>,

    /// ID of bank account to make the customer's new default for invoice payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    default_bank_account: Option<&'a str>,

    /// ID of card to make the customer's new default for invoice payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    default_card: Option<&'a str>,

    /// Provide the ID of a payment source already attached to this customer to make it this customer's default payment source.
    ///
    /// If you want to add a new payment source and make it the default, see the [source](https://stripe.com/docs/api/customers/update#update_customer-source) property.
    #[serde(skip_serializing_if = "Option::is_none")]
    default_source: Option<&'a str>,

    /// An arbitrary string that you can attach to a customer object.
    ///
    /// It is displayed alongside the customer in the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,

    /// Customer's email address.
    ///
    /// It's displayed alongside the customer in your dashboard and can be useful for searching and tracking.
    /// This may be up to *512 characters*.
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// The prefix for the customer used to generate unique invoice numbers.
    ///
    /// Must be 3–12 uppercase letters or numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_prefix: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_settings: Option<CustomerInvoiceSettings>,

    /// A set of key-value pairs that you can attach to a customer object.
    ///
    /// It can be useful for storing additional information about the customer in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Metadata>,

    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,

    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_locales: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping: Option<ShippingParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<SourceId>,

    /// The customer's tax exemption.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_exempt: Option<CustomerTaxExemptFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_info: Option<TaxInfoParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_end: Option<Scheduled>,
}

impl<'a> UpdateCustomer<'a> {
    pub fn new() -> Self {
        UpdateCustomer {
            account_balance: Default::default(),
            address: Default::default(),
            coupon: Default::default(),
            default_alipay_account: Default::default(),
            default_bank_account: Default::default(),
            default_card: Default::default(),
            default_source: Default::default(),
            description: Default::default(),
            email: Default::default(),
            expand: Default::default(),
            invoice_prefix: Default::default(),
            invoice_settings: Default::default(),
            metadata: Default::default(),
            name: Default::default(),
            phone: Default::default(),
            preferred_locales: Default::default(),
            shipping: Default::default(),
            source: Default::default(),
            tax_exempt: Default::default(),
            tax_info: Default::default(),
            trial_end: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomerInvoiceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxIdData {
    #[serde(rename = "type")]
    pub type_: TaxIdDataType,

    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxInfoParams {
    pub tax_id: String,

    #[serde(rename = "type")]
    pub type_: TaxInfoType,
}

/// An enum representing the possible values of an `Customer`'s `tax_exempt` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerTaxExempt {
    Exempt,
    None,
    Reverse,
}

/// An enum representing the possible values of an `CreateCustomer`'s `tax_exempt` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerTaxExemptFilter {
    Exempt,
    None,
    Reverse,
}

/// An enum representing the possible values of an `TaxIdData`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxIdDataType {
    AuAbn,
    EuVat,
    NzGst,
}

/// An enum representing the possible values of an `TaxInfoParams`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxInfoType {
    Vat,
}
