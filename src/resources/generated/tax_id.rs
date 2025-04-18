// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{TaxIdId};
use crate::params::{Deleted, Expand, Expandable, List, Object, Paginable, Timestamp};
use crate::resources::{Account, Application, Customer};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "tax_id".
///
/// For more details see <https://stripe.com/docs/api/tax_ids/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxId {
    /// Unique identifier for the object.
    pub id: TaxIdId,

    /// Two-letter ISO code representing the country of the tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// ID of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// The account or customer the tax ID belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<TaxIDsOwner>,

    /// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `al_tin`, `am_tin`, `ao_tin`, `ar_cuit`, `au_abn`, `au_arn`, `ba_tin`, `bb_tin`, `bg_uic`, `bh_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `bs_tin`, `by_tin`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `cd_nif`, `ch_uid`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `de_stn`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `gn_nif`, `hk_br`, `hr_oib`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kh_tin`, `kr_brn`, `kz_bin`, `li_uid`, `li_vat`, `ma_vat`, `md_vat`, `me_pib`, `mk_vat`, `mr_nif`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `np_pan`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sn_ninea`, `sr_fin`, `sv_nit`, `th_vat`, `tj_tin`, `tr_tin`, `tw_vat`, `tz_vat`, `ua_vat`, `ug_tin`, `us_ein`, `uy_ruc`, `uz_tin`, `uz_vat`, `ve_rif`, `vn_tin`, `za_vat`, `zm_tin`, or `zw_tin`.
    ///
    /// Note that some legacy tax IDs have type `unknown`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<TaxIdType>,

    /// Value of the tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Tax ID verification information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<TaxIdVerification>,
}

impl TaxId {

    /// Returns a list of tax IDs.
pub fn list(client: &Client, params: &ListTaxIds<'_>) -> Response<List<TaxId>> {
   client.get_query("/tax_ids", params)
}


    /// Creates a new account or customer `tax_id` object.
    pub fn create(client: &Client, params: CreateTaxId<'_>) -> Response<TaxId> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/tax_ids", &params)
    }

    /// Retrieves an account or customer `tax_id` object.
    pub fn retrieve(client: &Client, id: &TaxIdId, expand: &[&str]) -> Response<TaxId> {
        client.get_query(&format!("/tax_ids/{}", id), Expand { expand })
    }

    /// Deletes an existing account or customer `tax_id` object.
    pub fn delete(client: &Client, id: &TaxIdId) -> Response<Deleted<TaxIdId>> {
        client.delete(&format!("/tax_ids/{}", id))
    }
}

impl Object for TaxId {
    type Id = TaxIdId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax_id"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxIDsOwner {

    /// The account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Expandable<Account>>,

    /// The Connect Application being referenced when `type` is `application`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Expandable<Application>>,

    /// The customer being referenced when `type` is `customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// Type of owner referenced.
    #[serde(rename = "type")]
    pub type_: TaxIDsOwnerType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxIdVerification {

    /// Verification status, one of `pending`, `verified`, `unverified`, or `unavailable`.
    pub status: TaxIdVerificationStatus,

    /// Verified address.
    pub verified_address: Option<String>,

    /// Verified name.
    pub verified_name: Option<String>,
}

/// The parameters for `TaxId::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateTaxId<'a> {

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The account or customer the tax ID belongs to.
    ///
    /// Defaults to `owner[type]=self`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<CreateTaxIdOwner>,

    /// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `al_tin`, `am_tin`, `ao_tin`, `ar_cuit`, `au_abn`, `au_arn`, `ba_tin`, `bb_tin`, `bg_uic`, `bh_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `bs_tin`, `by_tin`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `cd_nif`, `ch_uid`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `de_stn`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `gn_nif`, `hk_br`, `hr_oib`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kh_tin`, `kr_brn`, `kz_bin`, `li_uid`, `li_vat`, `ma_vat`, `md_vat`, `me_pib`, `mk_vat`, `mr_nif`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `np_pan`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sn_ninea`, `sr_fin`, `sv_nit`, `th_vat`, `tj_tin`, `tr_tin`, `tw_vat`, `tz_vat`, `ua_vat`, `ug_tin`, `us_ein`, `uy_ruc`, `uz_tin`, `uz_vat`, `ve_rif`, `vn_tin`, `za_vat`, `zm_tin`, or `zw_tin`.
    #[serde(rename = "type")]
    pub type_: TaxIdTypeFilter,

