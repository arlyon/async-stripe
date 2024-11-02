use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Permanently deletes a one-off invoice draft.
/// This cannot be undone.
/// Attempts to delete invoices that are no longer in a draft state will fail; once an invoice has been finalized or if an invoice is for a subscription, it must be [voided](https://stripe.com/docs/api#void_invoice).
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteInvoice {
    invoice: stripe_shared::InvoiceId,
}
impl DeleteInvoice {
    /// Construct a new `DeleteInvoice`.
    pub fn new(invoice: impl Into<stripe_shared::InvoiceId>) -> Self {
        Self { invoice: invoice.into() }
    }
}
impl DeleteInvoice {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for DeleteInvoice {
    type Output = stripe_shared::DeletedInvoice;

    fn build(&self) -> RequestBuilder {
        let invoice = &self.invoice;
        RequestBuilder::new(StripeMethod::Delete, format!("/invoices/{invoice}"))
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ListInvoiceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_method: Option<stripe_shared::InvoiceCollectionMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_date: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::InvoiceStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription: Option<String>,
}
impl ListInvoiceBuilder {
    fn new() -> Self {
        Self {
            collection_method: None,
            created: None,
            customer: None,
            due_date: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
            subscription: None,
        }
    }
}
/// You can list all invoices, or list the invoices for a specific customer.
/// The invoices are returned sorted by creation date, with the most recently created invoices appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListInvoice {
    inner: ListInvoiceBuilder,
}
impl ListInvoice {
    /// Construct a new `ListInvoice`.
    pub fn new() -> Self {
        Self { inner: ListInvoiceBuilder::new() }
    }
    /// The collection method of the invoice to retrieve. Either `charge_automatically` or `send_invoice`.
    pub fn collection_method(
        mut self,
        collection_method: impl Into<stripe_shared::InvoiceCollectionMethod>,
    ) -> Self {
        self.inner.collection_method = Some(collection_method.into());
        self
    }
    /// Only return invoices that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// Only return invoices for the customer specified by this customer ID.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    pub fn due_date(mut self, due_date: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.due_date = Some(due_date.into());
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`.
    /// [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview).
    pub fn status(mut self, status: impl Into<stripe_shared::InvoiceStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
    /// Only return invoices for the subscription specified by this subscription ID.
    pub fn subscription(mut self, subscription: impl Into<String>) -> Self {
        self.inner.subscription = Some(subscription.into());
        self
    }
}
impl Default for ListInvoice {
    fn default() -> Self {
        Self::new()
    }
}
impl ListInvoice {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Invoice>> {
        stripe_client_core::ListPaginator::new_list("/invoices", &self.inner)
    }
}

impl StripeRequest for ListInvoice {
    type Output = stripe_types::List<stripe_shared::Invoice>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/invoices").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveInvoiceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveInvoiceBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the invoice with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveInvoice {
    inner: RetrieveInvoiceBuilder,
    invoice: stripe_shared::InvoiceId,
}
impl RetrieveInvoice {
    /// Construct a new `RetrieveInvoice`.
    pub fn new(invoice: impl Into<stripe_shared::InvoiceId>) -> Self {
        Self { invoice: invoice.into(), inner: RetrieveInvoiceBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveInvoice {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveInvoice {
    type Output = stripe_shared::Invoice;

    fn build(&self) -> RequestBuilder {
        let invoice = &self.invoice;
        RequestBuilder::new(StripeMethod::Get, format!("/invoices/{invoice}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct SearchInvoiceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<String>,
    query: String,
}
impl SearchInvoiceBuilder {
    fn new(query: impl Into<String>) -> Self {
        Self { expand: None, limit: None, page: None, query: query.into() }
    }
}
/// Search for invoices you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
/// Don’t use search in read-after-write flows where strict consistency is necessary.
/// Under normal operating.
/// conditions, data is searchable in less than a minute.
/// Occasionally, propagation of new or updated data can be up.
/// to an hour behind during outages. Search functionality is not available to merchants in India.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SearchInvoice {
    inner: SearchInvoiceBuilder,
}
impl SearchInvoice {
    /// Construct a new `SearchInvoice`.
    pub fn new(query: impl Into<String>) -> Self {
        Self { inner: SearchInvoiceBuilder::new(query.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for pagination across multiple pages of results.
    /// Don't include this parameter on the first call.
    /// Use the next_page value returned in a previous response to request subsequent results.
    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.inner.page = Some(page.into());
        self
    }
}
impl SearchInvoice {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::SearchList<stripe_shared::Invoice>> {
        stripe_client_core::ListPaginator::new_search_list("/invoices/search", &self.inner)
    }
}

impl StripeRequest for SearchInvoice {
    type Output = stripe_types::SearchList<stripe_shared::Invoice>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/invoices/search").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpcomingInvoiceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_tax: Option<UpcomingInvoiceAutomaticTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coupon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_details: Option<UpcomingInvoiceCustomerDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<DiscountsDataParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_items: Option<Vec<UpcomingInvoiceInvoiceItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<UpcomingInvoiceIssuer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_details: Option<UpcomingInvoiceScheduleDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_billing_cycle_anchor: Option<UpcomingInvoiceSubscriptionBillingCycleAnchor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_cancel_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_cancel_at_period_end: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_cancel_now: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_default_tax_rates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_details: Option<UpcomingInvoiceSubscriptionDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_items: Option<Vec<UpcomingInvoiceSubscriptionItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_proration_behavior: Option<UpcomingInvoiceSubscriptionProrationBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_proration_date: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_resume_at: Option<UpcomingInvoiceSubscriptionResumeAt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_start_date: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_trial_end: Option<UpcomingInvoiceSubscriptionTrialEnd>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_trial_from_plan: Option<bool>,
}
impl UpcomingInvoiceBuilder {
    fn new() -> Self {
        Self {
            automatic_tax: None,
            coupon: None,
            currency: None,
            customer: None,
            customer_details: None,
            discounts: None,
            expand: None,
            invoice_items: None,
            issuer: None,
            on_behalf_of: None,
            schedule: None,
            schedule_details: None,
            subscription: None,
            subscription_billing_cycle_anchor: None,
            subscription_cancel_at: None,
            subscription_cancel_at_period_end: None,
            subscription_cancel_now: None,
            subscription_default_tax_rates: None,
            subscription_details: None,
            subscription_items: None,
            subscription_proration_behavior: None,
            subscription_proration_date: None,
            subscription_resume_at: None,
            subscription_start_date: None,
            subscription_trial_end: None,
            subscription_trial_from_plan: None,
        }
    }
}
/// Settings for automatic tax lookup for this invoice preview.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceAutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<UpcomingInvoiceAutomaticTaxLiability>,
}
impl UpcomingInvoiceAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpcomingInvoiceAutomaticTaxLiabilityType,
}
impl UpcomingInvoiceAutomaticTaxLiability {
    pub fn new(type_: impl Into<UpcomingInvoiceAutomaticTaxLiabilityType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl UpcomingInvoiceAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceAutomaticTaxLiabilityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpcomingInvoiceAutomaticTaxLiabilityType")
        })
    }
}
/// Details about the customer you want to invoice or overrides for an existing customer.
/// If `automatic_tax` is enabled then one of `customer`, `customer_details`, `subscription`, or `schedule` must be set.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceCustomerDetails {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<OptionalFieldsAddress>,
    /// The customer's shipping information. Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CustomerShipping>,
    /// Tax details about the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxParam>,
    /// The customer's tax exemption. One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<UpcomingInvoiceCustomerDetailsTaxExempt>,
    /// The customer's tax IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<UpcomingInvoiceCustomerDetailsTaxIds>>,
}
impl UpcomingInvoiceCustomerDetails {
    pub fn new() -> Self {
        Self { address: None, shipping: None, tax: None, tax_exempt: None, tax_ids: None }
    }
}
impl Default for UpcomingInvoiceCustomerDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// The customer's tax exemption. One of `none`, `exempt`, or `reverse`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceCustomerDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}
impl UpcomingInvoiceCustomerDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceCustomerDetailsTaxExempt::*;
        match self {
            Exempt => "exempt",
            None => "none",
            Reverse => "reverse",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceCustomerDetailsTaxExempt {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceCustomerDetailsTaxExempt::*;
        match s {
            "exempt" => Ok(Exempt),
            "none" => Ok(None),
            "reverse" => Ok(Reverse),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceCustomerDetailsTaxExempt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceCustomerDetailsTaxExempt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpcomingInvoiceCustomerDetailsTaxExempt")
        })
    }
}
/// The customer's tax IDs.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceCustomerDetailsTaxIds {
    /// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bh_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `kz_bin`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: UpcomingInvoiceCustomerDetailsTaxIdsType,
    /// Value of the tax ID.
    pub value: String,
}
impl UpcomingInvoiceCustomerDetailsTaxIds {
    pub fn new(
        type_: impl Into<UpcomingInvoiceCustomerDetailsTaxIdsType>,
        value: impl Into<String>,
    ) -> Self {
        Self { type_: type_.into(), value: value.into() }
    }
}
/// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bh_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `kz_bin`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpcomingInvoiceCustomerDetailsTaxIdsType {
    AdNrt,
    AeTrn,
    ArCuit,
    AuAbn,
    AuArn,
    BgUic,
    BhVat,
    BoTin,
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
    CnTin,
    CoNit,
    CrTin,
    DoRcn,
    EcRuc,
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
    KzBin,
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NgTin,
    NoVat,
    NoVoec,
    NzGst,
    OmVat,
    PeRuc,
    PhTin,
    RoTin,
    RsPib,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    SvNit,
    ThVat,
    TrTin,
    TwVat,
    UaVat,
    UsEin,
    UyRuc,
    VeRif,
    VnTin,
    ZaVat,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpcomingInvoiceCustomerDetailsTaxIdsType {
    pub fn as_str(&self) -> &str {
        use UpcomingInvoiceCustomerDetailsTaxIdsType::*;
        match self {
            AdNrt => "ad_nrt",
            AeTrn => "ae_trn",
            ArCuit => "ar_cuit",
            AuAbn => "au_abn",
            AuArn => "au_arn",
            BgUic => "bg_uic",
            BhVat => "bh_vat",
            BoTin => "bo_tin",
            BrCnpj => "br_cnpj",
            BrCpf => "br_cpf",
            CaBn => "ca_bn",
            CaGstHst => "ca_gst_hst",
            CaPstBc => "ca_pst_bc",
            CaPstMb => "ca_pst_mb",
            CaPstSk => "ca_pst_sk",
            CaQst => "ca_qst",
            ChVat => "ch_vat",
            ClTin => "cl_tin",
            CnTin => "cn_tin",
            CoNit => "co_nit",
            CrTin => "cr_tin",
            DoRcn => "do_rcn",
            EcRuc => "ec_ruc",
            EgTin => "eg_tin",
            EsCif => "es_cif",
            EuOssVat => "eu_oss_vat",
            EuVat => "eu_vat",
            GbVat => "gb_vat",
            GeVat => "ge_vat",
            HkBr => "hk_br",
            HuTin => "hu_tin",
            IdNpwp => "id_npwp",
            IlVat => "il_vat",
            InGst => "in_gst",
            IsVat => "is_vat",
            JpCn => "jp_cn",
            JpRn => "jp_rn",
            JpTrn => "jp_trn",
            KePin => "ke_pin",
            KrBrn => "kr_brn",
            KzBin => "kz_bin",
            LiUid => "li_uid",
            MxRfc => "mx_rfc",
            MyFrp => "my_frp",
            MyItn => "my_itn",
            MySst => "my_sst",
            NgTin => "ng_tin",
            NoVat => "no_vat",
            NoVoec => "no_voec",
            NzGst => "nz_gst",
            OmVat => "om_vat",
            PeRuc => "pe_ruc",
            PhTin => "ph_tin",
            RoTin => "ro_tin",
            RsPib => "rs_pib",
            RuInn => "ru_inn",
            RuKpp => "ru_kpp",
            SaVat => "sa_vat",
            SgGst => "sg_gst",
            SgUen => "sg_uen",
            SiTin => "si_tin",
            SvNit => "sv_nit",
            ThVat => "th_vat",
            TrTin => "tr_tin",
            TwVat => "tw_vat",
            UaVat => "ua_vat",
            UsEin => "us_ein",
            UyRuc => "uy_ruc",
            VeRif => "ve_rif",
            VnTin => "vn_tin",
            ZaVat => "za_vat",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceCustomerDetailsTaxIdsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceCustomerDetailsTaxIdsType::*;
        match s {
            "ad_nrt" => Ok(AdNrt),
            "ae_trn" => Ok(AeTrn),
            "ar_cuit" => Ok(ArCuit),
            "au_abn" => Ok(AuAbn),
            "au_arn" => Ok(AuArn),
            "bg_uic" => Ok(BgUic),
            "bh_vat" => Ok(BhVat),
            "bo_tin" => Ok(BoTin),
            "br_cnpj" => Ok(BrCnpj),
            "br_cpf" => Ok(BrCpf),
            "ca_bn" => Ok(CaBn),
            "ca_gst_hst" => Ok(CaGstHst),
            "ca_pst_bc" => Ok(CaPstBc),
            "ca_pst_mb" => Ok(CaPstMb),
            "ca_pst_sk" => Ok(CaPstSk),
            "ca_qst" => Ok(CaQst),
            "ch_vat" => Ok(ChVat),
            "cl_tin" => Ok(ClTin),
            "cn_tin" => Ok(CnTin),
            "co_nit" => Ok(CoNit),
            "cr_tin" => Ok(CrTin),
            "do_rcn" => Ok(DoRcn),
            "ec_ruc" => Ok(EcRuc),
            "eg_tin" => Ok(EgTin),
            "es_cif" => Ok(EsCif),
            "eu_oss_vat" => Ok(EuOssVat),
            "eu_vat" => Ok(EuVat),
            "gb_vat" => Ok(GbVat),
            "ge_vat" => Ok(GeVat),
            "hk_br" => Ok(HkBr),
            "hu_tin" => Ok(HuTin),
            "id_npwp" => Ok(IdNpwp),
            "il_vat" => Ok(IlVat),
            "in_gst" => Ok(InGst),
            "is_vat" => Ok(IsVat),
            "jp_cn" => Ok(JpCn),
            "jp_rn" => Ok(JpRn),
            "jp_trn" => Ok(JpTrn),
            "ke_pin" => Ok(KePin),
            "kr_brn" => Ok(KrBrn),
            "kz_bin" => Ok(KzBin),
            "li_uid" => Ok(LiUid),
            "mx_rfc" => Ok(MxRfc),
            "my_frp" => Ok(MyFrp),
            "my_itn" => Ok(MyItn),
            "my_sst" => Ok(MySst),
            "ng_tin" => Ok(NgTin),
            "no_vat" => Ok(NoVat),
            "no_voec" => Ok(NoVoec),
            "nz_gst" => Ok(NzGst),
            "om_vat" => Ok(OmVat),
            "pe_ruc" => Ok(PeRuc),
            "ph_tin" => Ok(PhTin),
            "ro_tin" => Ok(RoTin),
            "rs_pib" => Ok(RsPib),
            "ru_inn" => Ok(RuInn),
            "ru_kpp" => Ok(RuKpp),
            "sa_vat" => Ok(SaVat),
            "sg_gst" => Ok(SgGst),
            "sg_uen" => Ok(SgUen),
            "si_tin" => Ok(SiTin),
            "sv_nit" => Ok(SvNit),
            "th_vat" => Ok(ThVat),
            "tr_tin" => Ok(TrTin),
            "tw_vat" => Ok(TwVat),
            "ua_vat" => Ok(UaVat),
            "us_ein" => Ok(UsEin),
            "uy_ruc" => Ok(UyRuc),
            "ve_rif" => Ok(VeRif),
            "vn_tin" => Ok(VnTin),
            "za_vat" => Ok(ZaVat),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceCustomerDetailsTaxIdsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceCustomerDetailsTaxIdsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceCustomerDetailsTaxIdsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceCustomerDetailsTaxIdsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// List of invoice items to add or update in the upcoming invoice preview.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceInvoiceItems {
    /// The integer amount in cents (or local equivalent) of previewed invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// Only applicable to new invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// An arbitrary string which you can attach to the invoice item.
    /// The description is displayed in the invoice for easy tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Explicitly controls whether discounts apply to this invoice item.
    /// Defaults to true, except for negative invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,
    /// The coupons to redeem into discounts for the invoice item in the preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// The ID of the invoice item to update in preview.
    /// If not specified, a new invoice item will be added to the preview of the upcoming invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoiceitem: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The period associated with this invoice item.
    /// When set to different values, the period will be rendered on the invoice.
    /// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
    /// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpcomingInvoiceInvoiceItemsPriceData>,
    /// Non-negative integer. The quantity of units for the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingInvoiceInvoiceItemsTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// The tax rates that apply to the item. When set, any `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
    /// The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    /// This unit_amount will be multiplied by the quantity to get the full amount.
    /// If you want to apply a credit to the customer's account, pass a negative unit_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpcomingInvoiceInvoiceItems {
    pub fn new() -> Self {
        Self {
            amount: None,
            currency: None,
            description: None,
            discountable: None,
            discounts: None,
            invoiceitem: None,
            metadata: None,
            period: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_behavior: None,
            tax_code: None,
            tax_rates: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
impl Default for UpcomingInvoiceInvoiceItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceInvoiceItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingInvoiceInvoiceItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpcomingInvoiceInvoiceItemsPriceData {
    pub fn new(currency: impl Into<stripe_types::Currency>, product: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpcomingInvoiceInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceInvoiceItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceInvoiceItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceInvoiceItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceInvoiceItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceInvoiceItemsPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingInvoiceInvoiceItemsPriceDataTaxBehavior",
            )
        })
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceInvoiceItemsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpcomingInvoiceInvoiceItemsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceInvoiceItemsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceInvoiceItemsTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceInvoiceItemsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceInvoiceItemsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceInvoiceItemsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceInvoiceItemsTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceInvoiceItemsTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpcomingInvoiceInvoiceItemsTaxBehavior")
        })
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpcomingInvoiceIssuerType,
}
impl UpcomingInvoiceIssuer {
    pub fn new(type_: impl Into<UpcomingInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceIssuerType {
    Account,
    Self_,
}
impl UpcomingInvoiceIssuerType {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceIssuerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for UpcomingInvoiceIssuerType"))
    }
}
/// The schedule creation or modification params to apply as a preview.
/// Cannot be used with `subscription` or `subscription_` prefixed fields.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceScheduleDetails {
    /// Behavior of the subscription schedule and underlying subscription when it ends.
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.
    /// `cancel` will end the subscription schedule and cancel the underlying subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_behavior: Option<UpcomingInvoiceScheduleDetailsEndBehavior>,
    /// List representing phases of the subscription schedule.
    /// Each phase can be customized to have different durations, plans, and coupons.
    /// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<UpcomingInvoiceScheduleDetailsPhases>>,
    /// In cases where the `schedule_details` params update the currently active phase, specifies if and how to prorate at the time of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<UpcomingInvoiceScheduleDetailsProrationBehavior>,
}
impl UpcomingInvoiceScheduleDetails {
    pub fn new() -> Self {
        Self { end_behavior: None, phases: None, proration_behavior: None }
    }
}
impl Default for UpcomingInvoiceScheduleDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// Behavior of the subscription schedule and underlying subscription when it ends.
/// Possible values are `release` or `cancel` with the default being `release`.
/// `release` will end the subscription schedule and keep the underlying subscription running.
/// `cancel` will end the subscription schedule and cancel the underlying subscription.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceScheduleDetailsEndBehavior {
    Cancel,
    Release,
}
impl UpcomingInvoiceScheduleDetailsEndBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceScheduleDetailsEndBehavior::*;
        match self {
            Cancel => "cancel",
            Release => "release",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceScheduleDetailsEndBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceScheduleDetailsEndBehavior::*;
        match s {
            "cancel" => Ok(Cancel),
            "release" => Ok(Release),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceScheduleDetailsEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceScheduleDetailsEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceScheduleDetailsEndBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceScheduleDetailsEndBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpcomingInvoiceScheduleDetailsEndBehavior")
        })
    }
}
/// List representing phases of the subscription schedule.
/// Each phase can be customized to have different durations, plans, and coupons.
/// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceScheduleDetailsPhases {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<Vec<UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItems>>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Automatic tax settings for this phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpcomingInvoiceScheduleDetailsPhasesAutomaticTax>,
    /// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
    /// Cannot be set to `phase_start` if this phase specifies a trial.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<UpcomingInvoiceScheduleDetailsPhasesBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<BillingThresholdsParam>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<stripe_shared::InvoiceCollectionMethod>,
    /// The ID of the coupon to apply to this phase of the subscription schedule.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `discounts` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// ID of the default payment method for the subscription schedule.
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will set the Subscription's [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates), which means they will be the Invoice's [`default_tax_rates`](https://stripe.com/docs/api/invoices/create#create_invoice-default_tax_rates) for any Invoices issued by the Subscription during this Phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,
    /// Subscription description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The coupons to redeem into discounts for the schedule phase.
    /// If not specified, inherits the discount from the subscription's customer.
    /// Pass an empty string to avoid inheriting any discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// The date at which this phase of the subscription schedule ends.
    /// If set, `iterations` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<UpcomingInvoiceScheduleDetailsPhasesEndDate>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<UpcomingInvoiceScheduleDetailsPhasesInvoiceSettings>,
    /// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
    pub items: Vec<UpcomingInvoiceScheduleDetailsPhasesItems>,
    /// Integer representing the multiplier applied to the price interval.
    /// For example, `iterations=2` applied to a price with `interval=month` and `interval_count=3` results in a phase of duration `2 * 3 months = 6 months`.
    /// If set, `end_date` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a phase.
    /// Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered, adding new keys and replacing existing keys in the subscription's `metadata`.
    /// Individual keys in the subscription's `metadata` can be unset by posting an empty value to them in the phase's `metadata`.
    /// To unset all keys in the subscription's `metadata`, update the subscription directly or unset every key individually from the phase's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The account on behalf of which to charge, for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    /// Whether the subscription schedule will create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase.
    /// The default value is `create_prorations`.
    /// This setting controls prorations when a phase is started asynchronously and it is persisted as a field on the phase.
    /// It's different from the request-level [proration_behavior](https://stripe.com/docs/api/subscription_schedules/update#update_subscription_schedule-proration_behavior) parameter which controls what happens if the update request affects the billing configuration of the current phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<UpcomingInvoiceScheduleDetailsPhasesProrationBehavior>,
    /// The date at which this phase of the subscription schedule starts or `now`.
    /// Must be set on the first phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<UpcomingInvoiceScheduleDetailsPhasesStartDate>,
    /// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferDataSpecs>,
    /// If set to true the entire phase is counted as a trial and the customer will not be charged for any fees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,
    /// Sets the phase to trialing from the start date to this date.
    /// Must be before the phase end date, can not be combined with `trial`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<UpcomingInvoiceScheduleDetailsPhasesTrialEnd>,
}
impl UpcomingInvoiceScheduleDetailsPhases {
    pub fn new(items: impl Into<Vec<UpcomingInvoiceScheduleDetailsPhasesItems>>) -> Self {
        Self {
            add_invoice_items: None,
            application_fee_percent: None,
            automatic_tax: None,
            billing_cycle_anchor: None,
            billing_thresholds: None,
            collection_method: None,
            coupon: None,
            currency: None,
            default_payment_method: None,
            default_tax_rates: None,
            description: None,
            discounts: None,
            end_date: None,
            invoice_settings: None,
            items: items.into(),
            iterations: None,
            metadata: None,
            on_behalf_of: None,
            proration_behavior: None,
            start_date: None,
            transfer_data: None,
            trial: None,
            trial_end: None,
        }
    }
}
/// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
/// You may pass up to 20 items.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItems {
    /// The coupons to redeem into discounts for the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceData>,
    /// Quantity for this item. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the item. When set, the `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItems {
    pub fn new() -> Self {
        Self { discounts: None, price: None, price_data: None, quantity: None, tax_rates: None }
    }
}
impl Default for UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceData {
    pub fn new(currency: impl Into<stripe_types::Currency>, product: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpcomingInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior"))
    }
}
/// Automatic tax settings for this phase.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceScheduleDetailsPhasesAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<UpcomingInvoiceScheduleDetailsPhasesAutomaticTaxLiability>,
}
impl UpcomingInvoiceScheduleDetailsPhasesAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceScheduleDetailsPhasesAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpcomingInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType,
}
impl UpcomingInvoiceScheduleDetailsPhasesAutomaticTaxLiability {
    pub fn new(
        type_: impl Into<UpcomingInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl UpcomingInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpcomingInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType",
            )
        })
    }
}
/// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
/// Cannot be set to `phase_start` if this phase specifies a trial.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    Automatic,
    PhaseStart,
}
impl UpcomingInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceScheduleDetailsPhasesBillingCycleAnchor::*;
        match self {
            Automatic => "automatic",
            PhaseStart => "phase_start",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceScheduleDetailsPhasesBillingCycleAnchor::*;
        match s {
            "automatic" => Ok(Automatic),
            "phase_start" => Ok(PhaseStart),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingInvoiceScheduleDetailsPhasesBillingCycleAnchor",
            )
        })
    }
}
/// The date at which this phase of the subscription schedule ends.
/// If set, `iterations` must not be set.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpcomingInvoiceScheduleDetailsPhasesEndDate {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceScheduleDetailsPhasesInvoiceSettings {
    /// The account tax IDs associated with this phase of the subscription schedule.
    /// Will be set on invoices generated by this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    /// This value will be `null` for subscription schedules where `billing=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<UpcomingInvoiceScheduleDetailsPhasesInvoiceSettingsIssuer>,
}
impl UpcomingInvoiceScheduleDetailsPhasesInvoiceSettings {
    pub fn new() -> Self {
        Self { account_tax_ids: None, days_until_due: None, issuer: None }
    }
}
impl Default for UpcomingInvoiceScheduleDetailsPhasesInvoiceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceScheduleDetailsPhasesInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpcomingInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType,
}
impl UpcomingInvoiceScheduleDetailsPhasesInvoiceSettingsIssuer {
    pub fn new(
        type_: impl Into<UpcomingInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    Account,
    Self_,
}
impl UpcomingInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpcomingInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType",
            )
        })
    }
}
/// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceScheduleDetailsPhasesItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<ItemBillingThresholdsParam>,
    /// The coupons to redeem into discounts for the subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a configuration item.
    /// Metadata on a configuration item will update the underlying subscription item's `metadata` when the phase is entered, adding new keys and replacing existing keys.
    /// Individual keys in the subscription item's `metadata` can be unset by posting an empty value to them in the configuration item's `metadata`.
    /// To unset all keys in the subscription item's `metadata`, update the subscription item directly or unset every key individually from the configuration item's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The plan ID to subscribe to. You may specify the same ID in `plan` and `price`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpcomingInvoiceScheduleDetailsPhasesItemsPriceData>,
    /// Quantity for the given price.
    /// Can be set only if the price's `usage_type` is `licensed` and not `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl UpcomingInvoiceScheduleDetailsPhasesItems {
    pub fn new() -> Self {
        Self {
            billing_thresholds: None,
            discounts: None,
            metadata: None,
            plan: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for UpcomingInvoiceScheduleDetailsPhasesItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceScheduleDetailsPhasesItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpcomingInvoiceScheduleDetailsPhasesItemsPriceData {
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        product: impl Into<String>,
        recurring: impl Into<UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurring>,
    ) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            recurring: recurring.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurring {
    pub fn new(
        interval: impl Into<UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval>,
    ) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval"))
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior",
            )
        })
    }
}
/// Whether the subscription schedule will create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase.
/// The default value is `create_prorations`.
/// This setting controls prorations when a phase is started asynchronously and it is persisted as a field on the phase.
/// It's different from the request-level [proration_behavior](https://stripe.com/docs/api/subscription_schedules/update#update_subscription_schedule-proration_behavior) parameter which controls what happens if the update request affects the billing configuration of the current phase.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceScheduleDetailsPhasesProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl UpcomingInvoiceScheduleDetailsPhasesProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceScheduleDetailsPhasesProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceScheduleDetailsPhasesProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceScheduleDetailsPhasesProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceScheduleDetailsPhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceScheduleDetailsPhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceScheduleDetailsPhasesProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceScheduleDetailsPhasesProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingInvoiceScheduleDetailsPhasesProrationBehavior",
            )
        })
    }
}
/// The date at which this phase of the subscription schedule starts or `now`.
/// Must be set on the first phase.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpcomingInvoiceScheduleDetailsPhasesStartDate {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// Sets the phase to trialing from the start date to this date.
/// Must be before the phase end date, can not be combined with `trial`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpcomingInvoiceScheduleDetailsPhasesTrialEnd {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// In cases where the `schedule_details` params update the currently active phase, specifies if and how to prorate at the time of the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceScheduleDetailsProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl UpcomingInvoiceScheduleDetailsProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceScheduleDetailsProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceScheduleDetailsProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceScheduleDetailsProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceScheduleDetailsProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceScheduleDetailsProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceScheduleDetailsProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceScheduleDetailsProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingInvoiceScheduleDetailsProrationBehavior",
            )
        })
    }
}
/// For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
/// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
/// For existing subscriptions, the value can only be set to `now` or `unchanged`.
/// This field has been deprecated and will be removed in a future API version.
/// Use `subscription_details.billing_cycle_anchor` instead.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpcomingInvoiceSubscriptionBillingCycleAnchor {
    Now,
    Unchanged,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// The subscription creation or modification params to apply as a preview.
