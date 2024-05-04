#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteExternalAccount {}
impl DeleteExternalAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteExternalAccount {
    /// Delete a specified external account for a given account.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_shared::AccountId,
        id: &str,
    ) -> stripe::Response<stripe_shared::DeletedExternalAccount> {
        client.send_form(
            &format!("/accounts/{account}/external_accounts/{id}"),
            self,
            http_types::Method::Delete,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListAccountExternalAccount<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Filter external accounts according to a particular object type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ListAccountExternalAccountObject>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListAccountExternalAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Filter external accounts according to a particular object type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListAccountExternalAccountObject {
    BankAccount,
    Card,
}
impl ListAccountExternalAccountObject {
    pub fn as_str(self) -> &'static str {
        use ListAccountExternalAccountObject::*;
        match self {
            BankAccount => "bank_account",
            Card => "card",
        }
    }
}

impl std::str::FromStr for ListAccountExternalAccountObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListAccountExternalAccountObject::*;
        match s {
            "bank_account" => Ok(BankAccount),
            "card" => Ok(Card),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ListAccountExternalAccountObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListAccountExternalAccountObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListAccountExternalAccountObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListAccountExternalAccountObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ListAccountExternalAccountObject")
        })
    }
}
impl<'a> ListAccountExternalAccount<'a> {
    /// List external accounts for an account.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_shared::AccountId,
    ) -> stripe::Response<stripe_types::List<stripe_shared::ExternalAccount>> {
        client.get_query(&format!("/accounts/{account}/external_accounts"), self)
    }
    pub fn paginate(
        self,
        account: &stripe_shared::AccountId,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_shared::ExternalAccount>> {
        stripe::ListPaginator::from_list_params(
            &format!("/accounts/{account}/external_accounts"),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveExternalAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveExternalAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveExternalAccount<'a> {
    /// Retrieve a specified external account for a given account.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_shared::AccountId,
        id: &str,
    ) -> stripe::Response<stripe_shared::ExternalAccount> {
        client.get_query(&format!("/accounts/{account}/external_accounts/{id}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountExternalAccount<'a> {
    /// When set to true, or if this is the first external account added in this currency, this account becomes the default external account for its currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Please refer to full [documentation](https://stripe.com/docs/api) instead.
    pub external_account: &'a str,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> CreateAccountExternalAccount<'a> {
    pub fn new(external_account: &'a str) -> Self {
        Self { default_for_currency: None, expand: None, external_account, metadata: None }
    }
}
impl<'a> CreateAccountExternalAccount<'a> {
    /// Create an external account for a given account.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_shared::AccountId,
    ) -> stripe::Response<stripe_shared::ExternalAccount> {
        client.send_form(
            &format!("/accounts/{account}/external_accounts"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateExternalAccount<'a> {
    /// The name of the person or business that owns the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<&'a str>,
    /// The type of entity that holds the account. This can be either `individual` or `company`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UpdateExternalAccountAccountHolderType>,
    /// The bank account type.
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<UpdateExternalAccountAccountType>,
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
    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<UpdateExternalAccountDocuments<'a>>,
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
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}
impl<'a> UpdateExternalAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The type of entity that holds the account. This can be either `individual` or `company`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateExternalAccountAccountHolderType {
    Company,
    Individual,
}
impl UpdateExternalAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use UpdateExternalAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for UpdateExternalAccountAccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateExternalAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UpdateExternalAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateExternalAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateExternalAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateExternalAccountAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateExternalAccountAccountHolderType")
        })
    }
}
/// The bank account type.
/// This can only be `checking` or `savings` in most countries.
/// In Japan, this can only be `futsu` or `toza`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateExternalAccountAccountType {
    Checking,
    Futsu,
    Savings,
    Toza,
}
impl UpdateExternalAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use UpdateExternalAccountAccountType::*;
        match self {
            Checking => "checking",
            Futsu => "futsu",
            Savings => "savings",
            Toza => "toza",
        }
    }
}

impl std::str::FromStr for UpdateExternalAccountAccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateExternalAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "futsu" => Ok(Futsu),
            "savings" => Ok(Savings),
            "toza" => Ok(Toza),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UpdateExternalAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateExternalAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateExternalAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateExternalAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateExternalAccountAccountType")
        })
    }
}
/// Documents that may be submitted to satisfy various informational requests.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateExternalAccountDocuments<'a> {
    /// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
    /// Must be a document associated with the bank account that displays the last 4 digits of the account number, either a statement or a voided check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification:
        Option<UpdateExternalAccountDocumentsBankAccountOwnershipVerification<'a>>,
}
impl<'a> UpdateExternalAccountDocuments<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
/// Must be a document associated with the bank account that displays the last 4 digits of the account number, either a statement or a voided check.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateExternalAccountDocumentsBankAccountOwnershipVerification<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> UpdateExternalAccountDocumentsBankAccountOwnershipVerification<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateExternalAccount<'a> {
    /// Updates the metadata, account holder name, account holder type of a bank account belonging to a [Custom account](https://stripe.com/docs/connect/custom-accounts), and optionally sets it as the default for its currency.
    /// Other bank account details are not editable by design.
    ///
    /// You can re-enable a disabled bank account by performing an update call without providing any arguments or changes.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_shared::AccountId,
        id: &str,
    ) -> stripe::Response<stripe_shared::ExternalAccount> {
        client.send_form(
            &format!("/accounts/{account}/external_accounts/{id}"),
            self,
            http_types::Method::Post,
        )
    }
}