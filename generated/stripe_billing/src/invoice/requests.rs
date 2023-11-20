#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SearchInvoice<'a> {
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
    /// See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for invoices](https://stripe.com/docs/search#query-fields-for-invoices).
    pub query: &'a str,
}
impl<'a> SearchInvoice<'a> {
    pub fn new(query: &'a str) -> Self {
        Self {
            expand: Default::default(),
            limit: Default::default(),
            page: Default::default(),
            query,
        }
    }
}
impl<'a> SearchInvoice<'a> {
    /// Search for invoices you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
    /// Don’t use search in read-after-write flows where strict consistency is necessary.
    ///
    /// Under normal operating conditions, data is searchable in less than a minute.
    /// Occasionally, propagation of new or updated data can be up to an hour behind during outages.
    /// Search functionality is not available to merchants in India.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<SearchReturned> {
        client.get_query("/invoices/search", self)
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SearchReturned {
    pub data: Vec<stripe_types::Invoice>,
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
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SearchReturnedObject {
    SearchResult,
}

impl SearchReturnedObject {
    pub fn as_str(self) -> &'static str {
        use SearchReturnedObject::*;
        match self {
            SearchResult => "search_result",
        }
    }
}

impl std::str::FromStr for SearchReturnedObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SearchReturnedObject::*;
        match s {
            "search_result" => Ok(SearchResult),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SearchReturnedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SearchReturnedObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SearchReturnedObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SearchReturnedObject"))
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingInvoice<'a> {
    /// Settings for automatic tax lookup for this invoice preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpcomingInvoiceAutomaticTax>,
    /// The code of the coupon to apply.
    ///
    /// If `subscription` or `subscription_items` is provided, the invoice returned will preview updating or creating a subscription with that coupon.
    /// Otherwise, it will preview applying that coupon to the customer for the next upcoming invoice from among the customer's subscriptions.
    /// The invoice can be previewed without a coupon by passing this value as an empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// The currency to preview this invoice in.
    ///
    /// Defaults to that of `customer` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// The identifier of the customer whose upcoming invoice you'd like to retrieve.
    ///
    /// If `automatic_tax` is enabled then one of `customer`, `customer_details`, `subscription`, or `schedule` must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// Details about the customer you want to invoice or overrides for an existing customer.
    ///
    /// If `automatic_tax` is enabled then one of `customer`, `customer_details`, `subscription`, or `schedule` must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_details: Option<UpcomingInvoiceCustomerDetails<'a>>,
    /// The coupons to redeem into discounts for the invoice preview.
    ///
    /// If not specified, inherits the discount from the customer or subscription.
    /// This only works for coupons directly applied to the invoice.
    /// To apply a coupon to a subscription, you must use the `coupon` parameter instead.
    /// Pass an empty string to avoid inheriting any discounts.
    /// To preview the upcoming invoice for a subscription that hasn't been created, use `coupon` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [UpcomingInvoiceDiscounts<'a>]>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// List of invoice items to add or update in the upcoming invoice preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_items: Option<&'a [UpcomingInvoiceInvoiceItems<'a>]>,
    /// The identifier of the schedule whose upcoming invoice you'd like to retrieve.
    ///
    /// Cannot be used with subscription or subscription fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<&'a str>,
    /// The identifier of the subscription for which you'd like to retrieve the upcoming invoice.
    ///
    /// If not provided, but a `subscription_items` is provided, you will preview creating a subscription with those items.
    /// If neither `subscription` nor `subscription_items` is provided, you will retrieve the next upcoming invoice from among the customer's subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
    /// For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
    ///
    /// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    /// For existing subscriptions, the value can only be set to `now` or `unchanged`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_billing_cycle_anchor: Option<UpcomingInvoiceSubscriptionBillingCycleAnchor>,
    /// Timestamp indicating when the subscription should be scheduled to cancel.
    ///
    /// Will prorate if within the current period and prorations have been enabled using `proration_behavior`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel_at: Option<stripe_types::Timestamp>,
    /// Boolean indicating whether this subscription should cancel at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel_at_period_end: Option<bool>,
    /// This simulates the subscription being canceled or expired immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel_now: Option<bool>,
    /// If provided, the invoice returned will preview updating or creating a subscription with these default tax rates.
    ///
    /// The default tax rates will apply to any line item that does not have `tax_rates` set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_default_tax_rates: Option<&'a [&'a str]>,
    /// A list of up to 20 subscription items, each with an attached price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_items: Option<&'a [UpcomingInvoiceSubscriptionItems<'a>]>,
    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    ///
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_behavior: Option<UpcomingInvoiceSubscriptionProrationBehavior>,
    /// If previewing an update to a subscription, and doing proration, `subscription_proration_date` forces the proration to be calculated as though the update was done at the specified time.
    ///
    /// The time given must be within the current subscription period and within the current phase of the schedule backing this subscription, if the schedule exists.
    /// If set, `subscription`, and one of `subscription_items`, or `subscription_trial_end` are required.
    /// Also, `subscription_proration_behavior` cannot be set to 'none'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_date: Option<stripe_types::Timestamp>,
    /// For paused subscriptions, setting `subscription_resume_at` to `now` will preview the invoice that will be generated if the subscription is resumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_resume_at: Option<UpcomingInvoiceSubscriptionResumeAt>,
    /// Date a subscription is intended to start (can be future or past).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_start_date: Option<stripe_types::Timestamp>,
    /// If provided, the invoice returned will preview updating or creating a subscription with that trial end.
    ///
    /// If set, one of `subscription_items` or `subscription` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_trial_end: Option<UpcomingInvoiceSubscriptionTrialEnd>,
    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    ///
    /// Setting `subscription_trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `subscription_trial_end` is not allowed.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_trial_from_plan: Option<bool>,
}
impl<'a> UpcomingInvoice<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings for automatic tax lookup for this invoice preview.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceAutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    ///
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
}
impl UpcomingInvoiceAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Details about the customer you want to invoice or overrides for an existing customer.
///
/// If `automatic_tax` is enabled then one of `customer`, `customer_details`, `subscription`, or `schedule` must be set.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingInvoiceCustomerDetails<'a> {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpcomingInvoiceCustomerDetailsAddress<'a>>,
    /// The customer's shipping information.
    ///
    /// Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<UpcomingInvoiceCustomerDetailsShipping<'a>>,
    /// Tax details about the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<UpcomingInvoiceCustomerDetailsTax<'a>>,
    /// The customer's tax exemption.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<UpcomingInvoiceCustomerDetailsTaxExempt>,
    /// The customer's tax IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<&'a [UpcomingInvoiceCustomerDetailsTaxIds<'a>]>,
}
impl<'a> UpcomingInvoiceCustomerDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingInvoiceCustomerDetailsAddress<'a> {
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
impl<'a> UpcomingInvoiceCustomerDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's shipping information.
///
/// Appears on invoices emailed to this customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceCustomerDetailsShipping<'a> {
    /// Customer shipping address.
    pub address: UpcomingInvoiceCustomerDetailsShippingAddress<'a>,
    /// Customer name.
    pub name: &'a str,
    /// Customer phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> UpcomingInvoiceCustomerDetailsShipping<'a> {
    pub fn new(address: UpcomingInvoiceCustomerDetailsShippingAddress<'a>, name: &'a str) -> Self {
        Self { address, name, phone: Default::default() }
    }
}
/// Customer shipping address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingInvoiceCustomerDetailsShippingAddress<'a> {
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
impl<'a> UpcomingInvoiceCustomerDetailsShippingAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tax details about the customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingInvoiceCustomerDetailsTax<'a> {
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    ///
    /// Stripe recommends updating the IP address when a new PaymentMethod is attached or the address field on the customer is updated.
    /// We recommend against updating this field more frequently since it could result in unexpected tax location/reporting outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,
}
impl<'a> UpcomingInvoiceCustomerDetailsTax<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's tax exemption.
///
/// One of `none`, `exempt`, or `reverse`.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceCustomerDetailsTaxExempt::*;
        match s {
            "exempt" => Ok(Exempt),
            "none" => Ok(None),
            "reverse" => Ok(Reverse),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingInvoiceCustomerDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// The customer's tax IDs.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceCustomerDetailsTaxIds<'a> {
    /// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: UpcomingInvoiceCustomerDetailsTaxIdsType,
    /// Value of the tax ID.
    pub value: &'a str,
}
impl<'a> UpcomingInvoiceCustomerDetailsTaxIds<'a> {
    pub fn new(type_: UpcomingInvoiceCustomerDetailsTaxIdsType, value: &'a str) -> Self {
        Self { type_, value }
    }
}
/// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpcomingInvoiceCustomerDetailsTaxIdsType {
    AdNrt,
    AeTrn,
    ArCuit,
    AuAbn,
    AuArn,
    BgUic,
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
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NoVat,
    NzGst,
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
    Unknown,
}