    /// Value of the tax ID.
    pub value: &'a str,
}

impl<'a> CreateTaxId<'a> {
    pub fn new(type_: TaxIdTypeFilter, value: &'a str) -> Self {
        CreateTaxId {
            expand: Default::default(),
            owner: Default::default(),
            type_,
            value,
        }
    }
}

/// The parameters for `TaxId::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListTaxIds<'a> {

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<TaxIdId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// The account or customer the tax ID belongs to.
    ///
    /// Defaults to `owner[type]=self`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<ListTaxIdsOwner>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<TaxIdId>,
}

impl<'a> ListTaxIds<'a> {
    pub fn new() -> Self {
        ListTaxIds {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            owner: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListTaxIds<'_> {
    type O = TaxId;
    fn set_last(&mut self, item: Self::O) {
                self.starting_after = Some(item.id());
            }}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTaxIdOwner {

    /// Account the tax ID belongs to.
    ///
    /// Required when `type=account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Customer the tax ID belongs to.
    ///
    /// Required when `type=customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// Type of owner referenced.
    #[serde(rename = "type")]
    pub type_: CreateTaxIdOwnerType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ListTaxIdsOwner {

    /// Account the tax ID belongs to.
    ///
    /// Required when `type=account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Customer the tax ID belongs to.
    ///
    /// Required when `type=customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// Type of owner referenced.
    #[serde(rename = "type")]
    pub type_: ListTaxIdsOwnerType,
}

/// An enum representing the possible values of an `CreateTaxIdOwner`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateTaxIdOwnerType {
    Account,
    Application,
    Customer,
    #[serde(rename = "self")]
    Self_,
}

impl CreateTaxIdOwnerType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateTaxIdOwnerType::Account => "account",
            CreateTaxIdOwnerType::Application => "application",
            CreateTaxIdOwnerType::Customer => "customer",
            CreateTaxIdOwnerType::Self_ => "self",
        }
    }
}

impl AsRef<str> for CreateTaxIdOwnerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxIdOwnerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateTaxIdOwnerType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `ListTaxIdsOwner`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ListTaxIdsOwnerType {
    Account,
    Application,
    Customer,
    #[serde(rename = "self")]
    Self_,
}

impl ListTaxIdsOwnerType {
    pub fn as_str(self) -> &'static str {
        match self {
            ListTaxIdsOwnerType::Account => "account",
            ListTaxIdsOwnerType::Application => "application",
            ListTaxIdsOwnerType::Customer => "customer",
            ListTaxIdsOwnerType::Self_ => "self",
        }
    }
}

impl AsRef<str> for ListTaxIdsOwnerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTaxIdsOwnerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ListTaxIdsOwnerType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `TaxIDsOwner`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxIDsOwnerType {
    Account,
    Application,
    Customer,
    #[serde(rename = "self")]
    Self_,
}

impl TaxIDsOwnerType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxIDsOwnerType::Account => "account",
            TaxIDsOwnerType::Application => "application",
            TaxIDsOwnerType::Customer => "customer",
            TaxIDsOwnerType::Self_ => "self",
        }
    }
}

impl AsRef<str> for TaxIDsOwnerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxIDsOwnerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxIDsOwnerType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `TaxId`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxIdType {
    AdNrt,
    AeTrn,
    AlTin,
    AmTin,
    AoTin,
    ArCuit,
    AuAbn,
    AuArn,
    BaTin,
    BbTin,
    BgUic,
    BhVat,
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
    KhTin,
    KrBrn,
    KzBin,
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
    Unknown,
    UsEin,
    UyRuc,
    UzTin,
    UzVat,
    VeRif,
    VnTin,
    ZaVat,
    ZmTin,
    ZwTin,
}

