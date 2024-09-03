use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveTokenBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTokenBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the token with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveToken<'a> {
    inner: RetrieveTokenBuilder<'a>,
    token: &'a stripe_core::TokenId,
}
impl<'a> RetrieveToken<'a> {
    /// Construct a new `RetrieveToken`.
    pub fn new(token: &'a stripe_core::TokenId) -> Self {
        Self { token, inner: RetrieveTokenBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveToken<'_> {
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

impl StripeRequest for RetrieveToken<'_> {
    type Output = stripe_core::Token;

    fn build(&self) -> RequestBuilder {
        let token = self.token;
        RequestBuilder::new(StripeMethod::Get, format!("/tokens/{token}")).query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateTokenBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<CreateTokenAccount<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bank_account: Option<CreateTokenBankAccount<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<CreateTokenCard<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cvc_update: Option<CreateTokenCvcUpdate<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    person: Option<CreateTokenPerson<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pii: Option<CreateTokenPii<'a>>,
}
impl<'a> CreateTokenBuilder<'a> {
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenAccount<'a> {
    /// The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<CreateTokenAccountBusinessType>,
    /// Information about the company or business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CreateTokenAccountCompany<'a>>,
    /// Information about the person represented by the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<CreateTokenAccountIndividual<'a>>,
    /// Whether the user described by the data in the token has been shown [the Stripe Connected Account Agreement](https://docs.stripe.com/connect/account-tokens#stripe-connected-account-agreement).
    /// When creating an account token to create a new Connect account, this value must be `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}
impl<'a> CreateTokenAccount<'a> {
    pub fn new() -> Self {
        Self { business_type: None, company: None, individual: None, tos_shown_and_accepted: None }
    }
}
impl<'a> Default for CreateTokenAccount<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The business type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTokenAccountBusinessType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
}
impl CreateTokenAccountBusinessType {
    pub fn as_str(self) -> &'static str {
        use CreateTokenAccountBusinessType::*;
        match self {
            Company => "company",
            GovernmentEntity => "government_entity",
            Individual => "individual",
            NonProfit => "non_profit",
        }
    }
}

impl std::str::FromStr for CreateTokenAccountBusinessType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTokenAccountBusinessType::*;
        match s {
            "company" => Ok(Company),
            "government_entity" => Ok(GovernmentEntity),
            "individual" => Ok(Individual),
            "non_profit" => Ok(NonProfit),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTokenAccountBusinessType")
        })
    }
}
/// Information about the company or business.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenAccountCompany<'a> {
    /// The company's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressSpecs<'a>>,
    /// The Kana variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<CreateTokenAccountCompanyAddressKana<'a>>,
    /// The Kanji variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<CreateTokenAccountCompanyAddressKanji<'a>>,
    /// Whether the company's directors have been provided.
    /// Set this Boolean to `true` after creating all the company's directors with [the Persons API](https://docs.stripe.com/api/persons) for accounts with a `relationship.director` requirement.
    /// This value is not automatically set to `true` after creating directors, so it needs to be updated to indicate all directors have been provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directors_provided: Option<bool>,
    /// Whether the company's executives have been provided.
    /// Set this Boolean to `true` after creating all the company's executives with [the Persons API](https://docs.stripe.com/api/persons) for accounts with a `relationship.executive` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executives_provided: Option<bool>,
    /// The export license ID number of the company, also referred as Import Export Code (India only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_license_id: Option<&'a str>,
    /// The purpose code to use for export transactions (India only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_purpose_code: Option<&'a str>,
    /// The company's legal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// The Kana variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<&'a str>,
    /// The Kanji variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kanji: Option<&'a str>,
    /// Whether the company's owners have been provided.
    /// Set this Boolean to `true` after creating all the company's owners with [the Persons API](https://docs.stripe.com/api/persons) for accounts with a `relationship.owner` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,
    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration: Option<CreateTokenAccountCompanyOwnershipDeclaration<'a>>,
    /// Whether the user described by the data in the token has been shown the Ownership Declaration and indicated that it is correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration_shown_and_signed: Option<bool>,
    /// The company's phone number (used for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// The identification number given to a company when it is registered or incorporated, if distinct from the identification number used for filing taxes.
    /// (Examples are the CIN for companies and LLP IN for partnerships in India, and the Company Registration Number in Hong Kong).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<&'a str>,
    /// The category identifying the legal structure of the company or legal entity.
    /// See [Business structure](https://docs.stripe.com/connect/identity-verification#business-structure) for more details.
    /// Pass an empty string to unset this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<CreateTokenAccountCompanyStructure>,
    /// The business ID number of the company, as appropriate for the company’s country.
    /// (Examples are an Employer ID Number in the U.S., a Business Number in Canada, or a Company Number in the UK.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<&'a str>,
    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<&'a str>,
    /// The VAT number of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<&'a str>,
    /// Information on the verification state of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<CreateTokenAccountCompanyVerification<'a>>,
}
impl<'a> CreateTokenAccountCompany<'a> {
    pub fn new() -> Self {
        Self {
            address: None,
            address_kana: None,
            address_kanji: None,
            directors_provided: None,
            executives_provided: None,
            export_license_id: None,
            export_purpose_code: None,
            name: None,
            name_kana: None,
            name_kanji: None,
            owners_provided: None,
            ownership_declaration: None,
            ownership_declaration_shown_and_signed: None,
            phone: None,
            registration_number: None,
            structure: None,
            tax_id: None,
            tax_id_registrar: None,
            vat_id: None,
            verification: None,
        }
    }
}
impl<'a> Default for CreateTokenAccountCompany<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kana variation of the company's primary address (Japan only).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenAccountCompanyAddressKana<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> CreateTokenAccountCompanyAddressKana<'a> {
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
impl<'a> Default for CreateTokenAccountCompanyAddressKana<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the company's primary address (Japan only).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenAccountCompanyAddressKanji<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> CreateTokenAccountCompanyAddressKanji<'a> {
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
impl<'a> Default for CreateTokenAccountCompanyAddressKanji<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenAccountCompanyOwnershipDeclaration<'a> {
    /// The Unix timestamp marking when the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<&'a str>,
    /// The user agent of the browser from which the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> CreateTokenAccountCompanyOwnershipDeclaration<'a> {
    pub fn new() -> Self {
        Self { date: None, ip: None, user_agent: None }
    }
}
impl<'a> Default for CreateTokenAccountCompanyOwnershipDeclaration<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The category identifying the legal structure of the company or legal entity.
/// See [Business structure](https://docs.stripe.com/connect/identity-verification#business-structure) for more details.
/// Pass an empty string to unset this value.
#[derive(Copy, Clone, Eq, PartialEq)]
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
    Unknown,
}
impl CreateTokenAccountCompanyStructure {
    pub fn as_str(self) -> &'static str {
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
            Unknown => "unknown",
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
            _ => Ok(Self::Unknown),
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
        Ok(Self::from_str(&s).unwrap())
    }
}
/// Information on the verification state of the company.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenAccountCompanyVerification<'a> {
    /// A document verifying the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<CreateTokenAccountCompanyVerificationDocument<'a>>,
}
impl<'a> CreateTokenAccountCompanyVerification<'a> {
    pub fn new() -> Self {
        Self { document: None }
    }
}
impl<'a> Default for CreateTokenAccountCompanyVerification<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// A document verifying the business.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenAccountCompanyVerificationDocument<'a> {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<&'a str>,
    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<&'a str>,
}
impl<'a> CreateTokenAccountCompanyVerificationDocument<'a> {
    pub fn new() -> Self {
        Self { back: None, front: None }
    }
}
impl<'a> Default for CreateTokenAccountCompanyVerificationDocument<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Information about the person represented by the account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenAccountIndividual<'a> {
    /// The individual's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressSpecs<'a>>,
    /// The Kana variation of the the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<CreateTokenAccountIndividualAddressKana<'a>>,
    /// The Kanji variation of the the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<CreateTokenAccountIndividualAddressKanji<'a>>,
    /// The individual's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirthSpecs>,
    /// The individual's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// The individual's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    /// The Kana variation of the the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<&'a str>,
    /// The Kanji variation of the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<&'a str>,
    /// A list of alternate names or aliases that the individual is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<&'a [&'a str]>,
    /// The individual's gender (International regulations require either "male" or "female").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<&'a str>,
    /// The government-issued ID number of the individual, as appropriate for the representative's country.
    /// (Examples are a Social Security Number in the U.S., or a Social Insurance Number in Canada).
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](https://docs.stripe.com/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<&'a str>,
    /// The government-issued secondary ID number of the individual, as appropriate for the representative's country, will be used for enhanced verification checks.
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](https://docs.stripe.com/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary: Option<&'a str>,
    /// The individual's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    /// The Kana variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<&'a str>,
    /// The Kanji variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<&'a str>,
    /// The individual's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The individual's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<CreateTokenAccountIndividualPoliticalExposure>,
    /// The individual's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<AddressSpecs<'a>>,
    /// Describes the person’s relationship to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<CreateTokenAccountIndividualRelationship<'a>>,
    /// The last four digits of the individual's Social Security Number (U.S. only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<&'a str>,
    /// The individual's verification document information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationSpecs<'a>>,
}
impl<'a> CreateTokenAccountIndividual<'a> {
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
impl<'a> Default for CreateTokenAccountIndividual<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kana variation of the the individual's primary address (Japan only).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenAccountIndividualAddressKana<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> CreateTokenAccountIndividualAddressKana<'a> {
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
impl<'a> Default for CreateTokenAccountIndividualAddressKana<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the the individual's primary address (Japan only).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenAccountIndividualAddressKanji<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> CreateTokenAccountIndividualAddressKanji<'a> {
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
impl<'a> Default for CreateTokenAccountIndividualAddressKanji<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTokenAccountIndividualPoliticalExposure {
    Existing,
    None,
}
impl CreateTokenAccountIndividualPoliticalExposure {
    pub fn as_str(self) -> &'static str {
        use CreateTokenAccountIndividualPoliticalExposure::*;
        match self {
            Existing => "existing",
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateTokenAccountIndividualPoliticalExposure {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTokenAccountIndividualPoliticalExposure::*;
        match s {
            "existing" => Ok(Existing),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateTokenAccountIndividualPoliticalExposure",
            )
        })
    }
}
/// Describes the person’s relationship to the account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenAccountIndividualRelationship<'a> {
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
    pub title: Option<&'a str>,
}
impl<'a> CreateTokenAccountIndividualRelationship<'a> {
    pub fn new() -> Self {
        Self { director: None, executive: None, owner: None, percent_ownership: None, title: None }
    }
}
impl<'a> Default for CreateTokenAccountIndividualRelationship<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The bank account this token will represent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenBankAccount<'a> {
    /// The name of the person or business that owns the bank account.
    /// This field is required when attaching the bank account to a `Customer` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<&'a str>,
    /// The type of entity that holds the account.
    /// It can be `company` or `individual`.
    /// This field is required when attaching the bank account to a `Customer` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<CreateTokenBankAccountAccountHolderType>,
    /// The account number for the bank account, in string form. Must be a checking account.
    pub account_number: &'a str,
    /// The bank account type.
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<CreateTokenBankAccountAccountType>,
    /// The country in which the bank account is located.
    pub country: &'a str,
    /// The currency the bank account is in.
    /// This must be a country/currency pairing that [Stripe supports.](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// The ID of a Payment Method with a `type` of `us_bank_account`.
    /// The Payment Method's bank account information will be copied and returned as a Bank Account Token.
    /// This parameter is exclusive with respect to all other parameters in the `bank_account` hash.
    /// You must include the top-level `customer` parameter if the Payment Method is attached to a `Customer` object.
    /// If the Payment Method is not attached to a `Customer` object, it will be consumed and cannot be used again.
    /// You may not use Payment Methods which were created by a Setup Intent with `attach_to_self=true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<&'a str>,
    /// The routing number, sort code, or other country-appropriateinstitution number for the bank account.
    /// For US bank accounts, this is required and should bethe ACH routing number, not the wire routing number.
    /// If you are providing an IBAN for`account_number`, this field is not required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
}
impl<'a> CreateTokenBankAccount<'a> {
    pub fn new(account_number: &'a str, country: &'a str) -> Self {
        Self {
            account_holder_name: None,
            account_holder_type: None,
            account_number,
            account_type: None,
            country,
            currency: None,
            payment_method: None,
            routing_number: None,
        }
    }
}
/// The type of entity that holds the account.
/// It can be `company` or `individual`.
/// This field is required when attaching the bank account to a `Customer` object.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTokenBankAccountAccountHolderType {
    Company,
    Individual,
}
impl CreateTokenBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use CreateTokenBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for CreateTokenBankAccountAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTokenBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTokenBankAccountAccountHolderType")
        })
    }
}
/// The bank account type.
/// This can only be `checking` or `savings` in most countries.
/// In Japan, this can only be `futsu` or `toza`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTokenBankAccountAccountType {
    Checking,
    Futsu,
    Savings,
    Toza,
}
impl CreateTokenBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use CreateTokenBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Futsu => "futsu",
            Savings => "savings",
            Toza => "toza",
        }
    }
}

