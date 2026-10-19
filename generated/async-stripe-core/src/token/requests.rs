use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveTokenBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveTokenBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveTokenBuilder").finish_non_exhaustive()
    }
}
impl RetrieveTokenBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the token with the given ID.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveToken {
    inner: RetrieveTokenBuilder,
    token: stripe_shared::TokenId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveToken").finish_non_exhaustive()
    }
}
impl RetrieveToken {
    /// Construct a new `RetrieveToken`.
    pub fn new(token: impl Into<stripe_shared::TokenId>) -> Self {
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
    type Output = stripe_shared::Token;

    fn build(&self) -> RequestBuilder {
        let token = &self.token;
        RequestBuilder::new(StripeMethod::Get, format!("/tokens/{token}")).query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenBuilder").finish_non_exhaustive()
    }
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTokenAccount {
    /// The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<stripe_shared::BusinessType>,
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccount").finish_non_exhaustive()
    }
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
/// Information about the company or business.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
    /// The location where the business is administered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_address: Option<CreateTokenAccountCompanyAdministrativeAddress>,
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
    pub ownership_exemption_reason: Option<stripe_shared::OwnershipExemptionReason>,
    /// The company's phone number (used for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The primary location where the business conducts operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_place_of_business: Option<CreateTokenAccountCompanyPrincipalPlaceOfBusiness>,
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
    pub structure: Option<stripe_shared::Structure>,
    /// The business ID number of the company, as appropriate for the company’s country.
    /// (Examples are an Employer ID Number in the U.S., a Business Number in Canada, or a Company Number in the UK.).
    ///
    /// Changing this value requires that the account re-accept the [terms of service](/api/accounts/object#account_object-tos_acceptance).
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountCompany {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountCompany").finish_non_exhaustive()
    }
}
impl CreateTokenAccountCompany {
    pub fn new() -> Self {
        Self {
            address: None,
            address_kana: None,
            address_kanji: None,
            administrative_address: None,
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
            principal_place_of_business: None,
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountCompanyAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountCompanyAddress").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountCompanyAddressKana {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountCompanyAddressKana").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountCompanyAddressKanji {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountCompanyAddressKanji").finish_non_exhaustive()
    }
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
/// The location where the business is administered.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTokenAccountCompanyAdministrativeAddress {
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountCompanyAdministrativeAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountCompanyAdministrativeAddress").finish_non_exhaustive()
    }
}
impl CreateTokenAccountCompanyAdministrativeAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateTokenAccountCompanyAdministrativeAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// This hash is used to attest that the directors information provided to Stripe is both current and correct.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountCompanyDirectorshipDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountCompanyDirectorshipDeclaration").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountCompanyOwnershipDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountCompanyOwnershipDeclaration").finish_non_exhaustive()
    }
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
/// The primary location where the business conducts operations.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTokenAccountCompanyPrincipalPlaceOfBusiness {
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountCompanyPrincipalPlaceOfBusiness {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountCompanyPrincipalPlaceOfBusiness").finish_non_exhaustive()
    }
}
impl CreateTokenAccountCompanyPrincipalPlaceOfBusiness {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateTokenAccountCompanyPrincipalPlaceOfBusiness {
    fn default() -> Self {
        Self::new()
    }
}
/// When the business was incorporated or registered.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTokenAccountCompanyRegistrationDate {
    /// The day of registration, between 1 and 31.
    pub day: i64,
    /// The month of registration, between 1 and 12.
    pub month: i64,
    /// The four-digit year of registration.
    pub year: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountCompanyRegistrationDate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountCompanyRegistrationDate").finish_non_exhaustive()
    }
}
impl CreateTokenAccountCompanyRegistrationDate {
    pub fn new(day: impl Into<i64>, month: impl Into<i64>, year: impl Into<i64>) -> Self {
        Self { day: day.into(), month: month.into(), year: year.into() }
    }
}
/// This hash is used to attest that the representative is authorized to act as the representative of their legal entity.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountCompanyRepresentativeDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountCompanyRepresentativeDeclaration").finish_non_exhaustive()
    }
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
/// Information on the verification state of the company.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTokenAccountCompanyVerification {
    /// A document verifying the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<CreateTokenAccountCompanyVerificationDocument>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountCompanyVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountCompanyVerification").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountCompanyVerificationDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountCompanyVerificationDocument").finish_non_exhaustive()
    }
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
    pub political_exposure: Option<stripe_shared::PoliticalExposure>,
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountIndividual {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountIndividual").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountIndividualAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountIndividualAddress").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountIndividualAddressKana {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountIndividualAddressKana").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountIndividualAddressKanji {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountIndividualAddressKanji").finish_non_exhaustive()
    }
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
/// The individual's registered address.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountIndividualRegisteredAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountIndividualRegisteredAddress").finish_non_exhaustive()
    }
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenAccountIndividualRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenAccountIndividualRelationship").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTokenBankAccount {
    /// The name of the person or business that owns the bank account.
    /// This field is required when attaching the bank account to a `Customer` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    /// The type of entity that holds the account.
    /// It can be `company` or `individual`.
    /// This field is required when attaching the bank account to a `Customer` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<stripe_shared::IssuingCardholderType>,
    /// The account number for the bank account, in string form. Must be a checking account.
    pub account_number: String,
    /// The bank account type.
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<stripe_shared::UpdateAccountCardAccountType>,
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenBankAccount").finish_non_exhaustive()
    }
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
/// The card this token will represent.
/// If you also pass in a customer, the card must be the ID of a card belonging to the customer.
/// Otherwise, if you do not pass in a customer, this is a dictionary containing a user's credit card details, with the options described below.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateTokenCard {
    #[serde(untagged)]
    CreditCardSpecs(CreateTokenCreditCardSpecs),
    #[serde(untagged)]
    String(String),
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenCard").finish_non_exhaustive()
    }
}
/// The card this token will represent.
/// If you also pass in a customer, the card must be the ID of a card belonging to the customer.
/// Otherwise, if you do not pass in a customer, this is a dictionary containing a user's credit card details, with the options described below.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenCreditCardSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenCreditCardSpecs").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTokenCreditCardSpecsNetworks {
    /// The customer's preferred card network for co-branded cards.
    /// Supports `cartes_bancaires`, `mastercard`, or `visa`.
    /// Selection of a network that does not apply to the card will be stored as `invalid_preference` on the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<stripe_shared::Preferred>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenCreditCardSpecsNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenCreditCardSpecsNetworks").finish_non_exhaustive()
    }
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
/// The updated CVC value this token represents.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTokenCvcUpdate {
    /// The CVC value, in string form.
    pub cvc: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenCvcUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenCvcUpdate").finish_non_exhaustive()
    }
}
impl CreateTokenCvcUpdate {
    pub fn new(cvc: impl Into<String>) -> Self {
        Self { cvc: cvc.into() }
    }
}
/// Information for the person this token represents.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
    ///
    /// Changing this value for the account's representative requires that the account re-accept the [terms of service](/api/accounts/object#account_object-tos_acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    /// The person's secondary ID number, as appropriate for their country, will be used for enhanced verification checks.
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token provided by Stripe.js](https://docs.stripe.com/js/tokens/create_token?type=pii).
    ///
    /// Changing this value for the account's representative requires that the account re-accept the [terms of service](/api/accounts/object#account_object-tos_acceptance).
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
    pub political_exposure: Option<stripe_shared::PoliticalExposure>,
    /// The person's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<CreateTokenPersonRegisteredAddress>,
    /// The relationship that this person has with the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<CreateTokenPersonRelationship>,
    /// The last four digits of the person's Social Security number (U.S. only).
    ///
    /// Changing this value for the account's representative requires that the account re-accept the [terms of service](/api/accounts/object#account_object-tos_acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<String>,
    /// Demographic data related to the person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_cfpb_data: Option<CreateTokenPersonUsCfpbData>,
    /// The person's verification status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenPerson {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenPerson").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTokenPersonAdditionalTosAcceptances {
    /// Details on the legal guardian's acceptance of the main Stripe service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<CreateTokenPersonAdditionalTosAcceptancesAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenPersonAdditionalTosAcceptances {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenPersonAdditionalTosAcceptances").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenPersonAdditionalTosAcceptancesAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenPersonAdditionalTosAcceptancesAccount").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenPersonAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenPersonAddress").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenPersonAddressKana {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenPersonAddressKana").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenPersonAddressKanji {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenPersonAddressKanji").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenPersonDocuments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenPersonDocuments").finish_non_exhaustive()
    }
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
/// The person's registered address.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenPersonRegisteredAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenPersonRegisteredAddress").finish_non_exhaustive()
    }
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenPersonRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenPersonRelationship").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenPersonUsCfpbData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenPersonUsCfpbData").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTokenPersonUsCfpbDataEthnicityDetails {
    /// The persons ethnicity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<Vec<stripe_shared::Ethnicity>>,
    /// Please specify your origin, when other is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethnicity_other: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenPersonUsCfpbDataEthnicityDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenPersonUsCfpbDataEthnicityDetails").finish_non_exhaustive()
    }
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
/// The persons race details
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTokenPersonUsCfpbDataRaceDetails {
    /// The persons race.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race: Option<Vec<stripe_shared::Race>>,
    /// Please specify your race, when other is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race_other: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenPersonUsCfpbDataRaceDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenPersonUsCfpbDataRaceDetails").finish_non_exhaustive()
    }
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
/// The PII this token represents.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTokenPii {
    /// The `id_number` for the PII, in string form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTokenPii {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTokenPii").finish_non_exhaustive()
    }
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateToken {
    inner: CreateTokenBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateToken").finish_non_exhaustive()
    }
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
    type Output = stripe_shared::Token;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/tokens").form(&self.inner)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DateOfBirthSpecs {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DateOfBirthSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DateOfBirthSpecs").finish_non_exhaustive()
    }
}
impl DateOfBirthSpecs {
    pub fn new(day: impl Into<i64>, month: impl Into<i64>, year: impl Into<i64>) -> Self {
        Self { day: day.into(), month: month.into(), year: year.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonVerificationDocumentSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonVerificationDocumentSpecs").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DocumentsParam {
    /// One or more document ids returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DocumentsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DocumentsParam").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PersonVerificationSpecs {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<PersonVerificationDocumentSpecs>,
    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PersonVerificationDocumentSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonVerificationSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonVerificationSpecs").finish_non_exhaustive()
    }
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
/// The business type.
pub type CreateTokenAccountBusinessType = stripe_shared::BusinessType;
/// This value is used to determine if a business is exempt from providing ultimate beneficial owners.
/// See [this support article](https://support.stripe.com/questions/exemption-from-providing-ownership-details) and [changelog](https://docs.stripe.com/changelog/acacia/2025-01-27/ownership-exemption-reason-accounts-api) for more details.
pub type CreateTokenAccountCompanyOwnershipExemptionReason =
    stripe_shared::OwnershipExemptionReason;
/// The category identifying the legal structure of the company or legal entity.
/// See [Business structure](/connect/identity-verification#business-structure) for more details.
/// Pass an empty string to unset this value.
pub type CreateTokenAccountCompanyStructure = stripe_shared::Structure;
/// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
pub type CreateTokenAccountIndividualPoliticalExposure = stripe_shared::PoliticalExposure;
/// The type of entity that holds the account.
/// It can be `company` or `individual`.
/// This field is required when attaching the bank account to a `Customer` object.
pub type CreateTokenBankAccountAccountHolderType = stripe_shared::IssuingCardholderType;
/// The bank account type.
/// This can only be `checking` or `savings` in most countries.
/// In Japan, this can only be `futsu` or `toza`.
pub type CreateTokenBankAccountAccountType = stripe_shared::UpdateAccountCardAccountType;
/// The customer's preferred card network for co-branded cards.
/// Supports `cartes_bancaires`, `mastercard`, or `visa`.
/// Selection of a network that does not apply to the card will be stored as `invalid_preference` on the card.
pub type CreateTokenCreditCardSpecsNetworksPreferred = stripe_shared::Preferred;
/// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
pub type CreateTokenPersonPoliticalExposure = stripe_shared::PoliticalExposure;
/// The persons ethnicity
pub type CreateTokenPersonUsCfpbDataEthnicityDetailsEthnicity = stripe_shared::Ethnicity;
/// The persons race.
pub type CreateTokenPersonUsCfpbDataRaceDetailsRace = stripe_shared::Race;
/// The individual's date of birth.
pub type CreateTokenAccountIndividualDob = DateOfBirthSpecs;
/// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
pub type CreateTokenAccountIndividualVerificationAdditionalDocument =
    PersonVerificationDocumentSpecs;
/// An identifying document, either a passport or local ID card.
pub type CreateTokenAccountIndividualVerificationDocument = PersonVerificationDocumentSpecs;
/// The person's date of birth.
pub type CreateTokenPersonDob = DateOfBirthSpecs;
/// One or more documents that demonstrate proof that this person is authorized to represent the company.
pub type CreateTokenPersonDocumentsCompanyAuthorization = DocumentsParam;
/// One or more documents showing the person's passport page with photo and personal data.
pub type CreateTokenPersonDocumentsPassport = DocumentsParam;
/// One or more documents showing the person's visa required for living in the country where they are residing.
pub type CreateTokenPersonDocumentsVisa = DocumentsParam;
/// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
pub type CreateTokenPersonVerificationAdditionalDocument = PersonVerificationDocumentSpecs;
/// An identifying document, either a passport or local ID card.
pub type CreateTokenPersonVerificationDocument = PersonVerificationDocumentSpecs;
/// The individual's verification document information.
pub type CreateTokenAccountIndividualVerification = PersonVerificationSpecs;
/// The person's verification status.
pub type CreateTokenPersonVerification = PersonVerificationSpecs;
