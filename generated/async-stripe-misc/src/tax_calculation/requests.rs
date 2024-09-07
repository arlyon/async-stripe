use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
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
impl ListLineItemsTaxCalculationBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Retrieves the line items of a persisted tax calculation as a collection.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListLineItemsTaxCalculation {
    inner: ListLineItemsTaxCalculationBuilder,
    calculation: stripe_misc::TaxCalculationId,
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxCalculationCustomerDetails {
    /// The customer's postal address (for example, home or business location).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateTaxCalculationCustomerDetailsAddress>,
    /// The type of customer address provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_source: Option<CreateTaxCalculationCustomerDetailsAddressSource>,
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
    pub taxability_override: Option<CreateTaxCalculationCustomerDetailsTaxabilityOverride>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxCalculationCustomerDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
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
    /// We recommend sending [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code value when possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
/// The type of customer address provided.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxCalculationCustomerDetailsAddressSource {
    Billing,
    Shipping,
}
impl CreateTaxCalculationCustomerDetailsAddressSource {
    pub fn as_str(self) -> &'static str {
        use CreateTaxCalculationCustomerDetailsAddressSource::*;
        match self {
            Billing => "billing",
            Shipping => "shipping",
        }
    }
}

impl std::str::FromStr for CreateTaxCalculationCustomerDetailsAddressSource {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxCalculationCustomerDetailsAddressSource::*;
        match s {
            "billing" => Ok(Billing),
            "shipping" => Ok(Shipping),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxCalculationCustomerDetailsAddressSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxCalculationCustomerDetailsAddressSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxCalculationCustomerDetailsAddressSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxCalculationCustomerDetailsAddressSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateTaxCalculationCustomerDetailsAddressSource",
            )
        })
    }
}
/// The customer's tax IDs.
/// Stripe Tax might consider a transaction with applicable tax IDs to be B2B, which might affect the tax calculation result.
/// Stripe Tax doesn't validate tax IDs for correctness.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxCalculationCustomerDetailsTaxIds {
    /// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bh_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `kz_bin`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: CreateTaxCalculationCustomerDetailsTaxIdsType,
    /// Value of the tax ID.
    pub value: String,
}
impl CreateTaxCalculationCustomerDetailsTaxIds {
    pub fn new(
        type_: impl Into<CreateTaxCalculationCustomerDetailsTaxIdsType>,
        value: impl Into<String>,
    ) -> Self {
        Self { type_: type_.into(), value: value.into() }
    }
}
/// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bh_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `kz_bin`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTaxCalculationCustomerDetailsTaxIdsType {
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
    Unknown,
}
impl CreateTaxCalculationCustomerDetailsTaxIdsType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxCalculationCustomerDetailsTaxIdsType::*;
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
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateTaxCalculationCustomerDetailsTaxIdsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxCalculationCustomerDetailsTaxIdsType::*;
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
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for CreateTaxCalculationCustomerDetailsTaxIdsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxCalculationCustomerDetailsTaxIdsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxCalculationCustomerDetailsTaxIdsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxCalculationCustomerDetailsTaxIdsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// Overrides the tax calculation result to allow you to not collect tax from your customer.
/// Use this if you've manually checked your customer's tax exemptions.
/// Prefer providing the customer's `tax_ids` where possible, which automatically determines whether `reverse_charge` applies.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxCalculationCustomerDetailsTaxabilityOverride {
    CustomerExempt,
    None,
    ReverseCharge,
}
impl CreateTaxCalculationCustomerDetailsTaxabilityOverride {
    pub fn as_str(self) -> &'static str {
        use CreateTaxCalculationCustomerDetailsTaxabilityOverride::*;
        match self {
            CustomerExempt => "customer_exempt",
            None => "none",
            ReverseCharge => "reverse_charge",
        }
    }
}

