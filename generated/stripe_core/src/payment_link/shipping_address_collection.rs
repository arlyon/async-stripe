#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ShippingAddressCollection {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for shipping locations.
    ///
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: Vec<ShippingAddressCollectionAllowedCountries>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ShippingAddressCollection {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// An array of two-letter ISO country codes representing which countries Checkout should provide as options for shipping locations.
///
/// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ShippingAddressCollectionAllowedCountries {
    #[serde(rename = "AC")]
    Ac,
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "AE")]
    Ae,
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AG")]
    Ag,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AO")]
    Ao,
    #[serde(rename = "AQ")]
    Aq,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "AW")]
    Aw,
    #[serde(rename = "AX")]
    Ax,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BB")]
    Bb,
    #[serde(rename = "BD")]
    Bd,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BF")]
    Bf,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "BJ")]
    Bj,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "BQ")]
    Bq,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BT")]
    Bt,
    #[serde(rename = "BV")]
    Bv,
    #[serde(rename = "BW")]
    Bw,
    #[serde(rename = "BY")]
    By,
    #[serde(rename = "BZ")]
    Bz,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "CF")]
    Cf,
    #[serde(rename = "CG")]
    Cg,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "CI")]
    Ci,
    #[serde(rename = "CK")]
    Ck,
    #[serde(rename = "CL")]
    Cl,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "CW")]
    Cw,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DM")]
    Dm,
    #[serde(rename = "DO")]
    Do,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "EG")]
    Eg,
    #[serde(rename = "EH")]
    Eh,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FK")]
    Fk,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GE")]
    Ge,
    #[serde(rename = "GF")]
    Gf,
    #[serde(rename = "GG")]
    Gg,
    #[serde(rename = "GH")]
    Gh,
    #[serde(rename = "GI")]
    Gi,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GP")]
    Gp,
    #[serde(rename = "GQ")]
    Gq,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "GS")]
    Gs,
    #[serde(rename = "GT")]
    Gt,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "GW")]
    Gw,
    #[serde(rename = "GY")]
    Gy,
    #[serde(rename = "HK")]
    Hk,
    #[serde(rename = "HN")]
    Hn,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IM")]
    Im,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "IQ")]
    Iq,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JE")]
    Je,
    #[serde(rename = "JM")]
    Jm,
    #[serde(rename = "JO")]
    Jo,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "KE")]
    Ke,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "KH")]
    Kh,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "KZ")]
    Kz,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LK")]
    Lk,
    #[serde(rename = "LR")]
    Lr,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MC")]
    Mc,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MQ")]
    Mq,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MU")]
    Mu,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "MW")]
    Mw,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "MZ")]
    Mz,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NI")]
    Ni,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "NP")]
    Np,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "NU")]
    Nu,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PE")]
    Pe,
    #[serde(rename = "PF")]
    Pf,
    #[serde(rename = "PG")]
    Pg,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "PN")]
    Pn,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PY")]
    Py,
    #[serde(rename = "QA")]
    Qa,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RS")]
    Rs,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SJ")]
    Sj,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SS")]
    Ss,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "SX")]
    Sx,
    #[serde(rename = "SZ")]
    Sz,
    #[serde(rename = "TA")]
    Ta,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TD")]
    Td,
    #[serde(rename = "TF")]
    Tf,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TJ")]
    Tj,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "TZ")]
    Tz,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "UY")]
    Uy,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "VC")]
    Vc,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VG")]
    Vg,
    #[serde(rename = "VN")]
    Vn,
    #[serde(rename = "VU")]
    Vu,
    #[serde(rename = "WF")]
    Wf,
    #[serde(rename = "WS")]
    Ws,
    #[serde(rename = "XK")]
    Xk,
    #[serde(rename = "YE")]
    Ye,
    #[serde(rename = "YT")]
    Yt,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "ZM")]
    Zm,
    #[serde(rename = "ZW")]
    Zw,
    #[serde(rename = "ZZ")]
    Zz,
}

