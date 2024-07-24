#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct LegalEntityCompany {
    pub address: Option<stripe_shared::Address>,
    /// The Kana variation of the company's primary address (Japan only).
    pub address_kana: Option<stripe_shared::LegalEntityJapanAddress>,
    /// The Kanji variation of the company's primary address (Japan only).
    pub address_kanji: Option<stripe_shared::LegalEntityJapanAddress>,
    /// Whether the company's directors have been provided.
    /// This Boolean will be `true` if you've manually indicated that all directors are provided via [the `directors_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-directors_provided).
    pub directors_provided: Option<bool>,
    /// Whether the company's executives have been provided.
    /// This Boolean will be `true` if you've manually indicated that all executives are provided via [the `executives_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-executives_provided), or if Stripe determined that sufficient executives were provided.
    pub executives_provided: Option<bool>,
    /// The export license ID number of the company, also referred as Import Export Code (India only).
    pub export_license_id: Option<String>,
    /// The purpose code to use for export transactions (India only).
    pub export_purpose_code: Option<String>,
    /// The company's legal name.
    pub name: Option<String>,
    /// The Kana variation of the company's legal name (Japan only).
    pub name_kana: Option<String>,
    /// The Kanji variation of the company's legal name (Japan only).
    pub name_kanji: Option<String>,
    /// Whether the company's owners have been provided.
    /// This Boolean will be `true` if you've manually indicated that all owners are provided via [the `owners_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-owners_provided), or if Stripe determined that sufficient owners were provided.
    /// Stripe determines ownership requirements using both the number of owners provided and their total percent ownership (calculated by adding the `percent_ownership` of each owner together).
    pub owners_provided: Option<bool>,
    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    pub ownership_declaration: Option<stripe_shared::LegalEntityUboDeclaration>,
    /// The company's phone number (used for verification).
    pub phone: Option<String>,
    /// The category identifying the legal structure of the company or legal entity.
    /// See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
    pub structure: Option<LegalEntityCompanyStructure>,
    /// Whether the company's business ID number was provided.
    pub tax_id_provided: Option<bool>,
    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    pub tax_id_registrar: Option<String>,
    /// Whether the company's business VAT number was provided.
    pub vat_id_provided: Option<bool>,
    /// Information on the verification state of the company.
    pub verification: Option<stripe_shared::LegalEntityCompanyVerification>,
}
#[doc(hidden)]
pub struct LegalEntityCompanyBuilder {
    address: Option<Option<stripe_shared::Address>>,
    address_kana: Option<Option<stripe_shared::LegalEntityJapanAddress>>,
    address_kanji: Option<Option<stripe_shared::LegalEntityJapanAddress>>,
    directors_provided: Option<Option<bool>>,
    executives_provided: Option<Option<bool>>,
    export_license_id: Option<Option<String>>,
    export_purpose_code: Option<Option<String>>,
    name: Option<Option<String>>,
    name_kana: Option<Option<String>>,
    name_kanji: Option<Option<String>>,
    owners_provided: Option<Option<bool>>,
    ownership_declaration: Option<Option<stripe_shared::LegalEntityUboDeclaration>>,
    phone: Option<Option<String>>,
    structure: Option<Option<LegalEntityCompanyStructure>>,
    tax_id_provided: Option<Option<bool>>,
    tax_id_registrar: Option<Option<String>>,
    vat_id_provided: Option<Option<bool>>,
    verification: Option<Option<stripe_shared::LegalEntityCompanyVerification>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for LegalEntityCompany {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LegalEntityCompany>,
        builder: LegalEntityCompanyBuilder,
    }

