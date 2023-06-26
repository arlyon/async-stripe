impl stripe_core::account::Account {
    /// Retrieves the details of an account.
    pub fn retrieve_for_my_account(
        client: &stripe::Client,
        params: RetrieveForMyAccountAccount,
    ) -> stripe::Response<stripe_core::account::Account> {
        client.get_query("/account", params)
    }
    /// Retrieves the details of an account.
    pub fn retrieve(
        client: &stripe::Client,
        account: &stripe_types::AccountId,
        params: RetrieveAccount,
    ) -> stripe::Response<stripe_core::account::Account> {
        client.get_query(&format!("/accounts/{account}", account = account), params)
    }
    /// Updates a [connected account](https://stripe.com/docs/connect/accounts) by setting the values of the parameters passed.
    ///
    /// Any parameters not provided are left unchanged.
    /// Most parameters can be changed only for Custom accounts.
    /// (These are marked **Custom Only** below.) Parameters marked **Custom and Express** are not supported for Standard accounts.  To update your own account, use the [Dashboard](https://dashboard.stripe.com/account).
    /// Refer to our [Connect](https://stripe.com/docs/connect/updating-accounts) documentation to learn more about updating accounts.
    pub fn update(
        client: &stripe::Client,
        account: &stripe_types::AccountId,
        params: UpdateAccount,
    ) -> stripe::Response<stripe_core::account::Account> {
        client.send_form(
            &format!("/accounts/{account}", account = account),
            params,
            http_types::Method::Post,
        )
    }
    /// Returns a list of accounts connected to your platform via [Connect](https://stripe.com/docs/connect).
    ///
    /// If you’re not a platform, the list is empty.
    pub fn list(
        client: &stripe::Client,
        params: ListAccount,
    ) -> stripe::Response<stripe_types::List<stripe_core::account::Account>> {
        client.get_query("/accounts", params)
    }
    /// With [Connect](https://stripe.com/docs/connect), you can create Stripe accounts for your users.
    /// To do this, you’ll first need to [register your platform](https://dashboard.stripe.com/account/applications/settings).
    pub fn create(
        client: &stripe::Client,
        params: CreateAccount,
    ) -> stripe::Response<stripe_core::account::Account> {
        client.send_form("/accounts", params, http_types::Method::Post)
    }
    /// With [Connect](https://stripe.com/docs/connect), you can delete accounts you manage.
    ///
    /// Accounts created using test-mode keys can be deleted at any time.
    ///
    /// Standard accounts created using live-mode keys cannot be deleted.
    /// Custom or Express accounts created using live-mode keys can only be deleted once all balances are zero.  If you want to delete your own account, use the [account information tab in your account settings](https://dashboard.stripe.com/account) instead.
    pub fn delete(
        client: &stripe::Client,
        account: &stripe_types::AccountId,
    ) -> stripe::Response<stripe_core::account::DeletedAccount> {
        client.send(&format!("/accounts/{account}", account = account), http_types::Method::Delete)
    }
    /// With [Connect](https://stripe.com/docs/connect), you may flag accounts as suspicious.
    ///
    /// Test-mode Custom and Express accounts can be rejected at any time.
    ///
    /// Accounts created using live-mode keys may only be rejected once all balances are zero.
    pub fn reject(
        client: &stripe::Client,
        account: &stripe_types::AccountId,
        params: RejectAccount,
    ) -> stripe::Response<stripe_core::account::Account> {
        client.send_form(
            &format!("/accounts/{account}/reject", account = account),
            params,
            http_types::Method::Post,
        )
    }
    /// Returns a list of people associated with the account’s legal entity.
    ///
    /// The people are returned sorted by creation date, with the most recent people appearing first.
    pub fn persons(
        client: &stripe::Client,
        account: &stripe_types::AccountId,
        params: PersonsAccount,
    ) -> stripe::Response<stripe_types::List<stripe_core::person::Person>> {
        client.get_query(&format!("/accounts/{account}/persons", account = account), params)
    }
    /// Returns a list of capabilities associated with the account.
    ///
    /// The capabilities are returned sorted by creation date, with the most recent capability appearing first.
    pub fn capabilities(
        client: &stripe::Client,
        account: &stripe_types::AccountId,
        params: CapabilitiesAccount,
    ) -> stripe::Response<stripe_types::List<stripe_core::capability::Capability>> {
        client.get_query(&format!("/accounts/{account}/capabilities", account = account), params)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveForMyAccountAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveForMyAccountAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccount<'a> {
    /// An [account token](https://stripe.com/docs/api#create_account_token), used to securely provide details to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_token: Option<&'a str>,
    /// Business information about the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<BusinessProfileSpecs<'a>>,
    /// The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<BusinessType>,
    /// Each key of the dictionary represents a capability, and each capability maps to its settings (e.g.
    ///
    /// whether it has been requested or not).
    /// Each capability will be inactive until you have provided its specific requirements and Stripe has verified them.
    /// An account may have some of its requested capabilities be active and some be inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<CapabilitiesParam>,
    /// Information about the company or business.
    ///
    /// This field is available for any `business_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<UpdateAccountCompany<'a>>,
    /// Three-letter ISO currency code representing the default currency for the account.
    ///
    /// This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<stripe_types::Currency>,
    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsSpecs<'a>>,
    /// The email address of the account holder.
    ///
    /// This is only to make the account easier to identify to you.
    /// Stripe only emails Custom accounts with your consent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A card or bank account to attach to the account for receiving [payouts](https://stripe.com/docs/connect/bank-debit-card-payouts) (you won’t be able to use it for top-ups).
    ///
    /// You can provide either a token, like the ones returned by [Stripe.js](https://stripe.com/docs/js), or a dictionary, as documented in the `external_account` parameter for [bank account](https://stripe.com/docs/api#account_create_bank_account) creation.
    /// By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists.
    /// To add additional external accounts without replacing the existing default for the currency, use the [bank account](https://stripe.com/docs/api#account_create_bank_account) or [card creation](https://stripe.com/docs/api#account_create_card) APIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<&'a str>,
    /// Information about the person represented by the account.
    ///
    /// This field is null unless `business_type` is set to `individual`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<UpdateAccountIndividual<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings<'a>>,
    /// Details on the account's acceptance of the [Stripe Services Agreement](https://stripe.com/docs/connect/updating-accounts#tos-acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<TosAcceptanceSpecs<'a>>,
}
impl<'a> UpdateAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about the company or business.
///
/// This field is available for any `business_type`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCompany<'a> {
    /// The company's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressSpecs<'a>>,
    /// The Kana variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<UpdateAccountCompanyAddressKana<'a>>,
    /// The Kanji variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<UpdateAccountCompanyAddressKanji<'a>>,
    /// Whether the company's directors have been provided.
    ///
    /// Set this Boolean to `true` after creating all the company's directors with [the Persons API](https://stripe.com/docs/api/persons) for accounts with a `relationship.director` requirement.
    /// This value is not automatically set to `true` after creating directors, so it needs to be updated to indicate all directors have been provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directors_provided: Option<bool>,
    /// Whether the company's executives have been provided.
    ///
    /// Set this Boolean to `true` after creating all the company's executives with [the Persons API](https://stripe.com/docs/api/persons) for accounts with a `relationship.executive` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executives_provided: Option<bool>,
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
    ///
    /// Set this Boolean to `true` after creating all the company's owners with [the Persons API](https://stripe.com/docs/api/persons) for accounts with a `relationship.owner` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,
    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration: Option<CompanyOwnershipDeclaration<'a>>,
    /// The company's phone number (used for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// The identification number given to a company when it is registered or incorporated, if distinct from the identification number used for filing taxes.
    ///
    /// (Examples are the CIN for companies and LLP IN for partnerships in India, and the Company Registration Number in Hong Kong).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<&'a str>,
    /// The category identifying the legal structure of the company or legal entity.
    ///
    /// See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<Structure>,
    /// The business ID number of the company, as appropriate for the company’s country.
    ///
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
    pub verification: Option<VerificationSpecs<'a>>,
}
impl<'a> UpdateAccountCompany<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kana variation of the company's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCompanyAddressKana<'a> {
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
impl<'a> UpdateAccountCompanyAddressKana<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kanji variation of the company's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCompanyAddressKanji<'a> {
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
impl<'a> UpdateAccountCompanyAddressKanji<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about the person represented by the account.
///
/// This field is null unless `business_type` is set to `individual`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountIndividual<'a> {
    /// The individual's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressSpecs<'a>>,
    /// The Kana variation of the the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<UpdateAccountIndividualAddressKana<'a>>,
    /// The Kanji variation of the the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<UpdateAccountIndividualAddressKanji<'a>>,
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
    /// The government-issued ID number of the individual, as appropriate for the representative’s country.
    ///
    /// (Examples are a Social Security Number in the U.S., or a Social Insurance Number in Canada).
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<&'a str>,
    /// The government-issued secondary ID number of the individual, as appropriate for the representative's country, will be used for enhanced verification checks.
    ///
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
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
    ///
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
    pub political_exposure: Option<PoliticalExposure>,
    /// The individual's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<AddressSpecs<'a>>,
    /// The last four digits of the individual's Social Security Number (U.S.
    ///
    /// only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<&'a str>,
    /// The individual's verification document information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationSpecs<'a>>,
}
impl<'a> UpdateAccountIndividual<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kana variation of the the individual's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountIndividualAddressKana<'a> {
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
impl<'a> UpdateAccountIndividualAddressKana<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kanji variation of the the individual's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountIndividualAddressKanji<'a> {
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
impl<'a> UpdateAccountIndividualAddressKanji<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListAccount<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
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
impl<'a> ListAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccount<'a> {
    /// An [account token](https://stripe.com/docs/api#create_account_token), used to securely provide details to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_token: Option<&'a str>,
    /// Business information about the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<BusinessProfileSpecs<'a>>,
    /// The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<BusinessType>,
    /// Each key of the dictionary represents a capability, and each capability maps to its settings (e.g.
    ///
    /// whether it has been requested or not).
    /// Each capability will be inactive until you have provided its specific requirements and Stripe has verified them.
    /// An account may have some of its requested capabilities be active and some be inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<CapabilitiesParam>,
    /// Information about the company or business.
    ///
    /// This field is available for any `business_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CreateAccountCompany<'a>>,
    /// The country in which the account holder resides, or in which the business is legally established.
    ///
    /// This should be an ISO 3166-1 alpha-2 country code.
    /// For example, if you are in the United States and the business for which you're creating an account is legally represented in Canada, you would use `CA` as the country for the account being created.
    /// Available countries include [Stripe's global markets](https://stripe.com/global) as well as countries where [cross-border payouts](https://stripe.com/docs/connect/cross-border-payouts) are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Three-letter ISO currency code representing the default currency for the account.
    ///
    /// This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<stripe_types::Currency>,
    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentsSpecs<'a>>,
    /// The email address of the account holder.
    ///
    /// This is only to make the account easier to identify to you.
    /// Stripe only emails Custom accounts with your consent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A card or bank account to attach to the account for receiving [payouts](https://stripe.com/docs/connect/bank-debit-card-payouts) (you won’t be able to use it for top-ups).
    ///
    /// You can provide either a token, like the ones returned by [Stripe.js](https://stripe.com/docs/js), or a dictionary, as documented in the `external_account` parameter for [bank account](https://stripe.com/docs/api#account_create_bank_account) creation.
    /// By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists.
    /// To add additional external accounts without replacing the existing default for the currency, use the [bank account](https://stripe.com/docs/api#account_create_bank_account) or [card creation](https://stripe.com/docs/api#account_create_card) APIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<&'a str>,
    /// Information about the person represented by the account.
    ///
    /// This field is null unless `business_type` is set to `individual`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<CreateAccountIndividual<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings<'a>>,
    /// Details on the account's acceptance of the [Stripe Services Agreement](https://stripe.com/docs/connect/updating-accounts#tos-acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<TosAcceptanceSpecs<'a>>,
    /// The type of Stripe account to create.
    ///
    /// May be one of `custom`, `express` or `standard`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateAccountType>,
}
impl<'a> CreateAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about the company or business.
///
/// This field is available for any `business_type`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCompany<'a> {
    /// The company's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressSpecs<'a>>,
    /// The Kana variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<CreateAccountCompanyAddressKana<'a>>,
    /// The Kanji variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<CreateAccountCompanyAddressKanji<'a>>,
    /// Whether the company's directors have been provided.
    ///
    /// Set this Boolean to `true` after creating all the company's directors with [the Persons API](https://stripe.com/docs/api/persons) for accounts with a `relationship.director` requirement.
    /// This value is not automatically set to `true` after creating directors, so it needs to be updated to indicate all directors have been provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directors_provided: Option<bool>,
    /// Whether the company's executives have been provided.
    ///
    /// Set this Boolean to `true` after creating all the company's executives with [the Persons API](https://stripe.com/docs/api/persons) for accounts with a `relationship.executive` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executives_provided: Option<bool>,
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
    ///
    /// Set this Boolean to `true` after creating all the company's owners with [the Persons API](https://stripe.com/docs/api/persons) for accounts with a `relationship.owner` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,
    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration: Option<CompanyOwnershipDeclaration<'a>>,
    /// The company's phone number (used for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// The identification number given to a company when it is registered or incorporated, if distinct from the identification number used for filing taxes.
    ///
    /// (Examples are the CIN for companies and LLP IN for partnerships in India, and the Company Registration Number in Hong Kong).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<&'a str>,
    /// The category identifying the legal structure of the company or legal entity.
    ///
    /// See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<Structure>,
    /// The business ID number of the company, as appropriate for the company’s country.
    ///
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
    pub verification: Option<VerificationSpecs<'a>>,
}
impl<'a> CreateAccountCompany<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kana variation of the company's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCompanyAddressKana<'a> {
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
impl<'a> CreateAccountCompanyAddressKana<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kanji variation of the company's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCompanyAddressKanji<'a> {
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
impl<'a> CreateAccountCompanyAddressKanji<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about the person represented by the account.
///
/// This field is null unless `business_type` is set to `individual`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountIndividual<'a> {
    /// The individual's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressSpecs<'a>>,
    /// The Kana variation of the the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<CreateAccountIndividualAddressKana<'a>>,
    /// The Kanji variation of the the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<CreateAccountIndividualAddressKanji<'a>>,
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
    /// The government-issued ID number of the individual, as appropriate for the representative’s country.
    ///
    /// (Examples are a Social Security Number in the U.S., or a Social Insurance Number in Canada).
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<&'a str>,
    /// The government-issued secondary ID number of the individual, as appropriate for the representative's country, will be used for enhanced verification checks.
    ///
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
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
    ///
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
    pub political_exposure: Option<PoliticalExposure>,
    /// The individual's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<AddressSpecs<'a>>,
    /// The last four digits of the individual's Social Security Number (U.S.
    ///
    /// only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<&'a str>,
    /// The individual's verification document information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationSpecs<'a>>,
}
impl<'a> CreateAccountIndividual<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kana variation of the the individual's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountIndividualAddressKana<'a> {
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
impl<'a> CreateAccountIndividualAddressKana<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kanji variation of the the individual's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountIndividualAddressKanji<'a> {
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
impl<'a> CreateAccountIndividualAddressKanji<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The type of Stripe account to create.
///
/// May be one of `custom`, `express` or `standard`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateAccountType {
    Custom,
    Express,
    Standard,
}

