/// You can add one or multiple tax IDs to a [customer](https://stripe.com/docs/api/customers) or account.
/// Customer and account tax IDs get displayed on related invoices and credit notes.
///
/// Related guides: [Customer tax identification numbers](https://stripe.com/docs/billing/taxes/tax-ids), [Account tax IDs](https://stripe.com/docs/invoicing/connect#account-tax-ids).
///
/// For more details see <<https://stripe.com/docs/api/tax_ids/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxId {
    /// Two-letter ISO code representing the country of the tax ID.
    pub country: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// ID of the customer.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::TaxIdId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The account or customer the tax ID belongs to.
    pub owner: Option<stripe_shared::TaxIDsOwner>,
    /// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `al_tin`, `am_tin`, `ao_tin`, `ar_cuit`, `au_abn`, `au_arn`, `aw_tin`, `az_tin`, `ba_tin`, `bb_tin`, `bd_bin`, `bf_ifu`, `bg_uic`, `bh_vat`, `bj_ifu`, `bo_tin`, `br_cnpj`, `br_cpf`, `bs_tin`, `by_tin`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `cd_nif`, `ch_uid`, `ch_vat`, `cl_tin`, `cm_niu`, `cn_tin`, `co_nit`, `cr_tin`, `cv_nif`, `de_stn`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `et_tin`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `gn_nif`, `hk_br`, `hr_oib`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kg_tin`, `kh_tin`, `kr_brn`, `kz_bin`, `la_tin`, `li_uid`, `li_vat`, `ma_vat`, `md_vat`, `me_pib`, `mk_vat`, `mr_nif`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `np_pan`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sn_ninea`, `sr_fin`, `sv_nit`, `th_vat`, `tj_tin`, `tr_tin`, `tw_vat`, `tz_vat`, `ua_vat`, `ug_tin`, `us_ein`, `uy_ruc`, `uz_tin`, `uz_vat`, `ve_rif`, `vn_tin`, `za_vat`, `zm_tin`, or `zw_tin`.
    /// Note that some legacy tax IDs have type `unknown`.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: TaxIdType,
    /// Value of the tax ID.
    pub value: String,
    /// Tax ID verification information.
    pub verification: Option<stripe_shared::TaxIdVerification>,
}
#[doc(hidden)]
pub struct TaxIdBuilder {
    country: Option<Option<String>>,
    created: Option<stripe_types::Timestamp>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    id: Option<stripe_shared::TaxIdId>,
    livemode: Option<bool>,
    owner: Option<Option<stripe_shared::TaxIDsOwner>>,
    type_: Option<TaxIdType>,
    value: Option<String>,
    verification: Option<Option<stripe_shared::TaxIdVerification>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxId {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxId>,
        builder: TaxIdBuilder,
    }

    impl Visitor for Place<TaxId> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxIdBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxIdBuilder {
        type Out = TaxId;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "country" => Deserialize::begin(&mut self.country),
                "created" => Deserialize::begin(&mut self.created),
                "customer" => Deserialize::begin(&mut self.customer),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "owner" => Deserialize::begin(&mut self.owner),
                "type" => Deserialize::begin(&mut self.type_),
                "value" => Deserialize::begin(&mut self.value),
                "verification" => Deserialize::begin(&mut self.verification),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                country: Deserialize::default(),
                created: Deserialize::default(),
                customer: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                owner: Deserialize::default(),
                type_: Deserialize::default(),
                value: Deserialize::default(),
                verification: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(country),
                Some(created),
                Some(customer),
                Some(id),
                Some(livemode),
                Some(owner),
                Some(type_),
                Some(value),
                Some(verification),
            ) = (
                self.country.take(),
                self.created,
                self.customer.take(),
                self.id.take(),
                self.livemode,
                self.owner.take(),
                self.type_,
                self.value.take(),
                self.verification.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                country,
                created,
                customer,
                id,
                livemode,
                owner,
                type_,
                value,
                verification,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for TaxId {
        type Builder = TaxIdBuilder;
    }

    impl FromValueOpt for TaxId {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxIdBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "country" => b.country = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "owner" => b.owner = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "value" => b.value = FromValueOpt::from_value(v),
                    "verification" => b.verification = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TaxId", 10)?;
        s.serialize_field("country", &self.country)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("owner", &self.owner)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("value", &self.value)?;
        s.serialize_field("verification", &self.verification)?;

        s.serialize_field("object", "tax_id")?;
        s.end()
    }
}
/// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `al_tin`, `am_tin`, `ao_tin`, `ar_cuit`, `au_abn`, `au_arn`, `aw_tin`, `az_tin`, `ba_tin`, `bb_tin`, `bd_bin`, `bf_ifu`, `bg_uic`, `bh_vat`, `bj_ifu`, `bo_tin`, `br_cnpj`, `br_cpf`, `bs_tin`, `by_tin`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `cd_nif`, `ch_uid`, `ch_vat`, `cl_tin`, `cm_niu`, `cn_tin`, `co_nit`, `cr_tin`, `cv_nif`, `de_stn`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `et_tin`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `gn_nif`, `hk_br`, `hr_oib`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kg_tin`, `kh_tin`, `kr_brn`, `kz_bin`, `la_tin`, `li_uid`, `li_vat`, `ma_vat`, `md_vat`, `me_pib`, `mk_vat`, `mr_nif`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `np_pan`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sn_ninea`, `sr_fin`, `sv_nit`, `th_vat`, `tj_tin`, `tr_tin`, `tw_vat`, `tz_vat`, `ua_vat`, `ug_tin`, `us_ein`, `uy_ruc`, `uz_tin`, `uz_vat`, `ve_rif`, `vn_tin`, `za_vat`, `zm_tin`, or `zw_tin`.
/// Note that some legacy tax IDs have type `unknown`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxIdType {
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
        use TaxIdType::*;
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
            Unknown => "unknown",
            UsEin => "us_ein",
            UyRuc => "uy_ruc",
            UzTin => "uz_tin",
            UzVat => "uz_vat",
            VeRif => "ve_rif",
            VnTin => "vn_tin",
            ZaVat => "za_vat",
            ZmTin => "zm_tin",
            ZwTin => "zw_tin",
        }
    }
}

impl std::str::FromStr for TaxIdType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxIdType::*;
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
            "unknown" => Ok(Unknown),
            "us_ein" => Ok(UsEin),
            "uy_ruc" => Ok(UyRuc),
            "uz_tin" => Ok(UzTin),
            "uz_vat" => Ok(UzVat),
            "ve_rif" => Ok(VeRif),
            "vn_tin" => Ok(VnTin),
            "za_vat" => Ok(ZaVat),
            "zm_tin" => Ok(ZmTin),
            "zw_tin" => Ok(ZwTin),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TaxIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxIdType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxIdType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxIdType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxIdType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxIdType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxIdType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxIdType"))
    }
}
impl stripe_types::Object for TaxId {
    type Id = stripe_shared::TaxIdId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TaxIdId);