    impl Visitor for Place<LegalEntityCompany> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: LegalEntityCompanyBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for LegalEntityCompanyBuilder {
        type Out = LegalEntityCompany;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "address_kana" => Deserialize::begin(&mut self.address_kana),
                "address_kanji" => Deserialize::begin(&mut self.address_kanji),
                "directors_provided" => Deserialize::begin(&mut self.directors_provided),
                "executives_provided" => Deserialize::begin(&mut self.executives_provided),
                "export_license_id" => Deserialize::begin(&mut self.export_license_id),
                "export_purpose_code" => Deserialize::begin(&mut self.export_purpose_code),
                "name" => Deserialize::begin(&mut self.name),
                "name_kana" => Deserialize::begin(&mut self.name_kana),
                "name_kanji" => Deserialize::begin(&mut self.name_kanji),
                "owners_provided" => Deserialize::begin(&mut self.owners_provided),
                "ownership_declaration" => Deserialize::begin(&mut self.ownership_declaration),
                "phone" => Deserialize::begin(&mut self.phone),
                "structure" => Deserialize::begin(&mut self.structure),
                "tax_id_provided" => Deserialize::begin(&mut self.tax_id_provided),
                "tax_id_registrar" => Deserialize::begin(&mut self.tax_id_registrar),
                "vat_id_provided" => Deserialize::begin(&mut self.vat_id_provided),
                "verification" => Deserialize::begin(&mut self.verification),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                address_kana: Deserialize::default(),
                address_kanji: Deserialize::default(),
                directors_provided: Deserialize::default(),
                executives_provided: Deserialize::default(),
                export_license_id: Deserialize::default(),
                export_purpose_code: Deserialize::default(),
                name: Deserialize::default(),
                name_kana: Deserialize::default(),
                name_kanji: Deserialize::default(),
                owners_provided: Deserialize::default(),
                ownership_declaration: Deserialize::default(),
                phone: Deserialize::default(),
                structure: Deserialize::default(),
                tax_id_provided: Deserialize::default(),
                tax_id_registrar: Deserialize::default(),
                vat_id_provided: Deserialize::default(),
                verification: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                address: self.address.take()?,
                address_kana: self.address_kana.take()?,
                address_kanji: self.address_kanji.take()?,
                directors_provided: self.directors_provided?,
                executives_provided: self.executives_provided?,
                export_license_id: self.export_license_id.take()?,
                export_purpose_code: self.export_purpose_code.take()?,
                name: self.name.take()?,
                name_kana: self.name_kana.take()?,
                name_kanji: self.name_kanji.take()?,
                owners_provided: self.owners_provided?,
                ownership_declaration: self.ownership_declaration.take()?,
                phone: self.phone.take()?,
                structure: self.structure?,
                tax_id_provided: self.tax_id_provided?,
                tax_id_registrar: self.tax_id_registrar.take()?,
                vat_id_provided: self.vat_id_provided?,
                verification: self.verification.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for LegalEntityCompany {
        type Builder = LegalEntityCompanyBuilder;
    }