/// Cannot be used with `schedule` or `schedule_details` fields.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceSubscriptionDetails {
    /// For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
    /// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    /// For existing subscriptions, the value can only be set to `now` or `unchanged`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<UpcomingInvoiceSubscriptionDetailsBillingCycleAnchor>,
    /// A timestamp at which the subscription should cancel.
    /// If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`.
    /// If set during a future period, this will always cause a proration for that period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at: Option<stripe_types::Timestamp>,
    /// Boolean indicating whether this subscription should cancel at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,
    /// This simulates the subscription being canceled or expired immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_now: Option<bool>,
    /// If provided, the invoice returned will preview updating or creating a subscription with these default tax rates.
    /// The default tax rates will apply to any line item that does not have `tax_rates` set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,
    /// A list of up to 20 subscription items, each with an attached price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UpcomingInvoiceSubscriptionDetailsItems>>,
    /// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<UpcomingInvoiceSubscriptionDetailsProrationBehavior>,
    /// If previewing an update to a subscription, and doing proration, `subscription_details.proration_date` forces the proration to be calculated as though the update was done at the specified time.
    /// The time given must be within the current subscription period and within the current phase of the schedule backing this subscription, if the schedule exists.
    /// If set, `subscription`, and one of `subscription_details.items`, or `subscription_details.trial_end` are required.
    /// Also, `subscription_details.proration_behavior` cannot be set to 'none'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<stripe_types::Timestamp>,
    /// For paused subscriptions, setting `subscription_details.resume_at` to `now` will preview the invoice that will be generated if the subscription is resumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_at: Option<UpcomingInvoiceSubscriptionDetailsResumeAt>,
    /// Date a subscription is intended to start (can be future or past).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<stripe_types::Timestamp>,
    /// If provided, the invoice returned will preview updating or creating a subscription with that trial end.
    /// If set, one of `subscription_details.items` or `subscription` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<UpcomingInvoiceSubscriptionDetailsTrialEnd>,
}
impl UpcomingInvoiceSubscriptionDetails {
    pub fn new() -> Self {
        Self {
            billing_cycle_anchor: None,
            cancel_at: None,
            cancel_at_period_end: None,
            cancel_now: None,
            default_tax_rates: None,
            items: None,
            proration_behavior: None,
            proration_date: None,
            resume_at: None,
            start_date: None,
            trial_end: None,
        }
    }
}
impl Default for UpcomingInvoiceSubscriptionDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
/// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
/// For existing subscriptions, the value can only be set to `now` or `unchanged`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpcomingInvoiceSubscriptionDetailsBillingCycleAnchor {
    Now,
    Unchanged,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// A list of up to 20 subscription items, each with an attached price.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceSubscriptionDetailsItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<ItemBillingThresholdsParam>,
    /// Delete all usage for a given subscription item.
    /// Allowed only when `deleted` is set to `true` and the current plan's `usage_type` is `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_usage: Option<bool>,
    /// A flag that, if set to `true`, will delete the specified item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The coupons to redeem into discounts for the subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// Subscription item to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// The ID of the price object.
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpcomingInvoiceSubscriptionDetailsItemsPriceData>,
    /// Quantity for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl UpcomingInvoiceSubscriptionDetailsItems {
    pub fn new() -> Self {
        Self {
            billing_thresholds: None,
            clear_usage: None,
            deleted: None,
            discounts: None,
            id: None,
            metadata: None,
            plan: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for UpcomingInvoiceSubscriptionDetailsItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceSubscriptionDetailsItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpcomingInvoiceSubscriptionDetailsItemsPriceData {
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        product: impl Into<String>,
        recurring: impl Into<UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurring>,
    ) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            recurring: recurring.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurring {
    pub fn new(
        interval: impl Into<UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval>,
    ) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpcomingInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval"))
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpcomingInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior",
            )
        })
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
/// The default value is `create_prorations`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceSubscriptionDetailsProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl UpcomingInvoiceSubscriptionDetailsProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceSubscriptionDetailsProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceSubscriptionDetailsProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceSubscriptionDetailsProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceSubscriptionDetailsProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceSubscriptionDetailsProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceSubscriptionDetailsProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceSubscriptionDetailsProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingInvoiceSubscriptionDetailsProrationBehavior",
            )
        })
    }
}
/// For paused subscriptions, setting `subscription_details.resume_at` to `now` will preview the invoice that will be generated if the subscription is resumed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceSubscriptionDetailsResumeAt {
    Now,
}
impl UpcomingInvoiceSubscriptionDetailsResumeAt {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceSubscriptionDetailsResumeAt::*;
        match self {
            Now => "now",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceSubscriptionDetailsResumeAt {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceSubscriptionDetailsResumeAt::*;
        match s {
            "now" => Ok(Now),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceSubscriptionDetailsResumeAt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceSubscriptionDetailsResumeAt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceSubscriptionDetailsResumeAt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceSubscriptionDetailsResumeAt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpcomingInvoiceSubscriptionDetailsResumeAt")
        })
    }
}
/// If provided, the invoice returned will preview updating or creating a subscription with that trial end.
/// If set, one of `subscription_details.items` or `subscription` is required.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpcomingInvoiceSubscriptionDetailsTrialEnd {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// A list of up to 20 subscription items, each with an attached price.
/// This field has been deprecated and will be removed in a future API version.
/// Use `subscription_details.items` instead.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceSubscriptionItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<ItemBillingThresholdsParam>,
    /// Delete all usage for a given subscription item.
    /// Allowed only when `deleted` is set to `true` and the current plan's `usage_type` is `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_usage: Option<bool>,
    /// A flag that, if set to `true`, will delete the specified item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The coupons to redeem into discounts for the subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// Subscription item to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// The ID of the price object.
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpcomingInvoiceSubscriptionItemsPriceData>,
    /// Quantity for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl UpcomingInvoiceSubscriptionItems {
    pub fn new() -> Self {
        Self {
            billing_thresholds: None,
            clear_usage: None,
            deleted: None,
            discounts: None,
            id: None,
            metadata: None,
            plan: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for UpcomingInvoiceSubscriptionItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceSubscriptionItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: UpcomingInvoiceSubscriptionItemsPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingInvoiceSubscriptionItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpcomingInvoiceSubscriptionItemsPriceData {
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        product: impl Into<String>,
        recurring: impl Into<UpcomingInvoiceSubscriptionItemsPriceDataRecurring>,
    ) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            recurring: recurring.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceSubscriptionItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpcomingInvoiceSubscriptionItemsPriceDataRecurring {
    pub fn new(
        interval: impl Into<UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval>,
    ) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval",
            )
        })
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceSubscriptionItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpcomingInvoiceSubscriptionItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceSubscriptionItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceSubscriptionItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceSubscriptionItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceSubscriptionItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceSubscriptionItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceSubscriptionItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceSubscriptionItemsPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingInvoiceSubscriptionItemsPriceDataTaxBehavior",
            )
        })
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
/// The default value is `create_prorations`.
/// This field has been deprecated and will be removed in a future API version.
/// Use `subscription_details.proration_behavior` instead.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceSubscriptionProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl UpcomingInvoiceSubscriptionProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceSubscriptionProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceSubscriptionProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceSubscriptionProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceSubscriptionProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceSubscriptionProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceSubscriptionProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceSubscriptionProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingInvoiceSubscriptionProrationBehavior",
            )
        })
    }
}
/// For paused subscriptions, setting `subscription_resume_at` to `now` will preview the invoice that will be generated if the subscription is resumed.
/// This field has been deprecated and will be removed in a future API version.
/// Use `subscription_details.resume_at` instead.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingInvoiceSubscriptionResumeAt {
    Now,
}
impl UpcomingInvoiceSubscriptionResumeAt {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceSubscriptionResumeAt::*;
        match self {
            Now => "now",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceSubscriptionResumeAt {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceSubscriptionResumeAt::*;
        match s {
            "now" => Ok(Now),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingInvoiceSubscriptionResumeAt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingInvoiceSubscriptionResumeAt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingInvoiceSubscriptionResumeAt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingInvoiceSubscriptionResumeAt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpcomingInvoiceSubscriptionResumeAt")
        })
    }
}
/// If provided, the invoice returned will preview updating or creating a subscription with that trial end.
/// If set, one of `subscription_items` or `subscription` is required.
/// This field has been deprecated and will be removed in a future API version.
/// Use `subscription_details.trial_end` instead.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpcomingInvoiceSubscriptionTrialEnd {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// At any time, you can preview the upcoming invoice for a customer.
/// This will show you all the charges that are pending, including subscription renewal charges, invoice item charges, etc.
/// It will also show you any discounts that are applicable to the invoice.
///
/// Note that when you are viewing an upcoming invoice, you are simply viewing a preview – the invoice has not yet been created.
/// As such, the upcoming invoice will not show up in invoice listing calls, and you cannot use the API to pay or edit the invoice.
/// If you want to change the amount that your customer will be billed, you can add, remove, or update pending invoice items, or update the customer’s discount.
///
/// You can preview the effects of updating a subscription, including a preview of what proration will take place.
/// To ensure that the actual proration is calculated exactly the same as the previewed proration, you should pass the `subscription_details.proration_date` parameter when doing the actual subscription update.
/// The recommended way to get only the prorations being previewed is to consider only proration line items where `period[start]` is equal to the `subscription_details.proration_date` value passed in the request.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoice {
    inner: UpcomingInvoiceBuilder,
}
impl UpcomingInvoice {
    /// Construct a new `UpcomingInvoice`.
    pub fn new() -> Self {
        Self { inner: UpcomingInvoiceBuilder::new() }
    }
    /// Settings for automatic tax lookup for this invoice preview.
    pub fn automatic_tax(mut self, automatic_tax: impl Into<UpcomingInvoiceAutomaticTax>) -> Self {
        self.inner.automatic_tax = Some(automatic_tax.into());
        self
    }
    /// The ID of the coupon to apply to this phase of the subscription schedule.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `discounts` instead.
    pub fn coupon(mut self, coupon: impl Into<String>) -> Self {
        self.inner.coupon = Some(coupon.into());
        self
    }
    /// The currency to preview this invoice in. Defaults to that of `customer` if not specified.
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// The identifier of the customer whose upcoming invoice you'd like to retrieve.
    /// If `automatic_tax` is enabled then one of `customer`, `customer_details`, `subscription`, or `schedule` must be set.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// Details about the customer you want to invoice or overrides for an existing customer.
    /// If `automatic_tax` is enabled then one of `customer`, `customer_details`, `subscription`, or `schedule` must be set.
    pub fn customer_details(
        mut self,
        customer_details: impl Into<UpcomingInvoiceCustomerDetails>,
    ) -> Self {
        self.inner.customer_details = Some(customer_details.into());
        self
    }
    /// The coupons to redeem into discounts for the invoice preview.
    /// If not specified, inherits the discount from the subscription or customer.
    /// This works for both coupons directly applied to an invoice and coupons applied to a subscription.
    /// Pass an empty string to avoid inheriting any discounts.
    pub fn discounts(mut self, discounts: impl Into<Vec<DiscountsDataParam>>) -> Self {
        self.inner.discounts = Some(discounts.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// List of invoice items to add or update in the upcoming invoice preview.
    pub fn invoice_items(
        mut self,
        invoice_items: impl Into<Vec<UpcomingInvoiceInvoiceItems>>,
    ) -> Self {
        self.inner.invoice_items = Some(invoice_items.into());
        self
    }
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    pub fn issuer(mut self, issuer: impl Into<UpcomingInvoiceIssuer>) -> Self {
        self.inner.issuer = Some(issuer.into());
        self
    }
    /// The account (if any) for which the funds of the invoice payment are intended.
    /// If set, the invoice will be presented with the branding and support information of the specified account.
    /// See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    pub fn on_behalf_of(mut self, on_behalf_of: impl Into<String>) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of.into());
        self
    }
    /// The identifier of the schedule whose upcoming invoice you'd like to retrieve.
    /// Cannot be used with subscription or subscription fields.
    pub fn schedule(mut self, schedule: impl Into<String>) -> Self {
        self.inner.schedule = Some(schedule.into());
        self
    }
    /// The schedule creation or modification params to apply as a preview.
    /// Cannot be used with `subscription` or `subscription_` prefixed fields.
    pub fn schedule_details(
        mut self,
        schedule_details: impl Into<UpcomingInvoiceScheduleDetails>,
    ) -> Self {
        self.inner.schedule_details = Some(schedule_details.into());
        self
    }
    /// The identifier of the subscription for which you'd like to retrieve the upcoming invoice.
    /// If not provided, but a `subscription_items` is provided, you will preview creating a subscription with those items.
    /// If neither `subscription` nor `subscription_items` is provided, you will retrieve the next upcoming invoice from among the customer's subscriptions.
    pub fn subscription(mut self, subscription: impl Into<String>) -> Self {
        self.inner.subscription = Some(subscription.into());
        self
    }
    /// For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
    /// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    /// For existing subscriptions, the value can only be set to `now` or `unchanged`.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.billing_cycle_anchor` instead.
    pub fn subscription_billing_cycle_anchor(
        mut self,
        subscription_billing_cycle_anchor: impl Into<UpcomingInvoiceSubscriptionBillingCycleAnchor>,
    ) -> Self {
        self.inner.subscription_billing_cycle_anchor =
            Some(subscription_billing_cycle_anchor.into());
        self
    }
    /// A timestamp at which the subscription should cancel.
    /// If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`.
    /// If set during a future period, this will always cause a proration for that period.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.cancel_at` instead.
    pub fn subscription_cancel_at(
        mut self,
        subscription_cancel_at: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        self.inner.subscription_cancel_at = Some(subscription_cancel_at.into());
        self
    }
    /// Boolean indicating whether this subscription should cancel at the end of the current period.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.cancel_at_period_end` instead.
    pub fn subscription_cancel_at_period_end(
        mut self,
        subscription_cancel_at_period_end: impl Into<bool>,
    ) -> Self {
        self.inner.subscription_cancel_at_period_end =
            Some(subscription_cancel_at_period_end.into());
        self
    }
    /// This simulates the subscription being canceled or expired immediately.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.cancel_now` instead.
    pub fn subscription_cancel_now(mut self, subscription_cancel_now: impl Into<bool>) -> Self {
        self.inner.subscription_cancel_now = Some(subscription_cancel_now.into());
        self
    }
    /// If provided, the invoice returned will preview updating or creating a subscription with these default tax rates.
    /// The default tax rates will apply to any line item that does not have `tax_rates` set.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.default_tax_rates` instead.
    pub fn subscription_default_tax_rates(
        mut self,
        subscription_default_tax_rates: impl Into<Vec<String>>,
    ) -> Self {
        self.inner.subscription_default_tax_rates = Some(subscription_default_tax_rates.into());
        self
    }
    /// The subscription creation or modification params to apply as a preview.
    /// Cannot be used with `schedule` or `schedule_details` fields.
    pub fn subscription_details(
        mut self,
        subscription_details: impl Into<UpcomingInvoiceSubscriptionDetails>,
    ) -> Self {
        self.inner.subscription_details = Some(subscription_details.into());
        self
    }
    /// A list of up to 20 subscription items, each with an attached price.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.items` instead.
    pub fn subscription_items(
        mut self,
        subscription_items: impl Into<Vec<UpcomingInvoiceSubscriptionItems>>,
    ) -> Self {
        self.inner.subscription_items = Some(subscription_items.into());
        self
    }
    /// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    /// The default value is `create_prorations`.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.proration_behavior` instead.
    pub fn subscription_proration_behavior(
        mut self,
        subscription_proration_behavior: impl Into<UpcomingInvoiceSubscriptionProrationBehavior>,
    ) -> Self {
        self.inner.subscription_proration_behavior = Some(subscription_proration_behavior.into());
        self
    }
    /// If previewing an update to a subscription, and doing proration, `subscription_proration_date` forces the proration to be calculated as though the update was done at the specified time.
    /// The time given must be within the current subscription period and within the current phase of the schedule backing this subscription, if the schedule exists.
    /// If set, `subscription`, and one of `subscription_items`, or `subscription_trial_end` are required.
    /// Also, `subscription_proration_behavior` cannot be set to 'none'.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.proration_date` instead.
    pub fn subscription_proration_date(
        mut self,
        subscription_proration_date: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        self.inner.subscription_proration_date = Some(subscription_proration_date.into());
        self
    }
    /// For paused subscriptions, setting `subscription_resume_at` to `now` will preview the invoice that will be generated if the subscription is resumed.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.resume_at` instead.
    pub fn subscription_resume_at(
        mut self,
        subscription_resume_at: impl Into<UpcomingInvoiceSubscriptionResumeAt>,
    ) -> Self {
        self.inner.subscription_resume_at = Some(subscription_resume_at.into());
        self
    }
    /// Date a subscription is intended to start (can be future or past).
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.start_date` instead.
    pub fn subscription_start_date(
        mut self,
        subscription_start_date: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        self.inner.subscription_start_date = Some(subscription_start_date.into());
        self
    }
    /// If provided, the invoice returned will preview updating or creating a subscription with that trial end.
    /// If set, one of `subscription_items` or `subscription` is required.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.trial_end` instead.
    pub fn subscription_trial_end(
        mut self,
        subscription_trial_end: impl Into<UpcomingInvoiceSubscriptionTrialEnd>,
    ) -> Self {
        self.inner.subscription_trial_end = Some(subscription_trial_end.into());
        self
    }
    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    /// Setting `subscription_trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `subscription_trial_end` is not allowed.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    pub fn subscription_trial_from_plan(
        mut self,
        subscription_trial_from_plan: impl Into<bool>,
    ) -> Self {
        self.inner.subscription_trial_from_plan = Some(subscription_trial_from_plan.into());
        self
    }
}
impl Default for UpcomingInvoice {
    fn default() -> Self {
        Self::new()
    }
}
impl UpcomingInvoice {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for UpcomingInvoice {
    type Output = stripe_shared::Invoice;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/invoices/upcoming").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpcomingLinesInvoiceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_tax: Option<UpcomingLinesInvoiceAutomaticTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coupon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_details: Option<UpcomingLinesInvoiceCustomerDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<DiscountsDataParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_items: Option<Vec<UpcomingLinesInvoiceInvoiceItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<UpcomingLinesInvoiceIssuer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_details: Option<UpcomingLinesInvoiceScheduleDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_billing_cycle_anchor: Option<UpcomingLinesInvoiceSubscriptionBillingCycleAnchor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_cancel_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_cancel_at_period_end: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_cancel_now: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_default_tax_rates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_details: Option<UpcomingLinesInvoiceSubscriptionDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_items: Option<Vec<UpcomingLinesInvoiceSubscriptionItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_proration_behavior: Option<UpcomingLinesInvoiceSubscriptionProrationBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_proration_date: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_resume_at: Option<UpcomingLinesInvoiceSubscriptionResumeAt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_start_date: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_trial_end: Option<UpcomingLinesInvoiceSubscriptionTrialEnd>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_trial_from_plan: Option<bool>,
}
impl UpcomingLinesInvoiceBuilder {
    fn new() -> Self {
        Self {
            automatic_tax: None,
            coupon: None,
            currency: None,
            customer: None,
            customer_details: None,
            discounts: None,
            ending_before: None,
            expand: None,
            invoice_items: None,
            issuer: None,
            limit: None,
            on_behalf_of: None,
            schedule: None,
            schedule_details: None,
            starting_after: None,
            subscription: None,
            subscription_billing_cycle_anchor: None,
            subscription_cancel_at: None,
            subscription_cancel_at_period_end: None,
            subscription_cancel_now: None,
            subscription_default_tax_rates: None,
            subscription_details: None,
            subscription_items: None,
            subscription_proration_behavior: None,
            subscription_proration_date: None,
            subscription_resume_at: None,
            subscription_start_date: None,
            subscription_trial_end: None,
            subscription_trial_from_plan: None,
        }
    }
}
/// Settings for automatic tax lookup for this invoice preview.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceAutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<UpcomingLinesInvoiceAutomaticTaxLiability>,
}
impl UpcomingLinesInvoiceAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpcomingLinesInvoiceAutomaticTaxLiabilityType,
}
impl UpcomingLinesInvoiceAutomaticTaxLiability {
    pub fn new(type_: impl Into<UpcomingLinesInvoiceAutomaticTaxLiabilityType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl UpcomingLinesInvoiceAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingLinesInvoiceAutomaticTaxLiabilityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingLinesInvoiceAutomaticTaxLiabilityType",
            )
        })
    }
}
/// Details about the customer you want to invoice or overrides for an existing customer.
/// If `automatic_tax` is enabled then one of `customer`, `customer_details`, `subscription`, or `schedule` must be set.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceCustomerDetails {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<OptionalFieldsAddress>,
    /// The customer's shipping information. Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CustomerShipping>,
    /// Tax details about the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxParam>,
    /// The customer's tax exemption. One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<UpcomingLinesInvoiceCustomerDetailsTaxExempt>,
    /// The customer's tax IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<UpcomingLinesInvoiceCustomerDetailsTaxIds>>,
}
impl UpcomingLinesInvoiceCustomerDetails {
    pub fn new() -> Self {
        Self { address: None, shipping: None, tax: None, tax_exempt: None, tax_ids: None }
    }
}
impl Default for UpcomingLinesInvoiceCustomerDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// The customer's tax exemption. One of `none`, `exempt`, or `reverse`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceCustomerDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}
impl UpcomingLinesInvoiceCustomerDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceCustomerDetailsTaxExempt::*;
        match self {
            Exempt => "exempt",
            None => "none",
            Reverse => "reverse",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceCustomerDetailsTaxExempt {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceCustomerDetailsTaxExempt::*;
        match s {
            "exempt" => Ok(Exempt),
            "none" => Ok(None),
            "reverse" => Ok(Reverse),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceCustomerDetailsTaxExempt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingLinesInvoiceCustomerDetailsTaxExempt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingLinesInvoiceCustomerDetailsTaxExempt",
            )
        })
    }
}
/// The customer's tax IDs.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceCustomerDetailsTaxIds {
    /// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bh_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `kz_bin`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: UpcomingLinesInvoiceCustomerDetailsTaxIdsType,
    /// Value of the tax ID.
    pub value: String,
}
impl UpcomingLinesInvoiceCustomerDetailsTaxIds {
    pub fn new(
        type_: impl Into<UpcomingLinesInvoiceCustomerDetailsTaxIdsType>,
        value: impl Into<String>,
    ) -> Self {
        Self { type_: type_.into(), value: value.into() }
    }
}
/// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bh_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `kz_bin`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpcomingLinesInvoiceCustomerDetailsTaxIdsType {
    AdNrt,
    AeTrn,
    ArCuit,
    AuAbn,
    AuArn,
    BgUic,
    BhVat,
    BoTin,
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
    CnTin,
    CoNit,
    CrTin,
    DoRcn,
    EcRuc,
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
    KzBin,
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NgTin,
    NoVat,
    NoVoec,
    NzGst,
    OmVat,
    PeRuc,
    PhTin,
    RoTin,
    RsPib,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    SvNit,
    ThVat,
    TrTin,
    TwVat,
    UaVat,
    UsEin,
    UyRuc,
    VeRif,
    VnTin,
    ZaVat,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpcomingLinesInvoiceCustomerDetailsTaxIdsType {
    pub fn as_str(&self) -> &str {
        use UpcomingLinesInvoiceCustomerDetailsTaxIdsType::*;
        match self {
            AdNrt => "ad_nrt",
            AeTrn => "ae_trn",
            ArCuit => "ar_cuit",
            AuAbn => "au_abn",
            AuArn => "au_arn",
            BgUic => "bg_uic",
            BhVat => "bh_vat",
            BoTin => "bo_tin",
            BrCnpj => "br_cnpj",
            BrCpf => "br_cpf",
            CaBn => "ca_bn",
            CaGstHst => "ca_gst_hst",
            CaPstBc => "ca_pst_bc",
            CaPstMb => "ca_pst_mb",
            CaPstSk => "ca_pst_sk",
            CaQst => "ca_qst",
            ChVat => "ch_vat",
            ClTin => "cl_tin",
            CnTin => "cn_tin",
            CoNit => "co_nit",
            CrTin => "cr_tin",
            DoRcn => "do_rcn",
            EcRuc => "ec_ruc",
            EgTin => "eg_tin",
            EsCif => "es_cif",
            EuOssVat => "eu_oss_vat",
            EuVat => "eu_vat",
            GbVat => "gb_vat",
            GeVat => "ge_vat",
            HkBr => "hk_br",
            HuTin => "hu_tin",
            IdNpwp => "id_npwp",
            IlVat => "il_vat",
            InGst => "in_gst",
            IsVat => "is_vat",
            JpCn => "jp_cn",
            JpRn => "jp_rn",
            JpTrn => "jp_trn",
            KePin => "ke_pin",
            KrBrn => "kr_brn",
            KzBin => "kz_bin",
            LiUid => "li_uid",
            MxRfc => "mx_rfc",
            MyFrp => "my_frp",
            MyItn => "my_itn",
            MySst => "my_sst",
            NgTin => "ng_tin",
            NoVat => "no_vat",
            NoVoec => "no_voec",
            NzGst => "nz_gst",
            OmVat => "om_vat",
            PeRuc => "pe_ruc",
            PhTin => "ph_tin",
            RoTin => "ro_tin",
            RsPib => "rs_pib",
            RuInn => "ru_inn",
            RuKpp => "ru_kpp",
            SaVat => "sa_vat",
            SgGst => "sg_gst",
            SgUen => "sg_uen",
            SiTin => "si_tin",
            SvNit => "sv_nit",
            ThVat => "th_vat",
            TrTin => "tr_tin",
            TwVat => "tw_vat",
            UaVat => "ua_vat",
            UsEin => "us_ein",
            UyRuc => "uy_ruc",
            VeRif => "ve_rif",
            VnTin => "vn_tin",
            ZaVat => "za_vat",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceCustomerDetailsTaxIdsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceCustomerDetailsTaxIdsType::*;
        match s {
            "ad_nrt" => Ok(AdNrt),
            "ae_trn" => Ok(AeTrn),
            "ar_cuit" => Ok(ArCuit),
            "au_abn" => Ok(AuAbn),
            "au_arn" => Ok(AuArn),
            "bg_uic" => Ok(BgUic),
            "bh_vat" => Ok(BhVat),
            "bo_tin" => Ok(BoTin),
            "br_cnpj" => Ok(BrCnpj),
            "br_cpf" => Ok(BrCpf),
            "ca_bn" => Ok(CaBn),
            "ca_gst_hst" => Ok(CaGstHst),
            "ca_pst_bc" => Ok(CaPstBc),
            "ca_pst_mb" => Ok(CaPstMb),
            "ca_pst_sk" => Ok(CaPstSk),
            "ca_qst" => Ok(CaQst),
            "ch_vat" => Ok(ChVat),
            "cl_tin" => Ok(ClTin),
            "cn_tin" => Ok(CnTin),
            "co_nit" => Ok(CoNit),
            "cr_tin" => Ok(CrTin),
            "do_rcn" => Ok(DoRcn),
            "ec_ruc" => Ok(EcRuc),
            "eg_tin" => Ok(EgTin),
            "es_cif" => Ok(EsCif),
            "eu_oss_vat" => Ok(EuOssVat),
            "eu_vat" => Ok(EuVat),
            "gb_vat" => Ok(GbVat),
            "ge_vat" => Ok(GeVat),
            "hk_br" => Ok(HkBr),
            "hu_tin" => Ok(HuTin),
            "id_npwp" => Ok(IdNpwp),
            "il_vat" => Ok(IlVat),
            "in_gst" => Ok(InGst),
            "is_vat" => Ok(IsVat),
            "jp_cn" => Ok(JpCn),
            "jp_rn" => Ok(JpRn),
            "jp_trn" => Ok(JpTrn),
            "ke_pin" => Ok(KePin),
            "kr_brn" => Ok(KrBrn),
            "kz_bin" => Ok(KzBin),
            "li_uid" => Ok(LiUid),
            "mx_rfc" => Ok(MxRfc),
            "my_frp" => Ok(MyFrp),
            "my_itn" => Ok(MyItn),
            "my_sst" => Ok(MySst),
            "ng_tin" => Ok(NgTin),
            "no_vat" => Ok(NoVat),
            "no_voec" => Ok(NoVoec),
            "nz_gst" => Ok(NzGst),
            "om_vat" => Ok(OmVat),
            "pe_ruc" => Ok(PeRuc),
            "ph_tin" => Ok(PhTin),
            "ro_tin" => Ok(RoTin),
            "rs_pib" => Ok(RsPib),
            "ru_inn" => Ok(RuInn),
            "ru_kpp" => Ok(RuKpp),
            "sa_vat" => Ok(SaVat),
            "sg_gst" => Ok(SgGst),
            "sg_uen" => Ok(SgUen),
            "si_tin" => Ok(SiTin),
            "sv_nit" => Ok(SvNit),
            "th_vat" => Ok(ThVat),
            "tr_tin" => Ok(TrTin),
            "tw_vat" => Ok(TwVat),
            "ua_vat" => Ok(UaVat),
            "us_ein" => Ok(UsEin),
            "uy_ruc" => Ok(UyRuc),
            "ve_rif" => Ok(VeRif),
            "vn_tin" => Ok(VnTin),
            "za_vat" => Ok(ZaVat),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceCustomerDetailsTaxIdsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceCustomerDetailsTaxIdsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceCustomerDetailsTaxIdsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingLinesInvoiceCustomerDetailsTaxIdsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// List of invoice items to add or update in the upcoming invoice preview.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceInvoiceItems {
    /// The integer amount in cents (or local equivalent) of previewed invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// Only applicable to new invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// An arbitrary string which you can attach to the invoice item.
    /// The description is displayed in the invoice for easy tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Explicitly controls whether discounts apply to this invoice item.
    /// Defaults to true, except for negative invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,
    /// The coupons to redeem into discounts for the invoice item in the preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// The ID of the invoice item to update in preview.
    /// If not specified, a new invoice item will be added to the preview of the upcoming invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoiceitem: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The period associated with this invoice item.
    /// When set to different values, the period will be rendered on the invoice.
    /// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
    /// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpcomingLinesInvoiceInvoiceItemsPriceData>,
    /// Non-negative integer. The quantity of units for the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingLinesInvoiceInvoiceItemsTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// The tax rates that apply to the item. When set, any `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
    /// The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    /// This unit_amount will be multiplied by the quantity to get the full amount.
    /// If you want to apply a credit to the customer's account, pass a negative unit_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpcomingLinesInvoiceInvoiceItems {
    pub fn new() -> Self {
        Self {
            amount: None,
            currency: None,
            description: None,
            discountable: None,
            discounts: None,
            invoiceitem: None,
            metadata: None,
            period: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_behavior: None,
            tax_code: None,
            tax_rates: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
impl Default for UpcomingLinesInvoiceInvoiceItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceInvoiceItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingLinesInvoiceInvoiceItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpcomingLinesInvoiceInvoiceItemsPriceData {
    pub fn new(currency: impl Into<stripe_types::Currency>, product: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpcomingLinesInvoiceInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceInvoiceItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceInvoiceItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceInvoiceItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceInvoiceItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingLinesInvoiceInvoiceItemsPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingLinesInvoiceInvoiceItemsPriceDataTaxBehavior",
            )
        })
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceInvoiceItemsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpcomingLinesInvoiceInvoiceItemsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceInvoiceItemsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceInvoiceItemsTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceInvoiceItemsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceInvoiceItemsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceInvoiceItemsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceInvoiceItemsTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingLinesInvoiceInvoiceItemsTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingLinesInvoiceInvoiceItemsTaxBehavior",
            )
        })
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpcomingLinesInvoiceIssuerType,
}
impl UpcomingLinesInvoiceIssuer {
    pub fn new(type_: impl Into<UpcomingLinesInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceIssuerType {
    Account,
    Self_,
}
impl UpcomingLinesInvoiceIssuerType {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingLinesInvoiceIssuerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpcomingLinesInvoiceIssuerType")
        })
    }
}
/// The schedule creation or modification params to apply as a preview.
/// Cannot be used with `subscription` or `subscription_` prefixed fields.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceScheduleDetails {
    /// Behavior of the subscription schedule and underlying subscription when it ends.
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.
    /// `cancel` will end the subscription schedule and cancel the underlying subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_behavior: Option<UpcomingLinesInvoiceScheduleDetailsEndBehavior>,
    /// List representing phases of the subscription schedule.
    /// Each phase can be customized to have different durations, plans, and coupons.
    /// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<UpcomingLinesInvoiceScheduleDetailsPhases>>,
    /// In cases where the `schedule_details` params update the currently active phase, specifies if and how to prorate at the time of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<UpcomingLinesInvoiceScheduleDetailsProrationBehavior>,
}
impl UpcomingLinesInvoiceScheduleDetails {
    pub fn new() -> Self {
        Self { end_behavior: None, phases: None, proration_behavior: None }
    }
}
impl Default for UpcomingLinesInvoiceScheduleDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// Behavior of the subscription schedule and underlying subscription when it ends.
/// Possible values are `release` or `cancel` with the default being `release`.
/// `release` will end the subscription schedule and keep the underlying subscription running.
/// `cancel` will end the subscription schedule and cancel the underlying subscription.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceScheduleDetailsEndBehavior {
    Cancel,
    Release,
}
impl UpcomingLinesInvoiceScheduleDetailsEndBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceScheduleDetailsEndBehavior::*;
        match self {
            Cancel => "cancel",
            Release => "release",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceScheduleDetailsEndBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceScheduleDetailsEndBehavior::*;
        match s {
            "cancel" => Ok(Cancel),
            "release" => Ok(Release),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceScheduleDetailsEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceScheduleDetailsEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceScheduleDetailsEndBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingLinesInvoiceScheduleDetailsEndBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingLinesInvoiceScheduleDetailsEndBehavior",
            )
        })
    }
}
/// List representing phases of the subscription schedule.
/// Each phase can be customized to have different durations, plans, and coupons.
/// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceScheduleDetailsPhases {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<Vec<UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItems>>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Automatic tax settings for this phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTax>,
    /// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
    /// Cannot be set to `phase_start` if this phase specifies a trial.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<UpcomingLinesInvoiceScheduleDetailsPhasesBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<BillingThresholdsParam>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<stripe_shared::InvoiceCollectionMethod>,
    /// The ID of the coupon to apply to this phase of the subscription schedule.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `discounts` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// ID of the default payment method for the subscription schedule.
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will set the Subscription's [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates), which means they will be the Invoice's [`default_tax_rates`](https://stripe.com/docs/api/invoices/create#create_invoice-default_tax_rates) for any Invoices issued by the Subscription during this Phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,
    /// Subscription description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The coupons to redeem into discounts for the schedule phase.
    /// If not specified, inherits the discount from the subscription's customer.
    /// Pass an empty string to avoid inheriting any discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// The date at which this phase of the subscription schedule ends.
    /// If set, `iterations` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<UpcomingLinesInvoiceScheduleDetailsPhasesEndDate>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettings>,
    /// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
    pub items: Vec<UpcomingLinesInvoiceScheduleDetailsPhasesItems>,
    /// Integer representing the multiplier applied to the price interval.
    /// For example, `iterations=2` applied to a price with `interval=month` and `interval_count=3` results in a phase of duration `2 * 3 months = 6 months`.
    /// If set, `end_date` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a phase.
    /// Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered, adding new keys and replacing existing keys in the subscription's `metadata`.
    /// Individual keys in the subscription's `metadata` can be unset by posting an empty value to them in the phase's `metadata`.
    /// To unset all keys in the subscription's `metadata`, update the subscription directly or unset every key individually from the phase's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The account on behalf of which to charge, for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    /// Whether the subscription schedule will create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase.
    /// The default value is `create_prorations`.
    /// This setting controls prorations when a phase is started asynchronously and it is persisted as a field on the phase.
    /// It's different from the request-level [proration_behavior](https://stripe.com/docs/api/subscription_schedules/update#update_subscription_schedule-proration_behavior) parameter which controls what happens if the update request affects the billing configuration of the current phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<UpcomingLinesInvoiceScheduleDetailsPhasesProrationBehavior>,
    /// The date at which this phase of the subscription schedule starts or `now`.
    /// Must be set on the first phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<UpcomingLinesInvoiceScheduleDetailsPhasesStartDate>,
    /// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferDataSpecs>,
    /// If set to true the entire phase is counted as a trial and the customer will not be charged for any fees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,
    /// Sets the phase to trialing from the start date to this date.
    /// Must be before the phase end date, can not be combined with `trial`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<UpcomingLinesInvoiceScheduleDetailsPhasesTrialEnd>,
}
impl UpcomingLinesInvoiceScheduleDetailsPhases {
    pub fn new(items: impl Into<Vec<UpcomingLinesInvoiceScheduleDetailsPhasesItems>>) -> Self {
        Self {
            add_invoice_items: None,
            application_fee_percent: None,
            automatic_tax: None,
            billing_cycle_anchor: None,
            billing_thresholds: None,
            collection_method: None,
            coupon: None,
            currency: None,
            default_payment_method: None,
            default_tax_rates: None,
            description: None,
            discounts: None,
            end_date: None,
            invoice_settings: None,
            items: items.into(),
            iterations: None,
            metadata: None,
            on_behalf_of: None,
            proration_behavior: None,
            start_date: None,
            transfer_data: None,
            trial: None,
            trial_end: None,
        }
    }
}
/// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
/// You may pass up to 20 items.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItems {
    /// The coupons to redeem into discounts for the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceData>,
    /// Quantity for this item. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the item. When set, the `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItems {
    pub fn new() -> Self {
        Self { discounts: None, price: None, price_data: None, quantity: None, tax_rates: None }
    }
}
impl Default for UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceData {
    pub fn new(currency: impl Into<stripe_types::Currency>, product: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr
    for UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpcomingLinesInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior"))
    }
}
/// Automatic tax settings for this phase.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTaxLiability>,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTaxLiability {
    pub fn new(
        type_: impl Into<UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpcomingLinesInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType"))
    }
}
/// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
/// Cannot be set to `phase_start` if this phase specifies a trial.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    Automatic,
    PhaseStart,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceScheduleDetailsPhasesBillingCycleAnchor::*;
        match self {
            Automatic => "automatic",
            PhaseStart => "phase_start",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceScheduleDetailsPhasesBillingCycleAnchor::*;
        match s {
            "automatic" => Ok(Automatic),
            "phase_start" => Ok(PhaseStart),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingLinesInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingLinesInvoiceScheduleDetailsPhasesBillingCycleAnchor",
            )
        })
    }
}
/// The date at which this phase of the subscription schedule ends.
/// If set, `iterations` must not be set.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpcomingLinesInvoiceScheduleDetailsPhasesEndDate {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettings {
    /// The account tax IDs associated with this phase of the subscription schedule.
    /// Will be set on invoices generated by this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    /// This value will be `null` for subscription schedules where `billing=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettingsIssuer>,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettings {
    pub fn new() -> Self {
        Self { account_tax_ids: None, days_until_due: None, issuer: None }
    }
}
impl Default for UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettingsIssuer {
    pub fn new(
        type_: impl Into<UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    Account,
    Self_,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpcomingLinesInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType"))
    }
}
/// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceScheduleDetailsPhasesItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<ItemBillingThresholdsParam>,
    /// The coupons to redeem into discounts for the subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a configuration item.
    /// Metadata on a configuration item will update the underlying subscription item's `metadata` when the phase is entered, adding new keys and replacing existing keys.
    /// Individual keys in the subscription item's `metadata` can be unset by posting an empty value to them in the configuration item's `metadata`.
    /// To unset all keys in the subscription item's `metadata`, update the subscription item directly or unset every key individually from the configuration item's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The plan ID to subscribe to. You may specify the same ID in `plan` and `price`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceData>,
    /// Quantity for the given price.
    /// Can be set only if the price's `usage_type` is `licensed` and not `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesItems {
    pub fn new() -> Self {
        Self {
            billing_thresholds: None,
            discounts: None,
            metadata: None,
            plan: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for UpcomingLinesInvoiceScheduleDetailsPhasesItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceData {
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        product: impl Into<String>,
        recurring: impl Into<UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurring>,
    ) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            recurring: recurring.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurring {
    pub fn new(
        interval: impl Into<UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval>,
    ) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr
    for UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval"))
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpcomingLinesInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior"))
    }
}
/// Whether the subscription schedule will create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase.
/// The default value is `create_prorations`.
/// This setting controls prorations when a phase is started asynchronously and it is persisted as a field on the phase.
/// It's different from the request-level [proration_behavior](https://stripe.com/docs/api/subscription_schedules/update#update_subscription_schedule-proration_behavior) parameter which controls what happens if the update request affects the billing configuration of the current phase.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceScheduleDetailsPhasesProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl UpcomingLinesInvoiceScheduleDetailsPhasesProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceScheduleDetailsPhasesProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceScheduleDetailsPhasesProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceScheduleDetailsPhasesProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceScheduleDetailsPhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceScheduleDetailsPhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceScheduleDetailsPhasesProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingLinesInvoiceScheduleDetailsPhasesProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingLinesInvoiceScheduleDetailsPhasesProrationBehavior",
            )
        })
    }
}
/// The date at which this phase of the subscription schedule starts or `now`.
/// Must be set on the first phase.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpcomingLinesInvoiceScheduleDetailsPhasesStartDate {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// Sets the phase to trialing from the start date to this date.
/// Must be before the phase end date, can not be combined with `trial`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpcomingLinesInvoiceScheduleDetailsPhasesTrialEnd {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// In cases where the `schedule_details` params update the currently active phase, specifies if and how to prorate at the time of the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceScheduleDetailsProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl UpcomingLinesInvoiceScheduleDetailsProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceScheduleDetailsProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceScheduleDetailsProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceScheduleDetailsProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceScheduleDetailsProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceScheduleDetailsProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceScheduleDetailsProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingLinesInvoiceScheduleDetailsProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingLinesInvoiceScheduleDetailsProrationBehavior",
            )
        })
    }
}
/// For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
/// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
/// For existing subscriptions, the value can only be set to `now` or `unchanged`.
/// This field has been deprecated and will be removed in a future API version.
/// Use `subscription_details.billing_cycle_anchor` instead.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpcomingLinesInvoiceSubscriptionBillingCycleAnchor {
    Now,
    Unchanged,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// The subscription creation or modification params to apply as a preview.