impl UpcomingInvoiceCustomerDetailsTaxIdsType {
    pub fn as_str(self) -> &'static str {
        use UpcomingInvoiceCustomerDetailsTaxIdsType::*;
        match self {
            AdNrt => "ad_nrt",
            AeTrn => "ae_trn",
            ArCuit => "ar_cuit",
            AuAbn => "au_abn",
            AuArn => "au_arn",
            BgUic => "bg_uic",
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
            LiUid => "li_uid",
            MxRfc => "mx_rfc",
            MyFrp => "my_frp",
            MyItn => "my_itn",
            MySst => "my_sst",
            NoVat => "no_vat",
            NzGst => "nz_gst",
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
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for UpcomingInvoiceCustomerDetailsTaxIdsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceCustomerDetailsTaxIdsType::*;
        match s {
            "ad_nrt" => Ok(AdNrt),
            "ae_trn" => Ok(AeTrn),
            "ar_cuit" => Ok(ArCuit),
            "au_abn" => Ok(AuAbn),
            "au_arn" => Ok(AuArn),
            "bg_uic" => Ok(BgUic),
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
            "li_uid" => Ok(LiUid),
            "mx_rfc" => Ok(MxRfc),
            "my_frp" => Ok(MyFrp),
            "my_itn" => Ok(MyItn),
            "my_sst" => Ok(MySst),
            "no_vat" => Ok(NoVat),
            "nz_gst" => Ok(NzGst),
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
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingInvoiceCustomerDetailsTaxIdsType {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// The coupons to redeem into discounts for the invoice preview.
///
/// If not specified, inherits the discount from the customer or subscription.
/// This only works for coupons directly applied to the invoice.
/// To apply a coupon to a subscription, you must use the `coupon` parameter instead.
/// Pass an empty string to avoid inheriting any discounts.
/// To preview the upcoming invoice for a subscription that hasn't been created, use `coupon` instead.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingInvoiceDiscounts<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
}
impl<'a> UpcomingInvoiceDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// List of invoice items to add or update in the upcoming invoice preview.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingInvoiceInvoiceItems<'a> {
    /// The integer amount in cents (or local equivalent) of previewed invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// Only applicable to new invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// An arbitrary string which you can attach to the invoice item.
    ///
    /// The description is displayed in the invoice for easy tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Explicitly controls whether discounts apply to this invoice item.
    ///
    /// Defaults to true, except for negative invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,
    /// The coupons to redeem into discounts for the invoice item in the preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [UpcomingInvoiceInvoiceItemsDiscounts<'a>]>,
    /// The ID of the invoice item to update in preview.
    ///
    /// If not specified, a new invoice item will be added to the preview of the upcoming invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoiceitem: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The period associated with this invoice item.
    ///
    /// When set to different values, the period will be rendered on the invoice.
    /// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
    /// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<UpcomingInvoiceInvoiceItemsPeriod>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpcomingInvoiceInvoiceItemsPriceData<'a>>,
    /// Non-negative integer.
    ///
    /// The quantity of units for the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingInvoiceInvoiceItemsTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// The tax rates that apply to the item.
    ///
    /// When set, any `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
    /// The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    ///
    /// This unit_amount will be multiplied by the quantity to get the full amount.
    /// If you want to apply a credit to the customer's account, pass a negative unit_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpcomingInvoiceInvoiceItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The coupons to redeem into discounts for the invoice item in the preview.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingInvoiceInvoiceItemsDiscounts<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
}
impl<'a> UpcomingInvoiceInvoiceItemsDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The period associated with this invoice item.
///
/// When set to different values, the period will be rendered on the invoice.
/// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
/// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceInvoiceItemsPeriod {
    /// The end of the period, which must be greater than or equal to the start.
    ///
    /// This value is inclusive.
    pub end: stripe_types::Timestamp,
    /// The start of the period.
    ///
    /// This value is inclusive.
    pub start: stripe_types::Timestamp,
}
impl UpcomingInvoiceInvoiceItemsPeriod {
    pub fn new(end: stripe_types::Timestamp, start: stripe_types::Timestamp) -> Self {
        Self { end, start }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceInvoiceItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingInvoiceInvoiceItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpcomingInvoiceInvoiceItemsPriceData<'a> {
    pub fn new(currency: stripe_types::Currency, product: &'a str) -> Self {
        Self {
            currency,
            product,
            tax_behavior: Default::default(),
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
///
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceInvoiceItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingInvoiceInvoiceItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
///
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceInvoiceItemsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingInvoiceInvoiceItemsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
///
/// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
/// For existing subscriptions, the value can only be set to `now` or `unchanged`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum UpcomingInvoiceSubscriptionBillingCycleAnchor {
    Now,
    Unchanged,
    StripeTypesTimestamp(stripe_types::Timestamp),
}
/// A list of up to 20 subscription items, each with an attached price.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingInvoiceSubscriptionItems<'a> {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<UpcomingInvoiceSubscriptionItemsBillingThresholds>,
    /// Delete all usage for a given subscription item.
    ///
    /// Allowed only when `deleted` is set to `true` and the current plan's `usage_type` is `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_usage: Option<bool>,
    /// A flag that, if set to `true`, will delete the specified item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Subscription item to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    /// The ID of the price object.
    ///
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpcomingInvoiceSubscriptionItemsPriceData<'a>>,
    /// Quantity for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> UpcomingInvoiceSubscriptionItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
///
/// When updating, pass an empty string to remove previously-defined thresholds.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceSubscriptionItemsBillingThresholds {
    /// Number of units that meets the billing threshold to advance the subscription to a new billing period (e.g., it takes 10 $5 units to meet a $50 [monetary threshold](https://stripe.com/docs/api/subscriptions/update#update_subscription-billing_thresholds-amount_gte)).
    pub usage_gte: i64,
}
impl UpcomingInvoiceSubscriptionItemsBillingThresholds {
    pub fn new(usage_gte: i64) -> Self {
        Self { usage_gte }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceSubscriptionItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: UpcomingInvoiceSubscriptionItemsPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingInvoiceSubscriptionItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpcomingInvoiceSubscriptionItemsPriceData<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        product: &'a str,
        recurring: UpcomingInvoiceSubscriptionItemsPriceDataRecurring,
    ) -> Self {
        Self {
            currency,
            product,
            recurring,
            tax_behavior: Default::default(),
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingInvoiceSubscriptionItemsPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpcomingInvoiceSubscriptionItemsPriceDataRecurring {
    pub fn new(interval: UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: Default::default() }
    }
}
/// Specifies billing frequency.
///
/// Either `day`, `week`, `month` or `year`.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingInvoiceSubscriptionItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
///
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceSubscriptionItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingInvoiceSubscriptionItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
///
/// The default value is `create_prorations`.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceSubscriptionProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingInvoiceSubscriptionProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// For paused subscriptions, setting `subscription_resume_at` to `now` will preview the invoice that will be generated if the subscription is resumed.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingInvoiceSubscriptionResumeAt::*;
        match s {
            "now" => Ok(Now),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingInvoiceSubscriptionResumeAt {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// If provided, the invoice returned will preview updating or creating a subscription with that trial end.
///
/// If set, one of `subscription_items` or `subscription` is required.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum UpcomingInvoiceSubscriptionTrialEnd {
    Now,
    StripeTypesTimestamp(stripe_types::Timestamp),
}
impl<'a> UpcomingInvoice<'a> {
    /// At any time, you can preview the upcoming invoice for a customer.
    ///
    /// This will show you all the charges that are pending, including subscription renewal charges, invoice item charges, etc.
    /// It will also show you any discounts that are applicable to the invoice.  Note that when you are viewing an upcoming invoice, you are simply viewing a preview – the invoice has not yet been created.
    /// As such, the upcoming invoice will not show up in invoice listing calls, and you cannot use the API to pay or edit the invoice.
    /// If you want to change the amount that your customer will be billed, you can add, remove, or update pending invoice items, or update the customer’s discount.  You can preview the effects of updating a subscription, including a preview of what proration will take place.
    /// To ensure that the actual proration is calculated exactly the same as the previewed proration, you should pass a `proration_date` parameter when doing the actual subscription update.
    /// The value passed in should be the same as the `subscription_proration_date` returned on the upcoming invoice resource.
    /// The recommended way to get only the prorations being previewed is to consider only proration line items where `period[start]` is equal to the `subscription_proration_date` on the upcoming invoice resource.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::Invoice> {
        client.get_query("/invoices/upcoming", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoice<'a> {
    /// The account tax IDs associated with the invoice.
    ///
    /// Only editable when the invoice is a draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<&'a [&'a str]>,
    /// A fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's Stripe account.
    ///
    /// The request must be made with an OAuth key or the Stripe-Account header in order to take an application fee.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/billing/invoices/connect#collecting-fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// Controls whether Stripe performs [automatic collection](https://stripe.com/docs/invoicing/integration/automatic-advancement-collection) of the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,
    /// Settings for automatic tax lookup for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpdateInvoiceAutomaticTax>,
    /// Either `charge_automatically` or `send_invoice`.
    ///
    /// This field can be updated only on `draft` invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<UpdateInvoiceCollectionMethod>,
    /// A list of up to 4 custom fields to be displayed on the invoice.
    ///
    /// If a value for `custom_fields` is specified, the list specified will replace the existing custom field list on this invoice.
    /// Pass an empty string to remove previously-defined fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<&'a [UpdateInvoiceCustomFields<'a>]>,
    /// The number of days from which the invoice is created until it is due.
    ///
    /// Only valid for invoices where `collection_method=send_invoice`.
    /// This field can only be updated on `draft` invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
    /// ID of the default payment method for the invoice.
    ///
    /// It must belong to the customer associated with the invoice.
    /// If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,
    /// ID of the default payment source for the invoice.
    ///
    /// It must belong to the customer associated with the invoice and be in a chargeable state.
    /// If not set, defaults to the subscription's default source, if any, or to the customer's default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<&'a str>,
    /// The tax rates that will apply to any line item that does not have `tax_rates` set.
    ///
    /// Pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<&'a [&'a str]>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    /// Referenced as 'memo' in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The discounts that will apply to the invoice.
    ///
    /// Pass an empty string to remove previously-defined discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [UpdateInvoiceDiscounts<'a>]>,
    /// The date on which payment for this invoice is due.
    ///
    /// Only valid for invoices where `collection_method=send_invoice`.
    /// This field can only be updated on `draft` invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<stripe_types::Timestamp>,
    /// The date when this invoice is in effect.
    ///
    /// Same as `finalized_at` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the invoice PDF and receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<stripe_types::Timestamp>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Footer to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The account (if any) for which the funds of the invoice payment are intended.
    ///
    /// If set, the invoice will be presented with the branding and support information of the specified account.
    /// See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// Configuration settings for the PaymentIntent that is generated when the invoice is finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<UpdateInvoicePaymentSettings<'a>>,
    /// The rendering-related settings that control how the invoice is displayed on customer-facing surfaces such as PDF and Hosted Invoice Page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering: Option<UpdateInvoiceRendering>,
    /// This is a legacy field that will be removed soon.
    ///
    /// For details about `rendering_options`, refer to `rendering` instead.
    /// Options for invoice PDF rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<UpdateInvoiceRenderingOptions>,
    /// Settings for the cost of shipping for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<UpdateInvoiceShippingCost<'a>>,
    /// Shipping details for the invoice.
    ///
    /// The Invoice PDF will use the `shipping_details` value if it is set, otherwise the PDF will render the shipping address from the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<UpdateInvoiceShippingDetails<'a>>,
    /// Extra information about a charge for the customer's credit card statement.
    ///
    /// It must contain at least one letter.
    /// If not specified and this invoice is part of a subscription, the default `statement_descriptor` will be set to the first subscription item's product's `statement_descriptor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// If specified, the funds from the invoice will be transferred to the destination and the ID of the resulting transfer will be found on the invoice's charge.
    ///
    /// This will be unset if you POST an empty value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<UpdateInvoiceTransferData<'a>>,
}
impl<'a> UpdateInvoice<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings for automatic tax lookup for this invoice.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceAutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    ///
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
}
impl UpdateInvoiceAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Either `charge_automatically` or `send_invoice`.
///
/// This field can be updated only on `draft` invoices.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoiceCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl UpdateInvoiceCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoiceCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for UpdateInvoiceCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoiceCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateInvoiceCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// A list of up to 4 custom fields to be displayed on the invoice.
///
/// If a value for `custom_fields` is specified, the list specified will replace the existing custom field list on this invoice.
/// Pass an empty string to remove previously-defined fields.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceCustomFields<'a> {
    /// The name of the custom field.
    ///
    /// This may be up to 30 characters.
    pub name: &'a str,
    /// The value of the custom field.
    ///
    /// This may be up to 30 characters.
    pub value: &'a str,
}
impl<'a> UpdateInvoiceCustomFields<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }
}
/// The discounts that will apply to the invoice.
///
/// Pass an empty string to remove previously-defined discounts.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoiceDiscounts<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
}
impl<'a> UpdateInvoiceDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration settings for the PaymentIntent that is generated when the invoice is finalized.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoicePaymentSettings<'a> {
    /// ID of the mandate to be used for this invoice.
    ///
    /// It must correspond to the payment method used to pay the invoice, including the invoice's default_payment_method or default_source, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mandate: Option<&'a str>,
    /// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<UpdateInvoicePaymentSettingsPaymentMethodOptions<'a>>,
    /// The list of payment method types (e.g.
    ///
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [UpdateInvoicePaymentSettingsPaymentMethodTypes]>,
}
impl<'a> UpdateInvoicePaymentSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptions<'a> {
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
    pub customer_balance:
        Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalance<'a>>,
    /// If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<&'a serde_json::Value>,
    /// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount<'a>>,
}
impl<'a> UpdateInvoicePaymentSettingsPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Verification method for the intent.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "fr" => Ok(Fr),
            "nl" => Ok(Nl),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsCard {
    /// Installment configuration for payments attempted on this invoice (Mexico Only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallments>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure>,
}
impl UpdateInvoicePaymentSettingsPaymentMethodOptionsCard {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Installment configuration for payments attempted on this invoice (Mexico Only).
///
/// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
        count: u64,
        interval: UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval,
        type_: UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType,
    ) -> Self {
        Self { count, interval, type_ }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval::*;
        match s {
            "month" => Ok(Month),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::*;
        match s {
            "fixed_count" => Ok(FixedCount),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
///
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::*;
        match self {
            Any => "any",
            Automatic => "automatic",
        }
    }
}

impl std::str::FromStr for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::*;
        match s {
            "any" => Ok(Any),
            "automatic" => Ok(Automatic),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalance<'a> {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a>>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<&'a str>,
}
impl<'a> UpdateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    /// Configuration for eu_bank_transfer funding type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<
        UpdateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<
            'a,
        >,
    >,
    /// The bank transfer type that can be used for funding.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<&'a str>,
}
impl<'a> UpdateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for eu_bank_transfer funding type.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<
    'a,
> {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a>
    UpdateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a>
{
    pub fn new(country: &'a str) -> Self {
        Self { country }
    }
}
/// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<
        UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections<'a>,
    >,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
impl<'a> UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Financial Connections Session creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[serde(skip_serializing_if = "Option::is_none")]
pub permissions: Option<&'a [UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions]>,
    /// List of data features that you would like to retrieve upon account creation.
#[serde(skip_serializing_if = "Option::is_none")]
pub prefetch: Option<&'a [UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch]>,

}
impl<'a> UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The list of permissions to request.
///
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// List of data features that you would like to retrieve upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    Balances,
}

impl UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match self {
            Balances => "balances",
        }
    }
}

impl std::str::FromStr
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Verification method for the intent.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for UpdateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// The list of payment method types (e.g.
///
/// card) to provide to the invoice’s PaymentIntent.
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateInvoicePaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Cashapp,
    CustomerBalance,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    Paynow,
    Paypal,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}