impl ShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ac => "AC",
            Self::Ad => "AD",
            Self::Ae => "AE",
            Self::Af => "AF",
            Self::Ag => "AG",
            Self::Ai => "AI",
            Self::Al => "AL",
            Self::Am => "AM",
            Self::Ao => "AO",
            Self::Aq => "AQ",
            Self::Ar => "AR",
            Self::At => "AT",
            Self::Au => "AU",
            Self::Aw => "AW",
            Self::Ax => "AX",
            Self::Az => "AZ",
            Self::Ba => "BA",
            Self::Bb => "BB",
            Self::Bd => "BD",
            Self::Be => "BE",
            Self::Bf => "BF",
            Self::Bg => "BG",
            Self::Bh => "BH",
            Self::Bi => "BI",
            Self::Bj => "BJ",
            Self::Bl => "BL",
            Self::Bm => "BM",
            Self::Bn => "BN",
            Self::Bo => "BO",
            Self::Bq => "BQ",
            Self::Br => "BR",
            Self::Bs => "BS",
            Self::Bt => "BT",
            Self::Bv => "BV",
            Self::Bw => "BW",
            Self::By => "BY",
            Self::Bz => "BZ",
            Self::Ca => "CA",
            Self::Cd => "CD",
            Self::Cf => "CF",
            Self::Cg => "CG",
            Self::Ch => "CH",
            Self::Ci => "CI",
            Self::Ck => "CK",
            Self::Cl => "CL",
            Self::Cm => "CM",
            Self::Cn => "CN",
            Self::Co => "CO",
            Self::Cr => "CR",
            Self::Cv => "CV",
            Self::Cw => "CW",
            Self::Cy => "CY",
            Self::Cz => "CZ",
            Self::De => "DE",
            Self::Dj => "DJ",
            Self::Dk => "DK",
            Self::Dm => "DM",
            Self::Do => "DO",
            Self::Dz => "DZ",
            Self::Ec => "EC",
            Self::Ee => "EE",
            Self::Eg => "EG",
            Self::Eh => "EH",
            Self::Er => "ER",
            Self::Es => "ES",
            Self::Et => "ET",
            Self::Fi => "FI",
            Self::Fj => "FJ",
            Self::Fk => "FK",
            Self::Fo => "FO",
            Self::Fr => "FR",
            Self::Ga => "GA",
            Self::Gb => "GB",
            Self::Gd => "GD",
            Self::Ge => "GE",
            Self::Gf => "GF",
            Self::Gg => "GG",
            Self::Gh => "GH",
            Self::Gi => "GI",
            Self::Gl => "GL",
            Self::Gm => "GM",
            Self::Gn => "GN",
            Self::Gp => "GP",
            Self::Gq => "GQ",
            Self::Gr => "GR",
            Self::Gs => "GS",
            Self::Gt => "GT",
            Self::Gu => "GU",
            Self::Gw => "GW",
            Self::Gy => "GY",
            Self::Hk => "HK",
            Self::Hn => "HN",
            Self::Hr => "HR",
            Self::Ht => "HT",
            Self::Hu => "HU",
            Self::Id => "ID",
            Self::Ie => "IE",
            Self::Il => "IL",
            Self::Im => "IM",
            Self::In => "IN",
            Self::Io => "IO",
            Self::Iq => "IQ",
            Self::Is => "IS",
            Self::It => "IT",
            Self::Je => "JE",
            Self::Jm => "JM",
            Self::Jo => "JO",
            Self::Jp => "JP",
            Self::Ke => "KE",
            Self::Kg => "KG",
            Self::Kh => "KH",
            Self::Ki => "KI",
            Self::Km => "KM",
            Self::Kn => "KN",
            Self::Kr => "KR",
            Self::Kw => "KW",
            Self::Ky => "KY",
            Self::Kz => "KZ",
            Self::La => "LA",
            Self::Lb => "LB",
            Self::Lc => "LC",
            Self::Li => "LI",
            Self::Lk => "LK",
            Self::Lr => "LR",
            Self::Ls => "LS",
            Self::Lt => "LT",
            Self::Lu => "LU",
            Self::Lv => "LV",
            Self::Ly => "LY",
            Self::Ma => "MA",
            Self::Mc => "MC",
            Self::Md => "MD",
            Self::Me => "ME",
            Self::Mf => "MF",
            Self::Mg => "MG",
            Self::Mk => "MK",
            Self::Ml => "ML",
            Self::Mm => "MM",
            Self::Mn => "MN",
            Self::Mo => "MO",
            Self::Mq => "MQ",
            Self::Mr => "MR",
            Self::Ms => "MS",
            Self::Mt => "MT",
            Self::Mu => "MU",
            Self::Mv => "MV",
            Self::Mw => "MW",
            Self::Mx => "MX",
            Self::My => "MY",
            Self::Mz => "MZ",
            Self::Na => "NA",
            Self::Nc => "NC",
            Self::Ne => "NE",
            Self::Ng => "NG",
            Self::Ni => "NI",
            Self::Nl => "NL",
            Self::No => "NO",
            Self::Np => "NP",
            Self::Nr => "NR",
            Self::Nu => "NU",
            Self::Nz => "NZ",
            Self::Om => "OM",
            Self::Pa => "PA",
            Self::Pe => "PE",
            Self::Pf => "PF",
            Self::Pg => "PG",
            Self::Ph => "PH",
            Self::Pk => "PK",
            Self::Pl => "PL",
            Self::Pm => "PM",
            Self::Pn => "PN",
            Self::Pr => "PR",
            Self::Ps => "PS",
            Self::Pt => "PT",
            Self::Py => "PY",
            Self::Qa => "QA",
            Self::Re => "RE",
            Self::Ro => "RO",
            Self::Rs => "RS",
            Self::Ru => "RU",
            Self::Rw => "RW",
            Self::Sa => "SA",
            Self::Sb => "SB",
            Self::Sc => "SC",
            Self::Se => "SE",
            Self::Sg => "SG",
            Self::Sh => "SH",
            Self::Si => "SI",
            Self::Sj => "SJ",
            Self::Sk => "SK",
            Self::Sl => "SL",
            Self::Sm => "SM",
            Self::Sn => "SN",
            Self::So => "SO",
            Self::Sr => "SR",
            Self::Ss => "SS",
            Self::St => "ST",
            Self::Sv => "SV",
            Self::Sx => "SX",
            Self::Sz => "SZ",
            Self::Ta => "TA",
            Self::Tc => "TC",
            Self::Td => "TD",
            Self::Tf => "TF",
            Self::Tg => "TG",
            Self::Th => "TH",
            Self::Tj => "TJ",
            Self::Tk => "TK",
            Self::Tl => "TL",
            Self::Tm => "TM",
            Self::Tn => "TN",
            Self::To => "TO",
            Self::Tr => "TR",
            Self::Tt => "TT",
            Self::Tv => "TV",
            Self::Tw => "TW",
            Self::Tz => "TZ",
            Self::Ua => "UA",
            Self::Ug => "UG",
            Self::Us => "US",
            Self::Uy => "UY",
            Self::Uz => "UZ",
            Self::Va => "VA",
            Self::Vc => "VC",
            Self::Ve => "VE",
            Self::Vg => "VG",
            Self::Vn => "VN",
            Self::Vu => "VU",
            Self::Wf => "WF",
            Self::Ws => "WS",
            Self::Xk => "XK",
            Self::Ye => "YE",
            Self::Yt => "YT",
            Self::Za => "ZA",
            Self::Zm => "ZM",
            Self::Zw => "ZW",
            Self::Zz => "ZZ",
        }
    }
}

impl AsRef<str> for ShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