impl CreateAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Custom => "custom",
            Self::Express => "express",
            Self::Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateAccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "custom" => Ok(Self::Custom),
            "express" => Ok(Self::Express),
            "standard" => Ok(Self::Standard),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RejectAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The reason for rejecting the account.
    ///
    /// Can be `fraud`, `terms_of_service`, or `other`.
    pub reason: &'a str,
}
impl<'a> RejectAccount<'a> {
    pub fn new(reason: &'a str) -> Self {
        Self { expand: Default::default(), reason }
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PersonsAccount<'a> {
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
    /// Filters on the list of people returned based on the person's relationship to the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<PersonsAccountRelationship>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> PersonsAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Filters on the list of people returned based on the person's relationship to the account's company.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PersonsAccountRelationship {
    /// A filter on the list of people returned based on whether these people are directors of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    /// A filter on the list of people returned based on whether these people are executives of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    /// A filter on the list of people returned based on whether these people are owners of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    /// A filter on the list of people returned based on whether these people are the representative of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,
}
impl PersonsAccountRelationship {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CapabilitiesAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CapabilitiesAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BusinessType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
}

impl BusinessType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::GovernmentEntity => "government_entity",
            Self::Individual => "individual",
            Self::NonProfit => "non_profit",
        }
    }
}

