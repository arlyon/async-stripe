use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Delete a specified external account for a given account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteExternalAccount {
    account: stripe_shared::AccountId,
    id: String,
}
impl DeleteExternalAccount {
    /// Construct a new `DeleteExternalAccount`.
    pub fn new(account: impl Into<stripe_shared::AccountId>, id: impl Into<String>) -> Self {
        Self { account: account.into(), id: id.into() }
    }
}
impl DeleteExternalAccount {
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

impl StripeRequest for DeleteExternalAccount {
    type Output = stripe_shared::DeletedExternalAccount;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Delete,
            format!("/accounts/{account}/external_accounts/{id}"),
        )
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ListAccountExternalAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object: Option<ListAccountExternalAccountObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListAccountExternalAccountBuilder {
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
pub struct ListAccountExternalAccount {
    inner: ListAccountExternalAccountBuilder,
    account: stripe_shared::AccountId,
}
impl ListAccountExternalAccount {
    /// Construct a new `ListAccountExternalAccount`.
    pub fn new(account: impl Into<stripe_shared::AccountId>) -> Self {
        Self { account: account.into(), inner: ListAccountExternalAccountBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// Filter external accounts according to a particular object type.
    pub fn object(mut self, object: impl Into<ListAccountExternalAccountObject>) -> Self {
        self.inner.object = Some(object.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl ListAccountExternalAccount {
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
        let account = &self.account;

        stripe_client_core::ListPaginator::new_list(
            format!("/accounts/{account}/external_accounts"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListAccountExternalAccount {
    type Output = stripe_types::List<stripe_shared::ExternalAccount>;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(StripeMethod::Get, format!("/accounts/{account}/external_accounts"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveExternalAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveExternalAccountBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieve a specified external account for a given account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveExternalAccount {
    inner: RetrieveExternalAccountBuilder,
    account: stripe_shared::AccountId,
    id: String,
}
impl RetrieveExternalAccount {
    /// Construct a new `RetrieveExternalAccount`.
    pub fn new(account: impl Into<stripe_shared::AccountId>, id: impl Into<String>) -> Self {
        Self {
            account: account.into(),
            id: id.into(),
            inner: RetrieveExternalAccountBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveExternalAccount {
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

impl StripeRequest for RetrieveExternalAccount {
    type Output = stripe_shared::ExternalAccount;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/accounts/{account}/external_accounts/{id}"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateAccountExternalAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_for_currency: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    external_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl CreateAccountExternalAccountBuilder {
    fn new(external_account: impl Into<String>) -> Self {
        Self {
            default_for_currency: None,
            expand: None,
            external_account: external_account.into(),
            metadata: None,
        }
    }
}
/// Create an external account for a given account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateAccountExternalAccount {
    inner: CreateAccountExternalAccountBuilder,
    account: stripe_shared::AccountId,
}
impl CreateAccountExternalAccount {
    /// Construct a new `CreateAccountExternalAccount`.
    pub fn new(
        account: impl Into<stripe_shared::AccountId>,
        external_account: impl Into<String>,
    ) -> Self {
        Self {
            account: account.into(),
            inner: CreateAccountExternalAccountBuilder::new(external_account.into()),
        }
    }
    /// When set to true, or if this is the first external account added in this currency, this account becomes the default external account for its currency.
    pub fn default_for_currency(mut self, default_for_currency: impl Into<bool>) -> Self {
        self.inner.default_for_currency = Some(default_for_currency.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl CreateAccountExternalAccount {
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

impl StripeRequest for CreateAccountExternalAccount {
    type Output = stripe_shared::ExternalAccount;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(StripeMethod::Post, format!("/accounts/{account}/external_accounts"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateExternalAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_holder_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_holder_type: Option<UpdateExternalAccountAccountHolderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_type: Option<UpdateExternalAccountAccountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_for_currency: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documents: Option<UpdateExternalAccountDocuments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exp_month: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exp_year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
impl UpdateExternalAccountBuilder {
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateExternalAccountDocuments {
    /// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
    /// Must be a document associated with the bank account that displays the last 4 digits of the account number, either a statement or a check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification:
        Option<UpdateExternalAccountDocumentsBankAccountOwnershipVerification>,
}
impl UpdateExternalAccountDocuments {
    pub fn new() -> Self {
        Self { bank_account_ownership_verification: None }
    }
}
impl Default for UpdateExternalAccountDocuments {
    fn default() -> Self {
        Self::new()
    }
}
/// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
/// Must be a document associated with the bank account that displays the last 4 digits of the account number, either a statement or a check.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateExternalAccountDocumentsBankAccountOwnershipVerification {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}
impl UpdateExternalAccountDocumentsBankAccountOwnershipVerification {
    pub fn new() -> Self {
        Self { files: None }
    }
}
impl Default for UpdateExternalAccountDocumentsBankAccountOwnershipVerification {
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
pub struct UpdateExternalAccount {
    inner: UpdateExternalAccountBuilder,
    account: stripe_shared::AccountId,
    id: String,
}
impl UpdateExternalAccount {
    /// Construct a new `UpdateExternalAccount`.
    pub fn new(account: impl Into<stripe_shared::AccountId>, id: impl Into<String>) -> Self {
        Self { account: account.into(), id: id.into(), inner: UpdateExternalAccountBuilder::new() }
    }
    /// The name of the person or business that owns the bank account.
    pub fn account_holder_name(mut self, account_holder_name: impl Into<String>) -> Self {
        self.inner.account_holder_name = Some(account_holder_name.into());
        self
    }
    /// The type of entity that holds the account. This can be either `individual` or `company`.
    pub fn account_holder_type(
        mut self,
        account_holder_type: impl Into<UpdateExternalAccountAccountHolderType>,
    ) -> Self {
        self.inner.account_holder_type = Some(account_holder_type.into());
        self
    }
    /// The bank account type.
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    pub fn account_type(
        mut self,
        account_type: impl Into<UpdateExternalAccountAccountType>,
    ) -> Self {
        self.inner.account_type = Some(account_type.into());
        self
    }
    /// City/District/Suburb/Town/Village.
    pub fn address_city(mut self, address_city: impl Into<String>) -> Self {
        self.inner.address_city = Some(address_city.into());
        self
    }
    /// Billing address country, if provided when creating card.
    pub fn address_country(mut self, address_country: impl Into<String>) -> Self {
        self.inner.address_country = Some(address_country.into());
        self
    }
    /// Address line 1 (Street address/PO Box/Company name).
    pub fn address_line1(mut self, address_line1: impl Into<String>) -> Self {
        self.inner.address_line1 = Some(address_line1.into());
        self
    }
    /// Address line 2 (Apartment/Suite/Unit/Building).
    pub fn address_line2(mut self, address_line2: impl Into<String>) -> Self {
        self.inner.address_line2 = Some(address_line2.into());
        self
    }
    /// State/County/Province/Region.
    pub fn address_state(mut self, address_state: impl Into<String>) -> Self {
        self.inner.address_state = Some(address_state.into());
        self
    }
    /// ZIP or postal code.
    pub fn address_zip(mut self, address_zip: impl Into<String>) -> Self {
        self.inner.address_zip = Some(address_zip.into());
        self
    }
    /// When set to true, this becomes the default external account for its currency.
    pub fn default_for_currency(mut self, default_for_currency: impl Into<bool>) -> Self {
        self.inner.default_for_currency = Some(default_for_currency.into());
        self
    }
    /// Documents that may be submitted to satisfy various informational requests.
    pub fn documents(mut self, documents: impl Into<UpdateExternalAccountDocuments>) -> Self {
        self.inner.documents = Some(documents.into());
        self
    }
    /// Two digit number representing the card’s expiration month.
    pub fn exp_month(mut self, exp_month: impl Into<String>) -> Self {
        self.inner.exp_month = Some(exp_month.into());
        self
    }
    /// Four digit number representing the card’s expiration year.
    pub fn exp_year(mut self, exp_year: impl Into<String>) -> Self {
        self.inner.exp_year = Some(exp_year.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// Cardholder name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
}
impl UpdateExternalAccount {
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

impl StripeRequest for UpdateExternalAccount {
    type Output = stripe_shared::ExternalAccount;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/accounts/{account}/external_accounts/{id}"),
        )
        .form(&self.inner)
    }
}
