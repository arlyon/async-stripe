use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveTaxCalculationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveTaxCalculationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveTaxCalculationBuilder").finish_non_exhaustive()
    }
}
impl RetrieveTaxCalculationBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a Tax `Calculation` object, if the calculation hasn’t expired.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveTaxCalculation {
    inner: RetrieveTaxCalculationBuilder,
    calculation: stripe_misc::TaxCalculationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveTaxCalculation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveTaxCalculation").finish_non_exhaustive()
    }
}
impl RetrieveTaxCalculation {
    /// Construct a new `RetrieveTaxCalculation`.
    pub fn new(calculation: impl Into<stripe_misc::TaxCalculationId>) -> Self {
        Self { calculation: calculation.into(), inner: RetrieveTaxCalculationBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTaxCalculation {
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

impl StripeRequest for RetrieveTaxCalculation {
    type Output = stripe_misc::TaxCalculation;

    fn build(&self) -> RequestBuilder {
        let calculation = &self.calculation;
        RequestBuilder::new(StripeMethod::Get, format!("/tax/calculations/{calculation}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListLineItemsTaxCalculationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListLineItemsTaxCalculationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListLineItemsTaxCalculationBuilder").finish_non_exhaustive()
    }
}
impl ListLineItemsTaxCalculationBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Retrieves the line items of a tax calculation as a collection, if the calculation hasn’t expired.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListLineItemsTaxCalculation {
    inner: ListLineItemsTaxCalculationBuilder,
    calculation: stripe_misc::TaxCalculationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListLineItemsTaxCalculation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListLineItemsTaxCalculation").finish_non_exhaustive()
    }
}
impl ListLineItemsTaxCalculation {
    /// Construct a new `ListLineItemsTaxCalculation`.
    pub fn new(calculation: impl Into<stripe_misc::TaxCalculationId>) -> Self {
        Self { calculation: calculation.into(), inner: ListLineItemsTaxCalculationBuilder::new() }
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
}
impl ListLineItemsTaxCalculation {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_misc::TaxCalculationLineItem>>
    {
        let calculation = &self.calculation;

        stripe_client_core::ListPaginator::new_list(
            format!("/tax/calculations/{calculation}/line_items"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListLineItemsTaxCalculation {
    type Output = stripe_types::List<stripe_misc::TaxCalculationLineItem>;

    fn build(&self) -> RequestBuilder {
        let calculation = &self.calculation;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/tax/calculations/{calculation}/line_items"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateTaxCalculationBuilder {
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_details: Option<CreateTaxCalculationCustomerDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    line_items: Vec<CreateTaxCalculationLineItems>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ship_from_details: Option<CreateTaxCalculationShipFromDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_cost: Option<CreateTaxCalculationShippingCost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_date: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxCalculationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxCalculationBuilder").finish_non_exhaustive()
    }
}
impl CreateTaxCalculationBuilder {
    fn new(
        currency: impl Into<stripe_types::Currency>,
        line_items: impl Into<Vec<CreateTaxCalculationLineItems>>,
    ) -> Self {
        Self {
            currency: currency.into(),
            customer: None,
            customer_details: None,
            expand: None,
            line_items: line_items.into(),
            ship_from_details: None,
            shipping_cost: None,
            tax_date: None,
        }
    }
}
/// Details about the customer, including address and tax IDs.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxCalculationCustomerDetails {
    /// The customer's postal address (for example, home or business location).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateTaxCalculationCustomerDetailsAddress>,
    /// The type of customer address provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_source: Option<stripe_shared::AddressSource>,
    /// The customer's IP address (IPv4 or IPv6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// The customer's tax IDs.
    /// Stripe Tax might consider a transaction with applicable tax IDs to be B2B, which might affect the tax calculation result.
    /// Stripe Tax doesn't validate tax IDs for correctness.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<CreateTaxCalculationCustomerDetailsTaxIds>>,
    /// Overrides the tax calculation result to allow you to not collect tax from your customer.
    /// Use this if you've manually checked your customer's tax exemptions.
    /// Prefer providing the customer's `tax_ids` where possible, which automatically determines whether `reverse_charge` applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxability_override: Option<stripe_shared::TaxabilityOverride>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxCalculationCustomerDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxCalculationCustomerDetails").finish_non_exhaustive()
    }
}
impl CreateTaxCalculationCustomerDetails {
    pub fn new() -> Self {
        Self {
            address: None,
            address_source: None,
            ip_address: None,
            tax_ids: None,
            taxability_override: None,
        }
    }
}
impl Default for CreateTaxCalculationCustomerDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// The customer's postal address (for example, home or business location).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxCalculationCustomerDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    /// We recommend sending [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code value when possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxCalculationCustomerDetailsAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxCalculationCustomerDetailsAddress").finish_non_exhaustive()
    }
}
impl CreateTaxCalculationCustomerDetailsAddress {
    pub fn new(country: impl Into<String>) -> Self {
        Self {
            city: None,
            country: country.into(),
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
        }
    }
}
/// The customer's tax IDs.
/// Stripe Tax might consider a transaction with applicable tax IDs to be B2B, which might affect the tax calculation result.
/// Stripe Tax doesn't validate tax IDs for correctness.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxCalculationCustomerDetailsTaxIds {
    /// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `al_tin`, `am_tin`, `ao_tin`, `ar_cuit`, `au_abn`, `au_arn`, `aw_tin`, `az_tin`, `ba_tin`, `bb_tin`, `bd_bin`, `bf_ifu`, `bg_uic`, `bh_vat`, `bj_ifu`, `bo_tin`, `br_cnpj`, `br_cpf`, `bs_tin`, `by_tin`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `cd_nif`, `ch_uid`, `ch_vat`, `cl_tin`, `cm_niu`, `cn_tin`, `co_nit`, `cr_tin`, `cv_nif`, `de_stn`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `et_tin`, `eu_oss_vat`, `eu_vat`, `fo_vat`, `gb_vat`, `ge_vat`, `gi_tin`, `gn_nif`, `hk_br`, `hr_oib`, `hu_tin`, `ic_nif`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `it_cf`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kg_tin`, `kh_tin`, `kr_brn`, `kz_bin`, `la_tin`, `li_uid`, `li_vat`, `lk_vat`, `ma_vat`, `md_vat`, `me_pib`, `mk_vat`, `mr_nif`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `np_pan`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `pl_nip`, `py_ruc`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sn_ninea`, `sr_fin`, `sv_nit`, `th_vat`, `tj_tin`, `tr_tin`, `tw_vat`, `tz_vat`, `ua_vat`, `ug_tin`, `us_ein`, `uy_ruc`, `uz_tin`, `uz_vat`, `ve_rif`, `vn_tin`, `za_vat`, `zm_tin`, or `zw_tin`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxIdType,
    /// Value of the tax ID.
    pub value: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxCalculationCustomerDetailsTaxIds {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxCalculationCustomerDetailsTaxIds").finish_non_exhaustive()
    }
}
impl CreateTaxCalculationCustomerDetailsTaxIds {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxIdType>, value: impl Into<String>) -> Self {
        Self { type_: type_.into(), value: value.into() }
    }
}
/// A list of items the customer is purchasing.
/// You can pass up to 100 line items, or 1,000 if your account has an increased limit.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxCalculationLineItems {
    /// A positive integer representing the line item's total price in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units).
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes are calculated on top of this amount.
    pub amount: i64,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// If provided, the product's `tax_code` will be used as the line item's `tax_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// The number of units of the item being purchased.
    /// Used to calculate the per-unit price from the total `amount` for the line.
    /// For example, if `amount=100` and `quantity=4`, the calculated unit price is 25.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A custom identifier for this line item, which must be unique across the line items in the calculation.
    /// The reference helps identify each line item in exported [tax reports](https://docs.stripe.com/tax/reports).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Specifies whether the `amount` includes taxes. Defaults to `exclusive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_shared::TaxBehavior>,
    /// A [tax code](https://docs.stripe.com/tax/tax-categories) ID to use for this line item.
    /// If not provided, we will use the tax code from the provided `product` param.
    /// If neither `tax_code` nor `product` is provided, we will use the default tax code from your Tax Settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxCalculationLineItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxCalculationLineItems").finish_non_exhaustive()
    }
}
impl CreateTaxCalculationLineItems {
    pub fn new(amount: impl Into<i64>) -> Self {
        Self {
            amount: amount.into(),
            metadata: None,
            product: None,
            quantity: None,
            reference: None,
            tax_behavior: None,
            tax_code: None,
        }
    }
}
/// Details about the address from which the goods are being shipped.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxCalculationShipFromDetails {
    /// The address from which the goods are being shipped from.
    pub address: CreateTaxCalculationShipFromDetailsAddress,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxCalculationShipFromDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxCalculationShipFromDetails").finish_non_exhaustive()
    }
}
impl CreateTaxCalculationShipFromDetails {
    pub fn new(address: impl Into<CreateTaxCalculationShipFromDetailsAddress>) -> Self {
        Self { address: address.into() }
    }
}
/// The address from which the goods are being shipped from.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxCalculationShipFromDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix, such as "NY" or "TX".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxCalculationShipFromDetailsAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxCalculationShipFromDetailsAddress").finish_non_exhaustive()
    }
}
impl CreateTaxCalculationShipFromDetailsAddress {
    pub fn new(country: impl Into<String>) -> Self {
        Self {
            city: None,
            country: country.into(),
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
        }
    }
}
/// Shipping cost details to be used for the calculation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxCalculationShippingCost {
    /// A positive integer in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units) representing the shipping charge.
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes are calculated on top of this amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// If provided, the [shipping rate](https://docs.stripe.com/api/shipping_rates/object)'s `amount`, `tax_code` and `tax_behavior` are used.
    /// If you provide a shipping rate, then you cannot pass the `amount`, `tax_code`, or `tax_behavior` parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
    /// Specifies whether the `amount` includes taxes.
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    /// Defaults to `exclusive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_shared::TaxBehavior>,
    /// The [tax code](https://docs.stripe.com/tax/tax-categories) used to calculate tax on shipping.
    /// If not provided, the default shipping tax code from your [Tax Settings](https://dashboard.stripe.com/settings/tax) is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxCalculationShippingCost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxCalculationShippingCost").finish_non_exhaustive()
    }
}
impl CreateTaxCalculationShippingCost {
    pub fn new() -> Self {
        Self { amount: None, shipping_rate: None, tax_behavior: None, tax_code: None }
    }
}
impl Default for CreateTaxCalculationShippingCost {
    fn default() -> Self {
        Self::new()
    }
}
/// Calculates tax based on the input and returns a Tax `Calculation` object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxCalculation {
    inner: CreateTaxCalculationBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxCalculation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxCalculation").finish_non_exhaustive()
    }
}
impl CreateTaxCalculation {
    /// Construct a new `CreateTaxCalculation`.
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        line_items: impl Into<Vec<CreateTaxCalculationLineItems>>,
    ) -> Self {
        Self { inner: CreateTaxCalculationBuilder::new(currency.into(), line_items.into()) }
    }
    /// The ID of an existing customer to use for this calculation.
    /// If provided, the customer's address and tax IDs are copied to `customer_details`.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// Details about the customer, including address and tax IDs.
    pub fn customer_details(
        mut self,
        customer_details: impl Into<CreateTaxCalculationCustomerDetails>,
    ) -> Self {
        self.inner.customer_details = Some(customer_details.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Details about the address from which the goods are being shipped.
    pub fn ship_from_details(
        mut self,
        ship_from_details: impl Into<CreateTaxCalculationShipFromDetails>,
    ) -> Self {
        self.inner.ship_from_details = Some(ship_from_details.into());
        self
    }
    /// Shipping cost details to be used for the calculation.
    pub fn shipping_cost(
        mut self,
        shipping_cost: impl Into<CreateTaxCalculationShippingCost>,
    ) -> Self {
        self.inner.shipping_cost = Some(shipping_cost.into());
        self
    }
    /// The calculation uses the tax rules and rates that are in effect at this timestamp.
    /// You can use a date up to 31 days in the past or up to 31 days in the future.
    /// If you use a future date, Stripe doesn't guarantee that the expected tax rules and rate being used match the actual rules and rate that will be in effect on that date.
    /// We deploy tax changes before their effective date, but not within a fixed window.
    pub fn tax_date(mut self, tax_date: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.tax_date = Some(tax_date.into());
        self
    }
}
impl CreateTaxCalculation {
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

impl StripeRequest for CreateTaxCalculation {
    type Output = stripe_misc::TaxCalculation;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/tax/calculations").form(&self.inner)
    }
}

/// The type of customer address provided.
pub type CreateTaxCalculationCustomerDetailsAddressSource = stripe_shared::AddressSource;
/// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `al_tin`, `am_tin`, `ao_tin`, `ar_cuit`, `au_abn`, `au_arn`, `aw_tin`, `az_tin`, `ba_tin`, `bb_tin`, `bd_bin`, `bf_ifu`, `bg_uic`, `bh_vat`, `bj_ifu`, `bo_tin`, `br_cnpj`, `br_cpf`, `bs_tin`, `by_tin`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `cd_nif`, `ch_uid`, `ch_vat`, `cl_tin`, `cm_niu`, `cn_tin`, `co_nit`, `cr_tin`, `cv_nif`, `de_stn`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `et_tin`, `eu_oss_vat`, `eu_vat`, `fo_vat`, `gb_vat`, `ge_vat`, `gi_tin`, `gn_nif`, `hk_br`, `hr_oib`, `hu_tin`, `ic_nif`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `it_cf`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kg_tin`, `kh_tin`, `kr_brn`, `kz_bin`, `la_tin`, `li_uid`, `li_vat`, `lk_vat`, `ma_vat`, `md_vat`, `me_pib`, `mk_vat`, `mr_nif`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `np_pan`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `pl_nip`, `py_ruc`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sn_ninea`, `sr_fin`, `sv_nit`, `th_vat`, `tj_tin`, `tr_tin`, `tw_vat`, `tz_vat`, `ua_vat`, `ug_tin`, `us_ein`, `uy_ruc`, `uz_tin`, `uz_vat`, `ve_rif`, `vn_tin`, `za_vat`, `zm_tin`, or `zw_tin`.
pub type CreateTaxCalculationCustomerDetailsTaxIdsType = stripe_shared::CreateTaxIdType;
/// Overrides the tax calculation result to allow you to not collect tax from your customer.
/// Use this if you've manually checked your customer's tax exemptions.
/// Prefer providing the customer's `tax_ids` where possible, which automatically determines whether `reverse_charge` applies.
pub type CreateTaxCalculationCustomerDetailsTaxabilityOverride = stripe_shared::TaxabilityOverride;
/// Specifies whether the `amount` includes taxes. Defaults to `exclusive`.
pub type CreateTaxCalculationLineItemsTaxBehavior = stripe_shared::TaxBehavior;
/// Specifies whether the `amount` includes taxes.
/// If `tax_behavior=inclusive`, then the amount includes taxes.
/// Defaults to `exclusive`.
pub type CreateTaxCalculationShippingCostTaxBehavior = stripe_shared::TaxBehavior;
