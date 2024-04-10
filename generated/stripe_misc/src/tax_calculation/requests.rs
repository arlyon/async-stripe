#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListLineItemsTaxCalculation<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListLineItemsTaxCalculation<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListLineItemsTaxCalculation<'a> {
    /// Retrieves the line items of a persisted tax calculation as a collection.
    pub fn send(
        &self,
        client: &stripe::Client,
        calculation: &stripe_misc::TaxCalculationId,
    ) -> stripe::Response<stripe_types::List<stripe_misc::TaxCalculationLineItem>> {
        client.get_query(&format!("/tax/calculations/{calculation}/line_items"), self)
    }
    pub fn paginate(
        self,
        calculation: &stripe_misc::TaxCalculationId,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_misc::TaxCalculationLineItem>> {
        stripe::ListPaginator::from_list_params(
            &format!("/tax/calculations/{calculation}/line_items"),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxCalculation<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of an existing customer to use for this calculation.
    /// If provided, the customer's address and tax IDs are copied to `customer_details`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// Details about the customer, including address and tax IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_details: Option<CreateTaxCalculationCustomerDetails<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A list of items the customer is purchasing.
    pub line_items: &'a [CreateTaxCalculationLineItems<'a>],
    /// Shipping cost details to be used for the calculation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<CreateTaxCalculationShippingCost<'a>>,
    /// Timestamp of date at which the tax rules and rates in effect applies for the calculation.
    /// Measured in seconds since the Unix epoch.
    /// Can be up to 48 hours in the past, and up to 48 hours in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_date: Option<stripe_types::Timestamp>,
}
impl<'a> CreateTaxCalculation<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        line_items: &'a [CreateTaxCalculationLineItems<'a>],
    ) -> Self {
        Self {
            currency,
            customer: None,
            customer_details: None,
            expand: None,
            line_items,
            shipping_cost: None,
            tax_date: None,
        }
    }
}
/// Details about the customer, including address and tax IDs.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTaxCalculationCustomerDetails<'a> {
    /// The customer's postal address (for example, home or business location).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateTaxCalculationCustomerDetailsAddress<'a>>,
    /// The type of customer address provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_source: Option<CreateTaxCalculationCustomerDetailsAddressSource>,
    /// The customer's IP address (IPv4 or IPv6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,
    /// The customer's tax IDs.
    /// Stripe Tax might consider a transaction with applicable tax IDs to be B2B, which might affect the tax calculation result.
    /// Stripe Tax doesn't validate tax IDs for correctness.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<&'a [CreateTaxCalculationCustomerDetailsTaxIds<'a>]>,
    /// Overrides the tax calculation result to allow you to not collect tax from your customer.
    /// Use this if you've manually checked your customer's tax exemptions.
    /// Prefer providing the customer's `tax_ids` where possible, which automatically determines whether `reverse_charge` applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxability_override: Option<CreateTaxCalculationCustomerDetailsTaxabilityOverride>,
}
impl<'a> CreateTaxCalculationCustomerDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's postal address (for example, home or business location).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxCalculationCustomerDetailsAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: &'a str,
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
    /// We recommend sending [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code value when possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreateTaxCalculationCustomerDetailsAddress<'a> {
    pub fn new(country: &'a str) -> Self {
        Self { city: None, country, line1: None, line2: None, postal_code: None, state: None }
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxCalculationCustomerDetailsAddressSource::*;
        match s {
            "billing" => Ok(Billing),
            "shipping" => Ok(Shipping),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxCalculationCustomerDetailsTaxIds<'a> {
    /// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: CreateTaxCalculationCustomerDetailsTaxIdsType,
    /// Value of the tax ID.
    pub value: &'a str,
}
impl<'a> CreateTaxCalculationCustomerDetailsTaxIds<'a> {
    pub fn new(type_: CreateTaxCalculationCustomerDetailsTaxIdsType, value: &'a str) -> Self {
        Self { type_, value }
    }
}
/// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTaxCalculationCustomerDetailsTaxIdsType {
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

impl std::str::FromStr for CreateTaxCalculationCustomerDetailsTaxIdsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxCalculationCustomerDetailsTaxIdsType::*;
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
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxCalculationCustomerDetailsTaxabilityOverride::*;
        match s {
            "customer_exempt" => Ok(CustomerExempt),
            "none" => Ok(None),
            "reverse_charge" => Ok(ReverseCharge),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxCalculationLineItems<'a> {
    /// A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) representing the line item's total price.
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes are calculated on top of this amount.
    pub amount: i64,
    /// If provided, the product's `tax_code` will be used as the line item's `tax_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
    /// The number of units of the item being purchased.
    /// Used to calculate the per-unit price from the total `amount` for the line.
    /// For example, if `amount=100` and `quantity=4`, the calculated unit price is 25.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A custom identifier for this line item, which must be unique across the line items in the calculation.
    /// The reference helps identify each line item in exported [tax reports](https://stripe.com/docs/tax/reports).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<&'a str>,
    /// Specifies whether the `amount` includes taxes. Defaults to `exclusive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateTaxCalculationLineItemsTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID to use for this line item.
    /// If not provided, we will use the tax code from the provided `product` param.
    /// If neither `tax_code` nor `product` is provided, we will use the default tax code from your Tax Settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
}
impl<'a> CreateTaxCalculationLineItems<'a> {
    pub fn new(amount: i64) -> Self {
        Self {
            amount,
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxCalculationLineItemsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            _ => Err(()),
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
/// Shipping cost details to be used for the calculation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTaxCalculationShippingCost<'a> {
    /// A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) representing the shipping charge.
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes are calculated on top of this amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// If provided, the [shipping rate](https://stripe.com/docs/api/shipping_rates/object)'s `amount`, `tax_code` and `tax_behavior` are used.
    /// If you provide a shipping rate, then you cannot pass the `amount`, `tax_code`, or `tax_behavior` parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<&'a str>,
    /// Specifies whether the `amount` includes taxes.
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    /// Defaults to `exclusive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateTaxCalculationShippingCostTaxBehavior>,
    /// The [tax code](https://stripe.com/docs/tax/tax-categories) used to calculate tax on shipping.
    /// If not provided, the default shipping tax code from your [Tax Settings](/settings/tax) is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
}
impl<'a> CreateTaxCalculationShippingCost<'a> {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxCalculationShippingCostTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            _ => Err(()),
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
impl<'a> CreateTaxCalculation<'a> {
    /// Calculates tax based on input and returns a Tax `Calculation` object.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_misc::TaxCalculation> {
        client.send_form("/tax/calculations", self, http_types::Method::Post)
    }
}