impl TaxIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxIdType::AdNrt => "ad_nrt",
            TaxIdType::AeTrn => "ae_trn",
            TaxIdType::AlTin => "al_tin",
            TaxIdType::AmTin => "am_tin",
            TaxIdType::AoTin => "ao_tin",
            TaxIdType::ArCuit => "ar_cuit",
            TaxIdType::AuAbn => "au_abn",
            TaxIdType::AuArn => "au_arn",
            TaxIdType::BaTin => "ba_tin",
            TaxIdType::BbTin => "bb_tin",
            TaxIdType::BgUic => "bg_uic",
            TaxIdType::BhVat => "bh_vat",
            TaxIdType::BoTin => "bo_tin",
            TaxIdType::BrCnpj => "br_cnpj",
            TaxIdType::BrCpf => "br_cpf",
            TaxIdType::BsTin => "bs_tin",
            TaxIdType::ByTin => "by_tin",
            TaxIdType::CaBn => "ca_bn",
            TaxIdType::CaGstHst => "ca_gst_hst",
            TaxIdType::CaPstBc => "ca_pst_bc",
            TaxIdType::CaPstMb => "ca_pst_mb",
            TaxIdType::CaPstSk => "ca_pst_sk",
            TaxIdType::CaQst => "ca_qst",
            TaxIdType::CdNif => "cd_nif",
            TaxIdType::ChUid => "ch_uid",
            TaxIdType::ChVat => "ch_vat",
            TaxIdType::ClTin => "cl_tin",
            TaxIdType::CnTin => "cn_tin",
            TaxIdType::CoNit => "co_nit",
            TaxIdType::CrTin => "cr_tin",
            TaxIdType::DeStn => "de_stn",
            TaxIdType::DoRcn => "do_rcn",
            TaxIdType::EcRuc => "ec_ruc",
            TaxIdType::EgTin => "eg_tin",
            TaxIdType::EsCif => "es_cif",
            TaxIdType::EuOssVat => "eu_oss_vat",
            TaxIdType::EuVat => "eu_vat",
            TaxIdType::GbVat => "gb_vat",
            TaxIdType::GeVat => "ge_vat",
            TaxIdType::GnNif => "gn_nif",
            TaxIdType::HkBr => "hk_br",
            TaxIdType::HrOib => "hr_oib",
            TaxIdType::HuTin => "hu_tin",
            TaxIdType::IdNpwp => "id_npwp",
            TaxIdType::IlVat => "il_vat",
            TaxIdType::InGst => "in_gst",
            TaxIdType::IsVat => "is_vat",
            TaxIdType::JpCn => "jp_cn",
            TaxIdType::JpRn => "jp_rn",
            TaxIdType::JpTrn => "jp_trn",
            TaxIdType::KePin => "ke_pin",
            TaxIdType::KhTin => "kh_tin",
            TaxIdType::KrBrn => "kr_brn",
            TaxIdType::KzBin => "kz_bin",
            TaxIdType::LiUid => "li_uid",
            TaxIdType::LiVat => "li_vat",
            TaxIdType::MaVat => "ma_vat",
            TaxIdType::MdVat => "md_vat",
            TaxIdType::MePib => "me_pib",
            TaxIdType::MkVat => "mk_vat",
            TaxIdType::MrNif => "mr_nif",
            TaxIdType::MxRfc => "mx_rfc",
            TaxIdType::MyFrp => "my_frp",
            TaxIdType::MyItn => "my_itn",
            TaxIdType::MySst => "my_sst",
            TaxIdType::NgTin => "ng_tin",
            TaxIdType::NoVat => "no_vat",
            TaxIdType::NoVoec => "no_voec",
            TaxIdType::NpPan => "np_pan",
            TaxIdType::NzGst => "nz_gst",
            TaxIdType::OmVat => "om_vat",
            TaxIdType::PeRuc => "pe_ruc",
            TaxIdType::PhTin => "ph_tin",
            TaxIdType::RoTin => "ro_tin",
            TaxIdType::RsPib => "rs_pib",
            TaxIdType::RuInn => "ru_inn",
            TaxIdType::RuKpp => "ru_kpp",
            TaxIdType::SaVat => "sa_vat",
            TaxIdType::SgGst => "sg_gst",
            TaxIdType::SgUen => "sg_uen",
            TaxIdType::SiTin => "si_tin",
            TaxIdType::SnNinea => "sn_ninea",
            TaxIdType::SrFin => "sr_fin",
            TaxIdType::SvNit => "sv_nit",
            TaxIdType::ThVat => "th_vat",
            TaxIdType::TjTin => "tj_tin",
            TaxIdType::TrTin => "tr_tin",
            TaxIdType::TwVat => "tw_vat",
            TaxIdType::TzVat => "tz_vat",
            TaxIdType::UaVat => "ua_vat",
            TaxIdType::UgTin => "ug_tin",
            TaxIdType::Unknown => "unknown",
            TaxIdType::UsEin => "us_ein",
            TaxIdType::UyRuc => "uy_ruc",
            TaxIdType::UzTin => "uz_tin",
            TaxIdType::UzVat => "uz_vat",
            TaxIdType::VeRif => "ve_rif",
            TaxIdType::VnTin => "vn_tin",
            TaxIdType::ZaVat => "za_vat",
            TaxIdType::ZmTin => "zm_tin",
            TaxIdType::ZwTin => "zw_tin",
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
impl std::default::Default for TaxIdType {
    fn default() -> Self {
        Self::AdNrt
    }
}

