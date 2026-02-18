use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrieveTokenBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTokenBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the token with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveToken {
    inner: RetrieveTokenBuilder,
    token: stripe_core::TokenId,
}
impl RetrieveToken {
    /// Construct a new `RetrieveToken`.
    pub fn new(token: impl Into<stripe_core::TokenId>) -> Self {
        Self { token: token.into(), inner: RetrieveTokenBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveToken {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveToken {
    type Output = stripe_core::Token;

    fn build(&self) -> RequestBuilder {
        let token = &self.token;
        RequestBuilder::new(StripeMethod::Get, format!("/tokens/{token}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateTokenBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<CreateTokenAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bank_account: Option<CreateTokenBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<CreateTokenCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cvc_update: Option<CreateTokenCvcUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    person: Option<CreateTokenPerson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pii: Option<CreateTokenPii>,
}
impl CreateTokenBuilder {
    fn new() -> Self {
        Self {
            account: None,
            bank_account: None,
            card: None,
            customer: None,
            cvc_update: None,
            expand: None,
            person: None,
            pii: None,
        }
    }
}
/// Information for the account this token represents.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTokenAccount {
    /// The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<CreateTokenAccountBusinessType>,
    /// Information about the company or business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CreateTokenAccountCompany>,
    /// Information about the person represented by the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<CreateTokenAccountIndividual>,
    /// Whether the user described by the data in the token has been shown [the Stripe Connected Account Agreement](/connect/account-tokens#stripe-connected-account-agreement).
    /// When creating an account token to create a new Connect account, this value must be `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}
impl CreateTokenAccount {
    pub fn new() -> Self {
        Self { business_type: None, company: None, individual: None, tos_shown_and_accepted: None }
    }
}
impl Default for CreateTokenAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// The business type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTokenAccountBusinessType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTokenAccountBusinessType {
    pub fn as_str(&self) -> &str {
        use CreateTokenAccountBusinessType::*;
        match self {
            Company => "company",
            GovernmentEntity => "government_entity",
            Individual => "individual",
            NonProfit => "non_profit",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTokenAccountBusinessType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTokenAccountBusinessType::*;
        match s {
            "company" => Ok(Company),
            "government_entity" => Ok(GovernmentEntity),
            "individual" => Ok(Individual),
            "non_profit" => Ok(NonProfit),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTokenAccountBusinessType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateTokenAccountBusinessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTokenAccountBusinessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTokenAccountBusinessType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTokenAccountBusinessType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Information about the company or business.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenAccountCompany {
    /// The company's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateTokenAccountCompanyAddress>,
    /// The Kana variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<CreateTokenAccountCompanyAddressKana>,
    /// The Kanji variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<CreateTokenAccountCompanyAddressKanji>,
    /// Whether the company's directors have been provided.
    /// Set this Boolean to `true` after creating all the company's directors with [the Persons API](/api/persons) for accounts with a `relationship.director` requirement.
    /// This value is not automatically set to `true` after creating directors, so it needs to be updated to indicate all directors have been provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directors_provided: Option<bool>,
    /// This hash is used to attest that the directors information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directorship_declaration: Option<CreateTokenAccountCompanyDirectorshipDeclaration>,
    /// Whether the company's executives have been provided.
    /// Set this Boolean to `true` after creating all the company's executives with [the Persons API](/api/persons) for accounts with a `relationship.executive` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executives_provided: Option<bool>,
    /// The export license ID number of the company, also referred as Import Export Code (India only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_license_id: Option<String>,
    /// The purpose code to use for export transactions (India only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_purpose_code: Option<String>,
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
    /// Set this Boolean to `true` after creating all the company's owners with [the Persons API](/api/persons) for accounts with a `relationship.owner` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,
    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration: Option<CreateTokenAccountCompanyOwnershipDeclaration>,
    /// Whether the user described by the data in the token has been shown the Ownership Declaration and indicated that it is correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration_shown_and_signed: Option<bool>,
    /// This value is used to determine if a business is exempt from providing ultimate beneficial owners.
    /// See [this support article](https://support.stripe.com/questions/exemption-from-providing-ownership-details) and [changelog](https://docs.stripe.com/changelog/acacia/2025-01-27/ownership-exemption-reason-accounts-api) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_exemption_reason: Option<CreateTokenAccountCompanyOwnershipExemptionReason>,
    /// The company's phone number (used for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// When the business was incorporated or registered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<CreateTokenAccountCompanyRegistrationDate>,
    /// The identification number given to a company when it is registered or incorporated, if distinct from the identification number used for filing taxes.
    /// (Examples are the CIN for companies and LLP IN for partnerships in India, and the Company Registration Number in Hong Kong).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    /// This hash is used to attest that the representative is authorized to act as the representative of their legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative_declaration: Option<CreateTokenAccountCompanyRepresentativeDeclaration>,
    /// The category identifying the legal structure of the company or legal entity.
    /// See [Business structure](/connect/identity-verification#business-structure) for more details.
    /// Pass an empty string to unset this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<CreateTokenAccountCompanyStructure>,
    /// The business ID number of the company, as appropriate for the company’s country.
    /// (Examples are an Employer ID Number in the U.S., a Business Number in Canada, or a Company Number in the UK.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<String>,
    /// The VAT number of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<String>,
    /// Information on the verification state of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<CreateTokenAccountCompanyVerification>,
}
impl CreateTokenAccountCompany {
    pub fn new() -> Self {
        Self {
            address: None,
            address_kana: None,
            address_kanji: None,
            directors_provided: None,
            directorship_declaration: None,
            executives_provided: None,
            export_license_id: None,
            export_purpose_code: None,
            name: None,
            name_kana: None,
            name_kanji: None,
            owners_provided: None,
            ownership_declaration: None,
            ownership_declaration_shown_and_signed: None,
            ownership_exemption_reason: None,
            phone: None,
            registration_date: None,
            registration_number: None,
            representative_declaration: None,
            structure: None,
            tax_id: None,
            tax_id_registrar: None,
            vat_id: None,
            verification: None,
        }
    }
}
impl Default for CreateTokenAccountCompany {
    fn default() -> Self {
        Self::new()
    }
}
/// The company's primary address.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenAccountCompanyAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl CreateTokenAccountCompanyAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateTokenAccountCompanyAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kana variation of the company's primary address (Japan only).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenAccountCompanyAddressKana {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
impl CreateTokenAccountCompanyAddressKana {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for CreateTokenAccountCompanyAddressKana {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the company's primary address (Japan only).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenAccountCompanyAddressKanji {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
impl CreateTokenAccountCompanyAddressKanji {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for CreateTokenAccountCompanyAddressKanji {
    fn default() -> Self {
        Self::new()
    }
}
/// This hash is used to attest that the directors information provided to Stripe is both current and correct.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenAccountCompanyDirectorshipDeclaration {
    /// The Unix timestamp marking when the directorship declaration attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the directorship declaration attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The user agent of the browser from which the directorship declaration attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
impl CreateTokenAccountCompanyDirectorshipDeclaration {
    pub fn new() -> Self {
        Self { date: None, ip: None, user_agent: None }
    }
}
impl Default for CreateTokenAccountCompanyDirectorshipDeclaration {
    fn default() -> Self {
        Self::new()
    }
}
/// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenAccountCompanyOwnershipDeclaration {
    /// The Unix timestamp marking when the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The user agent of the browser from which the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
impl CreateTokenAccountCompanyOwnershipDeclaration {
    pub fn new() -> Self {
        Self { date: None, ip: None, user_agent: None }
    }
}
impl Default for CreateTokenAccountCompanyOwnershipDeclaration {
    fn default() -> Self {
        Self::new()
    }
}
/// This value is used to determine if a business is exempt from providing ultimate beneficial owners.
/// See [this support article](https://support.stripe.com/questions/exemption-from-providing-ownership-details) and [changelog](https://docs.stripe.com/changelog/acacia/2025-01-27/ownership-exemption-reason-accounts-api) for more details.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTokenAccountCompanyOwnershipExemptionReason {
    QualifiedEntityExceedsOwnershipThreshold,
    QualifiesAsFinancialInstitution,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTokenAccountCompanyOwnershipExemptionReason {
    pub fn as_str(&self) -> &str {
        use CreateTokenAccountCompanyOwnershipExemptionReason::*;
        match self {
            QualifiedEntityExceedsOwnershipThreshold => {
                "qualified_entity_exceeds_ownership_threshold"
            }
            QualifiesAsFinancialInstitution => "qualifies_as_financial_institution",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTokenAccountCompanyOwnershipExemptionReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTokenAccountCompanyOwnershipExemptionReason::*;
        match s {
            "qualified_entity_exceeds_ownership_threshold" => {
                Ok(QualifiedEntityExceedsOwnershipThreshold)
            }
            "qualifies_as_financial_institution" => Ok(QualifiesAsFinancialInstitution),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTokenAccountCompanyOwnershipExemptionReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateTokenAccountCompanyOwnershipExemptionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTokenAccountCompanyOwnershipExemptionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTokenAccountCompanyOwnershipExemptionReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTokenAccountCompanyOwnershipExemptionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// When the business was incorporated or registered.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenAccountCompanyRegistrationDate {
    /// The day of registration, between 1 and 31.
    pub day: i64,
    /// The month of registration, between 1 and 12.
    pub month: i64,
    /// The four-digit year of registration.
    pub year: i64,
}
impl CreateTokenAccountCompanyRegistrationDate {
    pub fn new(day: impl Into<i64>, month: impl Into<i64>, year: impl Into<i64>) -> Self {
        Self { day: day.into(), month: month.into(), year: year.into() }
    }
}
/// This hash is used to attest that the representative is authorized to act as the representative of their legal entity.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenAccountCompanyRepresentativeDeclaration {
    /// The Unix timestamp marking when the representative declaration attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the representative declaration attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The user agent of the browser from which the representative declaration attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
impl CreateTokenAccountCompanyRepresentativeDeclaration {
    pub fn new() -> Self {
        Self { date: None, ip: None, user_agent: None }
    }
}
impl Default for CreateTokenAccountCompanyRepresentativeDeclaration {
    fn default() -> Self {
        Self::new()
    }
}
/// The category identifying the legal structure of the company or legal entity.
/// See [Business structure](/connect/identity-verification#business-structure) for more details.
/// Pass an empty string to unset this value.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTokenAccountCompanyStructure {
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
    Unknown(String),
}
impl CreateTokenAccountCompanyStructure {
    pub fn as_str(&self) -> &str {
        use CreateTokenAccountCompanyStructure::*;
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
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTokenAccountCompanyStructure {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTokenAccountCompanyStructure::*;
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
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTokenAccountCompanyStructure"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateTokenAccountCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTokenAccountCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTokenAccountCompanyStructure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTokenAccountCompanyStructure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Information on the verification state of the company.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenAccountCompanyVerification {
    /// A document verifying the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<CreateTokenAccountCompanyVerificationDocument>,
}
impl CreateTokenAccountCompanyVerification {
    pub fn new() -> Self {
        Self { document: None }
    }
}
impl Default for CreateTokenAccountCompanyVerification {
    fn default() -> Self {
        Self::new()
    }
}
/// A document verifying the business.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenAccountCompanyVerificationDocument {
    /// The back of a document returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `additional_verification`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,
    /// The front of a document returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `additional_verification`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}
impl CreateTokenAccountCompanyVerificationDocument {
    pub fn new() -> Self {
        Self { back: None, front: None }
    }
}
impl Default for CreateTokenAccountCompanyVerificationDocument {
    fn default() -> Self {
        Self::new()
    }
}
/// Information about the person represented by the account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTokenAccountIndividual {
    /// The individual's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateTokenAccountIndividualAddress>,
    /// The Kana variation of the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<CreateTokenAccountIndividualAddressKana>,
    /// The Kanji variation of the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<CreateTokenAccountIndividualAddressKanji>,
    /// The individual's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirthSpecs>,
    /// The individual's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The individual's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The Kana variation of the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,
    /// The Kanji variation of the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,
    /// A list of alternate names or aliases that the individual is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<Vec<String>>,
    /// The individual's gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// The government-issued ID number of the individual, as appropriate for the representative's country.
    /// (Examples are a Social Security Number in the U.S., or a Social Insurance Number in Canada).
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    /// The government-issued secondary ID number of the individual, as appropriate for the representative's country, will be used for enhanced verification checks.
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary: Option<String>,
    /// The individual's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The Kana variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,
    /// The Kanji variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,
    /// The individual's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The individual's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<CreateTokenAccountIndividualPoliticalExposure>,
    /// The individual's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<CreateTokenAccountIndividualRegisteredAddress>,
    /// Describes the person’s relationship to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<CreateTokenAccountIndividualRelationship>,
    /// The last four digits of the individual's Social Security Number (U.S. only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<String>,
    /// The individual's verification document information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationSpecs>,
}
impl CreateTokenAccountIndividual {
    pub fn new() -> Self {
        Self {
            address: None,
            address_kana: None,
            address_kanji: None,
            dob: None,
            email: None,
            first_name: None,
            first_name_kana: None,
            first_name_kanji: None,
            full_name_aliases: None,
            gender: None,
            id_number: None,
            id_number_secondary: None,
            last_name: None,
            last_name_kana: None,
            last_name_kanji: None,
            maiden_name: None,
            metadata: None,
            phone: None,
            political_exposure: None,
            registered_address: None,
            relationship: None,
            ssn_last_4: None,
            verification: None,
        }
    }
}
impl Default for CreateTokenAccountIndividual {
    fn default() -> Self {
        Self::new()
    }
}
/// The individual's primary address.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenAccountIndividualAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl CreateTokenAccountIndividualAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateTokenAccountIndividualAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kana variation of the individual's primary address (Japan only).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenAccountIndividualAddressKana {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
impl CreateTokenAccountIndividualAddressKana {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for CreateTokenAccountIndividualAddressKana {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the individual's primary address (Japan only).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenAccountIndividualAddressKanji {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
impl CreateTokenAccountIndividualAddressKanji {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for CreateTokenAccountIndividualAddressKanji {
    fn default() -> Self {
        Self::new()
    }
}
/// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTokenAccountIndividualPoliticalExposure {
    Existing,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTokenAccountIndividualPoliticalExposure {
    pub fn as_str(&self) -> &str {
        use CreateTokenAccountIndividualPoliticalExposure::*;
        match self {
            Existing => "existing",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTokenAccountIndividualPoliticalExposure {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTokenAccountIndividualPoliticalExposure::*;
        match s {
            "existing" => Ok(Existing),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTokenAccountIndividualPoliticalExposure"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateTokenAccountIndividualPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTokenAccountIndividualPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTokenAccountIndividualPoliticalExposure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTokenAccountIndividualPoliticalExposure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The individual's registered address.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenAccountIndividualRegisteredAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl CreateTokenAccountIndividualRegisteredAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateTokenAccountIndividualRegisteredAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// Describes the person’s relationship to the account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTokenAccountIndividualRelationship {
    /// Whether the person is a director of the account's legal entity.
    /// Directors are typically members of the governing board of the company, or responsible for ensuring the company meets its regulatory obligations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    /// Whether the person has significant responsibility to control, manage, or direct the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    /// Whether the person is an owner of the account’s legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    /// The percent owned by the person of the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_ownership: Option<f64>,
    /// The person's title (e.g., CEO, Support Engineer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl CreateTokenAccountIndividualRelationship {
    pub fn new() -> Self {
        Self { director: None, executive: None, owner: None, percent_ownership: None, title: None }
    }
}
impl Default for CreateTokenAccountIndividualRelationship {
    fn default() -> Self {
        Self::new()
    }
}
/// The bank account this token will represent.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenBankAccount {
    /// The name of the person or business that owns the bank account.
    /// This field is required when attaching the bank account to a `Customer` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    /// The type of entity that holds the account.
    /// It can be `company` or `individual`.
    /// This field is required when attaching the bank account to a `Customer` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<CreateTokenBankAccountAccountHolderType>,
    /// The account number for the bank account, in string form. Must be a checking account.
    pub account_number: String,
    /// The bank account type.
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<CreateTokenBankAccountAccountType>,
    /// The country in which the bank account is located.
    pub country: String,
    /// The currency the bank account is in.
    /// This must be a country/currency pairing that [Stripe supports.](https://docs.stripe.com/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// The ID of a Payment Method with a `type` of `us_bank_account`.
    /// The Payment Method's bank account information will be copied and returned as a Bank Account Token.
    /// This parameter is exclusive with respect to all other parameters in the `bank_account` hash.
    /// You must include the top-level `customer` parameter if the Payment Method is attached to a `Customer` object.
    /// If the Payment Method is not attached to a `Customer` object, it will be consumed and cannot be used again.
    /// You may not use Payment Methods which were created by a Setup Intent with `attach_to_self=true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// The routing number, sort code, or other country-appropriate institution number for the bank account.
    /// For US bank accounts, this is required and should be the ACH routing number, not the wire routing number.
    /// If you are providing an IBAN for `account_number`, this field is not required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}
impl CreateTokenBankAccount {
    pub fn new(account_number: impl Into<String>, country: impl Into<String>) -> Self {
        Self {
            account_holder_name: None,
            account_holder_type: None,
            account_number: account_number.into(),
            account_type: None,
            country: country.into(),
            currency: None,
            payment_method: None,
            routing_number: None,
        }
    }
}
/// The type of entity that holds the account.
/// It can be `company` or `individual`.
/// This field is required when attaching the bank account to a `Customer` object.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTokenBankAccountAccountHolderType {
    Company,
    Individual,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTokenBankAccountAccountHolderType {
    pub fn as_str(&self) -> &str {
        use CreateTokenBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTokenBankAccountAccountHolderType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTokenBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTokenBankAccountAccountHolderType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateTokenBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTokenBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTokenBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTokenBankAccountAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The bank account type.
/// This can only be `checking` or `savings` in most countries.
/// In Japan, this can only be `futsu` or `toza`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTokenBankAccountAccountType {
    Checking,
    Futsu,
    Savings,
    Toza,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTokenBankAccountAccountType {
    pub fn as_str(&self) -> &str {
        use CreateTokenBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Futsu => "futsu",
            Savings => "savings",
            Toza => "toza",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTokenBankAccountAccountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTokenBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "futsu" => Ok(Futsu),
            "savings" => Ok(Savings),
            "toza" => Ok(Toza),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTokenBankAccountAccountType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateTokenBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTokenBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTokenBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTokenBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The card this token will represent.
/// If you also pass in a customer, the card must be the ID of a card belonging to the customer.
/// Otherwise, if you do not pass in a customer, this is a dictionary containing a user's credit card details, with the options described below.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateTokenCard {
    #[serde(untagged)]
    CreditCardSpecs(CreateTokenCreditCardSpecs),
    #[serde(untagged)]
    String(String),
}
/// The card this token will represent.
/// If you also pass in a customer, the card must be the ID of a card belonging to the customer.
/// Otherwise, if you do not pass in a customer, this is a dictionary containing a user's credit card details, with the options described below.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenCreditCardSpecs {
    /// City / District / Suburb / Town / Village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<String>,
    /// Billing address country, if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<String>,
    /// Address line 1 (Street address / PO Box / Company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    /// Address line 2 (Apartment / Suite / Unit / Building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    /// State / County / Province / Region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<String>,
    /// Required in order to add the card to an account; in all other cases, this parameter is not used.
    /// When added to an account, the card (which must be a debit card) can be used as a transfer destination for funds in this currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// Card security code. Highly recommended to always include this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,
    /// Two-digit number representing the card's expiration month.
    pub exp_month: String,
    /// Two- or four-digit number representing the card's expiration year.
    pub exp_year: String,
    /// Cardholder's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Contains information about card networks used to process the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<CreateTokenCreditCardSpecsNetworks>,
    /// The card number, as a string without any separators.
    pub number: String,
}
impl CreateTokenCreditCardSpecs {
    pub fn new(
        exp_month: impl Into<String>,
        exp_year: impl Into<String>,
        number: impl Into<String>,
    ) -> Self {
        Self {
            address_city: None,
            address_country: None,
            address_line1: None,
            address_line2: None,
            address_state: None,
            address_zip: None,
            currency: None,
            cvc: None,
            exp_month: exp_month.into(),
            exp_year: exp_year.into(),
            name: None,
            networks: None,
            number: number.into(),
        }
    }
}
/// Contains information about card networks used to process the payment.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenCreditCardSpecsNetworks {
    /// The customer's preferred card network for co-branded cards.
    /// Supports `cartes_bancaires`, `mastercard`, or `visa`.
    /// Selection of a network that does not apply to the card will be stored as `invalid_preference` on the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<CreateTokenCreditCardSpecsNetworksPreferred>,
}
impl CreateTokenCreditCardSpecsNetworks {
    pub fn new() -> Self {
        Self { preferred: None }
    }
}
impl Default for CreateTokenCreditCardSpecsNetworks {
    fn default() -> Self {
        Self::new()
    }
}
/// The customer's preferred card network for co-branded cards.
/// Supports `cartes_bancaires`, `mastercard`, or `visa`.
/// Selection of a network that does not apply to the card will be stored as `invalid_preference` on the card.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTokenCreditCardSpecsNetworksPreferred {
    CartesBancaires,
    Mastercard,
    Visa,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTokenCreditCardSpecsNetworksPreferred {
    pub fn as_str(&self) -> &str {
        use CreateTokenCreditCardSpecsNetworksPreferred::*;
        match self {
            CartesBancaires => "cartes_bancaires",
            Mastercard => "mastercard",
            Visa => "visa",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTokenCreditCardSpecsNetworksPreferred {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTokenCreditCardSpecsNetworksPreferred::*;
        match s {
            "cartes_bancaires" => Ok(CartesBancaires),
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTokenCreditCardSpecsNetworksPreferred"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateTokenCreditCardSpecsNetworksPreferred {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTokenCreditCardSpecsNetworksPreferred {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTokenCreditCardSpecsNetworksPreferred {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTokenCreditCardSpecsNetworksPreferred {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The updated CVC value this token represents.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenCvcUpdate {
    /// The CVC value, in string form.
    pub cvc: String,
}
impl CreateTokenCvcUpdate {
    pub fn new(cvc: impl Into<String>) -> Self {
        Self { cvc: cvc.into() }
    }
}
/// Information for the person this token represents.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTokenPerson {
    /// Details on the legal guardian's or authorizer's acceptance of the required Stripe agreements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_tos_acceptances: Option<CreateTokenPersonAdditionalTosAcceptances>,
    /// The person's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateTokenPersonAddress>,
    /// The Kana variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<CreateTokenPersonAddressKana>,
    /// The Kanji variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<CreateTokenPersonAddressKanji>,
    /// The person's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirthSpecs>,
    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<CreateTokenPersonDocuments>,
    /// The person's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The person's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The Kana variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,
    /// The Kanji variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,
    /// A list of alternate names or aliases that the person is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<Vec<String>>,
    /// The person's gender (International regulations require either "male" or "female").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// The person's ID number, as appropriate for their country.
    /// For example, a social security number in the U.S., social insurance number in Canada, etc.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://docs.stripe.com/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    /// The person's secondary ID number, as appropriate for their country, will be used for enhanced verification checks.
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://docs.stripe.com/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary: Option<String>,
    /// The person's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The Kana variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,
    /// The Kanji variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,
    /// The person's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The country where the person is a national.
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)), or "XX" if unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// The person's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<CreateTokenPersonPoliticalExposure>,
    /// The person's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<CreateTokenPersonRegisteredAddress>,
    /// The relationship that this person has with the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<CreateTokenPersonRelationship>,
    /// The last four digits of the person's Social Security number (U.S. only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<String>,
    /// Demographic data related to the person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_cfpb_data: Option<CreateTokenPersonUsCfpbData>,
    /// The person's verification status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationSpecs>,
}
impl CreateTokenPerson {
    pub fn new() -> Self {
        Self {
            additional_tos_acceptances: None,
            address: None,
            address_kana: None,
            address_kanji: None,
            dob: None,
            documents: None,
            email: None,
            first_name: None,
            first_name_kana: None,
            first_name_kanji: None,
            full_name_aliases: None,
            gender: None,
            id_number: None,
            id_number_secondary: None,
            last_name: None,
            last_name_kana: None,
            last_name_kanji: None,
            maiden_name: None,
            metadata: None,
            nationality: None,
            phone: None,
            political_exposure: None,
            registered_address: None,
            relationship: None,
            ssn_last_4: None,
            us_cfpb_data: None,
            verification: None,
        }
    }
}
impl Default for CreateTokenPerson {
    fn default() -> Self {
        Self::new()
    }
}
/// Details on the legal guardian's or authorizer's acceptance of the required Stripe agreements.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenPersonAdditionalTosAcceptances {
    /// Details on the legal guardian's acceptance of the main Stripe service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<CreateTokenPersonAdditionalTosAcceptancesAccount>,
}
impl CreateTokenPersonAdditionalTosAcceptances {
    pub fn new() -> Self {
        Self { account: None }
    }
}
impl Default for CreateTokenPersonAdditionalTosAcceptances {
    fn default() -> Self {
        Self::new()
    }
}
/// Details on the legal guardian's acceptance of the main Stripe service agreement.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenPersonAdditionalTosAcceptancesAccount {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
impl CreateTokenPersonAdditionalTosAcceptancesAccount {
    pub fn new() -> Self {
        Self { date: None, ip: None, user_agent: None }
    }
}
impl Default for CreateTokenPersonAdditionalTosAcceptancesAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// The person's address.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenPersonAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl CreateTokenPersonAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateTokenPersonAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kana variation of the person's address (Japan only).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenPersonAddressKana {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
impl CreateTokenPersonAddressKana {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for CreateTokenPersonAddressKana {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the person's address (Japan only).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenPersonAddressKanji {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
impl CreateTokenPersonAddressKanji {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for CreateTokenPersonAddressKanji {
    fn default() -> Self {
        Self::new()
    }
}
/// Documents that may be submitted to satisfy various informational requests.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenPersonDocuments {
    /// One or more documents that demonstrate proof that this person is authorized to represent the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_authorization: Option<DocumentsParam>,
    /// One or more documents showing the person's passport page with photo and personal data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport: Option<DocumentsParam>,
    /// One or more documents showing the person's visa required for living in the country where they are residing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<DocumentsParam>,
}
impl CreateTokenPersonDocuments {
    pub fn new() -> Self {
        Self { company_authorization: None, passport: None, visa: None }
    }
}
impl Default for CreateTokenPersonDocuments {
    fn default() -> Self {
        Self::new()
    }
}
/// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTokenPersonPoliticalExposure {
    Existing,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTokenPersonPoliticalExposure {
    pub fn as_str(&self) -> &str {
        use CreateTokenPersonPoliticalExposure::*;
        match self {
            Existing => "existing",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTokenPersonPoliticalExposure {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTokenPersonPoliticalExposure::*;
        match s {
            "existing" => Ok(Existing),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTokenPersonPoliticalExposure"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateTokenPersonPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTokenPersonPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTokenPersonPoliticalExposure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTokenPersonPoliticalExposure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The person's registered address.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenPersonRegisteredAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl CreateTokenPersonRegisteredAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateTokenPersonRegisteredAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// The relationship that this person has with the account's legal entity.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTokenPersonRelationship {
    /// Whether the person is the authorizer of the account's representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer: Option<bool>,
    /// Whether the person is a director of the account's legal entity.
    /// Directors are typically members of the governing board of the company, or responsible for ensuring the company meets its regulatory obligations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    /// Whether the person has significant responsibility to control, manage, or direct the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    /// Whether the person is the legal guardian of the account's representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_guardian: Option<bool>,
    /// Whether the person is an owner of the account’s legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    /// The percent owned by the person of the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_ownership: Option<f64>,
    /// Whether the person is authorized as the primary representative of the account.
    /// This is the person nominated by the business to provide information about themselves, and general information about the account.
    /// There can only be one representative at any given time.
    /// At the time the account is created, this person should be set to the person responsible for opening the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,
    /// The person's title (e.g., CEO, Support Engineer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl CreateTokenPersonRelationship {
    pub fn new() -> Self {
        Self {
            authorizer: None,
            director: None,
            executive: None,
            legal_guardian: None,
            owner: None,
            percent_ownership: None,
            representative: None,
            title: None,
        }
    }
}
impl Default for CreateTokenPersonRelationship {
    fn default() -> Self {
        Self::new()
    }
}
/// Demographic data related to the person.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenPersonUsCfpbData {
    /// The persons ethnicity details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity_details: Option<CreateTokenPersonUsCfpbDataEthnicityDetails>,
    /// The persons race details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race_details: Option<CreateTokenPersonUsCfpbDataRaceDetails>,
    /// The persons self-identified gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_identified_gender: Option<String>,
}
impl CreateTokenPersonUsCfpbData {
    pub fn new() -> Self {
        Self { ethnicity_details: None, race_details: None, self_identified_gender: None }
    }
}
impl Default for CreateTokenPersonUsCfpbData {
    fn default() -> Self {
        Self::new()
    }
}
/// The persons ethnicity details
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenPersonUsCfpbDataEthnicityDetails {
    /// The persons ethnicity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<Vec<CreateTokenPersonUsCfpbDataEthnicityDetailsEthnicity>>,
    /// Please specify your origin, when other is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity_other: Option<String>,
}
impl CreateTokenPersonUsCfpbDataEthnicityDetails {
    pub fn new() -> Self {
        Self { ethnicity: None, ethnicity_other: None }
    }
}
impl Default for CreateTokenPersonUsCfpbDataEthnicityDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// The persons ethnicity
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTokenPersonUsCfpbDataEthnicityDetailsEthnicity {
    Cuban,
    HispanicOrLatino,
    Mexican,
    NotHispanicOrLatino,
    OtherHispanicOrLatino,
    PreferNotToAnswer,
    PuertoRican,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTokenPersonUsCfpbDataEthnicityDetailsEthnicity {
    pub fn as_str(&self) -> &str {
        use CreateTokenPersonUsCfpbDataEthnicityDetailsEthnicity::*;
        match self {
            Cuban => "cuban",
            HispanicOrLatino => "hispanic_or_latino",
            Mexican => "mexican",
            NotHispanicOrLatino => "not_hispanic_or_latino",
            OtherHispanicOrLatino => "other_hispanic_or_latino",
            PreferNotToAnswer => "prefer_not_to_answer",
            PuertoRican => "puerto_rican",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTokenPersonUsCfpbDataEthnicityDetailsEthnicity {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTokenPersonUsCfpbDataEthnicityDetailsEthnicity::*;
        match s {
            "cuban" => Ok(Cuban),
            "hispanic_or_latino" => Ok(HispanicOrLatino),
            "mexican" => Ok(Mexican),
            "not_hispanic_or_latino" => Ok(NotHispanicOrLatino),
            "other_hispanic_or_latino" => Ok(OtherHispanicOrLatino),
            "prefer_not_to_answer" => Ok(PreferNotToAnswer),
            "puerto_rican" => Ok(PuertoRican),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTokenPersonUsCfpbDataEthnicityDetailsEthnicity"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateTokenPersonUsCfpbDataEthnicityDetailsEthnicity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTokenPersonUsCfpbDataEthnicityDetailsEthnicity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTokenPersonUsCfpbDataEthnicityDetailsEthnicity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTokenPersonUsCfpbDataEthnicityDetailsEthnicity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The persons race details
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenPersonUsCfpbDataRaceDetails {
    /// The persons race.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race: Option<Vec<CreateTokenPersonUsCfpbDataRaceDetailsRace>>,
    /// Please specify your race, when other is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race_other: Option<String>,
}
impl CreateTokenPersonUsCfpbDataRaceDetails {
    pub fn new() -> Self {
        Self { race: None, race_other: None }
    }
}
impl Default for CreateTokenPersonUsCfpbDataRaceDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// The persons race.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTokenPersonUsCfpbDataRaceDetailsRace {
    AfricanAmerican,
    AmericanIndianOrAlaskaNative,
    Asian,
    AsianIndian,
    BlackOrAfricanAmerican,
    Chinese,
    Ethiopian,
    Filipino,
    GuamanianOrChamorro,
    Haitian,
    Jamaican,
    Japanese,
    Korean,
    NativeHawaiian,
    NativeHawaiianOrOtherPacificIslander,
    Nigerian,
    OtherAsian,
    OtherBlackOrAfricanAmerican,
    OtherPacificIslander,
    PreferNotToAnswer,
    Samoan,
    Somali,
    Vietnamese,
    White,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTokenPersonUsCfpbDataRaceDetailsRace {
    pub fn as_str(&self) -> &str {
        use CreateTokenPersonUsCfpbDataRaceDetailsRace::*;
        match self {
            AfricanAmerican => "african_american",
            AmericanIndianOrAlaskaNative => "american_indian_or_alaska_native",
            Asian => "asian",
            AsianIndian => "asian_indian",
            BlackOrAfricanAmerican => "black_or_african_american",
            Chinese => "chinese",
            Ethiopian => "ethiopian",
            Filipino => "filipino",
            GuamanianOrChamorro => "guamanian_or_chamorro",
            Haitian => "haitian",
            Jamaican => "jamaican",
            Japanese => "japanese",
            Korean => "korean",
            NativeHawaiian => "native_hawaiian",
            NativeHawaiianOrOtherPacificIslander => "native_hawaiian_or_other_pacific_islander",
            Nigerian => "nigerian",
            OtherAsian => "other_asian",
            OtherBlackOrAfricanAmerican => "other_black_or_african_american",
            OtherPacificIslander => "other_pacific_islander",
            PreferNotToAnswer => "prefer_not_to_answer",
            Samoan => "samoan",
            Somali => "somali",
            Vietnamese => "vietnamese",
            White => "white",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTokenPersonUsCfpbDataRaceDetailsRace {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTokenPersonUsCfpbDataRaceDetailsRace::*;
        match s {
            "african_american" => Ok(AfricanAmerican),
            "american_indian_or_alaska_native" => Ok(AmericanIndianOrAlaskaNative),
            "asian" => Ok(Asian),
            "asian_indian" => Ok(AsianIndian),
            "black_or_african_american" => Ok(BlackOrAfricanAmerican),
            "chinese" => Ok(Chinese),
            "ethiopian" => Ok(Ethiopian),
            "filipino" => Ok(Filipino),
            "guamanian_or_chamorro" => Ok(GuamanianOrChamorro),
            "haitian" => Ok(Haitian),
            "jamaican" => Ok(Jamaican),
            "japanese" => Ok(Japanese),
            "korean" => Ok(Korean),
            "native_hawaiian" => Ok(NativeHawaiian),
            "native_hawaiian_or_other_pacific_islander" => Ok(NativeHawaiianOrOtherPacificIslander),
            "nigerian" => Ok(Nigerian),
            "other_asian" => Ok(OtherAsian),
            "other_black_or_african_american" => Ok(OtherBlackOrAfricanAmerican),
            "other_pacific_islander" => Ok(OtherPacificIslander),
            "prefer_not_to_answer" => Ok(PreferNotToAnswer),
            "samoan" => Ok(Samoan),
            "somali" => Ok(Somali),
            "vietnamese" => Ok(Vietnamese),
            "white" => Ok(White),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTokenPersonUsCfpbDataRaceDetailsRace"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateTokenPersonUsCfpbDataRaceDetailsRace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTokenPersonUsCfpbDataRaceDetailsRace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTokenPersonUsCfpbDataRaceDetailsRace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTokenPersonUsCfpbDataRaceDetailsRace {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The PII this token represents.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateTokenPii {
    /// The `id_number` for the PII, in string form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
}
impl CreateTokenPii {
    pub fn new() -> Self {
        Self { id_number: None }
    }
}
impl Default for CreateTokenPii {
    fn default() -> Self {
        Self::new()
    }
}
/// Creates a single-use token that represents a bank account’s details.
/// You can use this token with any v1 API method in place of a bank account dictionary.
/// You can only use this token once.
/// To do so, attach it to a [connected account](https://stripe.com/docs/api#accounts) where <a href="/api/accounts/object#account_object-controller-requirement_collection">controller.requirement_collection</a> is `application`, which includes Custom accounts.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateToken {
    inner: CreateTokenBuilder,
}
impl CreateToken {
    /// Construct a new `CreateToken`.
    pub fn new() -> Self {
        Self { inner: CreateTokenBuilder::new() }
    }
    /// Information for the account this token represents.
    pub fn account(mut self, account: impl Into<CreateTokenAccount>) -> Self {
        self.inner.account = Some(account.into());
        self
    }
    /// The bank account this token will represent.
    pub fn bank_account(mut self, bank_account: impl Into<CreateTokenBankAccount>) -> Self {
        self.inner.bank_account = Some(bank_account.into());
        self
    }
    /// The card this token will represent.
    /// If you also pass in a customer, the card must be the ID of a card belonging to the customer.
    /// Otherwise, if you do not pass in a customer, this is a dictionary containing a user's credit card details, with the options described below.
    pub fn card(mut self, card: impl Into<CreateTokenCard>) -> Self {
        self.inner.card = Some(card.into());
        self
    }
    /// Create a token for the customer, which is owned by the application's account.
    /// You can only use this with an [OAuth access token](https://docs.stripe.com/connect/standard-accounts) or [Stripe-Account header](https://docs.stripe.com/connect/authentication).
    /// Learn more about [cloning saved payment methods](https://docs.stripe.com/connect/cloning-saved-payment-methods).
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// The updated CVC value this token represents.
    pub fn cvc_update(mut self, cvc_update: impl Into<CreateTokenCvcUpdate>) -> Self {
        self.inner.cvc_update = Some(cvc_update.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Information for the person this token represents.
    pub fn person(mut self, person: impl Into<CreateTokenPerson>) -> Self {
        self.inner.person = Some(person.into());
        self
    }
    /// The PII this token represents.
    pub fn pii(mut self, pii: impl Into<CreateTokenPii>) -> Self {
        self.inner.pii = Some(pii.into());
        self
    }
}
impl Default for CreateToken {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateToken {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateToken {
    type Output = stripe_core::Token;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/tokens").form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct DateOfBirthSpecs {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl DateOfBirthSpecs {
    pub fn new(day: impl Into<i64>, month: impl Into<i64>, year: impl Into<i64>) -> Self {
        Self { day: day.into(), month: month.into(), year: year.into() }
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct PersonVerificationDocumentSpecs {
    /// The back of an ID returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `identity_document`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,
    /// The front of an ID returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `identity_document`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}
impl PersonVerificationDocumentSpecs {
    pub fn new() -> Self {
        Self { back: None, front: None }
    }
}
impl Default for PersonVerificationDocumentSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct DocumentsParam {
    /// One or more document ids returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}
impl DocumentsParam {
    pub fn new() -> Self {
        Self { files: None }
    }
}
impl Default for DocumentsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct PersonVerificationSpecs {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<PersonVerificationDocumentSpecs>,
    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PersonVerificationDocumentSpecs>,
}
impl PersonVerificationSpecs {
    pub fn new() -> Self {
        Self { additional_document: None, document: None }
    }
}
impl Default for PersonVerificationSpecs {
    fn default() -> Self {
        Self::new()
    }
}