impl std::str::FromStr for CreateTaxCalculationCustomerDetailsTaxabilityOverride {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxCalculationCustomerDetailsTaxabilityOverride::*;
        match s {
            "customer_exempt" => Ok(CustomerExempt),
            "none" => Ok(None),
            "reverse_charge" => Ok(ReverseCharge),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxCalculationCustomerDetailsTaxabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxCalculationCustomerDetailsTaxabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxCalculationCustomerDetailsTaxabilityOverride {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxCalculationCustomerDetailsTaxabilityOverride {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateTaxCalculationCustomerDetailsTaxabilityOverride",
            )
        })
    }
}
/// A list of items the customer is purchasing.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxCalculationLineItems {
    /// A positive integer representing the line item's total price in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.0 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to twelve digits (e.g., a value of 999999999999 for a USD charge of $9,999,999,999.99).
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes are calculated on top of this amount.
    pub amount: i64,
    /// If provided, the product's `tax_code` will be used as the line item's `tax_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// The number of units of the item being purchased.
    /// Used to calculate the per-unit price from the total `amount` for the line.
    /// For example, if `amount=100` and `quantity=4`, the calculated unit price is 25.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A custom identifier for this line item, which must be unique across the line items in the calculation.
    /// The reference helps identify each line item in exported [tax reports](https://stripe.com/docs/tax/reports).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Specifies whether the `amount` includes taxes. Defaults to `exclusive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateTaxCalculationLineItemsTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID to use for this line item.
    /// If not provided, we will use the tax code from the provided `product` param.
    /// If neither `tax_code` nor `product` is provided, we will use the default tax code from your Tax Settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
}
impl CreateTaxCalculationLineItems {
    pub fn new(amount: impl Into<i64>) -> Self {
        Self {
            amount: amount.into(),
            product: None,
            quantity: None,
            reference: None,
            tax_behavior: None,
            tax_code: None,
        }
    }
}
/// Specifies whether the `amount` includes taxes. Defaults to `exclusive`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxCalculationLineItemsTaxBehavior {
    Exclusive,
    Inclusive,
}
impl CreateTaxCalculationLineItemsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateTaxCalculationLineItemsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
        }
    }
}

impl std::str::FromStr for CreateTaxCalculationLineItemsTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxCalculationLineItemsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxCalculationLineItemsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxCalculationLineItemsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxCalculationLineItemsTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxCalculationLineItemsTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxCalculationLineItemsTaxBehavior")
        })
    }
}
/// Details about the address from which the goods are being shipped.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxCalculationShipFromDetails {
    /// The address from which the goods are being shipped from.
    pub address: CreateTaxCalculationShipFromDetailsAddress,
}
impl CreateTaxCalculationShipFromDetails {
    pub fn new(address: impl Into<CreateTaxCalculationShipFromDetailsAddress>) -> Self {
        Self { address: address.into() }
    }
}
/// The address from which the goods are being shipped from.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxCalculationShipFromDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix.
    /// Example: "NY" or "TX".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxCalculationShippingCost {
    /// A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) representing the shipping charge.
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes are calculated on top of this amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// If provided, the [shipping rate](https://stripe.com/docs/api/shipping_rates/object)'s `amount`, `tax_code` and `tax_behavior` are used.
    /// If you provide a shipping rate, then you cannot pass the `amount`, `tax_code`, or `tax_behavior` parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
    /// Specifies whether the `amount` includes taxes.
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    /// Defaults to `exclusive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateTaxCalculationShippingCostTaxBehavior>,
    /// The [tax code](https://stripe.com/docs/tax/tax-categories) used to calculate tax on shipping.
    /// If not provided, the default shipping tax code from your [Tax Settings](/settings/tax) is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
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
/// Specifies whether the `amount` includes taxes.
/// If `tax_behavior=inclusive`, then the amount includes taxes.
/// Defaults to `exclusive`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxCalculationShippingCostTaxBehavior {
    Exclusive,
    Inclusive,
}
impl CreateTaxCalculationShippingCostTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateTaxCalculationShippingCostTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
        }
    }
}

impl std::str::FromStr for CreateTaxCalculationShippingCostTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxCalculationShippingCostTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxCalculationShippingCostTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxCalculationShippingCostTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxCalculationShippingCostTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxCalculationShippingCostTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateTaxCalculationShippingCostTaxBehavior",
            )
        })
    }
}
/// Calculates tax based on input and returns a Tax `Calculation` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxCalculation {
    inner: CreateTaxCalculationBuilder,
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
    /// Timestamp of date at which the tax rules and rates in effect applies for the calculation.
    /// Measured in seconds since the Unix epoch.
    /// Can be up to 48 hours in the past, and up to 48 hours in the future.
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