/// Cannot be used with `schedule` or `schedule_details` fields.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceSubscriptionDetails {
    /// For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
    /// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    /// For existing subscriptions, the value can only be set to `now` or `unchanged`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<UpcomingLinesInvoiceSubscriptionDetailsBillingCycleAnchor>,
    /// A timestamp at which the subscription should cancel.
    /// If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`.
    /// If set during a future period, this will always cause a proration for that period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at: Option<stripe_types::Timestamp>,
    /// Boolean indicating whether this subscription should cancel at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,
    /// This simulates the subscription being canceled or expired immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_now: Option<bool>,
    /// If provided, the invoice returned will preview updating or creating a subscription with these default tax rates.
    /// The default tax rates will apply to any line item that does not have `tax_rates` set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,
    /// A list of up to 20 subscription items, each with an attached price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UpcomingLinesInvoiceSubscriptionDetailsItems>>,
    /// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<UpcomingLinesInvoiceSubscriptionDetailsProrationBehavior>,
    /// If previewing an update to a subscription, and doing proration, `subscription_details.proration_date` forces the proration to be calculated as though the update was done at the specified time.
    /// The time given must be within the current subscription period and within the current phase of the schedule backing this subscription, if the schedule exists.
    /// If set, `subscription`, and one of `subscription_details.items`, or `subscription_details.trial_end` are required.
    /// Also, `subscription_details.proration_behavior` cannot be set to 'none'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<stripe_types::Timestamp>,
    /// For paused subscriptions, setting `subscription_details.resume_at` to `now` will preview the invoice that will be generated if the subscription is resumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_at: Option<UpcomingLinesInvoiceSubscriptionDetailsResumeAt>,
    /// Date a subscription is intended to start (can be future or past).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<stripe_types::Timestamp>,
    /// If provided, the invoice returned will preview updating or creating a subscription with that trial end.
    /// If set, one of `subscription_details.items` or `subscription` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<UpcomingLinesInvoiceSubscriptionDetailsTrialEnd>,
}
impl UpcomingLinesInvoiceSubscriptionDetails {
    pub fn new() -> Self {
        Self {
            billing_cycle_anchor: None,
            cancel_at: None,
            cancel_at_period_end: None,
            cancel_now: None,
            default_tax_rates: None,
            items: None,
            proration_behavior: None,
            proration_date: None,
            resume_at: None,
            start_date: None,
            trial_end: None,
        }
    }
}
impl Default for UpcomingLinesInvoiceSubscriptionDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
/// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
/// For existing subscriptions, the value can only be set to `now` or `unchanged`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpcomingLinesInvoiceSubscriptionDetailsBillingCycleAnchor {
    Now,
    Unchanged,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// A list of up to 20 subscription items, each with an attached price.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceSubscriptionDetailsItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<ItemBillingThresholdsParam>,
    /// Delete all usage for a given subscription item.
    /// Allowed only when `deleted` is set to `true` and the current plan's `usage_type` is `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_usage: Option<bool>,
    /// A flag that, if set to `true`, will delete the specified item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The coupons to redeem into discounts for the subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// Subscription item to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// The ID of the price object.
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpcomingLinesInvoiceSubscriptionDetailsItemsPriceData>,
    /// Quantity for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl UpcomingLinesInvoiceSubscriptionDetailsItems {
    pub fn new() -> Self {
        Self {
            billing_thresholds: None,
            clear_usage: None,
            deleted: None,
            discounts: None,
            id: None,
            metadata: None,
            plan: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for UpcomingLinesInvoiceSubscriptionDetailsItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceSubscriptionDetailsItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpcomingLinesInvoiceSubscriptionDetailsItemsPriceData {
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        product: impl Into<String>,
        recurring: impl Into<UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurring>,
    ) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            recurring: recurring.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurring {
    pub fn new(
        interval: impl Into<UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval>,
    ) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval"))
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpcomingLinesInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior"))
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
/// The default value is `create_prorations`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceSubscriptionDetailsProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl UpcomingLinesInvoiceSubscriptionDetailsProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceSubscriptionDetailsProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceSubscriptionDetailsProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceSubscriptionDetailsProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceSubscriptionDetailsProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceSubscriptionDetailsProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceSubscriptionDetailsProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingLinesInvoiceSubscriptionDetailsProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingLinesInvoiceSubscriptionDetailsProrationBehavior",
            )
        })
    }
}
/// For paused subscriptions, setting `subscription_details.resume_at` to `now` will preview the invoice that will be generated if the subscription is resumed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceSubscriptionDetailsResumeAt {
    Now,
}
impl UpcomingLinesInvoiceSubscriptionDetailsResumeAt {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceSubscriptionDetailsResumeAt::*;
        match self {
            Now => "now",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceSubscriptionDetailsResumeAt {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceSubscriptionDetailsResumeAt::*;
        match s {
            "now" => Ok(Now),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceSubscriptionDetailsResumeAt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceSubscriptionDetailsResumeAt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceSubscriptionDetailsResumeAt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingLinesInvoiceSubscriptionDetailsResumeAt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingLinesInvoiceSubscriptionDetailsResumeAt",
            )
        })
    }
}
/// If provided, the invoice returned will preview updating or creating a subscription with that trial end.
/// If set, one of `subscription_details.items` or `subscription` is required.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpcomingLinesInvoiceSubscriptionDetailsTrialEnd {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// A list of up to 20 subscription items, each with an attached price.
/// This field has been deprecated and will be removed in a future API version.
/// Use `subscription_details.items` instead.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceSubscriptionItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<ItemBillingThresholdsParam>,
    /// Delete all usage for a given subscription item.
    /// Allowed only when `deleted` is set to `true` and the current plan's `usage_type` is `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_usage: Option<bool>,
    /// A flag that, if set to `true`, will delete the specified item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The coupons to redeem into discounts for the subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// Subscription item to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// The ID of the price object.
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpcomingLinesInvoiceSubscriptionItemsPriceData>,
    /// Quantity for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl UpcomingLinesInvoiceSubscriptionItems {
    pub fn new() -> Self {
        Self {
            billing_thresholds: None,
            clear_usage: None,
            deleted: None,
            discounts: None,
            id: None,
            metadata: None,
            plan: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for UpcomingLinesInvoiceSubscriptionItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceSubscriptionItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingLinesInvoiceSubscriptionItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpcomingLinesInvoiceSubscriptionItemsPriceData {
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        product: impl Into<String>,
        recurring: impl Into<UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurring>,
    ) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            recurring: recurring.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurring {
    pub fn new(
        interval: impl Into<UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval>,
    ) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval",
            )
        })
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceSubscriptionItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpcomingLinesInvoiceSubscriptionItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceSubscriptionItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceSubscriptionItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceSubscriptionItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceSubscriptionItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceSubscriptionItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceSubscriptionItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingLinesInvoiceSubscriptionItemsPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingLinesInvoiceSubscriptionItemsPriceDataTaxBehavior",
            )
        })
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
/// The default value is `create_prorations`.
/// This field has been deprecated and will be removed in a future API version.
/// Use `subscription_details.proration_behavior` instead.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceSubscriptionProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl UpcomingLinesInvoiceSubscriptionProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceSubscriptionProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceSubscriptionProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceSubscriptionProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceSubscriptionProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceSubscriptionProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceSubscriptionProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingLinesInvoiceSubscriptionProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpcomingLinesInvoiceSubscriptionProrationBehavior",
            )
        })
    }
}
/// For paused subscriptions, setting `subscription_resume_at` to `now` will preview the invoice that will be generated if the subscription is resumed.
/// This field has been deprecated and will be removed in a future API version.
/// Use `subscription_details.resume_at` instead.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpcomingLinesInvoiceSubscriptionResumeAt {
    Now,
}
impl UpcomingLinesInvoiceSubscriptionResumeAt {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceSubscriptionResumeAt::*;
        match self {
            Now => "now",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceSubscriptionResumeAt {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceSubscriptionResumeAt::*;
        match s {
            "now" => Ok(Now),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpcomingLinesInvoiceSubscriptionResumeAt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpcomingLinesInvoiceSubscriptionResumeAt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpcomingLinesInvoiceSubscriptionResumeAt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpcomingLinesInvoiceSubscriptionResumeAt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpcomingLinesInvoiceSubscriptionResumeAt")
        })
    }
}
/// If provided, the invoice returned will preview updating or creating a subscription with that trial end.
/// If set, one of `subscription_items` or `subscription` is required.
/// This field has been deprecated and will be removed in a future API version.
/// Use `subscription_details.trial_end` instead.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpcomingLinesInvoiceSubscriptionTrialEnd {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// When retrieving an upcoming invoice, you’ll get a **lines** property containing the total count of line items and the first handful of those items.
/// There is also a URL where you can retrieve the full (paginated) list of line items.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoice {
    inner: UpcomingLinesInvoiceBuilder,
}
impl UpcomingLinesInvoice {
    /// Construct a new `UpcomingLinesInvoice`.
    pub fn new() -> Self {
        Self { inner: UpcomingLinesInvoiceBuilder::new() }
    }
    /// Settings for automatic tax lookup for this invoice preview.
    pub fn automatic_tax(
        mut self,
        automatic_tax: impl Into<UpcomingLinesInvoiceAutomaticTax>,
    ) -> Self {
        self.inner.automatic_tax = Some(automatic_tax.into());
        self
    }
    /// The ID of the coupon to apply to this phase of the subscription schedule.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `discounts` instead.
    pub fn coupon(mut self, coupon: impl Into<String>) -> Self {
        self.inner.coupon = Some(coupon.into());
        self
    }
    /// The currency to preview this invoice in. Defaults to that of `customer` if not specified.
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// The identifier of the customer whose upcoming invoice you'd like to retrieve.
    /// If `automatic_tax` is enabled then one of `customer`, `customer_details`, `subscription`, or `schedule` must be set.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// Details about the customer you want to invoice or overrides for an existing customer.
    /// If `automatic_tax` is enabled then one of `customer`, `customer_details`, `subscription`, or `schedule` must be set.
    pub fn customer_details(
        mut self,
        customer_details: impl Into<UpcomingLinesInvoiceCustomerDetails>,
    ) -> Self {
        self.inner.customer_details = Some(customer_details.into());
        self
    }
    /// The coupons to redeem into discounts for the invoice preview.
    /// If not specified, inherits the discount from the subscription or customer.
    /// This works for both coupons directly applied to an invoice and coupons applied to a subscription.
    /// Pass an empty string to avoid inheriting any discounts.
    pub fn discounts(mut self, discounts: impl Into<Vec<DiscountsDataParam>>) -> Self {
        self.inner.discounts = Some(discounts.into());
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// List of invoice items to add or update in the upcoming invoice preview.
    pub fn invoice_items(
        mut self,
        invoice_items: impl Into<Vec<UpcomingLinesInvoiceInvoiceItems>>,
    ) -> Self {
        self.inner.invoice_items = Some(invoice_items.into());
        self
    }
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    pub fn issuer(mut self, issuer: impl Into<UpcomingLinesInvoiceIssuer>) -> Self {
        self.inner.issuer = Some(issuer.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// The account (if any) for which the funds of the invoice payment are intended.
    /// If set, the invoice will be presented with the branding and support information of the specified account.
    /// See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    pub fn on_behalf_of(mut self, on_behalf_of: impl Into<String>) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of.into());
        self
    }
    /// The identifier of the schedule whose upcoming invoice you'd like to retrieve.
    /// Cannot be used with subscription or subscription fields.
    pub fn schedule(mut self, schedule: impl Into<String>) -> Self {
        self.inner.schedule = Some(schedule.into());
        self
    }
    /// The schedule creation or modification params to apply as a preview.
    /// Cannot be used with `subscription` or `subscription_` prefixed fields.
    pub fn schedule_details(
        mut self,
        schedule_details: impl Into<UpcomingLinesInvoiceScheduleDetails>,
    ) -> Self {
        self.inner.schedule_details = Some(schedule_details.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// The identifier of the subscription for which you'd like to retrieve the upcoming invoice.
    /// If not provided, but a `subscription_items` is provided, you will preview creating a subscription with those items.
    /// If neither `subscription` nor `subscription_items` is provided, you will retrieve the next upcoming invoice from among the customer's subscriptions.
    pub fn subscription(mut self, subscription: impl Into<String>) -> Self {
        self.inner.subscription = Some(subscription.into());
        self
    }
    /// For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
    /// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    /// For existing subscriptions, the value can only be set to `now` or `unchanged`.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.billing_cycle_anchor` instead.
    pub fn subscription_billing_cycle_anchor(
        mut self,
        subscription_billing_cycle_anchor: impl Into<UpcomingLinesInvoiceSubscriptionBillingCycleAnchor>,
    ) -> Self {
        self.inner.subscription_billing_cycle_anchor =
            Some(subscription_billing_cycle_anchor.into());
        self
    }
    /// A timestamp at which the subscription should cancel.
    /// If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`.
    /// If set during a future period, this will always cause a proration for that period.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.cancel_at` instead.
    pub fn subscription_cancel_at(
        mut self,
        subscription_cancel_at: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        self.inner.subscription_cancel_at = Some(subscription_cancel_at.into());
        self
    }
    /// Boolean indicating whether this subscription should cancel at the end of the current period.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.cancel_at_period_end` instead.
    pub fn subscription_cancel_at_period_end(
        mut self,
        subscription_cancel_at_period_end: impl Into<bool>,
    ) -> Self {
        self.inner.subscription_cancel_at_period_end =
            Some(subscription_cancel_at_period_end.into());
        self
    }
    /// This simulates the subscription being canceled or expired immediately.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.cancel_now` instead.
    pub fn subscription_cancel_now(mut self, subscription_cancel_now: impl Into<bool>) -> Self {
        self.inner.subscription_cancel_now = Some(subscription_cancel_now.into());
        self
    }
    /// If provided, the invoice returned will preview updating or creating a subscription with these default tax rates.
    /// The default tax rates will apply to any line item that does not have `tax_rates` set.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.default_tax_rates` instead.
    pub fn subscription_default_tax_rates(
        mut self,
        subscription_default_tax_rates: impl Into<Vec<String>>,
    ) -> Self {
        self.inner.subscription_default_tax_rates = Some(subscription_default_tax_rates.into());
        self
    }
    /// The subscription creation or modification params to apply as a preview.
    /// Cannot be used with `schedule` or `schedule_details` fields.
    pub fn subscription_details(
        mut self,
        subscription_details: impl Into<UpcomingLinesInvoiceSubscriptionDetails>,
    ) -> Self {
        self.inner.subscription_details = Some(subscription_details.into());
        self
    }
    /// A list of up to 20 subscription items, each with an attached price.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.items` instead.
    pub fn subscription_items(
        mut self,
        subscription_items: impl Into<Vec<UpcomingLinesInvoiceSubscriptionItems>>,
    ) -> Self {
        self.inner.subscription_items = Some(subscription_items.into());
        self
    }
    /// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    /// The default value is `create_prorations`.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.proration_behavior` instead.
    pub fn subscription_proration_behavior(
        mut self,
        subscription_proration_behavior: impl Into<UpcomingLinesInvoiceSubscriptionProrationBehavior>,
    ) -> Self {
        self.inner.subscription_proration_behavior = Some(subscription_proration_behavior.into());
        self
    }
    /// If previewing an update to a subscription, and doing proration, `subscription_proration_date` forces the proration to be calculated as though the update was done at the specified time.
    /// The time given must be within the current subscription period and within the current phase of the schedule backing this subscription, if the schedule exists.
    /// If set, `subscription`, and one of `subscription_items`, or `subscription_trial_end` are required.
    /// Also, `subscription_proration_behavior` cannot be set to 'none'.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.proration_date` instead.
    pub fn subscription_proration_date(
        mut self,
        subscription_proration_date: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        self.inner.subscription_proration_date = Some(subscription_proration_date.into());
        self
    }
    /// For paused subscriptions, setting `subscription_resume_at` to `now` will preview the invoice that will be generated if the subscription is resumed.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.resume_at` instead.
    pub fn subscription_resume_at(
        mut self,
        subscription_resume_at: impl Into<UpcomingLinesInvoiceSubscriptionResumeAt>,
    ) -> Self {
        self.inner.subscription_resume_at = Some(subscription_resume_at.into());
        self
    }
    /// Date a subscription is intended to start (can be future or past).
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.start_date` instead.
    pub fn subscription_start_date(
        mut self,
        subscription_start_date: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        self.inner.subscription_start_date = Some(subscription_start_date.into());
        self
    }
    /// If provided, the invoice returned will preview updating or creating a subscription with that trial end.
    /// If set, one of `subscription_items` or `subscription` is required.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `subscription_details.trial_end` instead.
    pub fn subscription_trial_end(
        mut self,
        subscription_trial_end: impl Into<UpcomingLinesInvoiceSubscriptionTrialEnd>,
    ) -> Self {
        self.inner.subscription_trial_end = Some(subscription_trial_end.into());
        self
    }
    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    /// Setting `subscription_trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `subscription_trial_end` is not allowed.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    pub fn subscription_trial_from_plan(
        mut self,
        subscription_trial_from_plan: impl Into<bool>,
    ) -> Self {
        self.inner.subscription_trial_from_plan = Some(subscription_trial_from_plan.into());
        self
    }
}
impl Default for UpcomingLinesInvoice {
    fn default() -> Self {
        Self::new()
    }
}
impl UpcomingLinesInvoice {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::InvoiceLineItem>> {
        stripe_client_core::ListPaginator::new_list("/invoices/upcoming/lines", &self.inner)
    }
}

impl StripeRequest for UpcomingLinesInvoice {
    type Output = stripe_types::List<stripe_shared::InvoiceLineItem>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/invoices/upcoming/lines").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateInvoiceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_tax_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_advance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_tax: Option<CreateInvoiceAutomaticTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_method: Option<stripe_shared::InvoiceCollectionMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_fields: Option<Vec<CustomFieldParams>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days_until_due: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_tax_rates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<DiscountsDataParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_date: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_invoice: Option<CreateInvoiceFromInvoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<CreateInvoiceIssuer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_settings: Option<CreateInvoicePaymentSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_invoice_items_behavior: Option<CreateInvoicePendingInvoiceItemsBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rendering: Option<CreateInvoiceRendering>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_cost: Option<CreateInvoiceShippingCost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_details: Option<RecipientShippingWithOptionalFieldsAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_data: Option<CreateInvoiceTransferData>,
}
impl CreateInvoiceBuilder {
    fn new() -> Self {
        Self {
            account_tax_ids: None,
            application_fee_amount: None,
            auto_advance: None,
            automatic_tax: None,
            collection_method: None,
            currency: None,
            custom_fields: None,
            customer: None,
            days_until_due: None,
            default_payment_method: None,
            default_source: None,
            default_tax_rates: None,
            description: None,
            discounts: None,
            due_date: None,
            effective_at: None,
            expand: None,
            footer: None,
            from_invoice: None,
            issuer: None,
            metadata: None,
            number: None,
            on_behalf_of: None,
            payment_settings: None,
            pending_invoice_items_behavior: None,
            rendering: None,
            shipping_cost: None,
            shipping_details: None,
            statement_descriptor: None,
            subscription: None,
            transfer_data: None,
        }
    }
}
/// Settings for automatic tax lookup for this invoice.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceAutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreateInvoiceAutomaticTaxLiability>,
}
impl CreateInvoiceAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateInvoiceAutomaticTaxLiabilityType,
}
impl CreateInvoiceAutomaticTaxLiability {
    pub fn new(type_: impl Into<CreateInvoiceAutomaticTaxLiabilityType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoiceAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl CreateInvoiceAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use CreateInvoiceAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreateInvoiceAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateInvoiceAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoiceAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoiceAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateInvoiceAutomaticTaxLiabilityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateInvoiceAutomaticTaxLiabilityType")
        })
    }
}
/// Revise an existing invoice.
/// The new invoice will be created in `status=draft`.
/// See the [revision documentation](https://stripe.com/docs/invoicing/invoice-revisions) for more details.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceFromInvoice {
    /// The relation between the new invoice and the original invoice.
    /// Currently, only 'revision' is permitted.
    pub action: CreateInvoiceFromInvoiceAction,
    /// The `id` of the invoice that will be cloned.
    pub invoice: String,
}
impl CreateInvoiceFromInvoice {
    pub fn new(
        action: impl Into<CreateInvoiceFromInvoiceAction>,
        invoice: impl Into<String>,
    ) -> Self {
        Self { action: action.into(), invoice: invoice.into() }
    }
}
/// The relation between the new invoice and the original invoice.
/// Currently, only 'revision' is permitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoiceFromInvoiceAction {
    Revision,
}
impl CreateInvoiceFromInvoiceAction {
    pub fn as_str(self) -> &'static str {
        use CreateInvoiceFromInvoiceAction::*;
        match self {
            Revision => "revision",
        }
    }
}

