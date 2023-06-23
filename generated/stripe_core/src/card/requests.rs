use stripe::{Client, Response};

impl stripe_core::card::Card {
    /// Update a specified source for a given customer.
    pub fn update_customer(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
        id: &str,
        params: UpdateCustomerCard,
    ) -> Response<UpdateCustomerReturned> {
        client.send_form(
            &format!("/customers/{customer}/sources/{id}", customer = customer, id = id),
            params,
            http_types::Method::Post,
        )
    }
    /// Delete a specified source for a given customer.
    pub fn delete_customer(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
        id: &str,
        params: DeleteCustomerCard,
    ) -> Response<DeleteCustomerReturned> {
        client.send_form(
            &format!("/customers/{customer}/sources/{id}", customer = customer, id = id),
            params,
            http_types::Method::Delete,
        )
    }
    /// Updates the metadata, account holder name, account holder type of a bank account belonging to a [Custom account](https://stripe.com/docs/connect/custom-accounts), and optionally sets it as the default for its currency.
    ///
    /// Other bank account details are not editable by design.  You can re-enable a disabled bank account by performing an update call without providing any arguments or changes.
    pub fn update_account(
        client: &Client,
        account: &stripe_types::AccountId,
        id: &str,
        params: UpdateAccountCard,
    ) -> Response<stripe_core::external_account::ExternalAccount> {
        client.send_form(
            &format!("/accounts/{account}/external_accounts/{id}", account = account, id = id),
            params,
            http_types::Method::Post,
        )
    }
    /// Delete a specified external account for a given account.
    pub fn delete_account(
        client: &Client,
        account: &stripe_types::AccountId,
        id: &str,
    ) -> Response<stripe_core::external_account::DeletedExternalAccount> {
        client.send(
            &format!("/accounts/{account}/external_accounts/{id}", account = account, id = id),
            http_types::Method::Delete,
        )
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum UpdateCustomerReturned {
    Card(stripe_core::card::Card),
    BankAccount(stripe_core::bank_account::BankAccount),
    Source(stripe_core::source::Source),
}
#[cfg(feature = "min-ser")]
impl crate::deser::StripeDeserialize for UpdateCustomerReturned {
    fn deserialize(str: &str) -> Result<Self, crate::StripeError> {
        use miniserde::json::from_str;
        let obj_name: crate::deser::ObjectName = from_str(str)?;
        Ok(match obj_name.object.as_str() {
            "card" => Self::Card(from_str(str)?),
            "bank_account" => Self::BankAccount(from_str(str)?),
            "source" => Self::Source(from_str(str)?),
            _ => return Err(crate::StripeError::JSONDeserialize("Unexpected object name".into())),
        })
    }
}

#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerCard<'a> {
    /// The name of the person or business that owns the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<&'a str>,
    /// The type of entity that holds the account.
    ///
    /// This can be either `individual` or `company`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UpdateCustomerCardAccountHolderType>,
    /// City/District/Suburb/Town/Village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<&'a str>,
    /// Billing address country, if provided when creating card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<&'a str>,
    /// Address line 1 (Street address/PO Box/Company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<&'a str>,
    /// Address line 2 (Apartment/Suite/Unit/Building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<&'a str>,
    /// State/County/Province/Region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<&'a str>,
    /// Two digit number representing the card’s expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<&'a str>,
    /// Four digit number representing the card’s expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<UpdateCustomerCardOwner<'a>>,
}
impl<'a> UpdateCustomerCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The type of entity that holds the account.
///
/// This can be either `individual` or `company`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateCustomerCardAccountHolderType {
    Company,
    Individual,
}

impl UpdateCustomerCardAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for UpdateCustomerCardAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateCustomerCardAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerCardOwner<'a> {
    /// Owner's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateCustomerCardOwnerAddress<'a>>,
    /// Owner's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Owner's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Owner's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> UpdateCustomerCardOwner<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Owner's address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerCardOwnerAddress<'a> {
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
impl<'a> UpdateCustomerCardOwnerAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum DeleteCustomerReturned {
    PaymentSource(stripe_core::payment_source::PaymentSource),
    DeletedPaymentSource(stripe_core::payment_source::DeletedPaymentSource),
}
#[cfg(feature = "min-ser")]
impl crate::deser::StripeDeserialize for DeleteCustomerReturned {
    fn deserialize(str: &str) -> Result<Self, crate::StripeError> {
        use miniserde::json::from_str;

        use crate::deser::StripeDeserialize;
        let maybe_deleted: crate::deser::MaybeDeleted = from_str(str)?;
        let deleted = maybe_deleted.deleted.unwrap_or(false);
        if deleted {
            Ok(Self::DeletedPaymentSource(StripeDeserialize::deserialize(str)?))
        } else {
            Ok(Self::PaymentSource(StripeDeserialize::deserialize(str)?))
        }
    }
}

#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteCustomerCard<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> DeleteCustomerCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCard<'a> {
    /// The name of the person or business that owns the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<&'a str>,
    /// The type of entity that holds the account.
    ///
    /// This can be either `individual` or `company`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UpdateAccountCardAccountHolderType>,
    /// The bank account type.
    ///
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<UpdateAccountCardAccountType>,
    /// City/District/Suburb/Town/Village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<&'a str>,
    /// Billing address country, if provided when creating card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<&'a str>,
    /// Address line 1 (Street address/PO Box/Company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<&'a str>,
    /// Address line 2 (Apartment/Suite/Unit/Building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<&'a str>,
    /// State/County/Province/Region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<&'a str>,
    /// When set to true, this becomes the default external account for its currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,
    /// Two digit number representing the card’s expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<&'a str>,
    /// Four digit number representing the card’s expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}
impl<'a> UpdateAccountCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The type of entity that holds the account.
///
/// This can be either `individual` or `company`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateAccountCardAccountHolderType {
    Company,
    Individual,
}

impl UpdateAccountCardAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for UpdateAccountCardAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateAccountCardAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The bank account type.
///
/// This can only be `checking` or `savings` in most countries.
/// In Japan, this can only be `futsu` or `toza`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateAccountCardAccountType {
    Checking,
    Futsu,
    Savings,
    Toza,
}

impl UpdateAccountCardAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Futsu => "futsu",
            Self::Savings => "savings",
            Self::Toza => "toza",
        }
    }
}

impl AsRef<str> for UpdateAccountCardAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateAccountCardAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
