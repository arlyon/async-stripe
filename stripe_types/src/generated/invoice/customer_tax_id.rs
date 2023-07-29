#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomerTaxId {
    /// The type of the tax ID, one of `eu_vat`, `br_cnpj`, `br_cpf`, `eu_oss_vat`, `gb_vat`, `nz_gst`, `au_abn`, `au_arn`, `in_gst`, `no_vat`, `za_vat`, `ch_vat`, `mx_rfc`, `sg_uen`, `ru_inn`, `ru_kpp`, `ca_bn`, `hk_br`, `es_cif`, `tw_vat`, `th_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `li_uid`, `my_itn`, `us_ein`, `kr_brn`, `ca_qst`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `my_sst`, `sg_gst`, `ae_trn`, `cl_tin`, `sa_vat`, `id_npwp`, `my_frp`, `il_vat`, `ge_vat`, `ua_vat`, `is_vat`, `bg_uic`, `hu_tin`, `si_tin`, `ke_pin`, `tr_tin`, `eg_tin`, `ph_tin`, or `unknown`.
    #[serde(rename = "type")]
    pub type_: CustomerTaxIdType,
    /// The value of the tax ID.
    pub value: Option<String>,
}
/// The type of the tax ID, one of `eu_vat`, `br_cnpj`, `br_cpf`, `eu_oss_vat`, `gb_vat`, `nz_gst`, `au_abn`, `au_arn`, `in_gst`, `no_vat`, `za_vat`, `ch_vat`, `mx_rfc`, `sg_uen`, `ru_inn`, `ru_kpp`, `ca_bn`, `hk_br`, `es_cif`, `tw_vat`, `th_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `li_uid`, `my_itn`, `us_ein`, `kr_brn`, `ca_qst`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `my_sst`, `sg_gst`, `ae_trn`, `cl_tin`, `sa_vat`, `id_npwp`, `my_frp`, `il_vat`, `ge_vat`, `ua_vat`, `is_vat`, `bg_uic`, `hu_tin`, `si_tin`, `ke_pin`, `tr_tin`, `eg_tin`, `ph_tin`, or `unknown`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CustomerTaxIdType {
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

impl CustomerTaxIdType {
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

impl std::str::FromStr for CustomerTaxIdType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ae_trn" => Ok(Self::AeTrn),
            "au_abn" => Ok(Self::AuAbn),
            "au_arn" => Ok(Self::AuArn),
            "bg_uic" => Ok(Self::BgUic),
            "br_cnpj" => Ok(Self::BrCnpj),
            "br_cpf" => Ok(Self::BrCpf),
            "ca_bn" => Ok(Self::CaBn),
            "ca_gst_hst" => Ok(Self::CaGstHst),
            "ca_pst_bc" => Ok(Self::CaPstBc),
            "ca_pst_mb" => Ok(Self::CaPstMb),
            "ca_pst_sk" => Ok(Self::CaPstSk),
            "ca_qst" => Ok(Self::CaQst),
            "ch_vat" => Ok(Self::ChVat),
            "cl_tin" => Ok(Self::ClTin),
            "eg_tin" => Ok(Self::EgTin),
            "es_cif" => Ok(Self::EsCif),
            "eu_oss_vat" => Ok(Self::EuOssVat),
            "eu_vat" => Ok(Self::EuVat),
            "gb_vat" => Ok(Self::GbVat),
            "ge_vat" => Ok(Self::GeVat),
            "hk_br" => Ok(Self::HkBr),
            "hu_tin" => Ok(Self::HuTin),
            "id_npwp" => Ok(Self::IdNpwp),
            "il_vat" => Ok(Self::IlVat),
            "in_gst" => Ok(Self::InGst),
            "is_vat" => Ok(Self::IsVat),
            "jp_cn" => Ok(Self::JpCn),
            "jp_rn" => Ok(Self::JpRn),
            "jp_trn" => Ok(Self::JpTrn),
            "ke_pin" => Ok(Self::KePin),
            "kr_brn" => Ok(Self::KrBrn),
            "li_uid" => Ok(Self::LiUid),
            "mx_rfc" => Ok(Self::MxRfc),
            "my_frp" => Ok(Self::MyFrp),
            "my_itn" => Ok(Self::MyItn),
            "my_sst" => Ok(Self::MySst),
            "no_vat" => Ok(Self::NoVat),
            "nz_gst" => Ok(Self::NzGst),
            "ph_tin" => Ok(Self::PhTin),
            "ru_inn" => Ok(Self::RuInn),
            "ru_kpp" => Ok(Self::RuKpp),
            "sa_vat" => Ok(Self::SaVat),
            "sg_gst" => Ok(Self::SgGst),
            "sg_uen" => Ok(Self::SgUen),
            "si_tin" => Ok(Self::SiTin),
            "th_vat" => Ok(Self::ThVat),
            "tr_tin" => Ok(Self::TrTin),
            "tw_vat" => Ok(Self::TwVat),
            "ua_vat" => Ok(Self::UaVat),
            "unknown" => Ok(Self::Unknown),
            "us_ein" => Ok(Self::UsEin),
            "za_vat" => Ok(Self::ZaVat),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CustomerTaxIdType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerTaxIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CustomerTaxIdType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerTaxIdType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CustomerTaxIdType"))
    }
}
