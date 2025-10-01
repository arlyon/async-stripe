use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Delete a specified external account for a given account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteAccountBankAccount {
    account: stripe_shared::AccountId,
    id: String,
}
impl DeleteAccountBankAccount {
    /// Construct a new `DeleteAccountBankAccount`.
    pub fn new(account: impl Into<stripe_shared::AccountId>, id: impl Into<String>) -> Self {
        Self { account: account.into(), id: id.into() }
    }
}
impl DeleteAccountBankAccount {
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

impl StripeRequest for DeleteAccountBankAccount {
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
struct DeleteCustomerBankAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl DeleteCustomerBankAccountBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Delete a specified source for a given customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteCustomerBankAccount {
    inner: DeleteCustomerBankAccountBuilder,
    customer: stripe_shared::CustomerId,
    id: String,
}
impl DeleteCustomerBankAccount {
    /// Construct a new `DeleteCustomerBankAccount`.
    pub fn new(customer: impl Into<stripe_shared::CustomerId>, id: impl Into<String>) -> Self {
        Self {
            customer: customer.into(),
            id: id.into(),
            inner: DeleteCustomerBankAccountBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl DeleteCustomerBankAccount {
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

impl StripeRequest for DeleteCustomerBankAccount {
    type Output = DeleteCustomerBankAccountReturned;

    fn build(&self) -> RequestBuilder {
        let customer = &self.customer;
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Delete, format!("/customers/{customer}/sources/{id}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(untagged))]
pub enum DeleteCustomerBankAccountReturned {
    PaymentSource(stripe_shared::PaymentSource),
    DeletedPaymentSource(stripe_shared::DeletedPaymentSource),
}

#[derive(Default)]
pub struct DeleteCustomerBankAccountReturnedBuilder {
    inner: stripe_types::miniserde_helpers::MaybeDeletedBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::MapBuilder;
    use stripe_types::miniserde_helpers::FromValueOpt;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<DeleteCustomerBankAccountReturned>,
        builder: DeleteCustomerBankAccountReturnedBuilder,
    }

    impl Deserialize for DeleteCustomerBankAccountReturned {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<DeleteCustomerBankAccountReturned> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for DeleteCustomerBankAccountReturnedBuilder {
        type Out = DeleteCustomerBankAccountReturned;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (deleted, o) = self.inner.finish_inner()?;
            Some(if deleted {
                DeleteCustomerBankAccountReturned::DeletedPaymentSource(FromValueOpt::from_value(
                    Value::Object(o),
                )?)
            } else {
                DeleteCustomerBankAccountReturned::PaymentSource(FromValueOpt::from_value(
                    Value::Object(o),
                )?)
            })
        }
    }

    impl stripe_types::ObjectDeser for DeleteCustomerBankAccountReturned {
        type Builder = DeleteCustomerBankAccountReturnedBuilder;
    }
};

#[derive(Clone, Debug, serde::Serialize)]
struct UpdateAccountBankAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_holder_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_holder_type: Option<UpdateAccountBankAccountAccountHolderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_type: Option<UpdateAccountBankAccountAccountType>,
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
    documents: Option<UpdateAccountBankAccountDocuments>,
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
impl UpdateAccountBankAccountBuilder {
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
pub enum UpdateAccountBankAccountAccountHolderType {
    Company,
    Individual,
}
impl UpdateAccountBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use UpdateAccountBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for UpdateAccountBankAccountAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateAccountBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateAccountBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateAccountBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateAccountBankAccountAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateAccountBankAccountAccountHolderType")
        })
    }
}
/// The bank account type.
/// This can only be `checking` or `savings` in most countries.
/// In Japan, this can only be `futsu` or `toza`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateAccountBankAccountAccountType {
    Checking,
    Futsu,
    Savings,
    Toza,
}
impl UpdateAccountBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use UpdateAccountBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Futsu => "futsu",
            Savings => "savings",
            Toza => "toza",
        }
    }
}

impl std::str::FromStr for UpdateAccountBankAccountAccountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "futsu" => Ok(Futsu),
            "savings" => Ok(Savings),
            "toza" => Ok(Toza),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateAccountBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateAccountBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateAccountBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateAccountBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateAccountBankAccountAccountType")
        })
    }
}
/// Documents that may be submitted to satisfy various informational requests.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateAccountBankAccountDocuments {
    /// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
    /// Must be a document associated with the bank account that displays the last 4 digits of the account number, either a statement or a check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification:
        Option<UpdateAccountBankAccountDocumentsBankAccountOwnershipVerification>,
}
impl UpdateAccountBankAccountDocuments {
    pub fn new() -> Self {
        Self { bank_account_ownership_verification: None }
    }
}
impl Default for UpdateAccountBankAccountDocuments {
    fn default() -> Self {
        Self::new()
    }
}
/// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
/// Must be a document associated with the bank account that displays the last 4 digits of the account number, either a statement or a check.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateAccountBankAccountDocumentsBankAccountOwnershipVerification {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}
impl UpdateAccountBankAccountDocumentsBankAccountOwnershipVerification {
    pub fn new() -> Self {
        Self { files: None }
    }
}
impl Default for UpdateAccountBankAccountDocumentsBankAccountOwnershipVerification {
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
pub struct UpdateAccountBankAccount {
    inner: UpdateAccountBankAccountBuilder,
    account: stripe_shared::AccountId,
    id: String,
}
impl UpdateAccountBankAccount {
    /// Construct a new `UpdateAccountBankAccount`.
    pub fn new(account: impl Into<stripe_shared::AccountId>, id: impl Into<String>) -> Self {
        Self {
            account: account.into(),
            id: id.into(),
            inner: UpdateAccountBankAccountBuilder::new(),
        }
    }
    /// The name of the person or business that owns the bank account.
    pub fn account_holder_name(mut self, account_holder_name: impl Into<String>) -> Self {
        self.inner.account_holder_name = Some(account_holder_name.into());
        self
    }
    /// The type of entity that holds the account. This can be either `individual` or `company`.
    pub fn account_holder_type(
        mut self,
        account_holder_type: impl Into<UpdateAccountBankAccountAccountHolderType>,
    ) -> Self {
        self.inner.account_holder_type = Some(account_holder_type.into());
        self
    }
    /// The bank account type.
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    pub fn account_type(
        mut self,
        account_type: impl Into<UpdateAccountBankAccountAccountType>,
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
    pub fn documents(mut self, documents: impl Into<UpdateAccountBankAccountDocuments>) -> Self {
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
impl UpdateAccountBankAccount {
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

impl StripeRequest for UpdateAccountBankAccount {
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
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateCustomerBankAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_holder_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_holder_type: Option<UpdateCustomerBankAccountAccountHolderType>,
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
    exp_month: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exp_year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<UpdateCustomerBankAccountOwner>,
}
impl UpdateCustomerBankAccountBuilder {
    fn new() -> Self {
        Self {
            account_holder_name: None,
            account_holder_type: None,
            address_city: None,
            address_country: None,
            address_line1: None,
            address_line2: None,
            address_state: None,
            address_zip: None,
            exp_month: None,
            exp_year: None,
            expand: None,
            metadata: None,
            name: None,
            owner: None,
        }
    }
}
/// The type of entity that holds the account. This can be either `individual` or `company`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateCustomerBankAccountAccountHolderType {
    Company,
    Individual,
}
impl UpdateCustomerBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use UpdateCustomerBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for UpdateCustomerBankAccountAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCustomerBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateCustomerBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateCustomerBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateCustomerBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateCustomerBankAccountAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateCustomerBankAccountAccountHolderType")
        })
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCustomerBankAccountOwner {
    /// Owner's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateCustomerBankAccountOwnerAddress>,
    /// Owner's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Owner's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Owner's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl UpdateCustomerBankAccountOwner {
    pub fn new() -> Self {
        Self { address: None, email: None, name: None, phone: None }
    }
}
impl Default for UpdateCustomerBankAccountOwner {
    fn default() -> Self {
        Self::new()
    }
}
/// Owner's address.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCustomerBankAccountOwnerAddress {
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
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl UpdateCustomerBankAccountOwnerAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for UpdateCustomerBankAccountOwnerAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// Update a specified source for a given customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCustomerBankAccount {
    inner: UpdateCustomerBankAccountBuilder,
    customer: stripe_shared::CustomerId,
    id: String,
}
impl UpdateCustomerBankAccount {
    /// Construct a new `UpdateCustomerBankAccount`.
    pub fn new(customer: impl Into<stripe_shared::CustomerId>, id: impl Into<String>) -> Self {
        Self {
            customer: customer.into(),
            id: id.into(),
            inner: UpdateCustomerBankAccountBuilder::new(),
        }
    }
    /// The name of the person or business that owns the bank account.
    pub fn account_holder_name(mut self, account_holder_name: impl Into<String>) -> Self {
        self.inner.account_holder_name = Some(account_holder_name.into());
        self
    }
    /// The type of entity that holds the account. This can be either `individual` or `company`.
    pub fn account_holder_type(
        mut self,
        account_holder_type: impl Into<UpdateCustomerBankAccountAccountHolderType>,
    ) -> Self {
        self.inner.account_holder_type = Some(account_holder_type.into());
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
    pub fn owner(mut self, owner: impl Into<UpdateCustomerBankAccountOwner>) -> Self {
        self.inner.owner = Some(owner.into());
        self
    }
}
impl UpdateCustomerBankAccount {
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

impl StripeRequest for UpdateCustomerBankAccount {
    type Output = UpdateCustomerBankAccountReturned;

