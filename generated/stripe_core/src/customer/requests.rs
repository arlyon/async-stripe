use stripe::{Client, Response};

impl stripe_core::customer::Customer {
    /// Search for customers you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
    /// Don’t use search in read-after-write flows where strict consistency is necessary.
    ///
    /// Under normal operating conditions, data is searchable in less than a minute.
    /// Occasionally, propagation of new or updated data can be up to an hour behind during outages.
    /// Search functionality is not available to merchants in India.
    pub fn search(client: &Client, params: SearchCustomer) -> Response<SearchReturned> {
        client.get_query("/customers/search", params)
    }
    /// Returns a list of your customers.
    ///
    /// The customers are returned sorted by creation date, with the most recent customers appearing first.
    pub fn list(
        client: &Client,
        params: ListCustomer,
    ) -> Response<stripe_types::List<stripe_core::customer::Customer>> {
        client.get_query("/customers", params)
    }
    /// Creates a new customer object.
    pub fn create(
        client: &Client,
        params: CreateCustomer,
    ) -> Response<stripe_core::customer::Customer> {
        client.send_form("/customers", params, http_types::Method::Post)
    }
    /// Retrieves a Customer object.
    pub fn retrieve(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
        params: RetrieveCustomer,
    ) -> Response<RetrieveReturned> {
        client.get_query(&format!("/customers/{customer}", customer = customer), params)
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
        customer: &stripe_core::customer::CustomerId,
        params: UpdateCustomer,
    ) -> Response<stripe_core::customer::Customer> {
        client.send_form(
            &format!("/customers/{customer}", customer = customer),
            params,
            http_types::Method::Post,
        )
    }
    /// Permanently deletes a customer.
    ///
    /// It cannot be undone.
    /// Also immediately cancels any active subscriptions on the customer.
    pub fn delete(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
    ) -> Response<stripe_core::customer::DeletedCustomer> {
        client.send(
            &format!("/customers/{customer}", customer = customer),
            http_types::Method::Delete,
        )
    }
    /// Returns a list of PaymentMethods for a given Customer.
    pub fn list_payment_methods(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
        params: ListPaymentMethodsCustomer,
    ) -> Response<stripe_types::List<stripe_core::payment_method::PaymentMethod>> {
        client.get_query(
            &format!("/customers/{customer}/payment_methods", customer = customer),
            params,
        )
    }
    /// Retrieves a PaymentMethod object for a given Customer.
    pub fn retrieve_payment_method(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
        payment_method: &stripe_core::payment_method::PaymentMethodId,
        params: RetrievePaymentMethodCustomer,
    ) -> Response<stripe_core::payment_method::PaymentMethod> {
        client.get_query(
            &format!(
                "/customers/{customer}/payment_methods/{payment_method}",
                customer = customer,
                payment_method = payment_method
            ),
            params,
        )
    }
    /// Returns a list of transactions that updated the customer’s [balances](https://stripe.com/docs/billing/customer/balance).
    pub fn balance_transactions(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
        params: BalanceTransactionsCustomer,
    ) -> Response<
        stripe_types::List<stripe_core::customer_balance_transaction::CustomerBalanceTransaction>,
    > {
        client.get_query(
            &format!("/customers/{customer}/balance_transactions", customer = customer),
            params,
        )
    }
    /// Create an incoming testmode bank transfer.
    pub fn fund_cash_balance(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
        params: FundCashBalanceCustomer,
    ) -> Response<stripe_core::customer_cash_balance_transaction::CustomerCashBalanceTransaction>
    {
        client.send_form(
            &format!("/test_helpers/customers/{customer}/fund_cash_balance", customer = customer),
            params,
            http_types::Method::Post,
        )
    }
    /// Retrieve funding instructions for a customer cash balance.
    ///
    /// If funding instructions do not yet exist for the customer, new funding instructions will be created.
    /// If funding instructions have already been created for a given customer, the same funding instructions will be retrieved.
    /// In other words, we will return the same funding instructions each time.
    pub fn create_funding_instructions(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
        params: CreateFundingInstructionsCustomer,
    ) -> Response<stripe_types::funding_instructions::FundingInstructions> {
        client.send_form(
            &format!("/customers/{customer}/funding_instructions", customer = customer),
            params,
            http_types::Method::Post,
        )
    }
    /// Removes the currently applied discount on a customer.
    pub fn delete_discount(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
    ) -> Response<stripe_core::discount::DeletedDiscount> {
        client.send(
            &format!("/customers/{customer}/discount", customer = customer),
            http_types::Method::Delete,
        )
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SearchReturned {
    pub data: Vec<stripe_core::customer::Customer>,
    pub has_more: bool,
    pub next_page: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SearchReturnedObject,
    /// The total number of objects that match the query, only accurate up to 10,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
    pub url: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SearchReturned {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SearchReturnedObject {
    SearchResult,
}

impl SearchReturnedObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SearchResult => "search_result",
        }
    }
}

impl AsRef<str> for SearchReturnedObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SearchReturnedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SearchCustomer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for pagination across multiple pages of results.
    ///
    /// Don't include this parameter on the first call.
    /// Use the next_page value returned in a previous response to request subsequent results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<&'a str>,
    /// The search query string.
    ///
    /// See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for customers](https://stripe.com/docs/search#query-fields-for-customers).
    pub query: &'a str,
}
impl<'a> SearchCustomer<'a> {
    pub fn new(query: &'a str) -> Self {
        Self {
            expand: Default::default(),
            limit: Default::default(),
            page: Default::default(),
            query,
        }
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListCustomer<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
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
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Provides a list of customers that are associated with the specified test clock.
    ///
    /// The response will not include customers with test clocks if this parameter is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<&'a str>,
}
impl<'a> ListCustomer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCustomer<'a> {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateCustomerAddress<'a>>,
    /// An integer amount in cents (or local equivalent) that represents the customer's current balance, which affect the customer's future invoices.
    ///
    /// A negative amount represents a credit that decreases the amount due on an invoice; a positive amount increases the amount due on an invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,
    /// Balance information and default balance settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<CreateCustomerCashBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The prefix for the customer used to generate unique invoice numbers.
    ///
    /// Must be 3–12 uppercase letters or numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<&'a str>,
    /// Default invoice settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreateCustomerInvoiceSettings<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// The sequence to be used on the customer's next invoice.
    ///
    /// Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<&'a str>,
    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// Customer's preferred languages, ordered by preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<&'a [&'a str]>,
    /// The API ID of a promotion code to apply to the customer.
    ///
    /// The customer will have a discount applied on all recurring payments.
    /// Charges you create through the API will not have the discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
    /// The customer's shipping information.
    ///
    /// Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateCustomerShipping<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    /// Tax details about the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<CreateCustomerTax<'a>>,
    /// The customer's tax exemption.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<CreateCustomerTaxExempt>,
    /// The customer's tax IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_data: Option<&'a [CreateCustomerTaxIdData<'a>]>,
    /// ID of the test clock to attach to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}
impl<'a> CreateCustomer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCustomerAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreateCustomerAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Balance information and default balance settings for this customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCustomerCashBalance {
    /// Settings controlling the behavior of the customer's cash balance,
    /// such as reconciliation of funds received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CreateCustomerCashBalanceSettings>,
}
impl CreateCustomerCashBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings controlling the behavior of the customer's cash balance,
/// such as reconciliation of funds received.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCustomerCashBalanceSettings {
    /// Controls how funds transferred by the customer are applied to payment intents and invoices.
    ///
    /// Valid options are `automatic` or `manual`.
    /// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconciliation_mode: Option<CreateCustomerCashBalanceSettingsReconciliationMode>,
}
impl CreateCustomerCashBalanceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls how funds transferred by the customer are applied to payment intents and invoices.
///
/// Valid options are `automatic` or `manual`.
/// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateCustomerCashBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
}

