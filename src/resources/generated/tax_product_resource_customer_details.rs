// ======================================
// This file was automatically generated.
// ======================================

use crate::resources::{TaxProductResourcePostalAddress};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TaxProductResourceCustomerDetails".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceCustomerDetails {

    /// The customer's postal address (for example, home or business location).
    pub address: Option<TaxProductResourcePostalAddress>,

    /// The type of customer address provided.
    pub address_source: Option<TaxProductResourceCustomerDetailsAddressSource>,

    /// The customer's IP address (IPv4 or IPv6).
    pub ip_address: Option<String>,

    /// The customer's tax IDs (for example, EU VAT numbers).
    pub tax_ids: Vec<TaxProductResourceCustomerDetailsResourceTaxId>,

    /// The taxability override used for taxation.
    pub taxability_override: TaxProductResourceCustomerDetailsTaxabilityOverride,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceCustomerDetailsResourceTaxId {

    /// The type of the tax ID, one of `ad_nrt`, `ar_cuit`, `eu_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eu_oss_vat`, `hr_oib`, `pe_ruc`, `ro_tin`, `rs_pib`, `sv_nit`, `uy_ruc`, `ve_rif`, `vn_tin`, `gb_vat`, `nz_gst`, `au_abn`, `au_arn`, `in_gst`, `no_vat`, `no_voec`, `za_vat`, `ch_vat`, `mx_rfc`, `sg_uen`, `ru_inn`, `ru_kpp`, `ca_bn`, `hk_br`, `es_cif`, `tw_vat`, `th_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `li_uid`, `my_itn`, `us_ein`, `kr_brn`, `ca_qst`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `my_sst`, `sg_gst`, `ae_trn`, `cl_tin`, `sa_vat`, `id_npwp`, `my_frp`, `il_vat`, `ge_vat`, `ua_vat`, `is_vat`, `bg_uic`, `hu_tin`, `si_tin`, `ke_pin`, `tr_tin`, `eg_tin`, `ph_tin`, `bh_vat`, `kz_bin`, `ng_tin`, `om_vat`, `de_stn`, `ch_uid`, or `unknown`.
    #[serde(rename = "type")]
    pub type_: TaxProductResourceCustomerDetailsResourceTaxIdType,

    /// The value of the tax ID.
    pub value: String,
}

/// An enum representing the possible values of an `TaxProductResourceCustomerDetails`'s `address_source` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceCustomerDetailsAddressSource {
    Billing,
    Shipping,
}

impl TaxProductResourceCustomerDetailsAddressSource {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductResourceCustomerDetailsAddressSource::Billing => "billing",
            TaxProductResourceCustomerDetailsAddressSource::Shipping => "shipping",
        }
    }
}

impl AsRef<str> for TaxProductResourceCustomerDetailsAddressSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceCustomerDetailsAddressSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductResourceCustomerDetailsAddressSource {
    fn default() -> Self {
        Self::Billing
    }
}

/// An enum representing the possible values of an `TaxProductResourceCustomerDetailsResourceTaxId`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceCustomerDetailsResourceTaxIdType {
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
    ChUid,
    ChVat,
    ClTin,
    CnTin,
    CoNit,
    CrTin,
    DeStn,
    DoRcn,
    EcRuc,
    EgTin,
    EsCif,
    EuOssVat,
    EuVat,
    GbVat,
    GeVat,
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
    Unknown,
    UsEin,
    UyRuc,
    VeRif,
    VnTin,
    ZaVat,
}

