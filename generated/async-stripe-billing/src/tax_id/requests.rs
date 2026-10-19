use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes an existing `tax_id` object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DeleteCustomerTaxId {
    customer: stripe_shared::CustomerId,
    id: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeleteCustomerTaxId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeleteCustomerTaxId").finish_non_exhaustive()
    }
}
impl DeleteCustomerTaxId {
    /// Construct a new `DeleteCustomerTaxId`.
    pub fn new(customer: impl Into<stripe_shared::CustomerId>, id: impl Into<String>) -> Self {
        Self { customer: customer.into(), id: id.into() }
    }
}
impl DeleteCustomerTaxId {
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

impl StripeRequest for DeleteCustomerTaxId {
    type Output = stripe_shared::DeletedTaxId;

    fn build(&self) -> RequestBuilder {
        let customer = &self.customer;
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Delete, format!("/customers/{customer}/tax_ids/{id}"))
    }
}
/// Deletes an existing account or customer `tax_id` object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DeleteIdTaxId {
    id: stripe_shared::TaxIdId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeleteIdTaxId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeleteIdTaxId").finish_non_exhaustive()
    }
}
impl DeleteIdTaxId {
    /// Construct a new `DeleteIdTaxId`.
    pub fn new(id: impl Into<stripe_shared::TaxIdId>) -> Self {
        Self { id: id.into() }
    }
}
impl DeleteIdTaxId {
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

impl StripeRequest for DeleteIdTaxId {
    type Output = stripe_shared::DeletedTaxId;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Delete, format!("/tax_ids/{id}"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListCustomerTaxIdBuilder {
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
impl std::fmt::Debug for ListCustomerTaxIdBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListCustomerTaxIdBuilder").finish_non_exhaustive()
    }
}
impl ListCustomerTaxIdBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of tax IDs for a customer.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListCustomerTaxId {
    inner: ListCustomerTaxIdBuilder,
    customer: stripe_shared::CustomerId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListCustomerTaxId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListCustomerTaxId").finish_non_exhaustive()
    }
}
impl ListCustomerTaxId {
    /// Construct a new `ListCustomerTaxId`.
    pub fn new(customer: impl Into<stripe_shared::CustomerId>) -> Self {
        Self { customer: customer.into(), inner: ListCustomerTaxIdBuilder::new() }
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
impl ListCustomerTaxId {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::TaxId>> {
        let customer = &self.customer;

        stripe_client_core::ListPaginator::new_list(
            format!("/customers/{customer}/tax_ids"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListCustomerTaxId {
    type Output = stripe_types::List<stripe_shared::TaxId>;

    fn build(&self) -> RequestBuilder {
        let customer = &self.customer;
        RequestBuilder::new(StripeMethod::Get, format!("/customers/{customer}/tax_ids"))
            .query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveCustomerTaxIdBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveCustomerTaxIdBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveCustomerTaxIdBuilder").finish_non_exhaustive()
    }
}
impl RetrieveCustomerTaxIdBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the `tax_id` object with the given identifier.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveCustomerTaxId {
    inner: RetrieveCustomerTaxIdBuilder,
    customer: stripe_shared::CustomerId,
    id: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveCustomerTaxId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveCustomerTaxId").finish_non_exhaustive()
    }
}
impl RetrieveCustomerTaxId {
    /// Construct a new `RetrieveCustomerTaxId`.
    pub fn new(customer: impl Into<stripe_shared::CustomerId>, id: impl Into<String>) -> Self {
        Self {
            customer: customer.into(),
            id: id.into(),
            inner: RetrieveCustomerTaxIdBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveCustomerTaxId {
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

impl StripeRequest for RetrieveCustomerTaxId {
    type Output = stripe_shared::TaxId;

    fn build(&self) -> RequestBuilder {
        let customer = &self.customer;
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/customers/{customer}/tax_ids/{id}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListTaxIdBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<OwnerParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListTaxIdBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListTaxIdBuilder").finish_non_exhaustive()
    }
}
impl ListTaxIdBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, owner: None, starting_after: None }
    }
}
/// Returns a list of tax IDs.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListTaxId {
    inner: ListTaxIdBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListTaxId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListTaxId").finish_non_exhaustive()
    }
}
impl ListTaxId {
    /// Construct a new `ListTaxId`.
    pub fn new() -> Self {
        Self { inner: ListTaxIdBuilder::new() }
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
    /// The account or customer the tax ID belongs to. Defaults to `owner[type]=self`.
    pub fn owner(mut self, owner: impl Into<OwnerParams>) -> Self {
        self.inner.owner = Some(owner.into());
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
impl Default for ListTaxId {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTaxId {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::TaxId>> {
        stripe_client_core::ListPaginator::new_list("/tax_ids", &self.inner)
    }
}

impl StripeRequest for ListTaxId {
    type Output = stripe_types::List<stripe_shared::TaxId>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/tax_ids").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveIdTaxIdBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveIdTaxIdBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveIdTaxIdBuilder").finish_non_exhaustive()
    }
}
impl RetrieveIdTaxIdBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves an account or customer `tax_id` object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveIdTaxId {
    inner: RetrieveIdTaxIdBuilder,
    id: stripe_shared::TaxIdId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveIdTaxId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveIdTaxId").finish_non_exhaustive()
    }
}
impl RetrieveIdTaxId {
    /// Construct a new `RetrieveIdTaxId`.
    pub fn new(id: impl Into<stripe_shared::TaxIdId>) -> Self {
        Self { id: id.into(), inner: RetrieveIdTaxIdBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveIdTaxId {
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

impl StripeRequest for RetrieveIdTaxId {
    type Output = stripe_shared::TaxId;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/tax_ids/{id}")).query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateCustomerTaxIdBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(rename = "type")]
    type_: stripe_shared::CreateTaxIdType,
    value: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCustomerTaxIdBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCustomerTaxIdBuilder").finish_non_exhaustive()
    }
}
impl CreateCustomerTaxIdBuilder {
    fn new(type_: impl Into<stripe_shared::CreateTaxIdType>, value: impl Into<String>) -> Self {
        Self { expand: None, type_: type_.into(), value: value.into() }
    }
}
/// Creates a new `tax_id` object for a customer.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCustomerTaxId {
    inner: CreateCustomerTaxIdBuilder,
    customer: stripe_shared::CustomerId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCustomerTaxId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCustomerTaxId").finish_non_exhaustive()
    }
}
impl CreateCustomerTaxId {
    /// Construct a new `CreateCustomerTaxId`.
    pub fn new(
        customer: impl Into<stripe_shared::CustomerId>,
        type_: impl Into<stripe_shared::CreateTaxIdType>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            customer: customer.into(),
            inner: CreateCustomerTaxIdBuilder::new(type_.into(), value.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CreateCustomerTaxId {
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

impl StripeRequest for CreateCustomerTaxId {
    type Output = stripe_shared::TaxId;

    fn build(&self) -> RequestBuilder {
        let customer = &self.customer;
        RequestBuilder::new(StripeMethod::Post, format!("/customers/{customer}/tax_ids"))
            .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateTaxIdBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<OwnerParams>,
    #[serde(rename = "type")]
    type_: stripe_shared::CreateTaxIdType,
    value: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxIdBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxIdBuilder").finish_non_exhaustive()
    }
}
impl CreateTaxIdBuilder {
    fn new(type_: impl Into<stripe_shared::CreateTaxIdType>, value: impl Into<String>) -> Self {
        Self { expand: None, owner: None, type_: type_.into(), value: value.into() }
    }
}
/// Creates a new account or customer `tax_id` object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxId {
    inner: CreateTaxIdBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxId").finish_non_exhaustive()
    }
}
impl CreateTaxId {
    /// Construct a new `CreateTaxId`.
    pub fn new(type_: impl Into<stripe_shared::CreateTaxIdType>, value: impl Into<String>) -> Self {
        Self { inner: CreateTaxIdBuilder::new(type_.into(), value.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The account or customer the tax ID belongs to. Defaults to `owner[type]=self`.
    pub fn owner(mut self, owner: impl Into<OwnerParams>) -> Self {
        self.inner.owner = Some(owner.into());
        self
    }
}
impl CreateTaxId {
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

impl StripeRequest for CreateTaxId {
    type Output = stripe_shared::TaxId;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/tax_ids").form(&self.inner)
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct OwnerParams {
    /// Connected Account the tax ID belongs to. Required when `type=account`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Customer the tax ID belongs to. Required when `type=customer`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    /// ID of the Account representing the customer that the tax ID belongs to.
    /// Can be used in place of `customer` when `type=customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    /// Type of owner referenced.
    #[serde(rename = "type")]
    pub type_: stripe_shared::TaxIDsOwnerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for OwnerParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("OwnerParams").finish_non_exhaustive()
    }
}
impl OwnerParams {
    pub fn new(type_: impl Into<stripe_shared::TaxIDsOwnerType>) -> Self {
        Self { account: None, customer: None, customer_account: None, type_: type_.into() }
    }
}
/// Type of owner referenced.
pub type ListTaxIdOwnerType = stripe_shared::TaxIDsOwnerType;
/// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `al_tin`, `am_tin`, `ao_tin`, `ar_cuit`, `au_abn`, `au_arn`, `aw_tin`, `az_tin`, `ba_tin`, `bb_tin`, `bd_bin`, `bf_ifu`, `bg_uic`, `bh_vat`, `bj_ifu`, `bo_tin`, `br_cnpj`, `br_cpf`, `bs_tin`, `by_tin`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `cd_nif`, `ch_uid`, `ch_vat`, `cl_tin`, `cm_niu`, `cn_tin`, `co_nit`, `cr_tin`, `cv_nif`, `de_stn`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `et_tin`, `eu_oss_vat`, `eu_vat`, `fo_vat`, `gb_vat`, `ge_vat`, `gi_tin`, `gn_nif`, `hk_br`, `hr_oib`, `hu_tin`, `ic_nif`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `it_cf`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kg_tin`, `kh_tin`, `kr_brn`, `kz_bin`, `la_tin`, `li_uid`, `li_vat`, `lk_vat`, `ma_vat`, `md_vat`, `me_pib`, `mk_vat`, `mr_nif`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `np_pan`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `pl_nip`, `py_ruc`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sn_ninea`, `sr_fin`, `sv_nit`, `th_vat`, `tj_tin`, `tr_tin`, `tw_vat`, `tz_vat`, `ua_vat`, `ug_tin`, `us_ein`, `uy_ruc`, `uz_tin`, `uz_vat`, `ve_rif`, `vn_tin`, `za_vat`, `zm_tin`, or `zw_tin`.
pub type CreateCustomerTaxIdType = stripe_shared::CreateTaxIdType;
/// Type of owner referenced.
pub type CreateTaxIdOwnerType = stripe_shared::TaxIDsOwnerType;
/// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `al_tin`, `am_tin`, `ao_tin`, `ar_cuit`, `au_abn`, `au_arn`, `aw_tin`, `az_tin`, `ba_tin`, `bb_tin`, `bd_bin`, `bf_ifu`, `bg_uic`, `bh_vat`, `bj_ifu`, `bo_tin`, `br_cnpj`, `br_cpf`, `bs_tin`, `by_tin`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `cd_nif`, `ch_uid`, `ch_vat`, `cl_tin`, `cm_niu`, `cn_tin`, `co_nit`, `cr_tin`, `cv_nif`, `de_stn`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `et_tin`, `eu_oss_vat`, `eu_vat`, `fo_vat`, `gb_vat`, `ge_vat`, `gi_tin`, `gn_nif`, `hk_br`, `hr_oib`, `hu_tin`, `ic_nif`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `it_cf`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kg_tin`, `kh_tin`, `kr_brn`, `kz_bin`, `la_tin`, `li_uid`, `li_vat`, `lk_vat`, `ma_vat`, `md_vat`, `me_pib`, `mk_vat`, `mr_nif`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `np_pan`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `pl_nip`, `py_ruc`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sn_ninea`, `sr_fin`, `sv_nit`, `th_vat`, `tj_tin`, `tr_tin`, `tw_vat`, `tz_vat`, `ua_vat`, `ug_tin`, `us_ein`, `uy_ruc`, `uz_tin`, `uz_vat`, `ve_rif`, `vn_tin`, `za_vat`, `zm_tin`, or `zw_tin`.
pub type CreateTaxIdType = stripe_shared::CreateTaxIdType;
/// The account or customer the tax ID belongs to. Defaults to `owner[type]=self`.
pub type ListTaxIdOwner = OwnerParams;
/// The account or customer the tax ID belongs to. Defaults to `owner[type]=self`.
pub type CreateTaxIdOwner = OwnerParams;