impl CreateCustomerCashBalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
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
/// Default invoice settings for this customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCustomerInvoiceSettings<'a> {
    /// Default custom fields to be displayed on invoices for this customer.
    ///
    /// When updating, pass an empty string to remove previously-defined fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<&'a [CreateCustomerInvoiceSettingsCustomFields<'a>]>,
    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,
    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,
    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CreateCustomerInvoiceSettingsRenderingOptions>,
}
impl<'a> CreateCustomerInvoiceSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Default custom fields to be displayed on invoices for this customer.
///
/// When updating, pass an empty string to remove previously-defined fields.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerInvoiceSettingsCustomFields<'a> {
    /// The name of the custom field.
    ///
    /// This may be up to 30 characters.
    pub name: &'a str,
    /// The value of the custom field.
    ///
    /// This may be up to 30 characters.
    pub value: &'a str,
}
impl<'a> CreateCustomerInvoiceSettingsCustomFields<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }
}
/// Default options for invoice PDF rendering for this customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCustomerInvoiceSettingsRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay>,
}
impl CreateCustomerInvoiceSettingsRenderingOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
///
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExcludeTax => "exclude_tax",
            Self::IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl AsRef<str> for CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The customer's shipping information.
///
/// Appears on invoices emailed to this customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerShipping<'a> {
    /// Customer shipping address.
    pub address: CreateCustomerShippingAddress<'a>,
    /// Customer name.
    pub name: &'a str,
    /// Customer phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> CreateCustomerShipping<'a> {
    pub fn new(address: CreateCustomerShippingAddress<'a>, name: &'a str) -> Self {
        Self { address, name, phone: Default::default() }
    }
}
/// Customer shipping address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCustomerShippingAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreateCustomerShippingAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tax details about the customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCustomerTax<'a> {
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    ///
    /// Stripe recommends updating the IP address when a new PaymentMethod is attached or the address field on the customer is updated.
    /// We recommend against updating this field more frequently since it could result in unexpected tax location/reporting outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,
}
impl<'a> CreateCustomerTax<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's tax exemption.
///
/// One of `none`, `exempt`, or `reverse`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateCustomerTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl CreateCustomerTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for CreateCustomerTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCustomerTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The customer's tax IDs.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerTaxIdData<'a> {
    /// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: CreateCustomerTaxIdDataType,
    /// Value of the tax ID.
    pub value: &'a str,
}
impl<'a> CreateCustomerTaxIdData<'a> {
    pub fn new(type_: CreateCustomerTaxIdDataType, value: &'a str) -> Self {
        Self { type_, value }
    }
}
/// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateCustomerTaxIdDataType {
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

impl CreateCustomerTaxIdDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AeTrn => "ae_trn",
            Self::AuAbn => "au_abn",
            Self::AuArn => "au_arn",
            Self::BgUic => "bg_uic",
            Self::BrCnpj => "br_cnpj",
            Self::BrCpf => "br_cpf",
            Self::CaBn => "ca_bn",
            Self::CaGstHst => "ca_gst_hst",
            Self::CaPstBc => "ca_pst_bc",
            Self::CaPstMb => "ca_pst_mb",
            Self::CaPstSk => "ca_pst_sk",
            Self::CaQst => "ca_qst",
            Self::ChVat => "ch_vat",
            Self::ClTin => "cl_tin",
            Self::EgTin => "eg_tin",
            Self::EsCif => "es_cif",
            Self::EuOssVat => "eu_oss_vat",
            Self::EuVat => "eu_vat",
            Self::GbVat => "gb_vat",
            Self::GeVat => "ge_vat",
            Self::HkBr => "hk_br",
            Self::HuTin => "hu_tin",
            Self::IdNpwp => "id_npwp",
            Self::IlVat => "il_vat",
            Self::InGst => "in_gst",
            Self::IsVat => "is_vat",
            Self::JpCn => "jp_cn",
            Self::JpRn => "jp_rn",
            Self::JpTrn => "jp_trn",
            Self::KePin => "ke_pin",
            Self::KrBrn => "kr_brn",
            Self::LiUid => "li_uid",
            Self::MxRfc => "mx_rfc",
            Self::MyFrp => "my_frp",
            Self::MyItn => "my_itn",
            Self::MySst => "my_sst",
            Self::NoVat => "no_vat",
            Self::NzGst => "nz_gst",
            Self::PhTin => "ph_tin",
            Self::RuInn => "ru_inn",
            Self::RuKpp => "ru_kpp",
            Self::SaVat => "sa_vat",
            Self::SgGst => "sg_gst",
            Self::SgUen => "sg_uen",
            Self::SiTin => "si_tin",
            Self::ThVat => "th_vat",
            Self::TrTin => "tr_tin",
            Self::TwVat => "tw_vat",
            Self::UaVat => "ua_vat",
            Self::UsEin => "us_ein",
            Self::ZaVat => "za_vat",
        }
    }
}

impl AsRef<str> for CreateCustomerTaxIdDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCustomerTaxIdDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum RetrieveReturned {
    Customer(stripe_core::customer::Customer),
    DeletedCustomer(stripe_core::customer::DeletedCustomer),
}
#[cfg(feature = "min-ser")]
impl crate::deser::StripeDeserialize for RetrieveReturned {
    fn deserialize(str: &str) -> Result<Self, crate::StripeError> {
        use miniserde::json::from_str;

        use crate::deser::StripeDeserialize;
        let maybe_deleted: crate::deser::MaybeDeleted = from_str(str)?;
        let deleted = maybe_deleted.deleted.unwrap_or(false);
        if deleted {
            Ok(Self::DeletedCustomer(StripeDeserialize::deserialize(str)?))
        } else {
            Ok(Self::Customer(StripeDeserialize::deserialize(str)?))
        }
    }
}