impl std::str::FromStr for BusinessType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "company" => Ok(Self::Company),
            "government_entity" => Ok(Self::GovernmentEntity),
            "individual" => Ok(Self::Individual),
            "non_profit" => Ok(Self::NonProfit),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for BusinessType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BusinessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for BusinessType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CapabilityParam {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CapabilityParam {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CompanyOwnershipDeclaration<'a> {
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
impl<'a> CompanyOwnershipDeclaration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Structure {
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

impl Structure {
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

impl std::str::FromStr for Structure {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "free_zone_establishment" => Ok(Self::FreeZoneEstablishment),
            "free_zone_llc" => Ok(Self::FreeZoneLlc),
            "government_instrumentality" => Ok(Self::GovernmentInstrumentality),
            "governmental_unit" => Ok(Self::GovernmentalUnit),
            "incorporated_non_profit" => Ok(Self::IncorporatedNonProfit),
            "limited_liability_partnership" => Ok(Self::LimitedLiabilityPartnership),
            "llc" => Ok(Self::Llc),
            "multi_member_llc" => Ok(Self::MultiMemberLlc),
            "private_company" => Ok(Self::PrivateCompany),
            "private_corporation" => Ok(Self::PrivateCorporation),
            "private_partnership" => Ok(Self::PrivatePartnership),
            "public_company" => Ok(Self::PublicCompany),
            "public_corporation" => Ok(Self::PublicCorporation),
            "public_partnership" => Ok(Self::PublicPartnership),
            "single_member_llc" => Ok(Self::SingleMemberLlc),
            "sole_establishment" => Ok(Self::SoleEstablishment),
            "sole_proprietorship" => Ok(Self::SoleProprietorship),
            "tax_exempt_government_instrumentality" => Ok(Self::TaxExemptGovernmentInstrumentality),
            "unincorporated_association" => Ok(Self::UnincorporatedAssociation),
            "unincorporated_non_profit" => Ok(Self::UnincorporatedNonProfit),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for Structure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Structure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct VerificationDocumentSpecs<'a> {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<&'a str>,
    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<&'a str>,
}
impl<'a> VerificationDocumentSpecs<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DocumentsParam<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> DocumentsParam<'a> {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PoliticalExposure {
    Existing,
    None,
}

impl PoliticalExposure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Existing => "existing",
            Self::None => "none",
        }
    }
}

impl std::str::FromStr for PoliticalExposure {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "existing" => Ok(Self::Existing),
            "none" => Ok(Self::None),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PoliticalExposure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PoliticalExposure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PersonVerificationDocumentSpecs<'a> {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<&'a str>,
    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<&'a str>,
}
impl<'a> PersonVerificationDocumentSpecs<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct BrandingSettingsSpecs<'a> {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account.
    ///
    /// Must be square and at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided.
    ///
    /// Must be at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<&'a str>,
    /// A CSS hex color value representing the primary branding color for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<&'a str>,
    /// A CSS hex color value representing the secondary branding color for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<&'a str>,
}
impl<'a> BrandingSettingsSpecs<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct SettingsTermsOfServiceSpecs<'a> {
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
impl<'a> SettingsTermsOfServiceSpecs<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeclineChargeOnSpecs {
    /// Whether Stripe automatically declines charges with an incorrect ZIP or postal code.
    ///
    /// This setting only applies when a ZIP or postal code is provided and they fail bank verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_failure: Option<bool>,
    /// Whether Stripe automatically declines charges with an incorrect CVC.
    ///
    /// This setting only applies when a CVC is provided and it fails bank verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_failure: Option<bool>,
}
impl DeclineChargeOnSpecs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PaymentsSettingsSpecs<'a> {
    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kana: Option<&'a str>,
    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kanji: Option<&'a str>,
}
impl<'a> PaymentsSettingsSpecs<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum DelayDays {
    Minimum,
    U32(u32),
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interval {
    Daily,
    Manual,
    Monthly,
    Weekly,
}