impl std::str::FromStr for CreateInvoiceFromInvoiceAction {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceFromInvoiceAction::*;
        match s {
            "revision" => Ok(Revision),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateInvoiceFromInvoiceAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoiceFromInvoiceAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoiceFromInvoiceAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateInvoiceFromInvoiceAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateInvoiceFromInvoiceAction")
        })
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateInvoiceIssuerType,
}
impl CreateInvoiceIssuer {
    pub fn new(type_: impl Into<CreateInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoiceIssuerType {
    Account,
    Self_,
}
impl CreateInvoiceIssuerType {
    pub fn as_str(self) -> &'static str {
        use CreateInvoiceIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreateInvoiceIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateInvoiceIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoiceIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoiceIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateInvoiceIssuerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreateInvoiceIssuerType"))
    }
}
/// Configuration settings for the PaymentIntent that is generated when the invoice is finalized.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateInvoicePaymentSettings {
    /// ID of the mandate to be used for this invoice.
    /// It must correspond to the payment method used to pay the invoice, including the invoice's default_payment_method or default_source, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mandate: Option<String>,
    /// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateInvoicePaymentSettingsPaymentMethodOptions>,
    /// The list of payment method types (e.g.
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<CreateInvoicePaymentSettingsPaymentMethodTypes>>,
}
impl CreateInvoicePaymentSettings {
    pub fn new() -> Self {
        Self { default_mandate: None, payment_method_options: None, payment_method_types: None }
    }
}
impl Default for CreateInvoicePaymentSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptions {
    /// If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit>,
    /// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsBancontact>,
    /// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsCard>,
    /// If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<InvoicePaymentMethodOptionsParam>,
    /// If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub konbini: Option<miniserde::json::Value>,
    /// If paying by `sepa_debit`, this sub-hash contains details about the SEPA Direct Debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub sepa_debit: Option<miniserde::json::Value>,
    /// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount>,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptions {
    pub fn new() -> Self {
        Self {
            acss_debit: None,
            bancontact: None,
            card: None,
            customer_balance: None,
            konbini: None,
            sepa_debit: None,
            us_bank_account: None,
        }
    }
}
impl Default for CreateInvoicePaymentSettingsPaymentMethodOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,
    /// Verification method for the intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit {
    pub fn new() -> Self {
        Self { mandate_options: None, verification_method: None }
    }
}
impl Default for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Mandate creation
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    pub fn new() -> Self {
        Self { transaction_type: None }
    }
}
impl Default for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType"))
    }
}
/// Verification method for the intent
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod"))
    }
}
/// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self { preferred_language: None }
    }
}
impl Default for CreateInvoicePaymentSettingsPaymentMethodOptionsBancontact {
    fn default() -> Self {
        Self::new()
    }
}
/// Preferred language of the Bancontact authorization page that the customer is redirected to.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::*;
        match self {
            De => "de",
            En => "en",
            Fr => "fr",
            Nl => "nl",
        }
    }
}