    impl FromValueOpt for LegalEntityCompany {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = LegalEntityCompanyBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = Some(FromValueOpt::from_value(v)?),
                    "address_kana" => b.address_kana = Some(FromValueOpt::from_value(v)?),
                    "address_kanji" => b.address_kanji = Some(FromValueOpt::from_value(v)?),
                    "directors_provided" => {
                        b.directors_provided = Some(FromValueOpt::from_value(v)?)
                    }
                    "executives_provided" => {
                        b.executives_provided = Some(FromValueOpt::from_value(v)?)
                    }
                    "export_license_id" => b.export_license_id = Some(FromValueOpt::from_value(v)?),
                    "export_purpose_code" => {
                        b.export_purpose_code = Some(FromValueOpt::from_value(v)?)
                    }
                    "name" => b.name = Some(FromValueOpt::from_value(v)?),
                    "name_kana" => b.name_kana = Some(FromValueOpt::from_value(v)?),
                    "name_kanji" => b.name_kanji = Some(FromValueOpt::from_value(v)?),
                    "owners_provided" => b.owners_provided = Some(FromValueOpt::from_value(v)?),
                    "ownership_declaration" => {
                        b.ownership_declaration = Some(FromValueOpt::from_value(v)?)
                    }
                    "phone" => b.phone = Some(FromValueOpt::from_value(v)?),
                    "structure" => b.structure = Some(FromValueOpt::from_value(v)?),
                    "tax_id_provided" => b.tax_id_provided = Some(FromValueOpt::from_value(v)?),
                    "tax_id_registrar" => b.tax_id_registrar = Some(FromValueOpt::from_value(v)?),
                    "vat_id_provided" => b.vat_id_provided = Some(FromValueOpt::from_value(v)?),
                    "verification" => b.verification = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The category identifying the legal structure of the company or legal entity.
/// See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum LegalEntityCompanyStructure {
    FreeZoneEstablishment,
    FreeZoneLlc,
    GovernmentInstrumentality,
    GovernmentalUnit,
    IncorporatedNonProfit,
    IncorporatedPartnership,
    LimitedLiabilityPartnership,
    Llc,
    MultiMemberLlc,
    PrivateCompany,
    PrivateCorporation,
    PrivatePartnership,
    PublicCompany,
    PublicCorporation,
    PublicPartnership,
    RegisteredCharity,
    SingleMemberLlc,
    SoleEstablishment,
    SoleProprietorship,
    TaxExemptGovernmentInstrumentality,
    UnincorporatedAssociation,
    UnincorporatedNonProfit,
    UnincorporatedPartnership,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl LegalEntityCompanyStructure {
    pub fn as_str(self) -> &'static str {
        use LegalEntityCompanyStructure::*;
        match self {
            FreeZoneEstablishment => "free_zone_establishment",
            FreeZoneLlc => "free_zone_llc",
            GovernmentInstrumentality => "government_instrumentality",
            GovernmentalUnit => "governmental_unit",
            IncorporatedNonProfit => "incorporated_non_profit",
            IncorporatedPartnership => "incorporated_partnership",
            LimitedLiabilityPartnership => "limited_liability_partnership",
            Llc => "llc",
            MultiMemberLlc => "multi_member_llc",
            PrivateCompany => "private_company",
            PrivateCorporation => "private_corporation",
            PrivatePartnership => "private_partnership",
            PublicCompany => "public_company",
            PublicCorporation => "public_corporation",
            PublicPartnership => "public_partnership",
            RegisteredCharity => "registered_charity",
            SingleMemberLlc => "single_member_llc",
            SoleEstablishment => "sole_establishment",
            SoleProprietorship => "sole_proprietorship",
            TaxExemptGovernmentInstrumentality => "tax_exempt_government_instrumentality",
            UnincorporatedAssociation => "unincorporated_association",
            UnincorporatedNonProfit => "unincorporated_non_profit",
            UnincorporatedPartnership => "unincorporated_partnership",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for LegalEntityCompanyStructure {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LegalEntityCompanyStructure::*;
        match s {
            "free_zone_establishment" => Ok(FreeZoneEstablishment),
            "free_zone_llc" => Ok(FreeZoneLlc),
            "government_instrumentality" => Ok(GovernmentInstrumentality),
            "governmental_unit" => Ok(GovernmentalUnit),
            "incorporated_non_profit" => Ok(IncorporatedNonProfit),
            "incorporated_partnership" => Ok(IncorporatedPartnership),
            "limited_liability_partnership" => Ok(LimitedLiabilityPartnership),
            "llc" => Ok(Llc),
            "multi_member_llc" => Ok(MultiMemberLlc),
            "private_company" => Ok(PrivateCompany),
            "private_corporation" => Ok(PrivateCorporation),
            "private_partnership" => Ok(PrivatePartnership),
            "public_company" => Ok(PublicCompany),
            "public_corporation" => Ok(PublicCorporation),
            "public_partnership" => Ok(PublicPartnership),
            "registered_charity" => Ok(RegisteredCharity),
            "single_member_llc" => Ok(SingleMemberLlc),
            "sole_establishment" => Ok(SoleEstablishment),
            "sole_proprietorship" => Ok(SoleProprietorship),
            "tax_exempt_government_instrumentality" => Ok(TaxExemptGovernmentInstrumentality),
            "unincorporated_association" => Ok(UnincorporatedAssociation),
            "unincorporated_non_profit" => Ok(UnincorporatedNonProfit),
            "unincorporated_partnership" => Ok(UnincorporatedPartnership),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for LegalEntityCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for LegalEntityCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for LegalEntityCompanyStructure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for LegalEntityCompanyStructure {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<LegalEntityCompanyStructure> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(LegalEntityCompanyStructure::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(LegalEntityCompanyStructure);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for LegalEntityCompanyStructure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