#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveCustomer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCustomer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomer<'a> {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateCustomerAddress<'a>>,
    /// An integer amount in cents (or local equivalent) that represents the customer's current balance, which affect the customer's future invoices.
    ///
    /// A negative amount represents a credit that decreases the amount due on an invoice; a positive amount increases the amount due on an invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,
    /// Balance information and default balance settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<UpdateCustomerCashBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// If you are using payment methods created via the PaymentMethods API, see the [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/update#update_customer-invoice_settings-default_payment_method) parameter.
    ///
    /// Provide the ID of a payment source already attached to this customer to make it this customer's default payment source.
    ///
    /// If you want to add a new payment source and make it the default, see the [source](https://stripe.com/docs/api/customers/update#update_customer-source) property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<&'a str>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The prefix for the customer used to generate unique invoice numbers.
    ///
    /// Must be 3–12 uppercase letters or numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<&'a str>,
    /// Default invoice settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<UpdateCustomerInvoiceSettings<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
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
    pub preferred_locales: Option<&'a [&'a str]>,
    /// The API ID of a promotion code to apply to the customer.
    ///
    /// The customer will have a discount applied on all recurring payments.
    /// Charges you create through the API will not have the discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
    /// The customer's shipping information.
    ///
    /// Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<UpdateCustomerShipping<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    /// Tax details about the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<UpdateCustomerTax<'a>>,
    /// The customer's tax exemption.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<UpdateCustomerTaxExempt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}
impl<'a> UpdateCustomer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdateCustomerAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Balance information and default balance settings for this customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerCashBalance {
    /// Settings controlling the behavior of the customer's cash balance,
    /// such as reconciliation of funds received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<UpdateCustomerCashBalanceSettings>,
}
impl UpdateCustomerCashBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings controlling the behavior of the customer's cash balance,
/// such as reconciliation of funds received.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerCashBalanceSettings {
    /// Controls how funds transferred by the customer are applied to payment intents and invoices.
    ///
    /// Valid options are `automatic` or `manual`.
    /// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconciliation_mode: Option<UpdateCustomerCashBalanceSettingsReconciliationMode>,
}
impl UpdateCustomerCashBalanceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls how funds transferred by the customer are applied to payment intents and invoices.
///
/// Valid options are `automatic` or `manual`.
/// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateCustomerCashBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
}

impl UpdateCustomerCashBalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
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
/// Default invoice settings for this customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerInvoiceSettings<'a> {
    /// Default custom fields to be displayed on invoices for this customer.
    ///
    /// When updating, pass an empty string to remove previously-defined fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<&'a [UpdateCustomerInvoiceSettingsCustomFields<'a>]>,
    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,
    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,
    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<UpdateCustomerInvoiceSettingsRenderingOptions>,
}
impl<'a> UpdateCustomerInvoiceSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Default custom fields to be displayed on invoices for this customer.
///
/// When updating, pass an empty string to remove previously-defined fields.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateCustomerInvoiceSettingsCustomFields<'a> {
    /// The name of the custom field.
    ///
    /// This may be up to 30 characters.
    pub name: &'a str,
    /// The value of the custom field.
    ///
    /// This may be up to 30 characters.
    pub value: &'a str,
}
impl<'a> UpdateCustomerInvoiceSettingsCustomFields<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }
}
/// Default options for invoice PDF rendering for this customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerInvoiceSettingsRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay>,
}
impl UpdateCustomerInvoiceSettingsRenderingOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
///
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExcludeTax => "exclude_tax",
            Self::IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl AsRef<str> for UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The customer's shipping information.