impl UpdateInvoicePaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoicePaymentSettingsPaymentMethodTypes::*;
        match self {
            AchCreditTransfer => "ach_credit_transfer",
            AchDebit => "ach_debit",
            AcssDebit => "acss_debit",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Boleto => "boleto",
            Card => "card",
            Cashapp => "cashapp",
            CustomerBalance => "customer_balance",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            Konbini => "konbini",
            Link => "link",
            Paynow => "paynow",
            Paypal => "paypal",
            Promptpay => "promptpay",
            SepaCreditTransfer => "sepa_credit_transfer",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for UpdateInvoicePaymentSettingsPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoicePaymentSettingsPaymentMethodTypes::*;
        match s {
            "ach_credit_transfer" => Ok(AchCreditTransfer),
            "ach_debit" => Ok(AchDebit),
            "acss_debit" => Ok(AcssDebit),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "cashapp" => Ok(Cashapp),
            "customer_balance" => Ok(CustomerBalance),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "konbini" => Ok(Konbini),
            "link" => Ok(Link),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "promptpay" => Ok(Promptpay),
            "sepa_credit_transfer" => Ok(SepaCreditTransfer),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoicePaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// The rendering-related settings that control how the invoice is displayed on customer-facing surfaces such as PDF and Hosted Invoice Page.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoiceRendering {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<UpdateInvoiceRenderingAmountTaxDisplay>,
    /// Invoice pdf rendering options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf: Option<UpdateInvoiceRenderingPdf>,
}
impl UpdateInvoiceRendering {
    pub fn new() -> Self {
        Self::default()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
///
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceRenderingAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoiceRenderingAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Invoice pdf rendering options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoiceRenderingPdf {
    /// Page size for invoice PDF.
    ///
    /// Can be set to `a4`, `letter`, or `auto`.  If set to `auto`, invoice PDF page size defaults to `a4` for customers with  Japanese locale and `letter` for customers with other locales.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<UpdateInvoiceRenderingPdfPageSize>,
}
impl UpdateInvoiceRenderingPdf {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Page size for invoice PDF.
///
/// Can be set to `a4`, `letter`, or `auto`.  If set to `auto`, invoice PDF page size defaults to `a4` for customers with  Japanese locale and `letter` for customers with other locales.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceRenderingPdfPageSize::*;
        match s {
            "a4" => Ok(A4),
            "auto" => Ok(Auto),
            "letter" => Ok(Letter),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoiceRenderingPdfPageSize {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// This is a legacy field that will be removed soon.
///
/// For details about `rendering_options`, refer to `rendering` instead.
/// Options for invoice PDF rendering.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoiceRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<UpdateInvoiceRenderingOptionsAmountTaxDisplay>,
}
impl UpdateInvoiceRenderingOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
///
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoiceRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl UpdateInvoiceRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoiceRenderingOptionsAmountTaxDisplay::*;
        match self {
            ExcludeTax => "exclude_tax",
            IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl std::str::FromStr for UpdateInvoiceRenderingOptionsAmountTaxDisplay {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceRenderingOptionsAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoiceRenderingOptionsAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateInvoiceRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceRenderingOptionsAmountTaxDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Settings for the cost of shipping for this invoice.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoiceShippingCost<'a> {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<&'a str>,
    /// Parameters to create a new ad-hoc shipping rate for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<UpdateInvoiceShippingCostShippingRateData<'a>>,
}
impl<'a> UpdateInvoiceShippingCost<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Parameters to create a new ad-hoc shipping rate for this order.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceShippingCostShippingRateData<'a> {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<UpdateInvoiceShippingCostShippingRateDataDeliveryEstimate>,
    /// The name of the shipping rate, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    pub display_name: &'a str,
    /// Describes a fixed amount to charge for shipping.
    ///
    /// Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<UpdateInvoiceShippingCostShippingRateDataFixedAmount<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateInvoiceShippingCostShippingRateDataTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    ///
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// The type of calculation to use on the shipping rate.
    ///
    /// Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpdateInvoiceShippingCostShippingRateDataType>,
}
impl<'a> UpdateInvoiceShippingCostShippingRateData<'a> {
    pub fn new(display_name: &'a str) -> Self {
        Self {
            delivery_estimate: Default::default(),
            display_name,
            fixed_amount: Default::default(),
            metadata: Default::default(),
            tax_behavior: Default::default(),
            tax_code: Default::default(),
            type_: Default::default(),
        }
    }
}
/// The estimated range for how long shipping will take, meant to be displayable to the customer.
///
/// This will appear on CheckoutSessions.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoiceShippingCostShippingRateDataDeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximum>,
    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimum>,
}
impl UpdateInvoiceShippingCostShippingRateDataDeliveryEstimate {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The upper bound of the estimated range.
///
/// If empty, represents no upper bound i.e., infinite.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximum {
    pub fn new(
        unit: UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit,
        value: i64,
    ) -> Self {
        Self { unit, value }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// The lower bound of the estimated range.
///
/// If empty, represents no lower bound.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimum {
    pub fn new(
        unit: UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit,
        value: i64,
    ) -> Self {
        Self { unit, value }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Describes a fixed amount to charge for shipping.
///
/// Must be present if type is `fixed_amount`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceShippingCostShippingRateDataFixedAmount<'a> {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Shipping rates defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        &'a std::collections::HashMap<
            stripe_types::Currency,
            UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptions,
        >,
    >,
}
impl<'a> UpdateInvoiceShippingCostShippingRateDataFixedAmount<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self { amount, currency, currency_options: Default::default() }
    }
}
/// Shipping rates defined in each available currency option.
///
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior>,
}
impl UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptions {
    pub fn new(amount: i64) -> Self {
        Self { amount, tax_behavior: Default::default() }
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceShippingCostShippingRateDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoiceShippingCostShippingRateDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// The type of calculation to use on the shipping rate.
///
/// Can only be `fixed_amount` for now.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceShippingCostShippingRateDataType::*;
        match s {
            "fixed_amount" => Ok(FixedAmount),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoiceShippingCostShippingRateDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Shipping details for the invoice.
///
/// The Invoice PDF will use the `shipping_details` value if it is set, otherwise the PDF will render the shipping address from the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceShippingDetails<'a> {
    /// Shipping address.
    pub address: UpdateInvoiceShippingDetailsAddress<'a>,
    /// Recipient name.
    pub name: &'a str,
    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> UpdateInvoiceShippingDetails<'a> {
    pub fn new(address: UpdateInvoiceShippingDetailsAddress<'a>, name: &'a str) -> Self {
        Self { address, name, phone: Default::default() }
    }
}
/// Shipping address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoiceShippingDetailsAddress<'a> {
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
impl<'a> UpdateInvoiceShippingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If specified, the funds from the invoice will be transferred to the destination and the ID of the resulting transfer will be found on the invoice's charge.
///
/// This will be unset if you POST an empty value.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceTransferData<'a> {
    /// The amount that will be transferred automatically when the invoice is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ID of an existing, connected Stripe account.
    pub destination: &'a str,
}
impl<'a> UpdateInvoiceTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount: Default::default(), destination }
    }
}
impl<'a> UpdateInvoice<'a> {
    /// Draft invoices are fully editable.
    ///
    /// Once an invoice is [finalized](https://stripe.com/docs/billing/invoices/workflow#finalized), monetary values, as well as `collection_method`, become uneditable.  If you would like to stop the Stripe Billing engine from automatically finalizing, reattempting payments on, sending reminders for, or [automatically reconciling](https://stripe.com/docs/billing/invoices/reconciliation) invoices, pass `auto_advance=false`.
    pub fn send(
        &self,
        client: &stripe::Client,
        invoice: &stripe_types::invoice::InvoiceId,
    ) -> stripe::Response<stripe_types::Invoice> {
        client.send_form(
            &format!("/invoices/{invoice}", invoice = invoice),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PayInvoice<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// In cases where the source used to pay the invoice has insufficient funds, passing `forgive=true` controls whether a charge should be attempted for the full amount available on the source, up to the amount to fully pay the invoice.
    ///
    /// This effectively forgives the difference between the amount available on the source and the amount due.
    /// Passing `forgive=false` will fail the charge if the source hasn't been pre-funded with the right amount.
    /// An example for this case is with ACH Credit Transfers and wires: if the amount wired is less than the amount due by a small amount, you might want to forgive the difference.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forgive: Option<bool>,
    /// ID of the mandate to be used for this invoice.
    ///
    /// It must correspond to the payment method used to pay the invoice, including the payment_method param or the invoice's default_payment_method or default_source, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<&'a str>,
    /// Indicates if a customer is on or off-session while an invoice payment is attempted.
    ///
    /// Defaults to `true` (off-session).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<bool>,
    /// Boolean representing whether an invoice is paid outside of Stripe.
    ///
    /// This will result in no charge being made.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_out_of_band: Option<bool>,
    /// A PaymentMethod to be charged.
    ///
    /// The PaymentMethod must be the ID of a PaymentMethod belonging to the customer associated with the invoice being paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<&'a str>,
    /// A payment source to be charged.
    ///
    /// The source must be the ID of a source belonging to the customer associated with the invoice being paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
}
impl<'a> PayInvoice<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> PayInvoice<'a> {
    /// Stripe automatically creates and then attempts to collect payment on invoices for customers on subscriptions according to your [subscriptions settings](https://dashboard.stripe.com/account/billing/automatic).
    ///
    /// However, if you’d like to attempt payment on an invoice out of the normal collection schedule or for some other reason, you can do so.
    pub fn send(
        &self,
        client: &stripe::Client,
        invoice: &stripe_types::invoice::InvoiceId,
    ) -> stripe::Response<stripe_types::Invoice> {
        client.send_form(
            &format!("/invoices/{invoice}/pay", invoice = invoice),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingLinesInvoice<'a> {
    /// Settings for automatic tax lookup for this invoice preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpcomingLinesInvoiceAutomaticTax>,
    /// The code of the coupon to apply.
    ///
    /// If `subscription` or `subscription_items` is provided, the invoice returned will preview updating or creating a subscription with that coupon.
    /// Otherwise, it will preview applying that coupon to the customer for the next upcoming invoice from among the customer's subscriptions.
    /// The invoice can be previewed without a coupon by passing this value as an empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// The currency to preview this invoice in.
    ///
    /// Defaults to that of `customer` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// The identifier of the customer whose upcoming invoice you'd like to retrieve.
    ///
    /// If `automatic_tax` is enabled then one of `customer`, `customer_details`, `subscription`, or `schedule` must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// Details about the customer you want to invoice or overrides for an existing customer.
    ///
    /// If `automatic_tax` is enabled then one of `customer`, `customer_details`, `subscription`, or `schedule` must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_details: Option<UpcomingLinesInvoiceCustomerDetails<'a>>,
    /// The coupons to redeem into discounts for the invoice preview.
    ///
    /// If not specified, inherits the discount from the customer or subscription.
    /// This only works for coupons directly applied to the invoice.
    /// To apply a coupon to a subscription, you must use the `coupon` parameter instead.
    /// Pass an empty string to avoid inheriting any discounts.
    /// To preview the upcoming invoice for a subscription that hasn't been created, use `coupon` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [UpcomingLinesInvoiceDiscounts<'a>]>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// List of invoice items to add or update in the upcoming invoice preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_items: Option<&'a [UpcomingLinesInvoiceInvoiceItems<'a>]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The identifier of the schedule whose upcoming invoice you'd like to retrieve.
    ///
    /// Cannot be used with subscription or subscription fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// The identifier of the subscription for which you'd like to retrieve the upcoming invoice.
    ///
    /// If not provided, but a `subscription_items` is provided, you will preview creating a subscription with those items.
    /// If neither `subscription` nor `subscription_items` is provided, you will retrieve the next upcoming invoice from among the customer's subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
    /// For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
    ///
    /// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    /// For existing subscriptions, the value can only be set to `now` or `unchanged`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_billing_cycle_anchor:
        Option<UpcomingLinesInvoiceSubscriptionBillingCycleAnchor>,
    /// Timestamp indicating when the subscription should be scheduled to cancel.
    ///
    /// Will prorate if within the current period and prorations have been enabled using `proration_behavior`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel_at: Option<stripe_types::Timestamp>,
    /// Boolean indicating whether this subscription should cancel at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel_at_period_end: Option<bool>,
    /// This simulates the subscription being canceled or expired immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel_now: Option<bool>,
    /// If provided, the invoice returned will preview updating or creating a subscription with these default tax rates.
    ///
    /// The default tax rates will apply to any line item that does not have `tax_rates` set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_default_tax_rates: Option<&'a [&'a str]>,
    /// A list of up to 20 subscription items, each with an attached price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_items: Option<&'a [UpcomingLinesInvoiceSubscriptionItems<'a>]>,
    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    ///
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_behavior: Option<UpcomingLinesInvoiceSubscriptionProrationBehavior>,
    /// If previewing an update to a subscription, and doing proration, `subscription_proration_date` forces the proration to be calculated as though the update was done at the specified time.
    ///
    /// The time given must be within the current subscription period and within the current phase of the schedule backing this subscription, if the schedule exists.
    /// If set, `subscription`, and one of `subscription_items`, or `subscription_trial_end` are required.
    /// Also, `subscription_proration_behavior` cannot be set to 'none'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_date: Option<stripe_types::Timestamp>,
    /// For paused subscriptions, setting `subscription_resume_at` to `now` will preview the invoice that will be generated if the subscription is resumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_resume_at: Option<UpcomingLinesInvoiceSubscriptionResumeAt>,
    /// Date a subscription is intended to start (can be future or past).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_start_date: Option<stripe_types::Timestamp>,
    /// If provided, the invoice returned will preview updating or creating a subscription with that trial end.
    ///
    /// If set, one of `subscription_items` or `subscription` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_trial_end: Option<UpcomingLinesInvoiceSubscriptionTrialEnd>,
    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    ///
    /// Setting `subscription_trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `subscription_trial_end` is not allowed.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_trial_from_plan: Option<bool>,
}
impl<'a> UpcomingLinesInvoice<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings for automatic tax lookup for this invoice preview.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceAutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    ///
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
}
impl UpcomingLinesInvoiceAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Details about the customer you want to invoice or overrides for an existing customer.
///
/// If `automatic_tax` is enabled then one of `customer`, `customer_details`, `subscription`, or `schedule` must be set.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingLinesInvoiceCustomerDetails<'a> {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpcomingLinesInvoiceCustomerDetailsAddress<'a>>,
    /// The customer's shipping information.
    ///
    /// Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<UpcomingLinesInvoiceCustomerDetailsShipping<'a>>,
    /// Tax details about the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<UpcomingLinesInvoiceCustomerDetailsTax<'a>>,
    /// The customer's tax exemption.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<UpcomingLinesInvoiceCustomerDetailsTaxExempt>,
    /// The customer's tax IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<&'a [UpcomingLinesInvoiceCustomerDetailsTaxIds<'a>]>,
}
impl<'a> UpcomingLinesInvoiceCustomerDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingLinesInvoiceCustomerDetailsAddress<'a> {
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
impl<'a> UpcomingLinesInvoiceCustomerDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's shipping information.
///
/// Appears on invoices emailed to this customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceCustomerDetailsShipping<'a> {
    /// Customer shipping address.
    pub address: UpcomingLinesInvoiceCustomerDetailsShippingAddress<'a>,
    /// Customer name.
    pub name: &'a str,
    /// Customer phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> UpcomingLinesInvoiceCustomerDetailsShipping<'a> {
    pub fn new(
        address: UpcomingLinesInvoiceCustomerDetailsShippingAddress<'a>,
        name: &'a str,
    ) -> Self {
        Self { address, name, phone: Default::default() }
    }
}
/// Customer shipping address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingLinesInvoiceCustomerDetailsShippingAddress<'a> {
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
impl<'a> UpcomingLinesInvoiceCustomerDetailsShippingAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tax details about the customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingLinesInvoiceCustomerDetailsTax<'a> {
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    ///
    /// Stripe recommends updating the IP address when a new PaymentMethod is attached or the address field on the customer is updated.
    /// We recommend against updating this field more frequently since it could result in unexpected tax location/reporting outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,
}
impl<'a> UpcomingLinesInvoiceCustomerDetailsTax<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's tax exemption.
///
/// One of `none`, `exempt`, or `reverse`.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceCustomerDetailsTaxExempt::*;
        match s {
            "exempt" => Ok(Exempt),
            "none" => Ok(None),
            "reverse" => Ok(Reverse),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingLinesInvoiceCustomerDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// The customer's tax IDs.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceCustomerDetailsTaxIds<'a> {
    /// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: UpcomingLinesInvoiceCustomerDetailsTaxIdsType,
    /// Value of the tax ID.
    pub value: &'a str,
}
impl<'a> UpcomingLinesInvoiceCustomerDetailsTaxIds<'a> {
    pub fn new(type_: UpcomingLinesInvoiceCustomerDetailsTaxIdsType, value: &'a str) -> Self {
        Self { type_, value }
    }
}
/// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpcomingLinesInvoiceCustomerDetailsTaxIdsType {
    AdNrt,
    AeTrn,
    ArCuit,
    AuAbn,
    AuArn,
    BgUic,
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
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NoVat,
    NzGst,
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
    Unknown,
}

impl UpcomingLinesInvoiceCustomerDetailsTaxIdsType {
    pub fn as_str(self) -> &'static str {
        use UpcomingLinesInvoiceCustomerDetailsTaxIdsType::*;
        match self {
            AdNrt => "ad_nrt",
            AeTrn => "ae_trn",
            ArCuit => "ar_cuit",
            AuAbn => "au_abn",
            AuArn => "au_arn",
            BgUic => "bg_uic",
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
            LiUid => "li_uid",
            MxRfc => "mx_rfc",
            MyFrp => "my_frp",
            MyItn => "my_itn",
            MySst => "my_sst",
            NoVat => "no_vat",
            NzGst => "nz_gst",
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
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for UpcomingLinesInvoiceCustomerDetailsTaxIdsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceCustomerDetailsTaxIdsType::*;
        match s {
            "ad_nrt" => Ok(AdNrt),
            "ae_trn" => Ok(AeTrn),
            "ar_cuit" => Ok(ArCuit),
            "au_abn" => Ok(AuAbn),
            "au_arn" => Ok(AuArn),
            "bg_uic" => Ok(BgUic),
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
            "li_uid" => Ok(LiUid),
            "mx_rfc" => Ok(MxRfc),
            "my_frp" => Ok(MyFrp),
            "my_itn" => Ok(MyItn),
            "my_sst" => Ok(MySst),
            "no_vat" => Ok(NoVat),
            "nz_gst" => Ok(NzGst),
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
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingLinesInvoiceCustomerDetailsTaxIdsType {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// The coupons to redeem into discounts for the invoice preview.
///
/// If not specified, inherits the discount from the customer or subscription.
/// This only works for coupons directly applied to the invoice.
/// To apply a coupon to a subscription, you must use the `coupon` parameter instead.
/// Pass an empty string to avoid inheriting any discounts.
/// To preview the upcoming invoice for a subscription that hasn't been created, use `coupon` instead.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingLinesInvoiceDiscounts<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
}
impl<'a> UpcomingLinesInvoiceDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// List of invoice items to add or update in the upcoming invoice preview.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingLinesInvoiceInvoiceItems<'a> {
    /// The integer amount in cents (or local equivalent) of previewed invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// Only applicable to new invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// An arbitrary string which you can attach to the invoice item.
    ///
    /// The description is displayed in the invoice for easy tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Explicitly controls whether discounts apply to this invoice item.
    ///
    /// Defaults to true, except for negative invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,
    /// The coupons to redeem into discounts for the invoice item in the preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [UpcomingLinesInvoiceInvoiceItemsDiscounts<'a>]>,
    /// The ID of the invoice item to update in preview.
    ///
    /// If not specified, a new invoice item will be added to the preview of the upcoming invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoiceitem: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The period associated with this invoice item.
    ///
    /// When set to different values, the period will be rendered on the invoice.
    /// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
    /// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<UpcomingLinesInvoiceInvoiceItemsPeriod>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpcomingLinesInvoiceInvoiceItemsPriceData<'a>>,
    /// Non-negative integer.
    ///
    /// The quantity of units for the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingLinesInvoiceInvoiceItemsTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// The tax rates that apply to the item.
    ///
    /// When set, any `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
    /// The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    ///
    /// This unit_amount will be multiplied by the quantity to get the full amount.
    /// If you want to apply a credit to the customer's account, pass a negative unit_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpcomingLinesInvoiceInvoiceItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The coupons to redeem into discounts for the invoice item in the preview.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingLinesInvoiceInvoiceItemsDiscounts<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
}
impl<'a> UpcomingLinesInvoiceInvoiceItemsDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The period associated with this invoice item.
///
/// When set to different values, the period will be rendered on the invoice.
/// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
/// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceInvoiceItemsPeriod {
    /// The end of the period, which must be greater than or equal to the start.
    ///
    /// This value is inclusive.
    pub end: stripe_types::Timestamp,
    /// The start of the period.
    ///
    /// This value is inclusive.
    pub start: stripe_types::Timestamp,
}
impl UpcomingLinesInvoiceInvoiceItemsPeriod {
    pub fn new(end: stripe_types::Timestamp, start: stripe_types::Timestamp) -> Self {
        Self { end, start }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceInvoiceItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingLinesInvoiceInvoiceItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpcomingLinesInvoiceInvoiceItemsPriceData<'a> {
    pub fn new(currency: stripe_types::Currency, product: &'a str) -> Self {
        Self {
            currency,
            product,
            tax_behavior: Default::default(),
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
///
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceInvoiceItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingLinesInvoiceInvoiceItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
///
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceInvoiceItemsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingLinesInvoiceInvoiceItemsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
///
/// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
/// For existing subscriptions, the value can only be set to `now` or `unchanged`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum UpcomingLinesInvoiceSubscriptionBillingCycleAnchor {
    Now,
    Unchanged,
    StripeTypesTimestamp(stripe_types::Timestamp),
}
/// A list of up to 20 subscription items, each with an attached price.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpcomingLinesInvoiceSubscriptionItems<'a> {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<UpcomingLinesInvoiceSubscriptionItemsBillingThresholds>,
    /// Delete all usage for a given subscription item.
    ///
    /// Allowed only when `deleted` is set to `true` and the current plan's `usage_type` is `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_usage: Option<bool>,
    /// A flag that, if set to `true`, will delete the specified item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Subscription item to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    /// The ID of the price object.
    ///
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpcomingLinesInvoiceSubscriptionItemsPriceData<'a>>,
    /// Quantity for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> UpcomingLinesInvoiceSubscriptionItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
///
/// When updating, pass an empty string to remove previously-defined thresholds.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceSubscriptionItemsBillingThresholds {
    /// Number of units that meets the billing threshold to advance the subscription to a new billing period (e.g., it takes 10 $5 units to meet a $50 [monetary threshold](https://stripe.com/docs/api/subscriptions/update#update_subscription-billing_thresholds-amount_gte)).
    pub usage_gte: i64,
}
impl UpcomingLinesInvoiceSubscriptionItemsBillingThresholds {
    pub fn new(usage_gte: i64) -> Self {
        Self { usage_gte }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceSubscriptionItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpcomingLinesInvoiceSubscriptionItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpcomingLinesInvoiceSubscriptionItemsPriceData<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        product: &'a str,
        recurring: UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurring,
    ) -> Self {
        Self {
            currency,
            product,
            recurring,
            tax_behavior: Default::default(),
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurring {
    pub fn new(interval: UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: Default::default() }
    }
}
/// Specifies billing frequency.
///
/// Either `day`, `week`, `month` or `year`.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingLinesInvoiceSubscriptionItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
///
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceSubscriptionItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingLinesInvoiceSubscriptionItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
///
/// The default value is `create_prorations`.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceSubscriptionProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingLinesInvoiceSubscriptionProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// For paused subscriptions, setting `subscription_resume_at` to `now` will preview the invoice that will be generated if the subscription is resumed.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpcomingLinesInvoiceSubscriptionResumeAt::*;
        match s {
            "now" => Ok(Now),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpcomingLinesInvoiceSubscriptionResumeAt {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// If provided, the invoice returned will preview updating or creating a subscription with that trial end.
///
/// If set, one of `subscription_items` or `subscription` is required.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum UpcomingLinesInvoiceSubscriptionTrialEnd {
    Now,
    StripeTypesTimestamp(stripe_types::Timestamp),
}
impl<'a> UpcomingLinesInvoice<'a> {
    /// When retrieving an upcoming invoice, you’ll get a **lines** property containing the total count of line items and the first handful of those items.
    ///
    /// There is also a URL where you can retrieve the full (paginated) list of line items.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::InvoiceLineItem>> {
        client.get_query("/invoices/upcoming/lines", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::InvoiceLineItem> {
        stripe::ListPaginator::from_params("/invoices/upcoming/lines", self)
    }
}
impl<'a> stripe::PaginationParams for UpcomingLinesInvoice<'a> {}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoice<'a> {
    /// The account tax IDs associated with the invoice.
    ///
    /// Only editable when the invoice is a draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<&'a [&'a str]>,
    /// A fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's Stripe account.
    ///
    /// The request must be made with an OAuth key or the Stripe-Account header in order to take an application fee.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/billing/invoices/connect#collecting-fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// Controls whether Stripe performs [automatic collection](https://stripe.com/docs/invoicing/integration/automatic-advancement-collection) of the invoice.
    ///
    /// If `false`, the invoice's state doesn't automatically advance without an explicit action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,
    /// Settings for automatic tax lookup for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateInvoiceAutomaticTax>,
    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer.
    /// When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CreateInvoiceCollectionMethod>,
    /// The currency to create this invoice in.
    ///
    /// Defaults to that of `customer` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// A list of up to 4 custom fields to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<&'a [CreateInvoiceCustomFields<'a>]>,
    /// The ID of the customer who will be billed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// The number of days from when the invoice is created until it is due.
    ///
    /// Valid only for invoices where `collection_method=send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
    /// ID of the default payment method for the invoice.
    ///
    /// It must belong to the customer associated with the invoice.
    /// If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,
    /// ID of the default payment source for the invoice.
    ///
    /// It must belong to the customer associated with the invoice and be in a chargeable state.
    /// If not set, defaults to the subscription's default source, if any, or to the customer's default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<&'a str>,
    /// The tax rates that will apply to any line item that does not have `tax_rates` set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<&'a [&'a str]>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    /// Referenced as 'memo' in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The coupons to redeem into discounts for the invoice.
    ///
    /// If not specified, inherits the discount from the invoice's customer.
    /// Pass an empty string to avoid inheriting any discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [CreateInvoiceDiscounts<'a>]>,
    /// The date on which payment for this invoice is due.
    ///
    /// Valid only for invoices where `collection_method=send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<stripe_types::Timestamp>,
    /// The date when this invoice is in effect.
    ///
    /// Same as `finalized_at` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the invoice PDF and receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<stripe_types::Timestamp>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Footer to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,
    /// Revise an existing invoice.
    ///
    /// The new invoice will be created in `status=draft`.
    /// See the [revision documentation](https://stripe.com/docs/invoicing/invoice-revisions) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_invoice: Option<CreateInvoiceFromInvoice<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The account (if any) for which the funds of the invoice payment are intended.
    ///
    /// If set, the invoice will be presented with the branding and support information of the specified account.
    /// See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// Configuration settings for the PaymentIntent that is generated when the invoice is finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<CreateInvoicePaymentSettings<'a>>,
    /// How to handle pending invoice items on invoice creation.
    ///
    /// One of `include` or `exclude`.
    /// `include` will include any pending invoice items, and will create an empty draft invoice if no pending invoice items exist.
    /// `exclude` will always create an empty invoice draft regardless if there are pending invoice items or not.
    /// Defaults to `exclude` if the parameter is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_invoice_items_behavior: Option<CreateInvoicePendingInvoiceItemsBehavior>,
    /// The rendering-related settings that control how the invoice is displayed on customer-facing surfaces such as PDF and Hosted Invoice Page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering: Option<CreateInvoiceRendering>,
    /// This is a legacy field that will be removed soon.
    ///
    /// For details about `rendering_options`, refer to `rendering` instead.
    /// Options for invoice PDF rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CreateInvoiceRenderingOptions>,
    /// Settings for the cost of shipping for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<CreateInvoiceShippingCost<'a>>,
    /// Shipping details for the invoice.
    ///
    /// The Invoice PDF will use the `shipping_details` value if it is set, otherwise the PDF will render the shipping address from the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<CreateInvoiceShippingDetails<'a>>,
    /// Extra information about a charge for the customer's credit card statement.
    ///
    /// It must contain at least one letter.
    /// If not specified and this invoice is part of a subscription, the default `statement_descriptor` will be set to the first subscription item's product's `statement_descriptor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// The ID of the subscription to invoice, if any.
    ///
    /// If set, the created invoice will only include pending invoice items for that subscription.
    /// The subscription's billing cycle and regular subscription events won't be affected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
    /// If specified, the funds from the invoice will be transferred to the destination and the ID of the resulting transfer will be found on the invoice's charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateInvoiceTransferData<'a>>,
}
impl<'a> CreateInvoice<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings for automatic tax lookup for this invoice.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceAutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    ///
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
}
impl CreateInvoiceAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Either `charge_automatically`, or `send_invoice`.
///
/// When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer.
/// When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
/// Defaults to `charge_automatically`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoiceCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl CreateInvoiceCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use CreateInvoiceCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for CreateInvoiceCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoiceCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoiceCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoiceCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoiceCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// A list of up to 4 custom fields to be displayed on the invoice.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceCustomFields<'a> {
    /// The name of the custom field.
    ///
    /// This may be up to 30 characters.
    pub name: &'a str,
    /// The value of the custom field.
    ///
    /// This may be up to 30 characters.
    pub value: &'a str,
}
impl<'a> CreateInvoiceCustomFields<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }
}
/// The coupons to redeem into discounts for the invoice.
///
/// If not specified, inherits the discount from the invoice's customer.
/// Pass an empty string to avoid inheriting any discounts.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoiceDiscounts<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
}
impl<'a> CreateInvoiceDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Revise an existing invoice.
///
/// The new invoice will be created in `status=draft`.
/// See the [revision documentation](https://stripe.com/docs/invoicing/invoice-revisions) for more details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceFromInvoice<'a> {
    /// The relation between the new invoice and the original invoice.
    ///
    /// Currently, only 'revision' is permitted.
    pub action: CreateInvoiceFromInvoiceAction,
    /// The `id` of the invoice that will be cloned.
    pub invoice: &'a str,
}
impl<'a> CreateInvoiceFromInvoice<'a> {
    pub fn new(action: CreateInvoiceFromInvoiceAction, invoice: &'a str) -> Self {
        Self { action, invoice }
    }
}
/// The relation between the new invoice and the original invoice.
///
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceFromInvoiceAction::*;
        match s {
            "revision" => Ok(Revision),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoiceFromInvoiceAction {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Configuration settings for the PaymentIntent that is generated when the invoice is finalized.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoicePaymentSettings<'a> {
    /// ID of the mandate to be used for this invoice.
    ///
    /// It must correspond to the payment method used to pay the invoice, including the invoice's default_payment_method or default_source, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mandate: Option<&'a str>,
    /// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateInvoicePaymentSettingsPaymentMethodOptions<'a>>,
    /// The list of payment method types (e.g.
    ///
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [CreateInvoicePaymentSettingsPaymentMethodTypes]>,
}
impl<'a> CreateInvoicePaymentSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptions<'a> {
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
    pub customer_balance:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalance<'a>>,
    /// If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<&'a serde_json::Value>,
    /// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount<'a>>,
}
impl<'a> CreateInvoicePaymentSettingsPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Verification method for the intent.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "fr" => Ok(Fr),
            "nl" => Ok(Nl),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsCard {
    /// Installment configuration for payments attempted on this invoice (Mexico Only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallments>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure>,
}
impl CreateInvoicePaymentSettingsPaymentMethodOptionsCard {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Installment configuration for payments attempted on this invoice (Mexico Only).
///
/// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
        count: u64,
        interval: CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval,
        type_: CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType,
    ) -> Self {
        Self { count, interval, type_ }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval::*;
        match s {
            "month" => Ok(Month),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::*;
        match s {
            "fixed_count" => Ok(FixedCount),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
///
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::*;
        match self {
            Any => "any",
            Automatic => "automatic",
        }
    }
}

impl std::str::FromStr for CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::*;
        match s {
            "any" => Ok(Any),
            "automatic" => Ok(Automatic),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalance<'a> {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a>>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<&'a str>,
}
impl<'a> CreateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    /// Configuration for eu_bank_transfer funding type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<
        CreateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<
            'a,
        >,
    >,
    /// The bank transfer type that can be used for funding.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<&'a str>,
}
impl<'a> CreateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for eu_bank_transfer funding type.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<
    'a,
> {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a>
    CreateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a>
{
    pub fn new(country: &'a str) -> Self {
        Self { country }
    }
}
/// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<
        CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections<'a>,
    >,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
impl<'a> CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Financial Connections Session creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[serde(skip_serializing_if = "Option::is_none")]
pub permissions: Option<&'a [CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions]>,
    /// List of data features that you would like to retrieve upon account creation.
#[serde(skip_serializing_if = "Option::is_none")]
pub prefetch: Option<&'a [CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch]>,

}
impl<'a> CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The list of permissions to request.
///
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// List of data features that you would like to retrieve upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    Balances,
}

impl CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    pub fn as_str(self) -> &'static str {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match self {
            Balances => "balances",
        }
    }
}

impl std::str::FromStr
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Verification method for the intent.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// The list of payment method types (e.g.
///
/// card) to provide to the invoice’s PaymentIntent.
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateInvoicePaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Cashapp,
    CustomerBalance,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    Paynow,
    Paypal,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}

impl CreateInvoicePaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        use CreateInvoicePaymentSettingsPaymentMethodTypes::*;
        match self {
            AchCreditTransfer => "ach_credit_transfer",
            AchDebit => "ach_debit",
            AcssDebit => "acss_debit",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Boleto => "boleto",
            Card => "card",
            Cashapp => "cashapp",
            CustomerBalance => "customer_balance",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            Konbini => "konbini",
            Link => "link",
            Paynow => "paynow",
            Paypal => "paypal",
            Promptpay => "promptpay",
            SepaCreditTransfer => "sepa_credit_transfer",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateInvoicePaymentSettingsPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePaymentSettingsPaymentMethodTypes::*;
        match s {
            "ach_credit_transfer" => Ok(AchCreditTransfer),
            "ach_debit" => Ok(AchDebit),
            "acss_debit" => Ok(AcssDebit),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "cashapp" => Ok(Cashapp),
            "customer_balance" => Ok(CustomerBalance),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "konbini" => Ok(Konbini),
            "link" => Ok(Link),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "promptpay" => Ok(Promptpay),
            "sepa_credit_transfer" => Ok(SepaCreditTransfer),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoicePaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// How to handle pending invoice items on invoice creation.
///
/// One of `include` or `exclude`.
/// `include` will include any pending invoice items, and will create an empty draft invoice if no pending invoice items exist.
/// `exclude` will always create an empty invoice draft regardless if there are pending invoice items or not.
/// Defaults to `exclude` if the parameter is omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoicePendingInvoiceItemsBehavior {
    Exclude,
    Include,
    IncludeAndRequire,
}

impl CreateInvoicePendingInvoiceItemsBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateInvoicePendingInvoiceItemsBehavior::*;
        match self {
            Exclude => "exclude",
            Include => "include",
            IncludeAndRequire => "include_and_require",
        }
    }
}

impl std::str::FromStr for CreateInvoicePendingInvoiceItemsBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoicePendingInvoiceItemsBehavior::*;
        match s {
            "exclude" => Ok(Exclude),
            "include" => Ok(Include),
            "include_and_require" => Ok(IncludeAndRequire),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoicePendingInvoiceItemsBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// The rendering-related settings that control how the invoice is displayed on customer-facing surfaces such as PDF and Hosted Invoice Page.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoiceRendering {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<CreateInvoiceRenderingAmountTaxDisplay>,
    /// Invoice pdf rendering options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf: Option<CreateInvoiceRenderingPdf>,
}
impl CreateInvoiceRendering {
    pub fn new() -> Self {
        Self::default()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
///
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceRenderingAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoiceRenderingAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Invoice pdf rendering options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoiceRenderingPdf {
    /// Page size for invoice PDF.
    ///
    /// Can be set to `a4`, `letter`, or `auto`.  If set to `auto`, invoice PDF page size defaults to `a4` for customers with  Japanese locale and `letter` for customers with other locales.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<CreateInvoiceRenderingPdfPageSize>,
}
impl CreateInvoiceRenderingPdf {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Page size for invoice PDF.
///
/// Can be set to `a4`, `letter`, or `auto`.  If set to `auto`, invoice PDF page size defaults to `a4` for customers with  Japanese locale and `letter` for customers with other locales.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceRenderingPdfPageSize::*;
        match s {
            "a4" => Ok(A4),
            "auto" => Ok(Auto),
            "letter" => Ok(Letter),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoiceRenderingPdfPageSize {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// This is a legacy field that will be removed soon.
///
/// For details about `rendering_options`, refer to `rendering` instead.
/// Options for invoice PDF rendering.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoiceRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<CreateInvoiceRenderingOptionsAmountTaxDisplay>,
}
impl CreateInvoiceRenderingOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
///
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoiceRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl CreateInvoiceRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        use CreateInvoiceRenderingOptionsAmountTaxDisplay::*;
        match self {
            ExcludeTax => "exclude_tax",
            IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl std::str::FromStr for CreateInvoiceRenderingOptionsAmountTaxDisplay {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceRenderingOptionsAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoiceRenderingOptionsAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoiceRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoiceRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoiceRenderingOptionsAmountTaxDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Settings for the cost of shipping for this invoice.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoiceShippingCost<'a> {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<&'a str>,
    /// Parameters to create a new ad-hoc shipping rate for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<CreateInvoiceShippingCostShippingRateData<'a>>,
}
impl<'a> CreateInvoiceShippingCost<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Parameters to create a new ad-hoc shipping rate for this order.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceShippingCostShippingRateData<'a> {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<CreateInvoiceShippingCostShippingRateDataDeliveryEstimate>,
    /// The name of the shipping rate, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    pub display_name: &'a str,
    /// Describes a fixed amount to charge for shipping.
    ///
    /// Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<CreateInvoiceShippingCostShippingRateDataFixedAmount<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateInvoiceShippingCostShippingRateDataTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    ///
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// The type of calculation to use on the shipping rate.
    ///
    /// Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateInvoiceShippingCostShippingRateDataType>,
}
impl<'a> CreateInvoiceShippingCostShippingRateData<'a> {
    pub fn new(display_name: &'a str) -> Self {
        Self {
            delivery_estimate: Default::default(),
            display_name,
            fixed_amount: Default::default(),
            metadata: Default::default(),
            tax_behavior: Default::default(),
            tax_code: Default::default(),
            type_: Default::default(),
        }
    }
}
/// The estimated range for how long shipping will take, meant to be displayable to the customer.
///
/// This will appear on CheckoutSessions.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoiceShippingCostShippingRateDataDeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximum>,
    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimum>,
}
impl CreateInvoiceShippingCostShippingRateDataDeliveryEstimate {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The upper bound of the estimated range.
///
/// If empty, represents no upper bound i.e., infinite.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximum {
    pub fn new(
        unit: CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit,
        value: i64,
    ) -> Self {
        Self { unit, value }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// The lower bound of the estimated range.
///
/// If empty, represents no lower bound.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimum {
    pub fn new(
        unit: CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit,
        value: i64,
    ) -> Self {
        Self { unit, value }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Describes a fixed amount to charge for shipping.
///
/// Must be present if type is `fixed_amount`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceShippingCostShippingRateDataFixedAmount<'a> {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Shipping rates defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        &'a std::collections::HashMap<
            stripe_types::Currency,
            CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptions,
        >,
    >,
}
impl<'a> CreateInvoiceShippingCostShippingRateDataFixedAmount<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self { amount, currency, currency_options: Default::default() }
    }
}
/// Shipping rates defined in each available currency option.
///
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior>,
}
impl CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptions {
    pub fn new(amount: i64) -> Self {
        Self { amount, tax_behavior: Default::default() }
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceShippingCostShippingRateDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoiceShippingCostShippingRateDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// The type of calculation to use on the shipping rate.
///
/// Can only be `fixed_amount` for now.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceShippingCostShippingRateDataType::*;
        match s {
            "fixed_amount" => Ok(FixedAmount),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateInvoiceShippingCostShippingRateDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// Shipping details for the invoice.
///
/// The Invoice PDF will use the `shipping_details` value if it is set, otherwise the PDF will render the shipping address from the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceShippingDetails<'a> {
    /// Shipping address.
    pub address: CreateInvoiceShippingDetailsAddress<'a>,
    /// Recipient name.
    pub name: &'a str,
    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> CreateInvoiceShippingDetails<'a> {
    pub fn new(address: CreateInvoiceShippingDetailsAddress<'a>, name: &'a str) -> Self {
        Self { address, name, phone: Default::default() }
    }
}
/// Shipping address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoiceShippingDetailsAddress<'a> {
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
impl<'a> CreateInvoiceShippingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If specified, the funds from the invoice will be transferred to the destination and the ID of the resulting transfer will be found on the invoice's charge.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceTransferData<'a> {
    /// The amount that will be transferred automatically when the invoice is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ID of an existing, connected Stripe account.
    pub destination: &'a str,
}
impl<'a> CreateInvoiceTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount: Default::default(), destination }
    }
}
impl<'a> CreateInvoice<'a> {
    /// This endpoint creates a draft invoice for a given customer.
    ///
    /// The invoice remains a draft until you [finalize](https://stripe.com/docs/api#finalize_invoice) the invoice, which allows you to [pay](https://stripe.com/docs/api#pay_invoice) or [send](https://stripe.com/docs/api#send_invoice) the invoice to your customers.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::Invoice> {
        client.send_form("/invoices", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListInvoice<'a> {
    /// The collection method of the invoice to retrieve.
    ///
    /// Either `charge_automatically` or `send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<ListInvoiceCollectionMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// Only return invoices for the customer specified by this customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
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
    pub starting_after: Option<&'a str>,
    /// The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`.
    ///
    /// [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListInvoiceStatus>,
    /// Only return invoices for the subscription specified by this subscription ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
}
impl<'a> ListInvoice<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The collection method of the invoice to retrieve.
///
/// Either `charge_automatically` or `send_invoice`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListInvoiceCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl ListInvoiceCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use ListInvoiceCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for ListInvoiceCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListInvoiceCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListInvoiceCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListInvoiceCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListInvoiceCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListInvoiceCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`.
///
/// [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListInvoiceStatus {
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
}

impl ListInvoiceStatus {
    pub fn as_str(self) -> &'static str {
        use ListInvoiceStatus::*;
        match self {
            Draft => "draft",
            Open => "open",
            Paid => "paid",
            Uncollectible => "uncollectible",
            Void => "void",
        }
    }
}

impl std::str::FromStr for ListInvoiceStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListInvoiceStatus::*;
        match s {
            "draft" => Ok(Draft),
            "open" => Ok(Open),
            "paid" => Ok(Paid),
            "uncollectible" => Ok(Uncollectible),
            "void" => Ok(Void),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListInvoiceStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListInvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListInvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListInvoiceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> ListInvoice<'a> {
    /// You can list all invoices, or list the invoices for a specific customer.
    ///
    /// The invoices are returned sorted by creation date, with the most recently created invoices appearing first.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::Invoice>> {
        client.get_query("/invoices", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::Invoice> {
        stripe::ListPaginator::from_params("/invoices", self)
    }
}
impl<'a> stripe::PaginationParams for ListInvoice<'a> {}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveInvoice<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveInvoice<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveInvoice<'a> {
    /// Retrieves the invoice with the given ID.
    pub fn send(
        &self,
        client: &stripe::Client,
        invoice: &stripe_types::invoice::InvoiceId,
    ) -> stripe::Response<stripe_types::Invoice> {
        client.get_query(&format!("/invoices/{invoice}", invoice = invoice), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteInvoice {}
impl DeleteInvoice {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteInvoice {
    /// Permanently deletes a one-off invoice draft.
    ///
    /// This cannot be undone.
    /// Attempts to delete invoices that are no longer in a draft state will fail; once an invoice has been finalized or if an invoice is for a subscription, it must be [voided](https://stripe.com/docs/api#void_invoice).
    pub fn send(
        &self,
        client: &stripe::Client,
        invoice: &stripe_types::invoice::InvoiceId,
    ) -> stripe::Response<stripe_types::DeletedInvoice> {
        client.send_form(
            &format!("/invoices/{invoice}", invoice = invoice),
            self,
            http_types::Method::Delete,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct FinalizeInvoiceInvoice<'a> {
    /// Controls whether Stripe performs [automatic collection](https://stripe.com/docs/invoicing/integration/automatic-advancement-collection) of the invoice.
    ///
    /// If `false`, the invoice's state doesn't automatically advance without an explicit action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> FinalizeInvoiceInvoice<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> FinalizeInvoiceInvoice<'a> {
    /// Stripe automatically finalizes drafts before sending and attempting payment on invoices.
    ///
    /// However, if you’d like to finalize a draft invoice manually, you can do so using this method.
    pub fn send(
        &self,
        client: &stripe::Client,
        invoice: &stripe_types::invoice::InvoiceId,
    ) -> stripe::Response<stripe_types::Invoice> {
        client.send_form(
            &format!("/invoices/{invoice}/finalize", invoice = invoice),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct SendInvoiceInvoice<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> SendInvoiceInvoice<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> SendInvoiceInvoice<'a> {
    /// Stripe will automatically send invoices to customers according to your [subscriptions settings](https://dashboard.stripe.com/account/billing/automatic).
    ///
    /// However, if you’d like to manually send an invoice to your customer out of the normal schedule, you can do so.
    /// When sending invoices that have already been paid, there will be no reference to the payment in the email.  Requests made in test-mode result in no emails being sent, despite sending an `invoice.sent` event.
    pub fn send(
        &self,
        client: &stripe::Client,
        invoice: &stripe_types::invoice::InvoiceId,
    ) -> stripe::Response<stripe_types::Invoice> {
        client.send_form(
            &format!("/invoices/{invoice}/send", invoice = invoice),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct MarkUncollectibleInvoice<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> MarkUncollectibleInvoice<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> MarkUncollectibleInvoice<'a> {
    /// Marking an invoice as uncollectible is useful for keeping track of bad debts that can be written off for accounting purposes.
    pub fn send(
        &self,
        client: &stripe::Client,
        invoice: &stripe_types::invoice::InvoiceId,
    ) -> stripe::Response<stripe_types::Invoice> {
        client.send_form(
            &format!("/invoices/{invoice}/mark_uncollectible", invoice = invoice),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct VoidInvoiceInvoice<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> VoidInvoiceInvoice<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> VoidInvoiceInvoice<'a> {
    /// Mark a finalized invoice as void.
    ///
    /// This cannot be undone.
    /// Voiding an invoice is similar to [deletion](https://stripe.com/docs/api#delete_invoice), however it only applies to finalized invoices and maintains a papertrail where the invoice can still be found.
    pub fn send(
        &self,
        client: &stripe::Client,
        invoice: &stripe_types::invoice::InvoiceId,
    ) -> stripe::Response<stripe_types::Invoice> {
        client.send_form(
            &format!("/invoices/{invoice}/void", invoice = invoice),
            self,
            http_types::Method::Post,
        )
    }
}