impl std::str::FromStr
    for CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "fr" => Ok(Fr),
            "nl" => Ok(Nl),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage"))
    }
}
/// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsCard {
    /// Installment configuration for payments attempted on this invoice (Mexico Only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallments>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure>,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsCard {
    pub fn new() -> Self {
        Self { installments: None, request_three_d_secure: None }
    }
}
impl Default for CreateInvoicePaymentSettingsPaymentMethodOptionsCard {
    fn default() -> Self {
        Self::new()
    }
}
/// Installment configuration for payments attempted on this invoice (Mexico Only).
///
/// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallments {
    /// Setting to true enables installments for this invoice.
    /// Setting to false will prevent any selected plan from applying to a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The selected installment plan to use for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlan>,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallments {
    pub fn new() -> Self {
        Self { enabled: None, plan: None }
    }
}
impl Default for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallments {
    fn default() -> Self {
        Self::new()
    }
}
/// The selected installment plan to use for this invoice.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlan {
    /// For `fixed_count` installment plans, this is the number of installment payments your customer will make to their credit card.
    pub count: u64,
    /// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
    /// One of `month`.
    pub interval: CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval,
    /// Type of installment plan, one of `fixed_count`.
    #[serde(rename = "type")]
    pub type_: CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlan {
    pub fn new(
        count: impl Into<u64>,
        interval: impl Into<
            CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval,
        >,
        type_: impl Into<CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType>,
    ) -> Self {
        Self { count: count.into(), interval: interval.into(), type_: type_.into() }
    }
}
/// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
/// One of `month`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    Month,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    pub fn as_str(self) -> &'static str {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval::*;
        match self {
            Month => "month",
        }
    }
}

impl std::str::FromStr
    for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval::*;
        match s {
            "month" => Ok(Month),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval"))
    }
}
/// Type of installment plan, one of `fixed_count`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    FixedCount,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    pub fn as_str(self) -> &'static str {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::*;
        match self {
            FixedCount => "fixed_count",
        }
    }
}

impl std::str::FromStr
    for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::*;
        match s {
            "fixed_count" => Ok(FixedCount),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType"))
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::*;
        match self {
            Any => "any",
            Automatic => "automatic",
            Challenge => "challenge",
        }
    }
}

impl std::str::FromStr for CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::*;
        match s {
            "any" => Ok(Any),
            "automatic" => Ok(Automatic),
            "challenge" => Ok(Challenge),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure"))
    }
}
/// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections>,
    /// Verification method for the intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount {
    pub fn new() -> Self {
        Self { financial_connections: None, verification_method: None }
    }
}
impl Default for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Financial Connections Session creation
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections {
        /// The list of permissions to request.
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[serde(skip_serializing_if = "Option::is_none")]
pub permissions: Option<Vec<CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions>>,
    /// List of data features that you would like to retrieve upon account creation.
#[serde(skip_serializing_if = "Option::is_none")]
pub prefetch: Option<Vec<CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch>>,

}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections {
    pub fn new() -> Self {
        Self { permissions: None, prefetch: None }
    }
}
impl Default for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections {
    fn default() -> Self {
        Self::new()
    }
}
/// The list of permissions to request.
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions"))
    }
}
/// List of data features that you would like to retrieve upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    Balances,
    Ownership,
    Transactions,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    pub fn as_str(self) -> &'static str {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch"))
    }
}
/// Verification method for the intent
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod"))
    }
}
/// The list of payment method types (e.g.
/// card) to provide to the invoice’s PaymentIntent.
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateInvoicePaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    P24,
    Paynow,
    Paypal,
    Promptpay,
    RevolutPay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateInvoicePaymentSettingsPaymentMethodTypes {
    pub fn as_str(&self) -> &str {
        use CreateInvoicePaymentSettingsPaymentMethodTypes::*;
        match self {
            AchCreditTransfer => "ach_credit_transfer",
            AchDebit => "ach_debit",
            AcssDebit => "acss_debit",
            AmazonPay => "amazon_pay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Boleto => "boleto",
            Card => "card",
            Cashapp => "cashapp",
            CustomerBalance => "customer_balance",
            Eps => "eps",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            Konbini => "konbini",
            Link => "link",
            P24 => "p24",
            Paynow => "paynow",
            Paypal => "paypal",
            Promptpay => "promptpay",
            RevolutPay => "revolut_pay",
            SepaCreditTransfer => "sepa_credit_transfer",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateInvoicePaymentSettingsPaymentMethodTypes {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodTypes::*;
        match s {
            "ach_credit_transfer" => Ok(AchCreditTransfer),
            "ach_debit" => Ok(AchDebit),
            "acss_debit" => Ok(AcssDebit),
            "amazon_pay" => Ok(AmazonPay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "cashapp" => Ok(Cashapp),
            "customer_balance" => Ok(CustomerBalance),
            "eps" => Ok(Eps),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "konbini" => Ok(Konbini),
            "link" => Ok(Link),
            "p24" => Ok(P24),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "promptpay" => Ok(Promptpay),
            "revolut_pay" => Ok(RevolutPay),
            "sepa_credit_transfer" => Ok(SepaCreditTransfer),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for CreateInvoicePaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoicePaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoicePaymentSettingsPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateInvoicePaymentSettingsPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// How to handle pending invoice items on invoice creation.
/// Defaults to `exclude` if the parameter is omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoicePendingInvoiceItemsBehavior {
    Exclude,
    Include,
}
impl CreateInvoicePendingInvoiceItemsBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateInvoicePendingInvoiceItemsBehavior::*;
        match self {
            Exclude => "exclude",
            Include => "include",
        }
    }
}

impl std::str::FromStr for CreateInvoicePendingInvoiceItemsBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePendingInvoiceItemsBehavior::*;
        match s {
            "exclude" => Ok(Exclude),
            "include" => Ok(Include),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateInvoicePendingInvoiceItemsBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoicePendingInvoiceItemsBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoicePendingInvoiceItemsBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateInvoicePendingInvoiceItemsBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateInvoicePendingInvoiceItemsBehavior")
        })
    }
}
/// The rendering-related settings that control how the invoice is displayed on customer-facing surfaces such as PDF and Hosted Invoice Page.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceRendering {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<CreateInvoiceRenderingAmountTaxDisplay>,
    /// Invoice pdf rendering options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf: Option<CreateInvoiceRenderingPdf>,
}
impl CreateInvoiceRendering {
    pub fn new() -> Self {
        Self { amount_tax_display: None, pdf: None }
    }
}
impl Default for CreateInvoiceRendering {
    fn default() -> Self {
        Self::new()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoiceRenderingAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}
impl CreateInvoiceRenderingAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        use CreateInvoiceRenderingAmountTaxDisplay::*;
        match self {
            ExcludeTax => "exclude_tax",
            IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl std::str::FromStr for CreateInvoiceRenderingAmountTaxDisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceRenderingAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateInvoiceRenderingAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoiceRenderingAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoiceRenderingAmountTaxDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateInvoiceRenderingAmountTaxDisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateInvoiceRenderingAmountTaxDisplay")
        })
    }
}
/// Invoice pdf rendering options
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceRenderingPdf {
    /// Page size for invoice PDF. Can be set to `a4`, `letter`, or `auto`.
    ///  If set to `auto`, invoice PDF page size defaults to `a4` for customers with
    ///  Japanese locale and `letter` for customers with other locales.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<CreateInvoiceRenderingPdfPageSize>,
}
impl CreateInvoiceRenderingPdf {
    pub fn new() -> Self {
        Self { page_size: None }
    }
}
impl Default for CreateInvoiceRenderingPdf {
    fn default() -> Self {
        Self::new()
    }
}
/// Page size for invoice PDF. Can be set to `a4`, `letter`, or `auto`.
///  If set to `auto`, invoice PDF page size defaults to `a4` for customers with
///  Japanese locale and `letter` for customers with other locales.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoiceRenderingPdfPageSize {
    A4,
    Auto,
    Letter,
}
impl CreateInvoiceRenderingPdfPageSize {
    pub fn as_str(self) -> &'static str {
        use CreateInvoiceRenderingPdfPageSize::*;
        match self {
            A4 => "a4",
            Auto => "auto",
            Letter => "letter",
        }
    }
}

impl std::str::FromStr for CreateInvoiceRenderingPdfPageSize {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceRenderingPdfPageSize::*;
        match s {
            "a4" => Ok(A4),
            "auto" => Ok(Auto),
            "letter" => Ok(Letter),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateInvoiceRenderingPdfPageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoiceRenderingPdfPageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoiceRenderingPdfPageSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateInvoiceRenderingPdfPageSize {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateInvoiceRenderingPdfPageSize")
        })
    }
}
/// Settings for the cost of shipping for this invoice.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceShippingCost {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
    /// Parameters to create a new ad-hoc shipping rate for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<CreateInvoiceShippingCostShippingRateData>,
}
impl CreateInvoiceShippingCost {
    pub fn new() -> Self {
        Self { shipping_rate: None, shipping_rate_data: None }
    }
}
impl Default for CreateInvoiceShippingCost {
    fn default() -> Self {
        Self::new()
    }
}
/// Parameters to create a new ad-hoc shipping rate for this order.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceShippingCostShippingRateData {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<CreateInvoiceShippingCostShippingRateDataDeliveryEstimate>,
    /// The name of the shipping rate, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    pub display_name: String,
    /// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<CreateInvoiceShippingCostShippingRateDataFixedAmount>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateInvoiceShippingCostShippingRateDataTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// The type of calculation to use on the shipping rate. Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateInvoiceShippingCostShippingRateDataType>,
}
impl CreateInvoiceShippingCostShippingRateData {
    pub fn new(display_name: impl Into<String>) -> Self {
        Self {
            delivery_estimate: None,
            display_name: display_name.into(),
            fixed_amount: None,
            metadata: None,
            tax_behavior: None,
            tax_code: None,
            type_: None,
        }
    }
}
/// The estimated range for how long shipping will take, meant to be displayable to the customer.
/// This will appear on CheckoutSessions.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceShippingCostShippingRateDataDeliveryEstimate {
    /// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximum>,
    /// The lower bound of the estimated range. If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimum>,
}
impl CreateInvoiceShippingCostShippingRateDataDeliveryEstimate {
    pub fn new() -> Self {
        Self { maximum: None, minimum: None }
    }
}
impl Default for CreateInvoiceShippingCostShippingRateDataDeliveryEstimate {
    fn default() -> Self {
        Self::new()
    }
}
/// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximum {
    pub fn new(
        unit: impl Into<CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit>,
        value: impl Into<i64>,
    ) -> Self {
        Self { unit: unit.into(), value: value.into() }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}
impl CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    pub fn as_str(self) -> &'static str {
        use CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit::*;
        match self {
            BusinessDay => "business_day",
            Day => "day",
            Hour => "hour",
            Month => "month",
            Week => "week",
        }
    }
}

impl std::str::FromStr for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit"))
    }
}
/// The lower bound of the estimated range. If empty, represents no lower bound.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimum {
    pub fn new(
        unit: impl Into<CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit>,
        value: impl Into<i64>,
    ) -> Self {
        Self { unit: unit.into(), value: value.into() }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}
impl CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    pub fn as_str(self) -> &'static str {
        use CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit::*;
        match self {
            BusinessDay => "business_day",
            Day => "day",
            Hour => "hour",
            Month => "month",
            Week => "week",
        }
    }
}

impl std::str::FromStr for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit"))
    }
}
/// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceShippingCostShippingRateDataFixedAmount {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Shipping rates defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        std::collections::HashMap<
            stripe_types::Currency,
            CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptions,
        >,
    >,
}
impl CreateInvoiceShippingCostShippingRateDataFixedAmount {
    pub fn new(amount: impl Into<i64>, currency: impl Into<stripe_types::Currency>) -> Self {
        Self { amount: amount.into(), currency: currency.into(), currency_options: None }
    }
}
/// Shipping rates defined in each available currency option.
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior>,
}
impl CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptions {
    pub fn new(amount: impl Into<i64>) -> Self {
        Self { amount: amount.into(), tax_behavior: None }
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr
    for CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior"))
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoiceShippingCostShippingRateDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreateInvoiceShippingCostShippingRateDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateInvoiceShippingCostShippingRateDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateInvoiceShippingCostShippingRateDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceShippingCostShippingRateDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateInvoiceShippingCostShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoiceShippingCostShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoiceShippingCostShippingRateDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateInvoiceShippingCostShippingRateDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateInvoiceShippingCostShippingRateDataTaxBehavior",
            )
        })
    }
}
/// The type of calculation to use on the shipping rate. Can only be `fixed_amount` for now.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoiceShippingCostShippingRateDataType {
    FixedAmount,
}
impl CreateInvoiceShippingCostShippingRateDataType {
    pub fn as_str(self) -> &'static str {
        use CreateInvoiceShippingCostShippingRateDataType::*;
        match self {
            FixedAmount => "fixed_amount",
        }
    }
}

impl std::str::FromStr for CreateInvoiceShippingCostShippingRateDataType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceShippingCostShippingRateDataType::*;
        match s {
            "fixed_amount" => Ok(FixedAmount),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateInvoiceShippingCostShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoiceShippingCostShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoiceShippingCostShippingRateDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateInvoiceShippingCostShippingRateDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateInvoiceShippingCostShippingRateDataType",
            )
        })
    }
}
/// If specified, the funds from the invoice will be transferred to the destination and the ID of the resulting transfer will be found on the invoice's charge.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceTransferData {
    /// The amount that will be transferred automatically when the invoice is paid.
    /// If no amount is set, the full amount is transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ID of an existing, connected Stripe account.
    pub destination: String,
}
impl CreateInvoiceTransferData {
    pub fn new(destination: impl Into<String>) -> Self {
        Self { amount: None, destination: destination.into() }
    }
}
/// This endpoint creates a draft invoice for a given customer.
/// The invoice remains a draft until you [finalize](https://stripe.com/docs/api#finalize_invoice) the invoice, which allows you to [pay](https://stripe.com/docs/api#pay_invoice) or [send](https://stripe.com/docs/api#send_invoice) the invoice to your customers.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateInvoice {
    inner: CreateInvoiceBuilder,
}
impl CreateInvoice {
    /// Construct a new `CreateInvoice`.
    pub fn new() -> Self {
        Self { inner: CreateInvoiceBuilder::new() }
    }
    /// The account tax IDs associated with the invoice. Only editable when the invoice is a draft.
    pub fn account_tax_ids(mut self, account_tax_ids: impl Into<Vec<String>>) -> Self {
        self.inner.account_tax_ids = Some(account_tax_ids.into());
        self
    }
    /// A fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's Stripe account.
    /// The request must be made with an OAuth key or the Stripe-Account header in order to take an application fee.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/billing/invoices/connect#collecting-fees).
    pub fn application_fee_amount(mut self, application_fee_amount: impl Into<i64>) -> Self {
        self.inner.application_fee_amount = Some(application_fee_amount.into());
        self
    }
    /// Controls whether Stripe performs [automatic collection](https://stripe.com/docs/invoicing/integration/automatic-advancement-collection) of the invoice.
    /// If `false`, the invoice's state doesn't automatically advance without an explicit action.
    pub fn auto_advance(mut self, auto_advance: impl Into<bool>) -> Self {
        self.inner.auto_advance = Some(auto_advance.into());
        self
    }
    /// Settings for automatic tax lookup for this invoice.
    pub fn automatic_tax(mut self, automatic_tax: impl Into<CreateInvoiceAutomaticTax>) -> Self {
        self.inner.automatic_tax = Some(automatic_tax.into());
        self
    }
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer.
    /// When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
    /// Defaults to `charge_automatically`.
    pub fn collection_method(
        mut self,
        collection_method: impl Into<stripe_shared::InvoiceCollectionMethod>,
    ) -> Self {
        self.inner.collection_method = Some(collection_method.into());
        self
    }
    /// The currency to create this invoice in. Defaults to that of `customer` if not specified.
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// A list of up to 4 custom fields to be displayed on the invoice.
    pub fn custom_fields(mut self, custom_fields: impl Into<Vec<CustomFieldParams>>) -> Self {
        self.inner.custom_fields = Some(custom_fields.into());
        self
    }
    /// The ID of the customer who will be billed.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// The number of days from when the invoice is created until it is due.
    /// Valid only for invoices where `collection_method=send_invoice`.
    pub fn days_until_due(mut self, days_until_due: impl Into<u32>) -> Self {
        self.inner.days_until_due = Some(days_until_due.into());
        self
    }
    /// ID of the default payment method for the invoice.
    /// It must belong to the customer associated with the invoice.
    /// If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings.
    pub fn default_payment_method(mut self, default_payment_method: impl Into<String>) -> Self {
        self.inner.default_payment_method = Some(default_payment_method.into());
        self
    }
    /// ID of the default payment source for the invoice.
    /// It must belong to the customer associated with the invoice and be in a chargeable state.
    /// If not set, defaults to the subscription's default source, if any, or to the customer's default source.
    pub fn default_source(mut self, default_source: impl Into<String>) -> Self {
        self.inner.default_source = Some(default_source.into());
        self
    }
    /// The tax rates that will apply to any line item that does not have `tax_rates` set.
    pub fn default_tax_rates(mut self, default_tax_rates: impl Into<Vec<String>>) -> Self {
        self.inner.default_tax_rates = Some(default_tax_rates.into());
        self
    }
    /// An arbitrary string attached to the object.
    /// Often useful for displaying to users.
    /// Referenced as 'memo' in the Dashboard.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// The coupons and promotion codes to redeem into discounts for the invoice.
    /// If not specified, inherits the discount from the invoice's customer.
    /// Pass an empty string to avoid inheriting any discounts.
    pub fn discounts(mut self, discounts: impl Into<Vec<DiscountsDataParam>>) -> Self {
        self.inner.discounts = Some(discounts.into());
        self
    }
    /// The date on which payment for this invoice is due.
    /// Valid only for invoices where `collection_method=send_invoice`.
    pub fn due_date(mut self, due_date: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.due_date = Some(due_date.into());
        self
    }
    /// The date when this invoice is in effect.
    /// Same as `finalized_at` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the invoice PDF and receipt.
    pub fn effective_at(mut self, effective_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.effective_at = Some(effective_at.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Footer to be displayed on the invoice.
    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.inner.footer = Some(footer.into());
        self
    }
    /// Revise an existing invoice.
    /// The new invoice will be created in `status=draft`.
    /// See the [revision documentation](https://stripe.com/docs/invoicing/invoice-revisions) for more details.
    pub fn from_invoice(mut self, from_invoice: impl Into<CreateInvoiceFromInvoice>) -> Self {
        self.inner.from_invoice = Some(from_invoice.into());
        self
    }
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    pub fn issuer(mut self, issuer: impl Into<CreateInvoiceIssuer>) -> Self {
        self.inner.issuer = Some(issuer.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// Set the number for this invoice.
    /// If no number is present then a number will be assigned automatically when the invoice is finalized.
    /// In many markets, regulations require invoices to be unique, sequential and / or gapless.
    /// You are responsible for ensuring this is true across all your different invoicing systems in the event that you edit the invoice number using our API.
    /// If you use only Stripe for your invoices and do not change invoice numbers, Stripe handles this aspect of compliance for you automatically.
    pub fn number(mut self, number: impl Into<String>) -> Self {
        self.inner.number = Some(number.into());
        self
    }
    /// The account (if any) for which the funds of the invoice payment are intended.
    /// If set, the invoice will be presented with the branding and support information of the specified account.
    /// See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    pub fn on_behalf_of(mut self, on_behalf_of: impl Into<String>) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of.into());
        self
    }
    /// Configuration settings for the PaymentIntent that is generated when the invoice is finalized.
    pub fn payment_settings(
        mut self,
        payment_settings: impl Into<CreateInvoicePaymentSettings>,
    ) -> Self {
        self.inner.payment_settings = Some(payment_settings.into());
        self
    }
    /// How to handle pending invoice items on invoice creation.
    /// Defaults to `exclude` if the parameter is omitted.
    pub fn pending_invoice_items_behavior(
        mut self,
        pending_invoice_items_behavior: impl Into<CreateInvoicePendingInvoiceItemsBehavior>,
    ) -> Self {
        self.inner.pending_invoice_items_behavior = Some(pending_invoice_items_behavior.into());
        self
    }
    /// The rendering-related settings that control how the invoice is displayed on customer-facing surfaces such as PDF and Hosted Invoice Page.
    pub fn rendering(mut self, rendering: impl Into<CreateInvoiceRendering>) -> Self {
        self.inner.rendering = Some(rendering.into());
        self
    }
    /// Settings for the cost of shipping for this invoice.
    pub fn shipping_cost(mut self, shipping_cost: impl Into<CreateInvoiceShippingCost>) -> Self {
        self.inner.shipping_cost = Some(shipping_cost.into());
        self
    }
    /// Shipping details for the invoice.
    /// The Invoice PDF will use the `shipping_details` value if it is set, otherwise the PDF will render the shipping address from the customer.
    pub fn shipping_details(
        mut self,
        shipping_details: impl Into<RecipientShippingWithOptionalFieldsAddress>,
    ) -> Self {
        self.inner.shipping_details = Some(shipping_details.into());
        self
    }
    /// Extra information about a charge for the customer's credit card statement.
    /// It must contain at least one letter.
    /// If not specified and this invoice is part of a subscription, the default `statement_descriptor` will be set to the first subscription item's product's `statement_descriptor`.
    pub fn statement_descriptor(mut self, statement_descriptor: impl Into<String>) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor.into());
        self
    }
    /// The ID of the subscription to invoice, if any.
    /// If set, the created invoice will only include pending invoice items for that subscription.
    /// The subscription's billing cycle and regular subscription events won't be affected.
    pub fn subscription(mut self, subscription: impl Into<String>) -> Self {
        self.inner.subscription = Some(subscription.into());
        self
    }
    /// If specified, the funds from the invoice will be transferred to the destination and the ID of the resulting transfer will be found on the invoice's charge.
    pub fn transfer_data(mut self, transfer_data: impl Into<CreateInvoiceTransferData>) -> Self {
        self.inner.transfer_data = Some(transfer_data.into());
        self
    }
}
impl Default for CreateInvoice {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateInvoice {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateInvoice {
    type Output = stripe_shared::Invoice;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/invoices").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateInvoiceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_tax_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_advance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_tax: Option<UpdateInvoiceAutomaticTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_method: Option<stripe_shared::InvoiceCollectionMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_fields: Option<Vec<CustomFieldParams>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days_until_due: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_tax_rates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<DiscountsDataParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_date: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<UpdateInvoiceIssuer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_settings: Option<UpdateInvoicePaymentSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rendering: Option<UpdateInvoiceRendering>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_cost: Option<UpdateInvoiceShippingCost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_details: Option<RecipientShippingWithOptionalFieldsAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_data: Option<UpdateInvoiceTransferData>,
}
impl UpdateInvoiceBuilder {
    fn new() -> Self {
        Self {
            account_tax_ids: None,
            application_fee_amount: None,
            auto_advance: None,
            automatic_tax: None,
            collection_method: None,
            custom_fields: None,
            days_until_due: None,
            default_payment_method: None,
            default_source: None,
            default_tax_rates: None,
            description: None,
            discounts: None,
            due_date: None,
            effective_at: None,
            expand: None,
            footer: None,
            issuer: None,
            metadata: None,
            number: None,
            on_behalf_of: None,
            payment_settings: None,
            rendering: None,
            shipping_cost: None,
            shipping_details: None,
            statement_descriptor: None,
            transfer_data: None,
        }
    }
}
/// Settings for automatic tax lookup for this invoice.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceAutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<UpdateInvoiceAutomaticTaxLiability>,
}
impl UpdateInvoiceAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdateInvoiceAutomaticTaxLiabilityType,
}
impl UpdateInvoiceAutomaticTaxLiability {
    pub fn new(type_: impl Into<UpdateInvoiceAutomaticTaxLiabilityType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoiceAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl UpdateInvoiceAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoiceAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpdateInvoiceAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateInvoiceAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateInvoiceAutomaticTaxLiabilityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateInvoiceAutomaticTaxLiabilityType")
        })
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdateInvoiceIssuerType,
}
impl UpdateInvoiceIssuer {
    pub fn new(type_: impl Into<UpdateInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoiceIssuerType {
    Account,
    Self_,
}
impl UpdateInvoiceIssuerType {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoiceIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpdateInvoiceIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateInvoiceIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateInvoiceIssuerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for UpdateInvoiceIssuerType"))
    }
}
/// Configuration settings for the PaymentIntent that is generated when the invoice is finalized.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoicePaymentSettings {
    /// ID of the mandate to be used for this invoice.
    /// It must correspond to the payment method used to pay the invoice, including the invoice's default_payment_method or default_source, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mandate: Option<String>,
    /// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<UpdateInvoicePaymentSettingsPaymentMethodOptions>,
    /// The list of payment method types (e.g.
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<UpdateInvoicePaymentSettingsPaymentMethodTypes>>,
}
impl UpdateInvoicePaymentSettings {
    pub fn new() -> Self {
        Self { default_mandate: None, payment_method_options: None, payment_method_types: None }
    }
}
impl Default for UpdateInvoicePaymentSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptions {
    /// If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit>,
    /// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontact>,
    /// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsCard>,
    /// If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<InvoicePaymentMethodOptionsParam>,
    /// If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub konbini: Option<miniserde::json::Value>,
    /// If paying by `sepa_debit`, this sub-hash contains details about the SEPA Direct Debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub sepa_debit: Option<miniserde::json::Value>,
    /// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount>,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptions {
    pub fn new() -> Self {
        Self {
            acss_debit: None,
            bancontact: None,
            card: None,
            customer_balance: None,
            konbini: None,
            sepa_debit: None,
            us_bank_account: None,
        }
    }
}
impl Default for UpdateInvoicePaymentSettingsPaymentMethodOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit {
    /// Additional fields for Mandate creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,
    /// Verification method for the intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit {
    pub fn new() -> Self {
        Self { mandate_options: None, verification_method: None }
    }
}
impl Default for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Mandate creation
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    pub fn new() -> Self {
        Self { transaction_type: None }
    }
}
impl Default for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType"))
    }
}
/// Verification method for the intent
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod"))
    }
}
/// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self { preferred_language: None }
    }
}
impl Default for UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontact {
    fn default() -> Self {
        Self::new()
    }
}
/// Preferred language of the Bancontact authorization page that the customer is redirected to.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::*;
        match self {
            De => "de",
            En => "en",
            Fr => "fr",
            Nl => "nl",
        }
    }
}

