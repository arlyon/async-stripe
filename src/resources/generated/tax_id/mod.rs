/// You can add one or multiple tax IDs to a [customer](https://stripe.com/docs/api/customers).
/// A customer's tax IDs are displayed on invoices and credit notes issued for the customer.
///
/// Related guide: [Customer Tax Identification Numbers](https://stripe.com/docs/billing/taxes/tax-ids).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxId {
    /// Two-letter ISO code representing the country of the tax ID.
    pub country: Option<String>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// ID of the customer.
    pub customer: Option<crate::Expandable<crate::customer::Customer>>,
    /// Unique identifier for the object.
    pub id: crate::tax_id::TaxIdId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TaxIdObject,
    /// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
    ///
    /// Note that some legacy tax IDs have type `unknown`.
    #[serde(rename = "type")]
    pub type_: TaxIdType,
    /// Value of the tax ID.
    pub value: String,
    /// Tax ID verification information.
    pub verification: Option<crate::tax_id::verification::Verification>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxId {
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
pub enum TaxIdObject {
    TaxId,
}

impl TaxIdObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TaxId => "tax_id",
        }
    }
}

impl AsRef<str> for TaxIdObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxIdObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
///
/// Note that some legacy tax IDs have type `unknown`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum TaxIdType {
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
    Unknown,
    UsEin,
    ZaVat,
}

impl TaxIdType {
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
            Self::Unknown => "unknown",
            Self::UsEin => "us_ein",
            Self::ZaVat => "za_vat",
        }
    }
}

impl AsRef<str> for TaxIdType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for TaxId {
    type Id = crate::tax_id::TaxIdId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(TaxIdId, "txi_");
pub mod deleted;
pub mod requests;
pub use deleted::DeletedTaxId;
pub mod verification;
pub use verification::Verification;