/// An enum representing the possible values of an `CreateTaxId`'s `type_` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxIdTypeFilter {
    AdNrt,
    AeTrn,
    AlTin,
    AmTin,
    AoTin,
    ArCuit,
    AuAbn,
    AuArn,
    BaTin,
    BbTin,
    BgUic,
    BhVat,
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
    KhTin,
    KrBrn,
    KzBin,
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
}

impl TaxIdTypeFilter {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxIdTypeFilter::AdNrt => "ad_nrt",
            TaxIdTypeFilter::AeTrn => "ae_trn",
            TaxIdTypeFilter::AlTin => "al_tin",
            TaxIdTypeFilter::AmTin => "am_tin",
            TaxIdTypeFilter::AoTin => "ao_tin",
            TaxIdTypeFilter::ArCuit => "ar_cuit",
            TaxIdTypeFilter::AuAbn => "au_abn",
            TaxIdTypeFilter::AuArn => "au_arn",
            TaxIdTypeFilter::BaTin => "ba_tin",
            TaxIdTypeFilter::BbTin => "bb_tin",
            TaxIdTypeFilter::BgUic => "bg_uic",
            TaxIdTypeFilter::BhVat => "bh_vat",
            TaxIdTypeFilter::BoTin => "bo_tin",
            TaxIdTypeFilter::BrCnpj => "br_cnpj",
            TaxIdTypeFilter::BrCpf => "br_cpf",
            TaxIdTypeFilter::BsTin => "bs_tin",
            TaxIdTypeFilter::ByTin => "by_tin",
            TaxIdTypeFilter::CaBn => "ca_bn",
            TaxIdTypeFilter::CaGstHst => "ca_gst_hst",
            TaxIdTypeFilter::CaPstBc => "ca_pst_bc",
            TaxIdTypeFilter::CaPstMb => "ca_pst_mb",
            TaxIdTypeFilter::CaPstSk => "ca_pst_sk",
            TaxIdTypeFilter::CaQst => "ca_qst",
            TaxIdTypeFilter::CdNif => "cd_nif",
            TaxIdTypeFilter::ChUid => "ch_uid",
            TaxIdTypeFilter::ChVat => "ch_vat",
            TaxIdTypeFilter::ClTin => "cl_tin",
            TaxIdTypeFilter::CnTin => "cn_tin",
            TaxIdTypeFilter::CoNit => "co_nit",
            TaxIdTypeFilter::CrTin => "cr_tin",
            TaxIdTypeFilter::DeStn => "de_stn",
            TaxIdTypeFilter::DoRcn => "do_rcn",
            TaxIdTypeFilter::EcRuc => "ec_ruc",
            TaxIdTypeFilter::EgTin => "eg_tin",
            TaxIdTypeFilter::EsCif => "es_cif",
            TaxIdTypeFilter::EuOssVat => "eu_oss_vat",
            TaxIdTypeFilter::EuVat => "eu_vat",
            TaxIdTypeFilter::GbVat => "gb_vat",
            TaxIdTypeFilter::GeVat => "ge_vat",
            TaxIdTypeFilter::GnNif => "gn_nif",
            TaxIdTypeFilter::HkBr => "hk_br",
            TaxIdTypeFilter::HrOib => "hr_oib",
            TaxIdTypeFilter::HuTin => "hu_tin",
            TaxIdTypeFilter::IdNpwp => "id_npwp",
            TaxIdTypeFilter::IlVat => "il_vat",
            TaxIdTypeFilter::InGst => "in_gst",
            TaxIdTypeFilter::IsVat => "is_vat",
            TaxIdTypeFilter::JpCn => "jp_cn",
            TaxIdTypeFilter::JpRn => "jp_rn",
            TaxIdTypeFilter::JpTrn => "jp_trn",
            TaxIdTypeFilter::KePin => "ke_pin",
            TaxIdTypeFilter::KhTin => "kh_tin",
            TaxIdTypeFilter::KrBrn => "kr_brn",
            TaxIdTypeFilter::KzBin => "kz_bin",
            TaxIdTypeFilter::LiUid => "li_uid",
            TaxIdTypeFilter::LiVat => "li_vat",
            TaxIdTypeFilter::MaVat => "ma_vat",
            TaxIdTypeFilter::MdVat => "md_vat",
            TaxIdTypeFilter::MePib => "me_pib",
            TaxIdTypeFilter::MkVat => "mk_vat",
            TaxIdTypeFilter::MrNif => "mr_nif",
            TaxIdTypeFilter::MxRfc => "mx_rfc",
            TaxIdTypeFilter::MyFrp => "my_frp",
            TaxIdTypeFilter::MyItn => "my_itn",
            TaxIdTypeFilter::MySst => "my_sst",
            TaxIdTypeFilter::NgTin => "ng_tin",
            TaxIdTypeFilter::NoVat => "no_vat",
            TaxIdTypeFilter::NoVoec => "no_voec",
            TaxIdTypeFilter::NpPan => "np_pan",
            TaxIdTypeFilter::NzGst => "nz_gst",
            TaxIdTypeFilter::OmVat => "om_vat",
            TaxIdTypeFilter::PeRuc => "pe_ruc",
            TaxIdTypeFilter::PhTin => "ph_tin",
            TaxIdTypeFilter::RoTin => "ro_tin",
            TaxIdTypeFilter::RsPib => "rs_pib",
            TaxIdTypeFilter::RuInn => "ru_inn",
            TaxIdTypeFilter::RuKpp => "ru_kpp",
            TaxIdTypeFilter::SaVat => "sa_vat",
            TaxIdTypeFilter::SgGst => "sg_gst",
            TaxIdTypeFilter::SgUen => "sg_uen",
            TaxIdTypeFilter::SiTin => "si_tin",
            TaxIdTypeFilter::SnNinea => "sn_ninea",
            TaxIdTypeFilter::SrFin => "sr_fin",
            TaxIdTypeFilter::SvNit => "sv_nit",
            TaxIdTypeFilter::ThVat => "th_vat",
            TaxIdTypeFilter::TjTin => "tj_tin",
            TaxIdTypeFilter::TrTin => "tr_tin",
            TaxIdTypeFilter::TwVat => "tw_vat",
            TaxIdTypeFilter::TzVat => "tz_vat",
            TaxIdTypeFilter::UaVat => "ua_vat",
            TaxIdTypeFilter::UgTin => "ug_tin",
            TaxIdTypeFilter::UsEin => "us_ein",
            TaxIdTypeFilter::UyRuc => "uy_ruc",
            TaxIdTypeFilter::UzTin => "uz_tin",
            TaxIdTypeFilter::UzVat => "uz_vat",
            TaxIdTypeFilter::VeRif => "ve_rif",
            TaxIdTypeFilter::VnTin => "vn_tin",
            TaxIdTypeFilter::ZaVat => "za_vat",
            TaxIdTypeFilter::ZmTin => "zm_tin",
            TaxIdTypeFilter::ZwTin => "zw_tin",
        }
    }
}

impl AsRef<str> for TaxIdTypeFilter {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxIdTypeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxIdTypeFilter {
    fn default() -> Self {
        Self::AdNrt
    }
}

/// An enum representing the possible values of an `TaxIdVerification`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxIdVerificationStatus {
    Pending,
    Unavailable,
    Unverified,
    Verified,
}

impl TaxIdVerificationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxIdVerificationStatus::Pending => "pending",
            TaxIdVerificationStatus::Unavailable => "unavailable",
            TaxIdVerificationStatus::Unverified => "unverified",
            TaxIdVerificationStatus::Verified => "verified",
        }
    }
}

impl AsRef<str> for TaxIdVerificationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxIdVerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxIdVerificationStatus {
    fn default() -> Self {
        Self::Pending
    }
}