impl std::str::FromStr
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "fr" => Ok(Fr),
            "nl" => Ok(Nl),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage"))
    }
}
/// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsCard {
    /// Installment configuration for payments attempted on this invoice (Mexico Only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallments>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure>,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsCard {
    pub fn new() -> Self {
        Self { installments: None, request_three_d_secure: None }
    }
}
impl Default for UpdateInvoicePaymentSettingsPaymentMethodOptionsCard {
    fn default() -> Self {
        Self::new()
    }
}
/// Installment configuration for payments attempted on this invoice (Mexico Only).
///
/// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallments {
    /// Setting to true enables installments for this invoice.
    /// Setting to false will prevent any selected plan from applying to a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The selected installment plan to use for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlan>,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallments {
    pub fn new() -> Self {
        Self { enabled: None, plan: None }
    }
}
impl Default for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallments {
    fn default() -> Self {
        Self::new()
    }
}
/// The selected installment plan to use for this invoice.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlan {
    /// For `fixed_count` installment plans, this is the number of installment payments your customer will make to their credit card.
    pub count: u64,
    /// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
    /// One of `month`.
    pub interval: UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval,
    /// Type of installment plan, one of `fixed_count`.
    #[serde(rename = "type")]
    pub type_: UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlan {
    pub fn new(
        count: impl Into<u64>,
        interval: impl Into<
            UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval,
        >,
        type_: impl Into<UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType>,
    ) -> Self {
        Self { count: count.into(), interval: interval.into(), type_: type_.into() }
    }
}
/// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
/// One of `month`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    Month,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval::*;
        match self {
            Month => "month",
        }
    }
}

impl std::str::FromStr
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval::*;
        match s {
            "month" => Ok(Month),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval"))
    }
}
/// Type of installment plan, one of `fixed_count`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    FixedCount,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::*;
        match self {
            FixedCount => "fixed_count",
        }
    }
}

impl std::str::FromStr
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::*;
        match s {
            "fixed_count" => Ok(FixedCount),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType"))
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::*;
        match self {
            Any => "any",
            Automatic => "automatic",
            Challenge => "challenge",
        }
    }
}

impl std::str::FromStr for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::*;
        match s {
            "any" => Ok(Any),
            "automatic" => Ok(Automatic),
            "challenge" => Ok(Challenge),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure"))
    }
}
/// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections>,
    /// Verification method for the intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount {
    pub fn new() -> Self {
        Self { financial_connections: None, verification_method: None }
    }
}
impl Default for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional fields for Financial Connections Session creation
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections {
        /// The list of permissions to request.
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[serde(skip_serializing_if = "Option::is_none")]
pub permissions: Option<Vec<UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions>>,
    /// List of data features that you would like to retrieve upon account creation.
#[serde(skip_serializing_if = "Option::is_none")]
pub prefetch: Option<Vec<UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch>>,

}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections {
    pub fn new() -> Self {
        Self { permissions: None, prefetch: None }
    }
}
impl Default for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections {
    fn default() -> Self {
        Self::new()
    }
}
/// The list of permissions to request.
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions"))
    }
}
/// List of data features that you would like to retrieve upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    Balances,
    Ownership,
    Transactions,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch"))
    }
}
/// Verification method for the intent
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod"))
    }
}
/// The list of payment method types (e.g.
/// card) to provide to the invoice’s PaymentIntent.
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateInvoicePaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    P24,
    Paynow,
    Paypal,
    Promptpay,
    RevolutPay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateInvoicePaymentSettingsPaymentMethodTypes {
    pub fn as_str(&self) -> &str {
        use UpdateInvoicePaymentSettingsPaymentMethodTypes::*;
        match self {
            AchCreditTransfer => "ach_credit_transfer",
            AchDebit => "ach_debit",
            AcssDebit => "acss_debit",
            AmazonPay => "amazon_pay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Boleto => "boleto",
            Card => "card",
            Cashapp => "cashapp",
            CustomerBalance => "customer_balance",
            Eps => "eps",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            Konbini => "konbini",
            Link => "link",
            P24 => "p24",
            Paynow => "paynow",
            Paypal => "paypal",
            Promptpay => "promptpay",
            RevolutPay => "revolut_pay",
            SepaCreditTransfer => "sepa_credit_transfer",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateInvoicePaymentSettingsPaymentMethodTypes {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodTypes::*;
        match s {
            "ach_credit_transfer" => Ok(AchCreditTransfer),
            "ach_debit" => Ok(AchDebit),
            "acss_debit" => Ok(AcssDebit),
            "amazon_pay" => Ok(AmazonPay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "cashapp" => Ok(Cashapp),
            "customer_balance" => Ok(CustomerBalance),
            "eps" => Ok(Eps),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "konbini" => Ok(Konbini),
            "link" => Ok(Link),
            "p24" => Ok(P24),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "promptpay" => Ok(Promptpay),
            "revolut_pay" => Ok(RevolutPay),
            "sepa_credit_transfer" => Ok(SepaCreditTransfer),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for UpdateInvoicePaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoicePaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoicePaymentSettingsPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateInvoicePaymentSettingsPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// The rendering-related settings that control how the invoice is displayed on customer-facing surfaces such as PDF and Hosted Invoice Page.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceRendering {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<UpdateInvoiceRenderingAmountTaxDisplay>,
    /// Invoice pdf rendering options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf: Option<UpdateInvoiceRenderingPdf>,
}
impl UpdateInvoiceRendering {
    pub fn new() -> Self {
        Self { amount_tax_display: None, pdf: None }
    }
}
impl Default for UpdateInvoiceRendering {
    fn default() -> Self {
        Self::new()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoiceRenderingAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}
impl UpdateInvoiceRenderingAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoiceRenderingAmountTaxDisplay::*;
        match self {
            ExcludeTax => "exclude_tax",
            IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl std::str::FromStr for UpdateInvoiceRenderingAmountTaxDisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceRenderingAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateInvoiceRenderingAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceRenderingAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceRenderingAmountTaxDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateInvoiceRenderingAmountTaxDisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateInvoiceRenderingAmountTaxDisplay")
        })
    }
}
/// Invoice pdf rendering options
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceRenderingPdf {
    /// Page size for invoice PDF. Can be set to `a4`, `letter`, or `auto`.
    ///  If set to `auto`, invoice PDF page size defaults to `a4` for customers with
    ///  Japanese locale and `letter` for customers with other locales.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<UpdateInvoiceRenderingPdfPageSize>,
}
impl UpdateInvoiceRenderingPdf {
    pub fn new() -> Self {
        Self { page_size: None }
    }
}
impl Default for UpdateInvoiceRenderingPdf {
    fn default() -> Self {
        Self::new()
    }
}
/// Page size for invoice PDF. Can be set to `a4`, `letter`, or `auto`.
///  If set to `auto`, invoice PDF page size defaults to `a4` for customers with
///  Japanese locale and `letter` for customers with other locales.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoiceRenderingPdfPageSize {
    A4,
    Auto,
    Letter,
}
impl UpdateInvoiceRenderingPdfPageSize {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoiceRenderingPdfPageSize::*;
        match self {
            A4 => "a4",
            Auto => "auto",
            Letter => "letter",
        }
    }
}

impl std::str::FromStr for UpdateInvoiceRenderingPdfPageSize {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceRenderingPdfPageSize::*;
        match s {
            "a4" => Ok(A4),
            "auto" => Ok(Auto),
            "letter" => Ok(Letter),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateInvoiceRenderingPdfPageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceRenderingPdfPageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceRenderingPdfPageSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateInvoiceRenderingPdfPageSize {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateInvoiceRenderingPdfPageSize")
        })
    }
}
/// Settings for the cost of shipping for this invoice.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceShippingCost {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
    /// Parameters to create a new ad-hoc shipping rate for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<UpdateInvoiceShippingCostShippingRateData>,
}
impl UpdateInvoiceShippingCost {
    pub fn new() -> Self {
        Self { shipping_rate: None, shipping_rate_data: None }
    }
}
impl Default for UpdateInvoiceShippingCost {
    fn default() -> Self {
        Self::new()
    }
}
/// Parameters to create a new ad-hoc shipping rate for this order.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceShippingCostShippingRateData {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<UpdateInvoiceShippingCostShippingRateDataDeliveryEstimate>,
    /// The name of the shipping rate, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    pub display_name: String,
    /// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<UpdateInvoiceShippingCostShippingRateDataFixedAmount>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateInvoiceShippingCostShippingRateDataTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// The type of calculation to use on the shipping rate. Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpdateInvoiceShippingCostShippingRateDataType>,
}
impl UpdateInvoiceShippingCostShippingRateData {
    pub fn new(display_name: impl Into<String>) -> Self {
        Self {
            delivery_estimate: None,
            display_name: display_name.into(),
            fixed_amount: None,
            metadata: None,
            tax_behavior: None,
            tax_code: None,
            type_: None,
        }
    }
}
/// The estimated range for how long shipping will take, meant to be displayable to the customer.
/// This will appear on CheckoutSessions.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceShippingCostShippingRateDataDeliveryEstimate {
    /// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximum>,
    /// The lower bound of the estimated range. If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimum>,
}
impl UpdateInvoiceShippingCostShippingRateDataDeliveryEstimate {
    pub fn new() -> Self {
        Self { maximum: None, minimum: None }
    }
}
impl Default for UpdateInvoiceShippingCostShippingRateDataDeliveryEstimate {
    fn default() -> Self {
        Self::new()
    }
}
/// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximum {
    pub fn new(
        unit: impl Into<UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit>,
        value: impl Into<i64>,
    ) -> Self {
        Self { unit: unit.into(), value: value.into() }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}
impl UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit::*;
        match self {
            BusinessDay => "business_day",
            Day => "day",
            Hour => "hour",
            Month => "month",
            Week => "week",
        }
    }
}

impl std::str::FromStr for UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit"))
    }
}
/// The lower bound of the estimated range. If empty, represents no lower bound.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimum {
    pub fn new(
        unit: impl Into<UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit>,
        value: impl Into<i64>,
    ) -> Self {
        Self { unit: unit.into(), value: value.into() }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}
impl UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit::*;
        match self {
            BusinessDay => "business_day",
            Day => "day",
            Hour => "hour",
            Month => "month",
            Week => "week",
        }
    }
}

impl std::str::FromStr for UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit"))
    }
}
/// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceShippingCostShippingRateDataFixedAmount {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Shipping rates defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        std::collections::HashMap<
            stripe_types::Currency,
            UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptions,
        >,
    >,
}
impl UpdateInvoiceShippingCostShippingRateDataFixedAmount {
    pub fn new(amount: impl Into<i64>, currency: impl Into<stripe_types::Currency>) -> Self {
        Self { amount: amount.into(), currency: currency.into(), currency_options: None }
    }
}
/// Shipping rates defined in each available currency option.
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior>,
}
impl UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptions {
    pub fn new(amount: impl Into<i64>) -> Self {
        Self { amount: amount.into(), tax_behavior: None }
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr
    for UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior"))
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoiceShippingCostShippingRateDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpdateInvoiceShippingCostShippingRateDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoiceShippingCostShippingRateDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateInvoiceShippingCostShippingRateDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceShippingCostShippingRateDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateInvoiceShippingCostShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceShippingCostShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceShippingCostShippingRateDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateInvoiceShippingCostShippingRateDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateInvoiceShippingCostShippingRateDataTaxBehavior",
            )
        })
    }
}
/// The type of calculation to use on the shipping rate. Can only be `fixed_amount` for now.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoiceShippingCostShippingRateDataType {
    FixedAmount,
}
impl UpdateInvoiceShippingCostShippingRateDataType {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoiceShippingCostShippingRateDataType::*;
        match self {
            FixedAmount => "fixed_amount",
        }
    }
}

impl std::str::FromStr for UpdateInvoiceShippingCostShippingRateDataType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceShippingCostShippingRateDataType::*;
        match s {
            "fixed_amount" => Ok(FixedAmount),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateInvoiceShippingCostShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceShippingCostShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceShippingCostShippingRateDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateInvoiceShippingCostShippingRateDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateInvoiceShippingCostShippingRateDataType",
            )
        })
    }
}
/// If specified, the funds from the invoice will be transferred to the destination and the ID of the resulting transfer will be found on the invoice's charge.
/// This will be unset if you POST an empty value.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceTransferData {
    /// The amount that will be transferred automatically when the invoice is paid.
    /// If no amount is set, the full amount is transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ID of an existing, connected Stripe account.
    pub destination: String,
}
impl UpdateInvoiceTransferData {
    pub fn new(destination: impl Into<String>) -> Self {
        Self { amount: None, destination: destination.into() }
    }
}
/// Draft invoices are fully editable.
/// Once an invoice is [finalized](https://stripe.com/docs/billing/invoices/workflow#finalized),.
/// monetary values, as well as `collection_method`, become uneditable.
///
/// If you would like to stop the Stripe Billing engine from automatically finalizing, reattempting payments on,.
/// sending reminders for, or [automatically reconciling](https://stripe.com/docs/billing/invoices/reconciliation) invoices, pass.
/// `auto_advance=false`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoice {
    inner: UpdateInvoiceBuilder,
    invoice: stripe_shared::InvoiceId,
}
impl UpdateInvoice {
    /// Construct a new `UpdateInvoice`.
    pub fn new(invoice: impl Into<stripe_shared::InvoiceId>) -> Self {
        Self { invoice: invoice.into(), inner: UpdateInvoiceBuilder::new() }
    }
    /// The account tax IDs associated with the invoice. Only editable when the invoice is a draft.
    pub fn account_tax_ids(mut self, account_tax_ids: impl Into<Vec<String>>) -> Self {
        self.inner.account_tax_ids = Some(account_tax_ids.into());
        self
    }
    /// A fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's Stripe account.
    /// The request must be made with an OAuth key or the Stripe-Account header in order to take an application fee.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/billing/invoices/connect#collecting-fees).
    pub fn application_fee_amount(mut self, application_fee_amount: impl Into<i64>) -> Self {
        self.inner.application_fee_amount = Some(application_fee_amount.into());
        self
    }
    /// Controls whether Stripe performs [automatic collection](https://stripe.com/docs/invoicing/integration/automatic-advancement-collection) of the invoice.
    pub fn auto_advance(mut self, auto_advance: impl Into<bool>) -> Self {
        self.inner.auto_advance = Some(auto_advance.into());
        self
    }
    /// Settings for automatic tax lookup for this invoice.
    pub fn automatic_tax(mut self, automatic_tax: impl Into<UpdateInvoiceAutomaticTax>) -> Self {
        self.inner.automatic_tax = Some(automatic_tax.into());
        self
    }
    /// Either `charge_automatically` or `send_invoice`.
    /// This field can be updated only on `draft` invoices.
    pub fn collection_method(
        mut self,
        collection_method: impl Into<stripe_shared::InvoiceCollectionMethod>,
    ) -> Self {
        self.inner.collection_method = Some(collection_method.into());
        self
    }
    /// A list of up to 4 custom fields to be displayed on the invoice.
    /// If a value for `custom_fields` is specified, the list specified will replace the existing custom field list on this invoice.
    /// Pass an empty string to remove previously-defined fields.
    pub fn custom_fields(mut self, custom_fields: impl Into<Vec<CustomFieldParams>>) -> Self {
        self.inner.custom_fields = Some(custom_fields.into());
        self
    }
    /// The number of days from which the invoice is created until it is due.
    /// Only valid for invoices where `collection_method=send_invoice`.
    /// This field can only be updated on `draft` invoices.
    pub fn days_until_due(mut self, days_until_due: impl Into<u32>) -> Self {
        self.inner.days_until_due = Some(days_until_due.into());
        self
    }
    /// ID of the default payment method for the invoice.
    /// It must belong to the customer associated with the invoice.
    /// If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings.
    pub fn default_payment_method(mut self, default_payment_method: impl Into<String>) -> Self {
        self.inner.default_payment_method = Some(default_payment_method.into());
        self
    }
    /// ID of the default payment source for the invoice.
    /// It must belong to the customer associated with the invoice and be in a chargeable state.
    /// If not set, defaults to the subscription's default source, if any, or to the customer's default source.
    pub fn default_source(mut self, default_source: impl Into<String>) -> Self {
        self.inner.default_source = Some(default_source.into());
        self
    }
    /// The tax rates that will apply to any line item that does not have `tax_rates` set.
    /// Pass an empty string to remove previously-defined tax rates.
    pub fn default_tax_rates(mut self, default_tax_rates: impl Into<Vec<String>>) -> Self {
        self.inner.default_tax_rates = Some(default_tax_rates.into());
        self
    }
    /// An arbitrary string attached to the object.
    /// Often useful for displaying to users.
    /// Referenced as 'memo' in the Dashboard.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// The discounts that will apply to the invoice.
    /// Pass an empty string to remove previously-defined discounts.
    pub fn discounts(mut self, discounts: impl Into<Vec<DiscountsDataParam>>) -> Self {
        self.inner.discounts = Some(discounts.into());
        self
    }
    /// The date on which payment for this invoice is due.
    /// Only valid for invoices where `collection_method=send_invoice`.
    /// This field can only be updated on `draft` invoices.
    pub fn due_date(mut self, due_date: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.due_date = Some(due_date.into());
        self
    }
    /// The date when this invoice is in effect.
    /// Same as `finalized_at` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the invoice PDF and receipt.
    pub fn effective_at(mut self, effective_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.effective_at = Some(effective_at.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Footer to be displayed on the invoice.
    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.inner.footer = Some(footer.into());
        self
    }
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    pub fn issuer(mut self, issuer: impl Into<UpdateInvoiceIssuer>) -> Self {
        self.inner.issuer = Some(issuer.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// Set the number for this invoice.
    /// If no number is present then a number will be assigned automatically when the invoice is finalized.
    /// In many markets, regulations require invoices to be unique, sequential and / or gapless.
    /// You are responsible for ensuring this is true across all your different invoicing systems in the event that you edit the invoice number using our API.
    /// If you use only Stripe for your invoices and do not change invoice numbers, Stripe handles this aspect of compliance for you automatically.
    pub fn number(mut self, number: impl Into<String>) -> Self {
        self.inner.number = Some(number.into());
        self
    }
    /// The account (if any) for which the funds of the invoice payment are intended.
    /// If set, the invoice will be presented with the branding and support information of the specified account.
    /// See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    pub fn on_behalf_of(mut self, on_behalf_of: impl Into<String>) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of.into());
        self
    }
    /// Configuration settings for the PaymentIntent that is generated when the invoice is finalized.
    pub fn payment_settings(
        mut self,
        payment_settings: impl Into<UpdateInvoicePaymentSettings>,
    ) -> Self {
        self.inner.payment_settings = Some(payment_settings.into());
        self
    }
    /// The rendering-related settings that control how the invoice is displayed on customer-facing surfaces such as PDF and Hosted Invoice Page.
    pub fn rendering(mut self, rendering: impl Into<UpdateInvoiceRendering>) -> Self {
        self.inner.rendering = Some(rendering.into());
        self
    }
    /// Settings for the cost of shipping for this invoice.
    pub fn shipping_cost(mut self, shipping_cost: impl Into<UpdateInvoiceShippingCost>) -> Self {
        self.inner.shipping_cost = Some(shipping_cost.into());
        self
    }
    /// Shipping details for the invoice.
    /// The Invoice PDF will use the `shipping_details` value if it is set, otherwise the PDF will render the shipping address from the customer.
    pub fn shipping_details(
        mut self,
        shipping_details: impl Into<RecipientShippingWithOptionalFieldsAddress>,
    ) -> Self {
        self.inner.shipping_details = Some(shipping_details.into());
        self
    }
    /// Extra information about a charge for the customer's credit card statement.
    /// It must contain at least one letter.
    /// If not specified and this invoice is part of a subscription, the default `statement_descriptor` will be set to the first subscription item's product's `statement_descriptor`.
    pub fn statement_descriptor(mut self, statement_descriptor: impl Into<String>) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor.into());
        self
    }
    /// If specified, the funds from the invoice will be transferred to the destination and the ID of the resulting transfer will be found on the invoice's charge.
    /// This will be unset if you POST an empty value.
    pub fn transfer_data(mut self, transfer_data: impl Into<UpdateInvoiceTransferData>) -> Self {
        self.inner.transfer_data = Some(transfer_data.into());
        self
    }
}
impl UpdateInvoice {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for UpdateInvoice {
    type Output = stripe_shared::Invoice;

