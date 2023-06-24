use stripe::{Client, Response};

impl stripe_core::tax_id::TaxId {
    /// Creates a new `TaxID` object for a customer.
    pub fn create(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
        params: CreateTaxId,
    ) -> Response<stripe_core::tax_id::TaxId> {
        client.send_form(
            &format!("/customers/{customer}/tax_ids", customer = customer),
            params,
            http_types::Method::Post,
        )
    }
    /// Retrieves the `TaxID` object with the given identifier.
    pub fn retrieve(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
        id: &str,
        params: RetrieveTaxId,
    ) -> Response<stripe_core::tax_id::TaxId> {
        client.get_query(
            &format!("/customers/{customer}/tax_ids/{id}", customer = customer, id = id),
            params,
        )
    }
    /// Returns a list of tax IDs for a customer.
    pub fn list(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
        params: ListTaxId,
    ) -> Response<stripe_types::List<stripe_core::tax_id::TaxId>> {
        client.get_query(&format!("/customers/{customer}/tax_ids", customer = customer), params)
    }
    /// Deletes an existing `TaxID` object.
    pub fn delete(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
        id: &str,
    ) -> Response<stripe_core::tax_id::DeletedTaxId> {
        client.send(
            &format!("/customers/{customer}/tax_ids/{id}", customer = customer, id = id),
            http_types::Method::Delete,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxId<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: CreateTaxIdType,
    /// Value of the tax ID.
    pub value: &'a str,
}
impl<'a> CreateTaxId<'a> {
    pub fn new(type_: CreateTaxIdType, value: &'a str) -> Self {
        Self { expand: Default::default(), type_, value }
    }
}
/// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateTaxIdType {
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
    UsEin,
    ZaVat,
}

impl CreateTaxIdType {
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
            Self::UsEin => "us_ein",
            Self::ZaVat => "za_vat",
        }
    }
}

impl std::str::FromStr for CreateTaxIdType {
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
            "us_ein" => Ok(Self::UsEin),
            "za_vat" => Ok(Self::ZaVat),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxIdType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateTaxIdType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTaxId<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTaxId<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListTaxId<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
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
    pub starting_after: Option<String>,
}
impl<'a> ListTaxId<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
