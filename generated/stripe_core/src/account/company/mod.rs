#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Company {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types::address::Address>,
    /// The Kana variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<stripe_core::person::japan_address::JapanAddress>,
    /// The Kanji variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<stripe_core::person::japan_address::JapanAddress>,
    /// Whether the company's directors have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all directors are provided via [the `directors_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-directors_provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directors_provided: Option<bool>,
    /// Whether the company's executives have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all executives are provided via [the `executives_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-executives_provided), or if Stripe determined that sufficient executives were provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executives_provided: Option<bool>,
    /// The company's legal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Kana variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<String>,
    /// The Kanji variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kanji: Option<String>,
    /// Whether the company's owners have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all owners are provided via [the `owners_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-owners_provided), or if Stripe determined that sufficient owners were provided.
    /// Stripe determines ownership requirements using both the number of owners provided and their total percent ownership (calculated by adding the `percent_ownership` of each owner together).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,
    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration:
        Option<stripe_core::account::company::ownership_declaration::OwnershipDeclaration>,
    /// The company's phone number (used for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The category identifying the legal structure of the company or legal entity.
    ///
    /// See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<CompanyStructure>,
    /// Whether the company's business ID number was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_provided: Option<bool>,
    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<String>,
    /// Whether the company's business VAT number was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id_provided: Option<bool>,
    /// Information on the verification state of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<stripe_core::account::company::verification::Verification>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Company {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The category identifying the legal structure of the company or legal entity.
///
/// See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CompanyStructure {
    FreeZoneEstablishment,
    FreeZoneLlc,
    GovernmentInstrumentality,
    GovernmentalUnit,
    IncorporatedNonProfit,
    LimitedLiabilityPartnership,
    Llc,
    MultiMemberLlc,
    PrivateCompany,
    PrivateCorporation,
    PrivatePartnership,
    PublicCompany,
    PublicCorporation,
    PublicPartnership,
    SingleMemberLlc,
    SoleEstablishment,
    SoleProprietorship,
    TaxExemptGovernmentInstrumentality,
    UnincorporatedAssociation,
    UnincorporatedNonProfit,
}

impl CompanyStructure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FreeZoneEstablishment => "free_zone_establishment",
            Self::FreeZoneLlc => "free_zone_llc",
            Self::GovernmentInstrumentality => "government_instrumentality",
            Self::GovernmentalUnit => "governmental_unit",
            Self::IncorporatedNonProfit => "incorporated_non_profit",
            Self::LimitedLiabilityPartnership => "limited_liability_partnership",
            Self::Llc => "llc",
            Self::MultiMemberLlc => "multi_member_llc",
            Self::PrivateCompany => "private_company",
            Self::PrivateCorporation => "private_corporation",
            Self::PrivatePartnership => "private_partnership",
            Self::PublicCompany => "public_company",
            Self::PublicCorporation => "public_corporation",
            Self::PublicPartnership => "public_partnership",
            Self::SingleMemberLlc => "single_member_llc",
            Self::SoleEstablishment => "sole_establishment",
            Self::SoleProprietorship => "sole_proprietorship",
            Self::TaxExemptGovernmentInstrumentality => "tax_exempt_government_instrumentality",
            Self::UnincorporatedAssociation => "unincorporated_association",
            Self::UnincorporatedNonProfit => "unincorporated_non_profit",
        }
    }
}

impl AsRef<str> for CompanyStructure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod verification;
pub use verification::Verification;
pub mod ownership_declaration;
pub use ownership_declaration::OwnershipDeclaration;