    fn build(&self) -> RequestBuilder {
        let customer = &self.customer;
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/customers/{customer}/sources/{id}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum UpdateCustomerBankAccountReturned {
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "bank_account"))]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct UpdateCustomerBankAccountReturnedBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::MapBuilder;
    use stripe_types::miniserde_helpers::FromValueOpt;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<UpdateCustomerBankAccountReturned>,
        builder: UpdateCustomerBankAccountReturnedBuilder,
    }

    impl Deserialize for UpdateCustomerBankAccountReturned {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<UpdateCustomerBankAccountReturned> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for UpdateCustomerBankAccountReturnedBuilder {
        type Out = UpdateCustomerBankAccountReturned;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            UpdateCustomerBankAccountReturned::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for UpdateCustomerBankAccountReturned {
        type Builder = UpdateCustomerBankAccountReturnedBuilder;
    }
    impl UpdateCustomerBankAccountReturned {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "bank_account" => Self::BankAccount(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for UpdateCustomerBankAccountReturned {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

#[derive(Clone, Debug, serde::Serialize)]
struct VerifyBankAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl VerifyBankAccountBuilder {
    fn new() -> Self {
        Self { amounts: None, expand: None }
    }
}
/// Verify a specified bank account for a given customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct VerifyBankAccount {
    inner: VerifyBankAccountBuilder,
    customer: stripe_shared::CustomerId,
    id: String,
}
impl VerifyBankAccount {
    /// Construct a new `VerifyBankAccount`.
    pub fn new(customer: impl Into<stripe_shared::CustomerId>, id: impl Into<String>) -> Self {
        Self { customer: customer.into(), id: id.into(), inner: VerifyBankAccountBuilder::new() }
    }
    /// Two positive integers, in *cents*, equal to the values of the microdeposits sent to the bank account.
    pub fn amounts(mut self, amounts: impl Into<Vec<i64>>) -> Self {
        self.inner.amounts = Some(amounts.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl VerifyBankAccount {
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

impl StripeRequest for VerifyBankAccount {
    type Output = stripe_shared::BankAccount;

    fn build(&self) -> RequestBuilder {
        let customer = &self.customer;
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/customers/{customer}/sources/{id}/verify"),
        )
        .form(&self.inner)
    }
}