impl std::str::FromStr for CreateTokenBankAccountAccountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTokenBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "futsu" => Ok(Futsu),
            "savings" => Ok(Savings),
            "toza" => Ok(Toza),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTokenBankAccountAccountType")
        })
    }
}
/// The card this token will represent.
/// If you also pass in a customer, the card must be the ID of a card belonging to the customer.
/// Otherwise, if you do not pass in a customer, this is a dictionary containing a user's credit card details, with the options described below.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateTokenCard<'a> {
    #[serde(untagged)]
    CreditCardSpecs(CreateTokenCreditCardSpecs<'a>),
    #[serde(untagged)]
    Str(&'a str),
}
/// The card this token will represent.
/// If you also pass in a customer, the card must be the ID of a card belonging to the customer.
/// Otherwise, if you do not pass in a customer, this is a dictionary containing a user's credit card details, with the options described below.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenCreditCardSpecs<'a> {
    /// City / District / Suburb / Town / Village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<&'a str>,
    /// Billing address country, if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<&'a str>,
    /// Address line 1 (Street address / PO Box / Company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<&'a str>,
    /// Address line 2 (Apartment / Suite / Unit / Building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<&'a str>,
    /// State / County / Province / Region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<&'a str>,
    /// Required in order to add the card to an account; in all other cases, this parameter is not used.
    /// When added to an account, the card (which must be a debit card) can be used as a transfer destination for funds in this currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// Card security code. Highly recommended to always include this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<&'a str>,
    /// Two-digit number representing the card's expiration month.
    pub exp_month: &'a str,
    /// Two- or four-digit number representing the card's expiration year.
    pub exp_year: &'a str,
    /// Cardholder's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Contains information about card networks used to process the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<CreateTokenCreditCardSpecsNetworks>,
    /// The card number, as a string without any separators.
    pub number: &'a str,
}
impl<'a> CreateTokenCreditCardSpecs<'a> {
    pub fn new(exp_month: &'a str, exp_year: &'a str, number: &'a str) -> Self {
        Self {
            address_city: None,
            address_country: None,
            address_line1: None,
            address_line2: None,
            address_state: None,
            address_zip: None,
            currency: None,
            cvc: None,
            exp_month,
            exp_year,
            name: None,
            networks: None,
            number,
        }
    }
}
/// Contains information about card networks used to process the payment.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTokenCreditCardSpecsNetworksPreferred {
    CartesBancaires,
    Mastercard,
    Visa,
}
impl CreateTokenCreditCardSpecsNetworksPreferred {
    pub fn as_str(self) -> &'static str {
        use CreateTokenCreditCardSpecsNetworksPreferred::*;
        match self {
            CartesBancaires => "cartes_bancaires",
            Mastercard => "mastercard",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for CreateTokenCreditCardSpecsNetworksPreferred {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTokenCreditCardSpecsNetworksPreferred::*;
        match s {
            "cartes_bancaires" => Ok(CartesBancaires),
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateTokenCreditCardSpecsNetworksPreferred",
            )
        })
    }
}
/// The updated CVC value this token represents.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenCvcUpdate<'a> {
    /// The CVC value, in string form.
    pub cvc: &'a str,
}
impl<'a> CreateTokenCvcUpdate<'a> {
    pub fn new(cvc: &'a str) -> Self {
        Self { cvc }
    }
}
/// Information for the person this token represents.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenPerson<'a> {
    /// Details on the legal guardian's acceptance of the required Stripe agreements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_tos_acceptances: Option<CreateTokenPersonAdditionalTosAcceptances<'a>>,
    /// The person's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressSpecs<'a>>,
    /// The Kana variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<CreateTokenPersonAddressKana<'a>>,
    /// The Kanji variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<CreateTokenPersonAddressKanji<'a>>,
    /// The person's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirthSpecs>,
    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<CreateTokenPersonDocuments<'a>>,
    /// The person's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// The person's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    /// The Kana variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<&'a str>,
    /// The Kanji variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<&'a str>,
    /// A list of alternate names or aliases that the person is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<&'a [&'a str]>,
    /// The person's gender (International regulations require either "male" or "female").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<&'a str>,
    /// The person's ID number, as appropriate for their country.
    /// For example, a social security number in the U.S., social insurance number in Canada, etc.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://docs.stripe.com/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<&'a str>,
    /// The person's secondary ID number, as appropriate for their country, will be used for enhanced verification checks.
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://docs.stripe.com/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary: Option<&'a str>,
    /// The person's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    /// The Kana variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<&'a str>,
    /// The Kanji variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<&'a str>,
    /// The person's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The country where the person is a national.
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)), or "XX" if unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<&'a str>,
    /// The person's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<&'a str>,
    /// The person's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<AddressSpecs<'a>>,
    /// The relationship that this person has with the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<CreateTokenPersonRelationship<'a>>,
    /// The last four digits of the person's Social Security number (U.S. only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<&'a str>,
    /// The person's verification status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationSpecs<'a>>,
}
impl<'a> CreateTokenPerson<'a> {
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
            verification: None,
        }
    }
}
impl<'a> Default for CreateTokenPerson<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Details on the legal guardian's acceptance of the required Stripe agreements.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenPersonAdditionalTosAcceptances<'a> {
    /// Details on the legal guardian's acceptance of the main Stripe service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<CreateTokenPersonAdditionalTosAcceptancesAccount<'a>>,
}
impl<'a> CreateTokenPersonAdditionalTosAcceptances<'a> {
    pub fn new() -> Self {
        Self { account: None }
    }
}
impl<'a> Default for CreateTokenPersonAdditionalTosAcceptances<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Details on the legal guardian's acceptance of the main Stripe service agreement.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenPersonAdditionalTosAcceptancesAccount<'a> {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<&'a str>,
    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> CreateTokenPersonAdditionalTosAcceptancesAccount<'a> {
    pub fn new() -> Self {
        Self { date: None, ip: None, user_agent: None }
    }
}
impl<'a> Default for CreateTokenPersonAdditionalTosAcceptancesAccount<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kana variation of the person's address (Japan only).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenPersonAddressKana<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> CreateTokenPersonAddressKana<'a> {
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
impl<'a> Default for CreateTokenPersonAddressKana<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the person's address (Japan only).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenPersonAddressKanji<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> CreateTokenPersonAddressKanji<'a> {
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
impl<'a> Default for CreateTokenPersonAddressKanji<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Documents that may be submitted to satisfy various informational requests.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenPersonDocuments<'a> {
    /// One or more documents that demonstrate proof that this person is authorized to represent the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_authorization: Option<DocumentsParam<'a>>,
    /// One or more documents showing the person's passport page with photo and personal data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport: Option<DocumentsParam<'a>>,
    /// One or more documents showing the person's visa required for living in the country where they are residing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<DocumentsParam<'a>>,
}
impl<'a> CreateTokenPersonDocuments<'a> {
    pub fn new() -> Self {
        Self { company_authorization: None, passport: None, visa: None }
    }
}
impl<'a> Default for CreateTokenPersonDocuments<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The relationship that this person has with the account's legal entity.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenPersonRelationship<'a> {
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
    pub title: Option<&'a str>,
}
impl<'a> CreateTokenPersonRelationship<'a> {
    pub fn new() -> Self {
        Self {
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
impl<'a> Default for CreateTokenPersonRelationship<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The PII this token represents.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTokenPii<'a> {
    /// The `id_number` for the PII, in string form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<&'a str>,
}
impl<'a> CreateTokenPii<'a> {
    pub fn new() -> Self {
        Self { id_number: None }
    }
}
impl<'a> Default for CreateTokenPii<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Creates a single-use token that represents a bank account’s details.
/// You can use this token with any API method in place of a bank account dictionary.
/// You can only use this token once.
/// To do so, attach it to a [connected account](https://stripe.com/docs/api#accounts) where <a href="/api/accounts/object#account_object-controller-requirement_collection">controller.requirement_collection</a> is `application`, which includes Custom accounts.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateToken<'a> {
    inner: CreateTokenBuilder<'a>,
}
impl<'a> CreateToken<'a> {
    /// Construct a new `CreateToken`.
    pub fn new() -> Self {
        Self { inner: CreateTokenBuilder::new() }
    }
    /// Information for the account this token represents.
    pub fn account(mut self, account: CreateTokenAccount<'a>) -> Self {
        self.inner.account = Some(account);
        self
    }
    /// The bank account this token will represent.
    pub fn bank_account(mut self, bank_account: CreateTokenBankAccount<'a>) -> Self {
        self.inner.bank_account = Some(bank_account);
        self
    }
    /// The card this token will represent.
    /// If you also pass in a customer, the card must be the ID of a card belonging to the customer.
    /// Otherwise, if you do not pass in a customer, this is a dictionary containing a user's credit card details, with the options described below.
    pub fn card(mut self, card: CreateTokenCard<'a>) -> Self {
        self.inner.card = Some(card);
        self
    }
    /// Create a token for the customer, which is owned by the application's account.
    /// You can only use this with an [OAuth access token](https://stripe.com/docs/connect/standard-accounts) or [Stripe-Account header](https://stripe.com/docs/connect/authentication).
    /// Learn more about [cloning saved payment methods](https://stripe.com/docs/connect/cloning-saved-payment-methods).
    pub fn customer(mut self, customer: &'a str) -> Self {
        self.inner.customer = Some(customer);
        self
    }
    /// The updated CVC value this token represents.
    pub fn cvc_update(mut self, cvc_update: CreateTokenCvcUpdate<'a>) -> Self {
        self.inner.cvc_update = Some(cvc_update);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Information for the person this token represents.
    pub fn person(mut self, person: CreateTokenPerson<'a>) -> Self {
        self.inner.person = Some(person);
        self
    }
    /// The PII this token represents.
    pub fn pii(mut self, pii: CreateTokenPii<'a>) -> Self {
        self.inner.pii = Some(pii);
        self
    }
}
impl<'a> Default for CreateToken<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateToken<'_> {
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

impl StripeRequest for CreateToken<'_> {
    type Output = stripe_core::Token;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/tokens").form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct AddressSpecs<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> AddressSpecs<'a> {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl<'a> Default for AddressSpecs<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct DateOfBirthSpecs {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl DateOfBirthSpecs {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PersonVerificationDocumentSpecs<'a> {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<&'a str>,
    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<&'a str>,
}
impl<'a> PersonVerificationDocumentSpecs<'a> {
    pub fn new() -> Self {
        Self { back: None, front: None }
    }
}
impl<'a> Default for PersonVerificationDocumentSpecs<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct DocumentsParam<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> DocumentsParam<'a> {
    pub fn new() -> Self {
        Self { files: None }
    }
}
impl<'a> Default for DocumentsParam<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PersonVerificationSpecs<'a> {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<PersonVerificationDocumentSpecs<'a>>,
    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PersonVerificationDocumentSpecs<'a>>,
}
impl<'a> PersonVerificationSpecs<'a> {
    pub fn new() -> Self {
        Self { additional_document: None, document: None }
    }
}
impl<'a> Default for PersonVerificationSpecs<'a> {
    fn default() -> Self {
        Self::new()
    }
}