impl Interval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Daily => "daily",
            Self::Manual => "manual",
            Self::Monthly => "monthly",
            Self::Weekly => "weekly",
        }
    }
}

impl std::str::FromStr for Interval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "daily" => Ok(Self::Daily),
            "manual" => Ok(Self::Manual),
            "monthly" => Ok(Self::Monthly),
            "weekly" => Ok(Self::Weekly),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for Interval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Interval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WeeklyAnchor {
    Friday,
    Monday,
    Saturday,
    Sunday,
    Thursday,
    Tuesday,
    Wednesday,
}

impl WeeklyAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Friday => "friday",
            Self::Monday => "monday",
            Self::Saturday => "saturday",
            Self::Sunday => "sunday",
            Self::Thursday => "thursday",
            Self::Tuesday => "tuesday",
            Self::Wednesday => "wednesday",
        }
    }
}

impl std::str::FromStr for WeeklyAnchor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "friday" => Ok(Self::Friday),
            "monday" => Ok(Self::Monday),
            "saturday" => Ok(Self::Saturday),
            "sunday" => Ok(Self::Sunday),
            "thursday" => Ok(Self::Thursday),
            "tuesday" => Ok(Self::Tuesday),
            "wednesday" => Ok(Self::Wednesday),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for WeeklyAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for WeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for WeeklyAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct TosAcceptanceSpecs<'a> {
    /// The Unix timestamp marking when the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<&'a str>,
    /// The user's service agreement type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_agreement: Option<&'a str>,
    /// The user agent of the browser from which the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> TosAcceptanceSpecs<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct BusinessProfileSpecs<'a> {
    /// [The merchant category code for the account](https://stripe.com/docs/connect/setting-mcc).
    ///
    /// MCCs are used to classify businesses based on the goods or services they provide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<&'a str>,
    /// The customer-facing business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Internal-only description of the product sold by, or service provided by, the business.
    ///
    /// Used by Stripe for risk and underwriting purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<&'a str>,
    /// A publicly available mailing address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_address: Option<AddressSpecs<'a>>,
    /// A publicly available email address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<&'a str>,
    /// A publicly available phone number to call with support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_phone: Option<&'a str>,
    /// A publicly available website for handling support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<&'a str>,
    /// The business's publicly available website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}