impl TaxProductResourceCustomerDetailsResourceTaxIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductResourceCustomerDetailsResourceTaxIdType::AdNrt => "ad_nrt",
            TaxProductResourceCustomerDetailsResourceTaxIdType::AeTrn => "ae_trn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ArCuit => "ar_cuit",
            TaxProductResourceCustomerDetailsResourceTaxIdType::AuAbn => "au_abn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::AuArn => "au_arn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BgUic => "bg_uic",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BhVat => "bh_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BoTin => "bo_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BrCnpj => "br_cnpj",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BrCpf => "br_cpf",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CaBn => "ca_bn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CaGstHst => "ca_gst_hst",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CaPstBc => "ca_pst_bc",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CaPstMb => "ca_pst_mb",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CaPstSk => "ca_pst_sk",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CaQst => "ca_qst",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ChUid => "ch_uid",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ChVat => "ch_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ClTin => "cl_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CnTin => "cn_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CoNit => "co_nit",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CrTin => "cr_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::DeStn => "de_stn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::DoRcn => "do_rcn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::EcRuc => "ec_ruc",
            TaxProductResourceCustomerDetailsResourceTaxIdType::EgTin => "eg_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::EsCif => "es_cif",
            TaxProductResourceCustomerDetailsResourceTaxIdType::EuOssVat => "eu_oss_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::EuVat => "eu_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::GbVat => "gb_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::GeVat => "ge_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::HkBr => "hk_br",
            TaxProductResourceCustomerDetailsResourceTaxIdType::HrOib => "hr_oib",
            TaxProductResourceCustomerDetailsResourceTaxIdType::HuTin => "hu_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::IdNpwp => "id_npwp",
            TaxProductResourceCustomerDetailsResourceTaxIdType::IlVat => "il_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::InGst => "in_gst",
            TaxProductResourceCustomerDetailsResourceTaxIdType::IsVat => "is_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::JpCn => "jp_cn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::JpRn => "jp_rn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::JpTrn => "jp_trn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::KePin => "ke_pin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::KrBrn => "kr_brn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::KzBin => "kz_bin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::LiUid => "li_uid",
            TaxProductResourceCustomerDetailsResourceTaxIdType::MxRfc => "mx_rfc",
            TaxProductResourceCustomerDetailsResourceTaxIdType::MyFrp => "my_frp",
            TaxProductResourceCustomerDetailsResourceTaxIdType::MyItn => "my_itn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::MySst => "my_sst",
            TaxProductResourceCustomerDetailsResourceTaxIdType::NgTin => "ng_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::NoVat => "no_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::NoVoec => "no_voec",
            TaxProductResourceCustomerDetailsResourceTaxIdType::NzGst => "nz_gst",
            TaxProductResourceCustomerDetailsResourceTaxIdType::OmVat => "om_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::PeRuc => "pe_ruc",
            TaxProductResourceCustomerDetailsResourceTaxIdType::PhTin => "ph_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::RoTin => "ro_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::RsPib => "rs_pib",
            TaxProductResourceCustomerDetailsResourceTaxIdType::RuInn => "ru_inn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::RuKpp => "ru_kpp",
            TaxProductResourceCustomerDetailsResourceTaxIdType::SaVat => "sa_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::SgGst => "sg_gst",
            TaxProductResourceCustomerDetailsResourceTaxIdType::SgUen => "sg_uen",
            TaxProductResourceCustomerDetailsResourceTaxIdType::SiTin => "si_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::SvNit => "sv_nit",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ThVat => "th_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::TrTin => "tr_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::TwVat => "tw_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::UaVat => "ua_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::Unknown => "unknown",
            TaxProductResourceCustomerDetailsResourceTaxIdType::UsEin => "us_ein",
            TaxProductResourceCustomerDetailsResourceTaxIdType::UyRuc => "uy_ruc",
            TaxProductResourceCustomerDetailsResourceTaxIdType::VeRif => "ve_rif",
            TaxProductResourceCustomerDetailsResourceTaxIdType::VnTin => "vn_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ZaVat => "za_vat",
        }
    }
}

impl AsRef<str> for TaxProductResourceCustomerDetailsResourceTaxIdType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceCustomerDetailsResourceTaxIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductResourceCustomerDetailsResourceTaxIdType {
    fn default() -> Self {
        Self::AdNrt
    }
}

/// An enum representing the possible values of an `TaxProductResourceCustomerDetails`'s `taxability_override` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceCustomerDetailsTaxabilityOverride {
    CustomerExempt,
    None,
    ReverseCharge,
}

impl TaxProductResourceCustomerDetailsTaxabilityOverride {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductResourceCustomerDetailsTaxabilityOverride::CustomerExempt => "customer_exempt",
            TaxProductResourceCustomerDetailsTaxabilityOverride::None => "none",
            TaxProductResourceCustomerDetailsTaxabilityOverride::ReverseCharge => "reverse_charge",
        }
    }
}

impl AsRef<str> for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn default() -> Self {
        Self::CustomerExempt
    }
}