    fn build(&self) -> RequestBuilder {
        let invoice = &self.invoice;
        RequestBuilder::new(StripeMethod::Post, format!("/invoices/{invoice}")).form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct FinalizeInvoiceInvoiceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_advance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl FinalizeInvoiceInvoiceBuilder {
    fn new() -> Self {
        Self { auto_advance: None, expand: None }
    }
}
/// Stripe automatically finalizes drafts before sending and attempting payment on invoices.
/// However, if you’d like to finalize a draft invoice manually, you can do so using this method.
#[derive(Clone, Debug, serde::Serialize)]
pub struct FinalizeInvoiceInvoice {
    inner: FinalizeInvoiceInvoiceBuilder,
    invoice: stripe_shared::InvoiceId,
}
impl FinalizeInvoiceInvoice {
    /// Construct a new `FinalizeInvoiceInvoice`.
    pub fn new(invoice: impl Into<stripe_shared::InvoiceId>) -> Self {
        Self { invoice: invoice.into(), inner: FinalizeInvoiceInvoiceBuilder::new() }
    }
    /// Controls whether Stripe performs [automatic collection](https://stripe.com/docs/invoicing/integration/automatic-advancement-collection) of the invoice.
    /// If `false`, the invoice's state doesn't automatically advance without an explicit action.
    pub fn auto_advance(mut self, auto_advance: impl Into<bool>) -> Self {
        self.inner.auto_advance = Some(auto_advance.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl FinalizeInvoiceInvoice {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for FinalizeInvoiceInvoice {
    type Output = stripe_shared::Invoice;

    fn build(&self) -> RequestBuilder {
        let invoice = &self.invoice;
        RequestBuilder::new(StripeMethod::Post, format!("/invoices/{invoice}/finalize"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct MarkUncollectibleInvoiceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl MarkUncollectibleInvoiceBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Marking an invoice as uncollectible is useful for keeping track of bad debts that can be written off for accounting purposes.
#[derive(Clone, Debug, serde::Serialize)]
pub struct MarkUncollectibleInvoice {
    inner: MarkUncollectibleInvoiceBuilder,
    invoice: stripe_shared::InvoiceId,
}
impl MarkUncollectibleInvoice {
    /// Construct a new `MarkUncollectibleInvoice`.
    pub fn new(invoice: impl Into<stripe_shared::InvoiceId>) -> Self {
        Self { invoice: invoice.into(), inner: MarkUncollectibleInvoiceBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl MarkUncollectibleInvoice {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for MarkUncollectibleInvoice {
    type Output = stripe_shared::Invoice;

    fn build(&self) -> RequestBuilder {
        let invoice = &self.invoice;
        RequestBuilder::new(StripeMethod::Post, format!("/invoices/{invoice}/mark_uncollectible"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct PayInvoiceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forgive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mandate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    off_session: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paid_out_of_band: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
}
impl PayInvoiceBuilder {
    fn new() -> Self {
        Self {
            expand: None,
            forgive: None,
            mandate: None,
            off_session: None,
            paid_out_of_band: None,
            payment_method: None,
            source: None,
        }
    }
}
/// Stripe automatically creates and then attempts to collect payment on invoices for customers on subscriptions according to your [subscriptions settings](https://dashboard.stripe.com/account/billing/automatic).
/// However, if you’d like to attempt payment on an invoice out of the normal collection schedule or for some other reason, you can do so.
#[derive(Clone, Debug, serde::Serialize)]
pub struct PayInvoice {
    inner: PayInvoiceBuilder,
    invoice: stripe_shared::InvoiceId,
}
impl PayInvoice {
    /// Construct a new `PayInvoice`.
    pub fn new(invoice: impl Into<stripe_shared::InvoiceId>) -> Self {
        Self { invoice: invoice.into(), inner: PayInvoiceBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// In cases where the source used to pay the invoice has insufficient funds, passing `forgive=true` controls whether a charge should be attempted for the full amount available on the source, up to the amount to fully pay the invoice.
    /// This effectively forgives the difference between the amount available on the source and the amount due.
    ///
    ///
    /// Passing `forgive=false` will fail the charge if the source hasn't been pre-funded with the right amount.
    /// An example for this case is with ACH Credit Transfers and wires: if the amount wired is less than the amount due by a small amount, you might want to forgive the difference.
    /// Defaults to `false`.
    pub fn forgive(mut self, forgive: impl Into<bool>) -> Self {
        self.inner.forgive = Some(forgive.into());
        self
    }
    /// ID of the mandate to be used for this invoice.
    /// It must correspond to the payment method used to pay the invoice, including the payment_method param or the invoice's default_payment_method or default_source, if set.
    pub fn mandate(mut self, mandate: impl Into<String>) -> Self {
        self.inner.mandate = Some(mandate.into());
        self
    }
    /// Indicates if a customer is on or off-session while an invoice payment is attempted.
    /// Defaults to `true` (off-session).
    pub fn off_session(mut self, off_session: impl Into<bool>) -> Self {
        self.inner.off_session = Some(off_session.into());
        self
    }
    /// Boolean representing whether an invoice is paid outside of Stripe.
    /// This will result in no charge being made.
    /// Defaults to `false`.
    pub fn paid_out_of_band(mut self, paid_out_of_band: impl Into<bool>) -> Self {
        self.inner.paid_out_of_band = Some(paid_out_of_band.into());
        self
    }
    /// A PaymentMethod to be charged.
    /// The PaymentMethod must be the ID of a PaymentMethod belonging to the customer associated with the invoice being paid.
    pub fn payment_method(mut self, payment_method: impl Into<String>) -> Self {
        self.inner.payment_method = Some(payment_method.into());
        self
    }
    /// A payment source to be charged.
    /// The source must be the ID of a source belonging to the customer associated with the invoice being paid.
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.inner.source = Some(source.into());
        self
    }
}
impl PayInvoice {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for PayInvoice {
    type Output = stripe_shared::Invoice;

    fn build(&self) -> RequestBuilder {
        let invoice = &self.invoice;
        RequestBuilder::new(StripeMethod::Post, format!("/invoices/{invoice}/pay"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct SendInvoiceInvoiceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl SendInvoiceInvoiceBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Stripe will automatically send invoices to customers according to your [subscriptions settings](https://dashboard.stripe.com/account/billing/automatic).
/// However, if you’d like to manually send an invoice to your customer out of the normal schedule, you can do so.
/// When sending invoices that have already been paid, there will be no reference to the payment in the email.
///
/// Requests made in test-mode result in no emails being sent, despite sending an `invoice.sent` event.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SendInvoiceInvoice {
    inner: SendInvoiceInvoiceBuilder,
    invoice: stripe_shared::InvoiceId,
}
impl SendInvoiceInvoice {
    /// Construct a new `SendInvoiceInvoice`.
    pub fn new(invoice: impl Into<stripe_shared::InvoiceId>) -> Self {
        Self { invoice: invoice.into(), inner: SendInvoiceInvoiceBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl SendInvoiceInvoice {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for SendInvoiceInvoice {
    type Output = stripe_shared::Invoice;

    fn build(&self) -> RequestBuilder {
        let invoice = &self.invoice;
        RequestBuilder::new(StripeMethod::Post, format!("/invoices/{invoice}/send"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct VoidInvoiceInvoiceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl VoidInvoiceInvoiceBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Mark a finalized invoice as void.
/// This cannot be undone.
/// Voiding an invoice is similar to [deletion](https://stripe.com/docs/api#delete_invoice), however it only applies to finalized invoices and maintains a papertrail where the invoice can still be found.
///
/// Consult with local regulations to determine whether and how an invoice might be amended, canceled, or voided in the jurisdiction you’re doing business in.
/// You might need to [issue another invoice](https://stripe.com/docs/api#create_invoice) or [credit note](https://stripe.com/docs/api#create_credit_note) instead.
/// Stripe recommends that you consult with your legal counsel for advice specific to your business.
#[derive(Clone, Debug, serde::Serialize)]
pub struct VoidInvoiceInvoice {
    inner: VoidInvoiceInvoiceBuilder,
    invoice: stripe_shared::InvoiceId,
}
impl VoidInvoiceInvoice {
    /// Construct a new `VoidInvoiceInvoice`.
    pub fn new(invoice: impl Into<stripe_shared::InvoiceId>) -> Self {
        Self { invoice: invoice.into(), inner: VoidInvoiceInvoiceBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl VoidInvoiceInvoice {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for VoidInvoiceInvoice {
    type Output = stripe_shared::Invoice;

    fn build(&self) -> RequestBuilder {
        let invoice = &self.invoice;
        RequestBuilder::new(StripeMethod::Post, format!("/invoices/{invoice}/void"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreatePreviewInvoiceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_tax: Option<CreatePreviewInvoiceAutomaticTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coupon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_details: Option<CreatePreviewInvoiceCustomerDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<DiscountsDataParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_items: Option<Vec<CreatePreviewInvoiceInvoiceItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<CreatePreviewInvoiceIssuer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_details: Option<CreatePreviewInvoiceScheduleDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_details: Option<CreatePreviewInvoiceSubscriptionDetails>,
}
impl CreatePreviewInvoiceBuilder {
    fn new() -> Self {
        Self {
            automatic_tax: None,
            coupon: None,
            currency: None,
            customer: None,
            customer_details: None,
            discounts: None,
            expand: None,
            invoice_items: None,
            issuer: None,
            on_behalf_of: None,
            schedule: None,
            schedule_details: None,
            subscription: None,
            subscription_details: None,
        }
    }
}
/// Settings for automatic tax lookup for this invoice preview.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceAutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreatePreviewInvoiceAutomaticTaxLiability>,
}
impl CreatePreviewInvoiceAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreatePreviewInvoiceAutomaticTaxLiabilityType,
}
impl CreatePreviewInvoiceAutomaticTaxLiability {
    pub fn new(type_: impl Into<CreatePreviewInvoiceAutomaticTaxLiabilityType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl CreatePreviewInvoiceAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePreviewInvoiceAutomaticTaxLiabilityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePreviewInvoiceAutomaticTaxLiabilityType",
            )
        })
    }
}
/// Details about the customer you want to invoice or overrides for an existing customer.
/// If `automatic_tax` is enabled then one of `customer`, `customer_details`, `subscription`, or `schedule` must be set.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceCustomerDetails {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<OptionalFieldsAddress>,
    /// The customer's shipping information. Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CustomerShipping>,
    /// Tax details about the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxParam>,
    /// The customer's tax exemption. One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<CreatePreviewInvoiceCustomerDetailsTaxExempt>,
    /// The customer's tax IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<CreatePreviewInvoiceCustomerDetailsTaxIds>>,
}
impl CreatePreviewInvoiceCustomerDetails {
    pub fn new() -> Self {
        Self { address: None, shipping: None, tax: None, tax_exempt: None, tax_ids: None }
    }
}
impl Default for CreatePreviewInvoiceCustomerDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// The customer's tax exemption. One of `none`, `exempt`, or `reverse`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceCustomerDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}
impl CreatePreviewInvoiceCustomerDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceCustomerDetailsTaxExempt::*;
        match self {
            Exempt => "exempt",
            None => "none",
            Reverse => "reverse",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceCustomerDetailsTaxExempt {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceCustomerDetailsTaxExempt::*;
        match s {
            "exempt" => Ok(Exempt),
            "none" => Ok(None),
            "reverse" => Ok(Reverse),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceCustomerDetailsTaxExempt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePreviewInvoiceCustomerDetailsTaxExempt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePreviewInvoiceCustomerDetailsTaxExempt",
            )
        })
    }
}
/// The customer's tax IDs.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceCustomerDetailsTaxIds {
    /// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bh_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `kz_bin`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: CreatePreviewInvoiceCustomerDetailsTaxIdsType,
    /// Value of the tax ID.
    pub value: String,
}
impl CreatePreviewInvoiceCustomerDetailsTaxIds {
    pub fn new(
        type_: impl Into<CreatePreviewInvoiceCustomerDetailsTaxIdsType>,
        value: impl Into<String>,
    ) -> Self {
        Self { type_: type_.into(), value: value.into() }
    }
}
/// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bh_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `kz_bin`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePreviewInvoiceCustomerDetailsTaxIdsType {
    AdNrt,
    AeTrn,
    ArCuit,
    AuAbn,
    AuArn,
    BgUic,
    BhVat,
    BoTin,
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
    CnTin,
    CoNit,
    CrTin,
    DoRcn,
    EcRuc,
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
    KzBin,
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NgTin,
    NoVat,
    NoVoec,
    NzGst,
    OmVat,
    PeRuc,
    PhTin,
    RoTin,
    RsPib,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    SvNit,
    ThVat,
    TrTin,
    TwVat,
    UaVat,
    UsEin,
    UyRuc,
    VeRif,
    VnTin,
    ZaVat,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePreviewInvoiceCustomerDetailsTaxIdsType {
    pub fn as_str(&self) -> &str {
        use CreatePreviewInvoiceCustomerDetailsTaxIdsType::*;
        match self {
            AdNrt => "ad_nrt",
            AeTrn => "ae_trn",
            ArCuit => "ar_cuit",
            AuAbn => "au_abn",
            AuArn => "au_arn",
            BgUic => "bg_uic",
            BhVat => "bh_vat",
            BoTin => "bo_tin",
            BrCnpj => "br_cnpj",
            BrCpf => "br_cpf",
            CaBn => "ca_bn",
            CaGstHst => "ca_gst_hst",
            CaPstBc => "ca_pst_bc",
            CaPstMb => "ca_pst_mb",
            CaPstSk => "ca_pst_sk",
            CaQst => "ca_qst",
            ChVat => "ch_vat",
            ClTin => "cl_tin",
            CnTin => "cn_tin",
            CoNit => "co_nit",
            CrTin => "cr_tin",
            DoRcn => "do_rcn",
            EcRuc => "ec_ruc",
            EgTin => "eg_tin",
            EsCif => "es_cif",
            EuOssVat => "eu_oss_vat",
            EuVat => "eu_vat",
            GbVat => "gb_vat",
            GeVat => "ge_vat",
            HkBr => "hk_br",
            HuTin => "hu_tin",
            IdNpwp => "id_npwp",
            IlVat => "il_vat",
            InGst => "in_gst",
            IsVat => "is_vat",
            JpCn => "jp_cn",
            JpRn => "jp_rn",
            JpTrn => "jp_trn",
            KePin => "ke_pin",
            KrBrn => "kr_brn",
            KzBin => "kz_bin",
            LiUid => "li_uid",
            MxRfc => "mx_rfc",
            MyFrp => "my_frp",
            MyItn => "my_itn",
            MySst => "my_sst",
            NgTin => "ng_tin",
            NoVat => "no_vat",
            NoVoec => "no_voec",
            NzGst => "nz_gst",
            OmVat => "om_vat",
            PeRuc => "pe_ruc",
            PhTin => "ph_tin",
            RoTin => "ro_tin",
            RsPib => "rs_pib",
            RuInn => "ru_inn",
            RuKpp => "ru_kpp",
            SaVat => "sa_vat",
            SgGst => "sg_gst",
            SgUen => "sg_uen",
            SiTin => "si_tin",
            SvNit => "sv_nit",
            ThVat => "th_vat",
            TrTin => "tr_tin",
            TwVat => "tw_vat",
            UaVat => "ua_vat",
            UsEin => "us_ein",
            UyRuc => "uy_ruc",
            VeRif => "ve_rif",
            VnTin => "vn_tin",
            ZaVat => "za_vat",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceCustomerDetailsTaxIdsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceCustomerDetailsTaxIdsType::*;
        match s {
            "ad_nrt" => Ok(AdNrt),
            "ae_trn" => Ok(AeTrn),
            "ar_cuit" => Ok(ArCuit),
            "au_abn" => Ok(AuAbn),
            "au_arn" => Ok(AuArn),
            "bg_uic" => Ok(BgUic),
            "bh_vat" => Ok(BhVat),
            "bo_tin" => Ok(BoTin),
            "br_cnpj" => Ok(BrCnpj),
            "br_cpf" => Ok(BrCpf),
            "ca_bn" => Ok(CaBn),
            "ca_gst_hst" => Ok(CaGstHst),
            "ca_pst_bc" => Ok(CaPstBc),
            "ca_pst_mb" => Ok(CaPstMb),
            "ca_pst_sk" => Ok(CaPstSk),
            "ca_qst" => Ok(CaQst),
            "ch_vat" => Ok(ChVat),
            "cl_tin" => Ok(ClTin),
            "cn_tin" => Ok(CnTin),
            "co_nit" => Ok(CoNit),
            "cr_tin" => Ok(CrTin),
            "do_rcn" => Ok(DoRcn),
            "ec_ruc" => Ok(EcRuc),
            "eg_tin" => Ok(EgTin),
            "es_cif" => Ok(EsCif),
            "eu_oss_vat" => Ok(EuOssVat),
            "eu_vat" => Ok(EuVat),
            "gb_vat" => Ok(GbVat),
            "ge_vat" => Ok(GeVat),
            "hk_br" => Ok(HkBr),
            "hu_tin" => Ok(HuTin),
            "id_npwp" => Ok(IdNpwp),
            "il_vat" => Ok(IlVat),
            "in_gst" => Ok(InGst),
            "is_vat" => Ok(IsVat),
            "jp_cn" => Ok(JpCn),
            "jp_rn" => Ok(JpRn),
            "jp_trn" => Ok(JpTrn),
            "ke_pin" => Ok(KePin),
            "kr_brn" => Ok(KrBrn),
            "kz_bin" => Ok(KzBin),
            "li_uid" => Ok(LiUid),
            "mx_rfc" => Ok(MxRfc),
            "my_frp" => Ok(MyFrp),
            "my_itn" => Ok(MyItn),
            "my_sst" => Ok(MySst),
            "ng_tin" => Ok(NgTin),
            "no_vat" => Ok(NoVat),
            "no_voec" => Ok(NoVoec),
            "nz_gst" => Ok(NzGst),
            "om_vat" => Ok(OmVat),
            "pe_ruc" => Ok(PeRuc),
            "ph_tin" => Ok(PhTin),
            "ro_tin" => Ok(RoTin),
            "rs_pib" => Ok(RsPib),
            "ru_inn" => Ok(RuInn),
            "ru_kpp" => Ok(RuKpp),
            "sa_vat" => Ok(SaVat),
            "sg_gst" => Ok(SgGst),
            "sg_uen" => Ok(SgUen),
            "si_tin" => Ok(SiTin),
            "sv_nit" => Ok(SvNit),
            "th_vat" => Ok(ThVat),
            "tr_tin" => Ok(TrTin),
            "tw_vat" => Ok(TwVat),
            "ua_vat" => Ok(UaVat),
            "us_ein" => Ok(UsEin),
            "uy_ruc" => Ok(UyRuc),
            "ve_rif" => Ok(VeRif),
            "vn_tin" => Ok(VnTin),
            "za_vat" => Ok(ZaVat),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceCustomerDetailsTaxIdsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceCustomerDetailsTaxIdsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceCustomerDetailsTaxIdsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePreviewInvoiceCustomerDetailsTaxIdsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// List of invoice items to add or update in the upcoming invoice preview.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceInvoiceItems {
    /// The integer amount in cents (or local equivalent) of previewed invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// Only applicable to new invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// An arbitrary string which you can attach to the invoice item.
    /// The description is displayed in the invoice for easy tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Explicitly controls whether discounts apply to this invoice item.
    /// Defaults to true, except for negative invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,
    /// The coupons to redeem into discounts for the invoice item in the preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// The ID of the invoice item to update in preview.
    /// If not specified, a new invoice item will be added to the preview of the upcoming invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoiceitem: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The period associated with this invoice item.
    /// When set to different values, the period will be rendered on the invoice.
    /// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
    /// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreatePreviewInvoiceInvoiceItemsPriceData>,
    /// Non-negative integer. The quantity of units for the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreatePreviewInvoiceInvoiceItemsTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// The tax rates that apply to the item. When set, any `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
    /// The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    /// This unit_amount will be multiplied by the quantity to get the full amount.
    /// If you want to apply a credit to the customer's account, pass a negative unit_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl CreatePreviewInvoiceInvoiceItems {
    pub fn new() -> Self {
        Self {
            amount: None,
            currency: None,
            description: None,
            discountable: None,
            discounts: None,
            invoiceitem: None,
            metadata: None,
            period: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_behavior: None,
            tax_code: None,
            tax_rates: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
impl Default for CreatePreviewInvoiceInvoiceItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceInvoiceItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreatePreviewInvoiceInvoiceItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl CreatePreviewInvoiceInvoiceItemsPriceData {
    pub fn new(currency: impl Into<stripe_types::Currency>, product: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreatePreviewInvoiceInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceInvoiceItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceInvoiceItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceInvoiceItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceInvoiceItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePreviewInvoiceInvoiceItemsPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePreviewInvoiceInvoiceItemsPriceDataTaxBehavior",
            )
        })
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceInvoiceItemsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreatePreviewInvoiceInvoiceItemsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceInvoiceItemsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceInvoiceItemsTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceInvoiceItemsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceInvoiceItemsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceInvoiceItemsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceInvoiceItemsTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePreviewInvoiceInvoiceItemsTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePreviewInvoiceInvoiceItemsTaxBehavior",
            )
        })
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreatePreviewInvoiceIssuerType,
}
impl CreatePreviewInvoiceIssuer {
    pub fn new(type_: impl Into<CreatePreviewInvoiceIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceIssuerType {
    Account,
    Self_,
}
impl CreatePreviewInvoiceIssuerType {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePreviewInvoiceIssuerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreatePreviewInvoiceIssuerType")
        })
    }
}
/// The schedule creation or modification params to apply as a preview.
/// Cannot be used with `subscription` or `subscription_` prefixed fields.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceScheduleDetails {
    /// Behavior of the subscription schedule and underlying subscription when it ends.
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.
    /// `cancel` will end the subscription schedule and cancel the underlying subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_behavior: Option<CreatePreviewInvoiceScheduleDetailsEndBehavior>,
    /// List representing phases of the subscription schedule.
    /// Each phase can be customized to have different durations, plans, and coupons.
    /// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<CreatePreviewInvoiceScheduleDetailsPhases>>,
    /// In cases where the `schedule_details` params update the currently active phase, specifies if and how to prorate at the time of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<CreatePreviewInvoiceScheduleDetailsProrationBehavior>,
}
impl CreatePreviewInvoiceScheduleDetails {
    pub fn new() -> Self {
        Self { end_behavior: None, phases: None, proration_behavior: None }
    }
}
impl Default for CreatePreviewInvoiceScheduleDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// Behavior of the subscription schedule and underlying subscription when it ends.
/// Possible values are `release` or `cancel` with the default being `release`.
/// `release` will end the subscription schedule and keep the underlying subscription running.
/// `cancel` will end the subscription schedule and cancel the underlying subscription.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceScheduleDetailsEndBehavior {
    Cancel,
    Release,
}
impl CreatePreviewInvoiceScheduleDetailsEndBehavior {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceScheduleDetailsEndBehavior::*;
        match self {
            Cancel => "cancel",
            Release => "release",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceScheduleDetailsEndBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceScheduleDetailsEndBehavior::*;
        match s {
            "cancel" => Ok(Cancel),
            "release" => Ok(Release),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceScheduleDetailsEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceScheduleDetailsEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceScheduleDetailsEndBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePreviewInvoiceScheduleDetailsEndBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePreviewInvoiceScheduleDetailsEndBehavior",
            )
        })
    }
}
/// List representing phases of the subscription schedule.
/// Each phase can be customized to have different durations, plans, and coupons.
/// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceScheduleDetailsPhases {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<Vec<CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItems>>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Automatic tax settings for this phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTax>,
    /// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
    /// Cannot be set to `phase_start` if this phase specifies a trial.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<CreatePreviewInvoiceScheduleDetailsPhasesBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<BillingThresholdsParam>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<stripe_shared::InvoiceCollectionMethod>,
    /// The ID of the coupon to apply to this phase of the subscription schedule.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `discounts` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// ID of the default payment method for the subscription schedule.
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will set the Subscription's [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates), which means they will be the Invoice's [`default_tax_rates`](https://stripe.com/docs/api/invoices/create#create_invoice-default_tax_rates) for any Invoices issued by the Subscription during this Phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,
    /// Subscription description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The coupons to redeem into discounts for the schedule phase.
    /// If not specified, inherits the discount from the subscription's customer.
    /// Pass an empty string to avoid inheriting any discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// The date at which this phase of the subscription schedule ends.
    /// If set, `iterations` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<CreatePreviewInvoiceScheduleDetailsPhasesEndDate>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettings>,
    /// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
    pub items: Vec<CreatePreviewInvoiceScheduleDetailsPhasesItems>,
    /// Integer representing the multiplier applied to the price interval.
    /// For example, `iterations=2` applied to a price with `interval=month` and `interval_count=3` results in a phase of duration `2 * 3 months = 6 months`.
    /// If set, `end_date` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a phase.
    /// Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered, adding new keys and replacing existing keys in the subscription's `metadata`.
    /// Individual keys in the subscription's `metadata` can be unset by posting an empty value to them in the phase's `metadata`.
    /// To unset all keys in the subscription's `metadata`, update the subscription directly or unset every key individually from the phase's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The account on behalf of which to charge, for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    /// Whether the subscription schedule will create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase.
    /// The default value is `create_prorations`.
    /// This setting controls prorations when a phase is started asynchronously and it is persisted as a field on the phase.
    /// It's different from the request-level [proration_behavior](https://stripe.com/docs/api/subscription_schedules/update#update_subscription_schedule-proration_behavior) parameter which controls what happens if the update request affects the billing configuration of the current phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<CreatePreviewInvoiceScheduleDetailsPhasesProrationBehavior>,
    /// The date at which this phase of the subscription schedule starts or `now`.
    /// Must be set on the first phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<CreatePreviewInvoiceScheduleDetailsPhasesStartDate>,
    /// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferDataSpecs>,
    /// If set to true the entire phase is counted as a trial and the customer will not be charged for any fees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,
    /// Sets the phase to trialing from the start date to this date.
    /// Must be before the phase end date, can not be combined with `trial`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<CreatePreviewInvoiceScheduleDetailsPhasesTrialEnd>,
}
impl CreatePreviewInvoiceScheduleDetailsPhases {
    pub fn new(items: impl Into<Vec<CreatePreviewInvoiceScheduleDetailsPhasesItems>>) -> Self {
        Self {
            add_invoice_items: None,
            application_fee_percent: None,
            automatic_tax: None,
            billing_cycle_anchor: None,
            billing_thresholds: None,
            collection_method: None,
            coupon: None,
            currency: None,
            default_payment_method: None,
            default_tax_rates: None,
            description: None,
            discounts: None,
            end_date: None,
            invoice_settings: None,
            items: items.into(),
            iterations: None,
            metadata: None,
            on_behalf_of: None,
            proration_behavior: None,
            start_date: None,
            transfer_data: None,
            trial: None,
            trial_end: None,
        }
    }
}
/// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
/// You may pass up to 20 items.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItems {
    /// The coupons to redeem into discounts for the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceData>,
    /// Quantity for this item. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the item. When set, the `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItems {
    pub fn new() -> Self {
        Self { discounts: None, price: None, price_data: None, quantity: None, tax_rates: None }
    }
}
impl Default for CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceData {
    pub fn new(currency: impl Into<stripe_types::Currency>, product: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr
    for CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePreviewInvoiceScheduleDetailsPhasesAddInvoiceItemsPriceDataTaxBehavior"))
    }
}
/// Automatic tax settings for this phase.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTaxLiability>,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTaxLiability {
    pub fn new(
        type_: impl Into<CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePreviewInvoiceScheduleDetailsPhasesAutomaticTaxLiabilityType"))
    }
}
/// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
/// Cannot be set to `phase_start` if this phase specifies a trial.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    Automatic,
    PhaseStart,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceScheduleDetailsPhasesBillingCycleAnchor::*;
        match self {
            Automatic => "automatic",
            PhaseStart => "phase_start",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceScheduleDetailsPhasesBillingCycleAnchor::*;
        match s {
            "automatic" => Ok(Automatic),
            "phase_start" => Ok(PhaseStart),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePreviewInvoiceScheduleDetailsPhasesBillingCycleAnchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePreviewInvoiceScheduleDetailsPhasesBillingCycleAnchor",
            )
        })
    }
}
/// The date at which this phase of the subscription schedule ends.
/// If set, `iterations` must not be set.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePreviewInvoiceScheduleDetailsPhasesEndDate {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettings {
    /// The account tax IDs associated with this phase of the subscription schedule.
    /// Will be set on invoices generated by this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    /// This value will be `null` for subscription schedules where `billing=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettingsIssuer>,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettings {
    pub fn new() -> Self {
        Self { account_tax_ids: None, days_until_due: None, issuer: None }
    }
}
impl Default for CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettingsIssuer {
    pub fn new(
        type_: impl Into<CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    Account,
    Self_,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePreviewInvoiceScheduleDetailsPhasesInvoiceSettingsIssuerType"))
    }
}
/// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceScheduleDetailsPhasesItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<ItemBillingThresholdsParam>,
    /// The coupons to redeem into discounts for the subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a configuration item.
    /// Metadata on a configuration item will update the underlying subscription item's `metadata` when the phase is entered, adding new keys and replacing existing keys.
    /// Individual keys in the subscription item's `metadata` can be unset by posting an empty value to them in the configuration item's `metadata`.
    /// To unset all keys in the subscription item's `metadata`, update the subscription item directly or unset every key individually from the configuration item's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The plan ID to subscribe to. You may specify the same ID in `plan` and `price`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceData>,
    /// Quantity for the given price.
    /// Can be set only if the price's `usage_type` is `licensed` and not `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesItems {
    pub fn new() -> Self {
        Self {
            billing_thresholds: None,
            discounts: None,
            metadata: None,
            plan: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for CreatePreviewInvoiceScheduleDetailsPhasesItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceData {
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        product: impl Into<String>,
        recurring: impl Into<CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurring>,
    ) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            recurring: recurring.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurring {
    pub fn new(
        interval: impl Into<CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval>,
    ) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr
    for CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataRecurringInterval"))
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePreviewInvoiceScheduleDetailsPhasesItemsPriceDataTaxBehavior"))
    }
}
/// Whether the subscription schedule will create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase.
/// The default value is `create_prorations`.
/// This setting controls prorations when a phase is started asynchronously and it is persisted as a field on the phase.
/// It's different from the request-level [proration_behavior](https://stripe.com/docs/api/subscription_schedules/update#update_subscription_schedule-proration_behavior) parameter which controls what happens if the update request affects the billing configuration of the current phase.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceScheduleDetailsPhasesProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl CreatePreviewInvoiceScheduleDetailsPhasesProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceScheduleDetailsPhasesProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceScheduleDetailsPhasesProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceScheduleDetailsPhasesProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceScheduleDetailsPhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceScheduleDetailsPhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceScheduleDetailsPhasesProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePreviewInvoiceScheduleDetailsPhasesProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePreviewInvoiceScheduleDetailsPhasesProrationBehavior",
            )
        })
    }
}
/// The date at which this phase of the subscription schedule starts or `now`.
/// Must be set on the first phase.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePreviewInvoiceScheduleDetailsPhasesStartDate {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// Sets the phase to trialing from the start date to this date.
/// Must be before the phase end date, can not be combined with `trial`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePreviewInvoiceScheduleDetailsPhasesTrialEnd {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// In cases where the `schedule_details` params update the currently active phase, specifies if and how to prorate at the time of the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceScheduleDetailsProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl CreatePreviewInvoiceScheduleDetailsProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceScheduleDetailsProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceScheduleDetailsProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceScheduleDetailsProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceScheduleDetailsProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceScheduleDetailsProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceScheduleDetailsProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePreviewInvoiceScheduleDetailsProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePreviewInvoiceScheduleDetailsProrationBehavior",
            )
        })
    }
}
/// The subscription creation or modification params to apply as a preview.
/// Cannot be used with `schedule` or `schedule_details` fields.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceSubscriptionDetails {
    /// For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
    /// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    /// For existing subscriptions, the value can only be set to `now` or `unchanged`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<CreatePreviewInvoiceSubscriptionDetailsBillingCycleAnchor>,
    /// A timestamp at which the subscription should cancel.
    /// If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`.
    /// If set during a future period, this will always cause a proration for that period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at: Option<stripe_types::Timestamp>,
    /// Boolean indicating whether this subscription should cancel at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,
    /// This simulates the subscription being canceled or expired immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_now: Option<bool>,
    /// If provided, the invoice returned will preview updating or creating a subscription with these default tax rates.
    /// The default tax rates will apply to any line item that does not have `tax_rates` set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,
    /// A list of up to 20 subscription items, each with an attached price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CreatePreviewInvoiceSubscriptionDetailsItems>>,
    /// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<CreatePreviewInvoiceSubscriptionDetailsProrationBehavior>,
    /// If previewing an update to a subscription, and doing proration, `subscription_details.proration_date` forces the proration to be calculated as though the update was done at the specified time.
    /// The time given must be within the current subscription period and within the current phase of the schedule backing this subscription, if the schedule exists.
    /// If set, `subscription`, and one of `subscription_details.items`, or `subscription_details.trial_end` are required.
    /// Also, `subscription_details.proration_behavior` cannot be set to 'none'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<stripe_types::Timestamp>,
    /// For paused subscriptions, setting `subscription_details.resume_at` to `now` will preview the invoice that will be generated if the subscription is resumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_at: Option<CreatePreviewInvoiceSubscriptionDetailsResumeAt>,
    /// Date a subscription is intended to start (can be future or past).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<stripe_types::Timestamp>,
    /// If provided, the invoice returned will preview updating or creating a subscription with that trial end.
    /// If set, one of `subscription_details.items` or `subscription` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<CreatePreviewInvoiceSubscriptionDetailsTrialEnd>,
}
impl CreatePreviewInvoiceSubscriptionDetails {
    pub fn new() -> Self {
        Self {
            billing_cycle_anchor: None,
            cancel_at: None,
            cancel_at_period_end: None,
            cancel_now: None,
            default_tax_rates: None,
            items: None,
            proration_behavior: None,
            proration_date: None,
            resume_at: None,
            start_date: None,
            trial_end: None,
        }
    }
}
impl Default for CreatePreviewInvoiceSubscriptionDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
/// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
/// For existing subscriptions, the value can only be set to `now` or `unchanged`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePreviewInvoiceSubscriptionDetailsBillingCycleAnchor {
    Now,
    Unchanged,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// A list of up to 20 subscription items, each with an attached price.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceSubscriptionDetailsItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<ItemBillingThresholdsParam>,
    /// Delete all usage for a given subscription item.
    /// Allowed only when `deleted` is set to `true` and the current plan's `usage_type` is `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_usage: Option<bool>,
    /// A flag that, if set to `true`, will delete the specified item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The coupons to redeem into discounts for the subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// Subscription item to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// The ID of the price object.
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreatePreviewInvoiceSubscriptionDetailsItemsPriceData>,
    /// Quantity for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl CreatePreviewInvoiceSubscriptionDetailsItems {
    pub fn new() -> Self {
        Self {
            billing_thresholds: None,
            clear_usage: None,
            deleted: None,
            discounts: None,
            id: None,
            metadata: None,
            plan: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for CreatePreviewInvoiceSubscriptionDetailsItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceSubscriptionDetailsItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl CreatePreviewInvoiceSubscriptionDetailsItemsPriceData {
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        product: impl Into<String>,
        recurring: impl Into<CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurring>,
    ) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            recurring: recurring.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurring {
    pub fn new(
        interval: impl Into<CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval>,
    ) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataRecurringInterval"))
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePreviewInvoiceSubscriptionDetailsItemsPriceDataTaxBehavior"))
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
/// The default value is `create_prorations`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceSubscriptionDetailsProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl CreatePreviewInvoiceSubscriptionDetailsProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceSubscriptionDetailsProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceSubscriptionDetailsProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceSubscriptionDetailsProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceSubscriptionDetailsProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceSubscriptionDetailsProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceSubscriptionDetailsProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePreviewInvoiceSubscriptionDetailsProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePreviewInvoiceSubscriptionDetailsProrationBehavior",
            )
        })
    }
}
/// For paused subscriptions, setting `subscription_details.resume_at` to `now` will preview the invoice that will be generated if the subscription is resumed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePreviewInvoiceSubscriptionDetailsResumeAt {
    Now,
}
impl CreatePreviewInvoiceSubscriptionDetailsResumeAt {
    pub fn as_str(self) -> &'static str {
        use CreatePreviewInvoiceSubscriptionDetailsResumeAt::*;
        match self {
            Now => "now",
        }
    }
}

impl std::str::FromStr for CreatePreviewInvoiceSubscriptionDetailsResumeAt {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePreviewInvoiceSubscriptionDetailsResumeAt::*;
        match s {
            "now" => Ok(Now),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePreviewInvoiceSubscriptionDetailsResumeAt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePreviewInvoiceSubscriptionDetailsResumeAt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePreviewInvoiceSubscriptionDetailsResumeAt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePreviewInvoiceSubscriptionDetailsResumeAt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePreviewInvoiceSubscriptionDetailsResumeAt",
            )
        })
    }
}
/// If provided, the invoice returned will preview updating or creating a subscription with that trial end.
/// If set, one of `subscription_details.items` or `subscription` is required.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePreviewInvoiceSubscriptionDetailsTrialEnd {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// At any time, you can preview the upcoming invoice for a customer.
/// This will show you all the charges that are pending, including subscription renewal charges, invoice item charges, etc.
/// It will also show you any discounts that are applicable to the invoice.
///
/// Note that when you are viewing an upcoming invoice, you are simply viewing a preview – the invoice has not yet been created.
/// As such, the upcoming invoice will not show up in invoice listing calls, and you cannot use the API to pay or edit the invoice.
/// If you want to change the amount that your customer will be billed, you can add, remove, or update pending invoice items, or update the customer’s discount.
///
/// You can preview the effects of updating a subscription, including a preview of what proration will take place.
/// To ensure that the actual proration is calculated exactly the same as the previewed proration, you should pass the `subscription_details.proration_date` parameter when doing the actual subscription update.
/// The recommended way to get only the prorations being previewed is to consider only proration line items where `period[start]` is equal to the `subscription_details.proration_date` value passed in the request.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePreviewInvoice {
    inner: CreatePreviewInvoiceBuilder,
}
impl CreatePreviewInvoice {
    /// Construct a new `CreatePreviewInvoice`.
    pub fn new() -> Self {
        Self { inner: CreatePreviewInvoiceBuilder::new() }
    }
    /// Settings for automatic tax lookup for this invoice preview.
    pub fn automatic_tax(
        mut self,
        automatic_tax: impl Into<CreatePreviewInvoiceAutomaticTax>,
    ) -> Self {
        self.inner.automatic_tax = Some(automatic_tax.into());
        self
    }
    /// The ID of the coupon to apply to this phase of the subscription schedule.
    /// This field has been deprecated and will be removed in a future API version.
    /// Use `discounts` instead.
    pub fn coupon(mut self, coupon: impl Into<String>) -> Self {
        self.inner.coupon = Some(coupon.into());
        self
    }
    /// The currency to preview this invoice in. Defaults to that of `customer` if not specified.
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// The identifier of the customer whose upcoming invoice you'd like to retrieve.
    /// If `automatic_tax` is enabled then one of `customer`, `customer_details`, `subscription`, or `schedule` must be set.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// Details about the customer you want to invoice or overrides for an existing customer.
    /// If `automatic_tax` is enabled then one of `customer`, `customer_details`, `subscription`, or `schedule` must be set.
    pub fn customer_details(
        mut self,
        customer_details: impl Into<CreatePreviewInvoiceCustomerDetails>,
    ) -> Self {
        self.inner.customer_details = Some(customer_details.into());
        self
    }
    /// The coupons to redeem into discounts for the invoice preview.
    /// If not specified, inherits the discount from the subscription or customer.
    /// This works for both coupons directly applied to an invoice and coupons applied to a subscription.
    /// Pass an empty string to avoid inheriting any discounts.
    pub fn discounts(mut self, discounts: impl Into<Vec<DiscountsDataParam>>) -> Self {
        self.inner.discounts = Some(discounts.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// List of invoice items to add or update in the upcoming invoice preview.
    pub fn invoice_items(
        mut self,
        invoice_items: impl Into<Vec<CreatePreviewInvoiceInvoiceItems>>,
    ) -> Self {
        self.inner.invoice_items = Some(invoice_items.into());
        self
    }
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    pub fn issuer(mut self, issuer: impl Into<CreatePreviewInvoiceIssuer>) -> Self {
        self.inner.issuer = Some(issuer.into());
        self
    }
    /// The account (if any) for which the funds of the invoice payment are intended.
    /// If set, the invoice will be presented with the branding and support information of the specified account.
    /// See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    pub fn on_behalf_of(mut self, on_behalf_of: impl Into<String>) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of.into());
        self
    }
    /// The identifier of the schedule whose upcoming invoice you'd like to retrieve.
    /// Cannot be used with subscription or subscription fields.
    pub fn schedule(mut self, schedule: impl Into<String>) -> Self {
        self.inner.schedule = Some(schedule.into());
        self
    }
    /// The schedule creation or modification params to apply as a preview.
    /// Cannot be used with `subscription` or `subscription_` prefixed fields.
    pub fn schedule_details(
        mut self,
        schedule_details: impl Into<CreatePreviewInvoiceScheduleDetails>,
    ) -> Self {
        self.inner.schedule_details = Some(schedule_details.into());
        self
    }
    /// The identifier of the subscription for which you'd like to retrieve the upcoming invoice.
    /// If not provided, but a `subscription_items` is provided, you will preview creating a subscription with those items.
    /// If neither `subscription` nor `subscription_items` is provided, you will retrieve the next upcoming invoice from among the customer's subscriptions.
    pub fn subscription(mut self, subscription: impl Into<String>) -> Self {
        self.inner.subscription = Some(subscription.into());
        self
    }
    /// The subscription creation or modification params to apply as a preview.
    /// Cannot be used with `schedule` or `schedule_details` fields.
    pub fn subscription_details(
        mut self,
        subscription_details: impl Into<CreatePreviewInvoiceSubscriptionDetails>,
    ) -> Self {
        self.inner.subscription_details = Some(subscription_details.into());
        self
    }
}
impl Default for CreatePreviewInvoice {
    fn default() -> Self {
        Self::new()
    }
}
impl CreatePreviewInvoice {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreatePreviewInvoice {
    type Output = stripe_shared::Invoice;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/invoices/create_preview").form(&self.inner)
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct OptionalFieldsAddress {
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
impl OptionalFieldsAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for OptionalFieldsAddress {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct TaxParam {
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    /// Stripe recommends updating the IP address when a new PaymentMethod is attached or the address field on the customer is updated.
    /// We recommend against updating this field more frequently since it could result in unexpected tax location/reporting outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
impl TaxParam {
    pub fn new() -> Self {
        Self { ip_address: None }
    }
}
impl Default for TaxParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct DiscountsDataParam {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
    /// ID of the promotion code to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}
impl DiscountsDataParam {
    pub fn new() -> Self {
        Self { coupon: None, discount: None, promotion_code: None }
    }
}
impl Default for DiscountsDataParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct Period {
    /// The end of the period, which must be greater than or equal to the start. This value is inclusive.
    pub end: stripe_types::Timestamp,
    /// The start of the period. This value is inclusive.
    pub start: stripe_types::Timestamp,
}
impl Period {
    pub fn new(
        end: impl Into<stripe_types::Timestamp>,
        start: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        Self { end: end.into(), start: start.into() }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct BillingThresholdsParam {
    /// Monetary threshold that triggers the subscription to advance to a new billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,
    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}
impl BillingThresholdsParam {
    pub fn new() -> Self {
        Self { amount_gte: None, reset_billing_cycle_anchor: None }
    }
}
impl Default for BillingThresholdsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ItemBillingThresholdsParam {
    /// Number of units that meets the billing threshold to advance the subscription to a new billing period (e.g., it takes 10 $5 units to meet a $50 [monetary threshold](https://stripe.com/docs/api/subscriptions/update#update_subscription-billing_thresholds-amount_gte)).
    pub usage_gte: i64,
}
impl ItemBillingThresholdsParam {
    pub fn new(usage_gte: impl Into<i64>) -> Self {
        Self { usage_gte: usage_gte.into() }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct TransferDataSpecs {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: String,
}
impl TransferDataSpecs {
    pub fn new(destination: impl Into<String>) -> Self {
        Self { amount_percent: None, destination: destination.into() }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct CustomFieldParams {
    /// The name of the custom field. This may be up to 40 characters.
    pub name: String,
    /// The value of the custom field. This may be up to 140 characters.
    pub value: String,
}
impl CustomFieldParams {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self { name: name.into(), value: value.into() }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct EuBankTransferParam {
    /// The desired country code of the bank account information.
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: String,
}
impl EuBankTransferParam {
    pub fn new(country: impl Into<String>) -> Self {
        Self { country: country.into() }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct CustomerShipping {
    /// Customer shipping address.
    pub address: OptionalFieldsAddress,
    /// Customer name.
    pub name: String,
    /// Customer phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl CustomerShipping {
    pub fn new(address: impl Into<OptionalFieldsAddress>, name: impl Into<String>) -> Self {
        Self { address: address.into(), name: name.into(), phone: None }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct BankTransferParam {
    /// Configuration for eu_bank_transfer funding type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<EuBankTransferParam>,
    /// The bank transfer type that can be used for funding.
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl BankTransferParam {
    pub fn new() -> Self {
        Self { eu_bank_transfer: None, type_: None }
    }
}
impl Default for BankTransferParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct RecipientShippingWithOptionalFieldsAddress {
    /// Shipping address
    pub address: OptionalFieldsAddress,
    /// Recipient name.
    pub name: String,
    /// Recipient phone (including extension)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl RecipientShippingWithOptionalFieldsAddress {
    pub fn new(address: impl Into<OptionalFieldsAddress>, name: impl Into<String>) -> Self {
        Self { address: address.into(), name: name.into(), phone: None }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct InvoicePaymentMethodOptionsParam {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<BankTransferParam>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<String>,
}
impl InvoicePaymentMethodOptionsParam {
    pub fn new() -> Self {
        Self { bank_transfer: None, funding_type: None }
    }
}
impl Default for InvoicePaymentMethodOptionsParam {
    fn default() -> Self {
        Self::new()
    }
}
