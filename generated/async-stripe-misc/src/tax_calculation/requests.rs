use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveTaxCalculationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTaxCalculationBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a Tax `Calculation` object, if the calculation hasn’t expired.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTaxCalculation {
    inner: RetrieveTaxCalculationBuilder,
    calculation: stripe_misc::TaxCalculationId,
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
/// Retrieves the line items of a tax calculation as a collection, if the calculation hasn’t expired.
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTaxCalculationCustomerDetailsAddressSource {
    Billing,
    Shipping,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTaxCalculationCustomerDetailsAddressSource {
    pub fn as_str(&self) -> &str {
        use CreateTaxCalculationCustomerDetailsAddressSource::*;
        match self {
            Billing => "billing",
            Shipping => "shipping",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTaxCalculationCustomerDetailsAddressSource {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxCalculationCustomerDetailsAddressSource::*;
        match s {
            "billing" => Ok(Billing),
            "shipping" => Ok(Shipping),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTaxCalculationCustomerDetailsAddressSource"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The customer's tax IDs.
/// Stripe Tax might consider a transaction with applicable tax IDs to be B2B, which might affect the tax calculation result.
/// Stripe Tax doesn't validate tax IDs for correctness.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxCalculationCustomerDetailsTaxIds {
    /// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `al_tin`, `am_tin`, `ao_tin`, `ar_cuit`, `au_abn`, `au_arn`, `aw_tin`, `az_tin`, `ba_tin`, `bb_tin`, `bd_bin`, `bf_ifu`, `bg_uic`, `bh_vat`, `bj_ifu`, `bo_tin`, `br_cnpj`, `br_cpf`, `bs_tin`, `by_tin`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `cd_nif`, `ch_uid`, `ch_vat`, `cl_tin`, `cm_niu`, `cn_tin`, `co_nit`, `cr_tin`, `cv_nif`, `de_stn`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `et_tin`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `gn_nif`, `hk_br`, `hr_oib`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kg_tin`, `kh_tin`, `kr_brn`, `kz_bin`, `la_tin`, `li_uid`, `li_vat`, `ma_vat`, `md_vat`, `me_pib`, `mk_vat`, `mr_nif`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `np_pan`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sn_ninea`, `sr_fin`, `sv_nit`, `th_vat`, `tj_tin`, `tr_tin`, `tw_vat`, `tz_vat`, `ua_vat`, `ug_tin`, `us_ein`, `uy_ruc`, `uz_tin`, `uz_vat`, `ve_rif`, `vn_tin`, `za_vat`, `zm_tin`, or `zw_tin`.
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
/// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `al_tin`, `am_tin`, `ao_tin`, `ar_cuit`, `au_abn`, `au_arn`, `aw_tin`, `az_tin`, `ba_tin`, `bb_tin`, `bd_bin`, `bf_ifu`, `bg_uic`, `bh_vat`, `bj_ifu`, `bo_tin`, `br_cnpj`, `br_cpf`, `bs_tin`, `by_tin`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `cd_nif`, `ch_uid`, `ch_vat`, `cl_tin`, `cm_niu`, `cn_tin`, `co_nit`, `cr_tin`, `cv_nif`, `de_stn`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `et_tin`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `gn_nif`, `hk_br`, `hr_oib`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kg_tin`, `kh_tin`, `kr_brn`, `kz_bin`, `la_tin`, `li_uid`, `li_vat`, `ma_vat`, `md_vat`, `me_pib`, `mk_vat`, `mr_nif`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `np_pan`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sn_ninea`, `sr_fin`, `sv_nit`, `th_vat`, `tj_tin`, `tr_tin`, `tw_vat`, `tz_vat`, `ua_vat`, `ug_tin`, `us_ein`, `uy_ruc`, `uz_tin`, `uz_vat`, `ve_rif`, `vn_tin`, `za_vat`, `zm_tin`, or `zw_tin`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTaxCalculationCustomerDetailsTaxIdsType {
    AdNrt,
    AeTrn,
    AlTin,
    AmTin,
    AoTin,
    ArCuit,
    AuAbn,
    AuArn,
    AwTin,
    AzTin,
    BaTin,
    BbTin,
    BdBin,
    BfIfu,
    BgUic,
    BhVat,
    BjIfu,
    BoTin,
    BrCnpj,
    BrCpf,
    BsTin,
    ByTin,
    CaBn,
    CaGstHst,
    CaPstBc,
    CaPstMb,
    CaPstSk,
    CaQst,
    CdNif,
    ChUid,
    ChVat,
    ClTin,
    CmNiu,
    CnTin,
    CoNit,
    CrTin,
    CvNif,
    DeStn,
    DoRcn,
    EcRuc,
    EgTin,
    EsCif,
    EtTin,
    EuOssVat,
    EuVat,
    GbVat,
    GeVat,
    GnNif,
    HkBr,
    HrOib,
    HuTin,
    IdNpwp,
    IlVat,
    InGst,
    IsVat,
    JpCn,
    JpRn,
    JpTrn,
    KePin,
    KgTin,
    KhTin,
    KrBrn,
    KzBin,
    LaTin,
    LiUid,
    LiVat,
    MaVat,
    MdVat,
    MePib,
    MkVat,
    MrNif,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NgTin,
    NoVat,
    NoVoec,
    NpPan,
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
    SnNinea,
    SrFin,
    SvNit,
    ThVat,
    TjTin,
    TrTin,
    TwVat,
    TzVat,
    UaVat,
    UgTin,
    UsEin,
    UyRuc,
    UzTin,
    UzVat,
    VeRif,
    VnTin,
    ZaVat,
    ZmTin,
    ZwTin,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTaxCalculationCustomerDetailsTaxIdsType {
    pub fn as_str(&self) -> &str {
        use CreateTaxCalculationCustomerDetailsTaxIdsType::*;
        match self {
            AdNrt => "ad_nrt",
            AeTrn => "ae_trn",
            AlTin => "al_tin",
            AmTin => "am_tin",
            AoTin => "ao_tin",
            ArCuit => "ar_cuit",
            AuAbn => "au_abn",
            AuArn => "au_arn",
            AwTin => "aw_tin",
            AzTin => "az_tin",
            BaTin => "ba_tin",
            BbTin => "bb_tin",
            BdBin => "bd_bin",
            BfIfu => "bf_ifu",
            BgUic => "bg_uic",
            BhVat => "bh_vat",
            BjIfu => "bj_ifu",
            BoTin => "bo_tin",
            BrCnpj => "br_cnpj",
            BrCpf => "br_cpf",
            BsTin => "bs_tin",
            ByTin => "by_tin",
            CaBn => "ca_bn",
            CaGstHst => "ca_gst_hst",
            CaPstBc => "ca_pst_bc",
            CaPstMb => "ca_pst_mb",
            CaPstSk => "ca_pst_sk",
            CaQst => "ca_qst",
            CdNif => "cd_nif",
            ChUid => "ch_uid",
            ChVat => "ch_vat",
            ClTin => "cl_tin",
            CmNiu => "cm_niu",
            CnTin => "cn_tin",
            CoNit => "co_nit",
            CrTin => "cr_tin",
            CvNif => "cv_nif",
            DeStn => "de_stn",
            DoRcn => "do_rcn",
            EcRuc => "ec_ruc",
            EgTin => "eg_tin",
            EsCif => "es_cif",
            EtTin => "et_tin",
            EuOssVat => "eu_oss_vat",
            EuVat => "eu_vat",
            GbVat => "gb_vat",
            GeVat => "ge_vat",
            GnNif => "gn_nif",
            HkBr => "hk_br",
            HrOib => "hr_oib",
            HuTin => "hu_tin",
            IdNpwp => "id_npwp",
            IlVat => "il_vat",
            InGst => "in_gst",
            IsVat => "is_vat",
            JpCn => "jp_cn",
            JpRn => "jp_rn",
            JpTrn => "jp_trn",
            KePin => "ke_pin",
            KgTin => "kg_tin",
            KhTin => "kh_tin",
            KrBrn => "kr_brn",
            KzBin => "kz_bin",
            LaTin => "la_tin",
            LiUid => "li_uid",
            LiVat => "li_vat",
            MaVat => "ma_vat",
            MdVat => "md_vat",
            MePib => "me_pib",
            MkVat => "mk_vat",
            MrNif => "mr_nif",
            MxRfc => "mx_rfc",
            MyFrp => "my_frp",
            MyItn => "my_itn",
            MySst => "my_sst",
            NgTin => "ng_tin",
            NoVat => "no_vat",
            NoVoec => "no_voec",
            NpPan => "np_pan",
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
            SnNinea => "sn_ninea",
            SrFin => "sr_fin",
            SvNit => "sv_nit",
            ThVat => "th_vat",
            TjTin => "tj_tin",
            TrTin => "tr_tin",
            TwVat => "tw_vat",
            TzVat => "tz_vat",
            UaVat => "ua_vat",
            UgTin => "ug_tin",
            UsEin => "us_ein",
            UyRuc => "uy_ruc",
            UzTin => "uz_tin",
            UzVat => "uz_vat",
            VeRif => "ve_rif",
            VnTin => "vn_tin",
            ZaVat => "za_vat",
            ZmTin => "zm_tin",
            ZwTin => "zw_tin",
            Unknown(v) => v,
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
            "al_tin" => Ok(AlTin),
            "am_tin" => Ok(AmTin),
            "ao_tin" => Ok(AoTin),
            "ar_cuit" => Ok(ArCuit),
            "au_abn" => Ok(AuAbn),
            "au_arn" => Ok(AuArn),
            "aw_tin" => Ok(AwTin),
            "az_tin" => Ok(AzTin),
            "ba_tin" => Ok(BaTin),
            "bb_tin" => Ok(BbTin),
            "bd_bin" => Ok(BdBin),
            "bf_ifu" => Ok(BfIfu),
            "bg_uic" => Ok(BgUic),
            "bh_vat" => Ok(BhVat),
            "bj_ifu" => Ok(BjIfu),
            "bo_tin" => Ok(BoTin),
            "br_cnpj" => Ok(BrCnpj),
            "br_cpf" => Ok(BrCpf),
            "bs_tin" => Ok(BsTin),
            "by_tin" => Ok(ByTin),
            "ca_bn" => Ok(CaBn),
            "ca_gst_hst" => Ok(CaGstHst),
            "ca_pst_bc" => Ok(CaPstBc),
            "ca_pst_mb" => Ok(CaPstMb),
            "ca_pst_sk" => Ok(CaPstSk),
            "ca_qst" => Ok(CaQst),
            "cd_nif" => Ok(CdNif),
            "ch_uid" => Ok(ChUid),
            "ch_vat" => Ok(ChVat),
            "cl_tin" => Ok(ClTin),
            "cm_niu" => Ok(CmNiu),
            "cn_tin" => Ok(CnTin),
            "co_nit" => Ok(CoNit),
            "cr_tin" => Ok(CrTin),
            "cv_nif" => Ok(CvNif),
            "de_stn" => Ok(DeStn),
            "do_rcn" => Ok(DoRcn),
            "ec_ruc" => Ok(EcRuc),
            "eg_tin" => Ok(EgTin),
            "es_cif" => Ok(EsCif),
            "et_tin" => Ok(EtTin),
            "eu_oss_vat" => Ok(EuOssVat),
            "eu_vat" => Ok(EuVat),
            "gb_vat" => Ok(GbVat),
            "ge_vat" => Ok(GeVat),
            "gn_nif" => Ok(GnNif),
            "hk_br" => Ok(HkBr),
            "hr_oib" => Ok(HrOib),
            "hu_tin" => Ok(HuTin),
            "id_npwp" => Ok(IdNpwp),
            "il_vat" => Ok(IlVat),
            "in_gst" => Ok(InGst),
            "is_vat" => Ok(IsVat),
            "jp_cn" => Ok(JpCn),
            "jp_rn" => Ok(JpRn),
            "jp_trn" => Ok(JpTrn),
            "ke_pin" => Ok(KePin),
            "kg_tin" => Ok(KgTin),
            "kh_tin" => Ok(KhTin),
            "kr_brn" => Ok(KrBrn),
            "kz_bin" => Ok(KzBin),
            "la_tin" => Ok(LaTin),
            "li_uid" => Ok(LiUid),
            "li_vat" => Ok(LiVat),
            "ma_vat" => Ok(MaVat),
            "md_vat" => Ok(MdVat),
            "me_pib" => Ok(MePib),
            "mk_vat" => Ok(MkVat),
            "mr_nif" => Ok(MrNif),
            "mx_rfc" => Ok(MxRfc),
            "my_frp" => Ok(MyFrp),
            "my_itn" => Ok(MyItn),
            "my_sst" => Ok(MySst),
            "ng_tin" => Ok(NgTin),
            "no_vat" => Ok(NoVat),
            "no_voec" => Ok(NoVoec),
            "np_pan" => Ok(NpPan),
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
            "sn_ninea" => Ok(SnNinea),
            "sr_fin" => Ok(SrFin),
            "sv_nit" => Ok(SvNit),
            "th_vat" => Ok(ThVat),
            "tj_tin" => Ok(TjTin),
            "tr_tin" => Ok(TrTin),
            "tw_vat" => Ok(TwVat),
            "tz_vat" => Ok(TzVat),
            "ua_vat" => Ok(UaVat),
            "ug_tin" => Ok(UgTin),
            "us_ein" => Ok(UsEin),
            "uy_ruc" => Ok(UyRuc),
            "uz_tin" => Ok(UzTin),
            "uz_vat" => Ok(UzVat),
            "ve_rif" => Ok(VeRif),
            "vn_tin" => Ok(VnTin),
            "za_vat" => Ok(ZaVat),
            "zm_tin" => Ok(ZmTin),
            "zw_tin" => Ok(ZwTin),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTaxCalculationCustomerDetailsTaxIdsType"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Overrides the tax calculation result to allow you to not collect tax from your customer.
/// Use this if you've manually checked your customer's tax exemptions.
/// Prefer providing the customer's `tax_ids` where possible, which automatically determines whether `reverse_charge` applies.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTaxCalculationCustomerDetailsTaxabilityOverride {
    CustomerExempt,
    None,
    ReverseCharge,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTaxCalculationCustomerDetailsTaxabilityOverride {
    pub fn as_str(&self) -> &str {
        use CreateTaxCalculationCustomerDetailsTaxabilityOverride::*;
        match self {
            CustomerExempt => "customer_exempt",
            None => "none",
            ReverseCharge => "reverse_charge",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTaxCalculationCustomerDetailsTaxabilityOverride {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxCalculationCustomerDetailsTaxabilityOverride::*;
        match s {
            "customer_exempt" => Ok(CustomerExempt),
            "none" => Ok(None),
            "reverse_charge" => Ok(ReverseCharge),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTaxCalculationCustomerDetailsTaxabilityOverride"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// A list of items the customer is purchasing.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxCalculationLineItems {
    /// A positive integer representing the line item's total price in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes are calculated on top of this amount.
    pub amount: i64,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
            metadata: None,
            product: None,
            quantity: None,
            reference: None,
            tax_behavior: None,
            tax_code: None,
        }
    }
}
/// Specifies whether the `amount` includes taxes. Defaults to `exclusive`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTaxCalculationLineItemsTaxBehavior {
    Exclusive,
    Inclusive,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTaxCalculationLineItemsTaxBehavior {
    pub fn as_str(&self) -> &str {
        use CreateTaxCalculationLineItemsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTaxCalculationLineItemsTaxBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxCalculationLineItemsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTaxCalculationLineItemsTaxBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
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
    /// If not provided, the default shipping tax code from your [Tax Settings](https://dashboard.stripe.com/settings/tax) is used.
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTaxCalculationShippingCostTaxBehavior {
    Exclusive,
    Inclusive,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTaxCalculationShippingCostTaxBehavior {
    pub fn as_str(&self) -> &str {
        use CreateTaxCalculationShippingCostTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTaxCalculationShippingCostTaxBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxCalculationShippingCostTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTaxCalculationShippingCostTaxBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Calculates tax based on the input and returns a Tax `Calculation` object.
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
