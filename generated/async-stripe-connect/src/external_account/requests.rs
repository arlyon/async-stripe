use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Delete a specified external account for a given account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteExternalAccount<'a> {
    account: &'a stripe_shared::AccountId,
    id: &'a str,
}
impl<'a> DeleteExternalAccount<'a> {
    /// Construct a new `DeleteExternalAccount`.
    pub fn new(account: &'a stripe_shared::AccountId, id: &'a str) -> Self {
        Self { account, id }
    }
}
impl DeleteExternalAccount<'_> {
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

impl StripeRequest for DeleteExternalAccount<'_> {
    type Output = stripe_shared::DeletedExternalAccount;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        let id = self.id;
        RequestBuilder::new(
            StripeMethod::Delete,
            format!("/accounts/{account}/external_accounts/{id}"),
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListAccountExternalAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object: Option<ListAccountExternalAccountObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListAccountExternalAccountBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, object: None, starting_after: None }
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListAccountExternalAccountObject::*;
        match s {
            "bank_account" => Ok(BankAccount),
            "card" => Ok(Card),
            _ => Err(stripe_types::StripeParseError),
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
/// List external accounts for an account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListAccountExternalAccount<'a> {
    inner: ListAccountExternalAccountBuilder<'a>,
    account: &'a stripe_shared::AccountId,
}
impl<'a> ListAccountExternalAccount<'a> {
    /// Construct a new `ListAccountExternalAccount`.
    pub fn new(account: &'a stripe_shared::AccountId) -> Self {
        Self { account, inner: ListAccountExternalAccountBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// Filter external accounts according to a particular object type.
    pub fn object(mut self, object: ListAccountExternalAccountObject) -> Self {
        self.inner.object = Some(object);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl ListAccountExternalAccount<'_> {
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

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::ExternalAccount>> {
        let account = self.account;

        stripe_client_core::ListPaginator::new_list(
            format!("/accounts/{account}/external_accounts"),
            self.inner,
        )
    }
}

impl StripeRequest for ListAccountExternalAccount<'_> {
    type Output = stripe_types::List<stripe_shared::ExternalAccount>;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        RequestBuilder::new(StripeMethod::Get, format!("/accounts/{account}/external_accounts"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveExternalAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveExternalAccountBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieve a specified external account for a given account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveExternalAccount<'a> {
    inner: RetrieveExternalAccountBuilder<'a>,
    account: &'a stripe_shared::AccountId,
    id: &'a str,
}
impl<'a> RetrieveExternalAccount<'a> {
    /// Construct a new `RetrieveExternalAccount`.
    pub fn new(account: &'a stripe_shared::AccountId, id: &'a str) -> Self {
        Self { account, id, inner: RetrieveExternalAccountBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveExternalAccount<'_> {
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

impl StripeRequest for RetrieveExternalAccount<'_> {
    type Output = stripe_shared::ExternalAccount;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        let id = self.id;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/accounts/{account}/external_accounts/{id}"),
        )
        .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateAccountExternalAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_for_currency: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    external_account: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> CreateAccountExternalAccountBuilder<'a> {
    fn new(external_account: &'a str) -> Self {
        Self { default_for_currency: None, expand: None, external_account, metadata: None }
    }
}
/// Create an external account for a given account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateAccountExternalAccount<'a> {
    inner: CreateAccountExternalAccountBuilder<'a>,
    account: &'a stripe_shared::AccountId,
}
impl<'a> CreateAccountExternalAccount<'a> {
    /// Construct a new `CreateAccountExternalAccount`.
    pub fn new(account: &'a stripe_shared::AccountId, external_account: &'a str) -> Self {
        Self { account, inner: CreateAccountExternalAccountBuilder::new(external_account) }
    }
    /// When set to true, or if this is the first external account added in this currency, this account becomes the default external account for its currency.
    pub fn default_for_currency(mut self, default_for_currency: bool) -> Self {
        self.inner.default_for_currency = Some(default_for_currency);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
}
impl CreateAccountExternalAccount<'_> {
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

impl StripeRequest for CreateAccountExternalAccount<'_> {
    type Output = stripe_shared::ExternalAccount;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        RequestBuilder::new(StripeMethod::Post, format!("/accounts/{account}/external_accounts"))
            .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateExternalAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_holder_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_holder_type: Option<UpdateExternalAccountAccountHolderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_type: Option<UpdateExternalAccountAccountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_city: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_country: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line1: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line2: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_state: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_zip: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_for_currency: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documents: Option<UpdateExternalAccountDocuments<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exp_month: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exp_year: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
}
impl<'a> UpdateExternalAccountBuilder<'a> {
    fn new() -> Self {
        Self {
            account_holder_name: None,
            account_holder_type: None,
            account_type: None,
            address_city: None,
            address_country: None,
            address_line1: None,
            address_line2: None,
            address_state: None,
            address_zip: None,
            default_for_currency: None,
            documents: None,
            exp_month: None,
            exp_year: None,
            expand: None,
            metadata: None,
            name: None,
        }
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateExternalAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateExternalAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "futsu" => Ok(Futsu),
            "savings" => Ok(Savings),
            "toza" => Ok(Toza),
            _ => Err(stripe_types::StripeParseError),
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateExternalAccountDocuments<'a> {
    /// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
    /// Must be a document associated with the bank account that displays the last 4 digits of the account number, either a statement or a voided check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification:
        Option<UpdateExternalAccountDocumentsBankAccountOwnershipVerification<'a>>,
}
impl<'a> UpdateExternalAccountDocuments<'a> {
    pub fn new() -> Self {
        Self { bank_account_ownership_verification: None }
    }
}
impl<'a> Default for UpdateExternalAccountDocuments<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
/// Must be a document associated with the bank account that displays the last 4 digits of the account number, either a statement or a voided check.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateExternalAccountDocumentsBankAccountOwnershipVerification<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> UpdateExternalAccountDocumentsBankAccountOwnershipVerification<'a> {
    pub fn new() -> Self {
        Self { files: None }
    }
}
impl<'a> Default for UpdateExternalAccountDocumentsBankAccountOwnershipVerification<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Updates the metadata, account holder name, account holder type of a bank account belonging to
/// a connected account and optionally sets it as the default for its currency. Other bank account
/// details are not editable by design.
///
/// You can only update bank accounts when <a href="/api/accounts/object#account_object-controller-requirement_collection">account.controller.requirement_collection</a> is `application`, which includes <a href="/connect/custom-accounts">Custom accounts</a>.
///
/// You can re-enable a disabled bank account by performing an update call without providing any
/// arguments or changes.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateExternalAccount<'a> {
    inner: UpdateExternalAccountBuilder<'a>,
    account: &'a stripe_shared::AccountId,
    id: &'a str,
}
impl<'a> UpdateExternalAccount<'a> {
    /// Construct a new `UpdateExternalAccount`.
    pub fn new(account: &'a stripe_shared::AccountId, id: &'a str) -> Self {
        Self { account, id, inner: UpdateExternalAccountBuilder::new() }
    }
    /// The name of the person or business that owns the bank account.
    pub fn account_holder_name(mut self, account_holder_name: &'a str) -> Self {
        self.inner.account_holder_name = Some(account_holder_name);
        self
    }
    /// The type of entity that holds the account. This can be either `individual` or `company`.
    pub fn account_holder_type(
        mut self,
        account_holder_type: UpdateExternalAccountAccountHolderType,
    ) -> Self {
        self.inner.account_holder_type = Some(account_holder_type);
        self
    }
    /// The bank account type.
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    pub fn account_type(mut self, account_type: UpdateExternalAccountAccountType) -> Self {
        self.inner.account_type = Some(account_type);
        self
    }
    /// City/District/Suburb/Town/Village.
    pub fn address_city(mut self, address_city: &'a str) -> Self {
        self.inner.address_city = Some(address_city);
        self
    }
    /// Billing address country, if provided when creating card.
    pub fn address_country(mut self, address_country: &'a str) -> Self {
        self.inner.address_country = Some(address_country);
        self
    }
    /// Address line 1 (Street address/PO Box/Company name).
    pub fn address_line1(mut self, address_line1: &'a str) -> Self {
        self.inner.address_line1 = Some(address_line1);
        self
    }
    /// Address line 2 (Apartment/Suite/Unit/Building).
    pub fn address_line2(mut self, address_line2: &'a str) -> Self {
        self.inner.address_line2 = Some(address_line2);
        self
    }
    /// State/County/Province/Region.
    pub fn address_state(mut self, address_state: &'a str) -> Self {
        self.inner.address_state = Some(address_state);
        self
    }
    /// ZIP or postal code.
    pub fn address_zip(mut self, address_zip: &'a str) -> Self {
        self.inner.address_zip = Some(address_zip);
        self
    }
    /// When set to true, this becomes the default external account for its currency.
    pub fn default_for_currency(mut self, default_for_currency: bool) -> Self {
        self.inner.default_for_currency = Some(default_for_currency);
        self
    }
    /// Documents that may be submitted to satisfy various informational requests.
    pub fn documents(mut self, documents: UpdateExternalAccountDocuments<'a>) -> Self {
        self.inner.documents = Some(documents);
        self
    }
    /// Two digit number representing the card’s expiration month.
    pub fn exp_month(mut self, exp_month: &'a str) -> Self {
        self.inner.exp_month = Some(exp_month);
        self
    }
    /// Four digit number representing the card’s expiration year.
    pub fn exp_year(mut self, exp_year: &'a str) -> Self {
        self.inner.exp_year = Some(exp_year);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// Cardholder name.
    pub fn name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }
}
impl UpdateExternalAccount<'_> {
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

impl StripeRequest for UpdateExternalAccount<'_> {
    type Output = stripe_shared::ExternalAccount;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        let id = self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/accounts/{account}/external_accounts/{id}"),
        )
        .form(&self.inner)
    }
}