impl<'a> BusinessProfileSpecs<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CapabilitiesParam {
    /// The acss_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments: Option<CapabilityParam>,
    /// The affirm_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm_payments: Option<CapabilityParam>,
    /// The afterpay_clearpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<CapabilityParam>,
    /// The au_becs_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<CapabilityParam>,
    /// The bacs_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<CapabilityParam>,
    /// The bancontact_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments: Option<CapabilityParam>,
    /// The bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer_payments: Option<CapabilityParam>,
    /// The blik_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik_payments: Option<CapabilityParam>,
    /// The boleto_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_payments: Option<CapabilityParam>,
    /// The card_issuing capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<CapabilityParam>,
    /// The card_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CapabilityParam>,
    /// The cartes_bancaires_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<CapabilityParam>,
    /// The eps_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps_payments: Option<CapabilityParam>,
    /// The fpx_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx_payments: Option<CapabilityParam>,
    /// The giropay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay_payments: Option<CapabilityParam>,
    /// The grabpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments: Option<CapabilityParam>,
    /// The ideal_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_payments: Option<CapabilityParam>,
    /// The jcb_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<CapabilityParam>,
    /// The klarna_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna_payments: Option<CapabilityParam>,
    /// The konbini_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini_payments: Option<CapabilityParam>,
    /// The legacy_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<CapabilityParam>,
    /// The link_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_payments: Option<CapabilityParam>,
    /// The oxxo_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments: Option<CapabilityParam>,
    /// The p24_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24_payments: Option<CapabilityParam>,
    /// The paynow_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow_payments: Option<CapabilityParam>,
    /// The promptpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay_payments: Option<CapabilityParam>,
    /// The sepa_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<CapabilityParam>,
    /// The sofort_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<CapabilityParam>,
    /// The tax_reporting_us_1099_k capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_k: Option<CapabilityParam>,
    /// The tax_reporting_us_1099_misc capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_misc: Option<CapabilityParam>,
    /// The transfers capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<CapabilityParam>,
    /// The treasury capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<CapabilityParam>,
    /// The us_bank_account_ach_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account_ach_payments: Option<CapabilityParam>,
}
impl CapabilitiesParam {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct VerificationSpecs<'a> {
    /// A document verifying the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<VerificationDocumentSpecs<'a>>,
}
impl<'a> VerificationSpecs<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DocumentsSpecs<'a> {
    /// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
    ///
    /// Must be a document associated with the account’s primary active bank account that displays the last 4 digits of the account number, either a statement or a voided check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification: Option<DocumentsParam<'a>>,
    /// One or more documents that demonstrate proof of a company's license to operate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_license: Option<DocumentsParam<'a>>,
    /// One or more documents showing the company's Memorandum of Association.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_memorandum_of_association: Option<DocumentsParam<'a>>,
    /// (Certain countries only) One or more documents showing the ministerial decree legalizing the company's establishment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_ministerial_decree: Option<DocumentsParam<'a>>,
    /// One or more documents that demonstrate proof of a company's registration with the appropriate local authorities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_registration_verification: Option<DocumentsParam<'a>>,
    /// One or more documents that demonstrate proof of a company's tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_tax_id_verification: Option<DocumentsParam<'a>>,
    /// One or more documents showing the company’s proof of registration with the national business registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_registration: Option<DocumentsParam<'a>>,
}
impl<'a> DocumentsSpecs<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CardIssuingSettingsSpecs<'a> {
    /// Details on the account's acceptance of the [Stripe Issuing Terms and Disclosures](https://stripe.com/docs/issuing/connect/tos_acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<SettingsTermsOfServiceSpecs<'a>>,
}
impl<'a> CardIssuingSettingsSpecs<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CardPaymentsSettingsSpecs<'a> {
    /// Automatically declines certain charge types regardless of whether the card issuer accepted or declined the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<DeclineChargeOnSpecs>,
    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    /// `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<&'a str>,
    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    ///
    /// This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge.
    /// `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix_kana: Option<&'a str>,
    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    ///
    /// This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge.
    /// `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix_kanji: Option<&'a str>,
}
impl<'a> CardPaymentsSettingsSpecs<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct TransferScheduleSpecs {
    /// The number of days charge funds are held before being paid out.
    ///
    /// May also be set to `minimum`, representing the lowest available value for the account country.
    /// Default is `minimum`.
    /// The `delay_days` parameter does not apply when the `interval` is `manual`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_days: Option<DelayDays>,
    /// How frequently available funds are paid out.
    ///
    /// One of: `daily`, `manual`, `weekly`, or `monthly`.
    /// Default is `daily`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<Interval>,
    /// The day of the month when available funds are paid out, specified as a number between 1--31.
    ///
    /// Payouts nominally scheduled between the 29th and 31st of the month are instead sent on the last day of a shorter month.
    /// Required and applicable only if `interval` is `monthly`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_anchor: Option<u8>,
    /// The day of the week when available funds are paid out, specified as `monday`, `tuesday`, etc.
    ///
    /// (required and applicable only if `interval` is `weekly`.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<WeeklyAnchor>,
}
impl TransferScheduleSpecs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct TreasurySettingsSpecs<'a> {
    /// Details on the account's acceptance of the Stripe Treasury Services Agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<SettingsTermsOfServiceSpecs<'a>>,
}
impl<'a> TreasurySettingsSpecs<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PayoutSettingsSpecs<'a> {
    /// A Boolean indicating whether Stripe should try to reclaim negative balances from an attached bank account.
    ///
    /// For details, see [Understanding Connect Account Balances](https://stripe.com/docs/connect/account-balances).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_negative_balances: Option<bool>,
    /// Details on when funds from charges are available, and when they are paid out to an external account.
    ///
    /// For details, see our [Setting Bank and Debit Card Payouts](https://stripe.com/docs/connect/bank-transfers#payout-information) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<TransferScheduleSpecs>,
    /// The text that appears on the bank account statement for payouts.
    ///
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}
impl<'a> PayoutSettingsSpecs<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct Settings<'a> {
    /// Settings used to apply the account's branding to email receipts, invoices, Checkout, and other products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding: Option<BrandingSettingsSpecs<'a>>,
    /// Settings specific to the account's use of the Card Issuing product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<CardIssuingSettingsSpecs<'a>>,
    /// Settings specific to card charging on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CardPaymentsSettingsSpecs<'a>>,
    /// Settings that apply across payment methods for charging on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<PaymentsSettingsSpecs<'a>>,
    /// Settings specific to the account's payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<PayoutSettingsSpecs<'a>>,
    /// Settings specific to the account's Treasury FinancialAccounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<TreasurySettingsSpecs<'a>>,
}
impl<'a> Settings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