///
/// Appears on invoices emailed to this customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateCustomerShipping<'a> {
    /// Customer shipping address.
    pub address: UpdateCustomerShippingAddress<'a>,
    /// Customer name.
    pub name: &'a str,
    /// Customer phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> UpdateCustomerShipping<'a> {
    pub fn new(address: UpdateCustomerShippingAddress<'a>, name: &'a str) -> Self {
        Self { address, name, phone: Default::default() }
    }
}
/// Customer shipping address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerShippingAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdateCustomerShippingAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tax details about the customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerTax<'a> {
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    ///
    /// Stripe recommends updating the IP address when a new PaymentMethod is attached or the address field on the customer is updated.
    /// We recommend against updating this field more frequently since it could result in unexpected tax location/reporting outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,
}
impl<'a> UpdateCustomerTax<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's tax exemption.
///
/// One of `none`, `exempt`, or `reverse`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateCustomerTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl UpdateCustomerTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for UpdateCustomerTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateCustomerTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListPaymentMethodsCustomer<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// A required filter on the list, based on the object `type` field.
    #[serde(rename = "type")]
    pub type_: ListPaymentMethodsCustomerType,
}
impl<'a> ListPaymentMethodsCustomer<'a> {
    pub fn new(type_: ListPaymentMethodsCustomerType) -> Self {
        Self {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            type_,
        }
    }
}
/// A required filter on the list, based on the object `type` field.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListPaymentMethodsCustomerType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    CardPresent,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl ListPaymentMethodsCustomerType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::CardPresent => "card_present",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for ListPaymentMethodsCustomerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListPaymentMethodsCustomerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePaymentMethodCustomer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePaymentMethodCustomer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BalanceTransactionsCustomer<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> BalanceTransactionsCustomer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct FundCashBalanceCustomer<'a> {
    /// Amount to be used for this test cash balance transaction.
    ///
    /// A positive integer representing how much to fund in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to fund $1.00 or 100 to fund ¥100, a zero-decimal currency).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A description of the test funding.
    ///
    /// This simulates free-text references supplied by customers when making bank transfers to their cash balance.
    /// You can use this to test how Stripe's [reconciliation algorithm](https://stripe.com/docs/payments/customer-balance/reconciliation) applies to different user inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<&'a str>,
}
impl<'a> FundCashBalanceCustomer<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self { amount, currency, expand: Default::default(), reference: Default::default() }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFundingInstructionsCustomer<'a> {
    /// Additional parameters for `bank_transfer` funding types.
    pub bank_transfer: CreateFundingInstructionsCustomerBankTransfer<'a>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The `funding_type` to get the instructions for.
    pub funding_type: CreateFundingInstructionsCustomerFundingType,
}
impl<'a> CreateFundingInstructionsCustomer<'a> {
    pub fn new(
        bank_transfer: CreateFundingInstructionsCustomerBankTransfer<'a>,
        currency: stripe_types::Currency,
        funding_type: CreateFundingInstructionsCustomerFundingType,
    ) -> Self {
        Self { bank_transfer, currency, expand: Default::default(), funding_type }
    }
}
/// Additional parameters for `bank_transfer` funding types.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFundingInstructionsCustomerBankTransfer<'a> {
    /// Configuration for eu_bank_transfer funding type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<CreateFundingInstructionsCustomerBankTransferEuBankTransfer<'a>>,
    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types:
        Option<&'a [CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes]>,
    /// The type of the `bank_transfer`.
    #[serde(rename = "type")]
    pub type_: CreateFundingInstructionsCustomerBankTransferType,
}
impl<'a> CreateFundingInstructionsCustomerBankTransfer<'a> {
    pub fn new(type_: CreateFundingInstructionsCustomerBankTransferType) -> Self {
        Self {
            eu_bank_transfer: Default::default(),
            requested_address_types: Default::default(),
            type_,
        }
    }
}
/// Configuration for eu_bank_transfer funding type.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFundingInstructionsCustomerBankTransferEuBankTransfer<'a> {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a> CreateFundingInstructionsCustomerBankTransferEuBankTransfer<'a> {
    pub fn new(country: &'a str) -> Self {
        Self { country }
    }
}
/// List of address types that should be returned in the financial_addresses response.
///
/// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    Iban,
    SortCode,
    Spei,
    Zengin,
}

impl CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Iban => "iban",
            Self::SortCode => "sort_code",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl AsRef<str> for CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The type of the `bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateFundingInstructionsCustomerBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl CreateFundingInstructionsCustomerBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::GbBankTransfer => "gb_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
            Self::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl AsRef<str> for CreateFundingInstructionsCustomerBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateFundingInstructionsCustomerBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The `funding_type` to get the instructions for.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateFundingInstructionsCustomerFundingType {
    BankTransfer,
}

impl CreateFundingInstructionsCustomerFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for CreateFundingInstructionsCustomerFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateFundingInstructionsCustomerFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
