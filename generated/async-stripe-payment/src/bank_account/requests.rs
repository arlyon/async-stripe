use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Delete a specified external account for a given account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteAccountBankAccount<'a> {
    account: &'a stripe_shared::AccountId,
    id: &'a str,
}
impl<'a> DeleteAccountBankAccount<'a> {
    /// Construct a new `DeleteAccountBankAccount`.
    pub fn new(account: &'a stripe_shared::AccountId, id: &'a str) -> Self {
        Self { account, id }
    }
}
impl DeleteAccountBankAccount<'_> {
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

impl StripeRequest for DeleteAccountBankAccount<'_> {
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
struct DeleteCustomerBankAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> DeleteCustomerBankAccountBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Delete a specified source for a given customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteCustomerBankAccount<'a> {
    inner: DeleteCustomerBankAccountBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
    id: &'a str,
}
impl<'a> DeleteCustomerBankAccount<'a> {
    /// Construct a new `DeleteCustomerBankAccount`.
    pub fn new(customer: &'a stripe_shared::CustomerId, id: &'a str) -> Self {
        Self { customer, id, inner: DeleteCustomerBankAccountBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl DeleteCustomerBankAccount<'_> {
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

impl StripeRequest for DeleteCustomerBankAccount<'_> {
    type Output = DeleteCustomerBankAccountReturned;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        let id = self.id;
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

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

    impl<'a> Map for Builder<'a> {
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

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateAccountBankAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_holder_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_holder_type: Option<UpdateAccountBankAccountAccountHolderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_type: Option<UpdateAccountBankAccountAccountType>,
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
    documents: Option<UpdateAccountBankAccountDocuments<'a>>,
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
impl<'a> UpdateAccountBankAccountBuilder<'a> {
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateAccountBankAccountDocuments<'a> {
    /// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
    /// Must be a document associated with the bank account that displays the last 4 digits of the account number, either a statement or a voided check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification:
        Option<UpdateAccountBankAccountDocumentsBankAccountOwnershipVerification<'a>>,
}
impl<'a> UpdateAccountBankAccountDocuments<'a> {
    pub fn new() -> Self {
        Self { bank_account_ownership_verification: None }
    }
}
impl<'a> Default for UpdateAccountBankAccountDocuments<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
/// Must be a document associated with the bank account that displays the last 4 digits of the account number, either a statement or a voided check.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateAccountBankAccountDocumentsBankAccountOwnershipVerification<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> UpdateAccountBankAccountDocumentsBankAccountOwnershipVerification<'a> {
    pub fn new() -> Self {
        Self { files: None }
    }
}
impl<'a> Default for UpdateAccountBankAccountDocumentsBankAccountOwnershipVerification<'a> {
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
pub struct UpdateAccountBankAccount<'a> {
    inner: UpdateAccountBankAccountBuilder<'a>,
    account: &'a stripe_shared::AccountId,
    id: &'a str,
}
impl<'a> UpdateAccountBankAccount<'a> {
    /// Construct a new `UpdateAccountBankAccount`.
    pub fn new(account: &'a stripe_shared::AccountId, id: &'a str) -> Self {
        Self { account, id, inner: UpdateAccountBankAccountBuilder::new() }
    }
    /// The name of the person or business that owns the bank account.
    pub fn account_holder_name(mut self, account_holder_name: &'a str) -> Self {
        self.inner.account_holder_name = Some(account_holder_name);
        self
    }
    /// The type of entity that holds the account. This can be either `individual` or `company`.
    pub fn account_holder_type(
        mut self,
        account_holder_type: UpdateAccountBankAccountAccountHolderType,
    ) -> Self {
        self.inner.account_holder_type = Some(account_holder_type);
        self
    }
    /// The bank account type.
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    pub fn account_type(mut self, account_type: UpdateAccountBankAccountAccountType) -> Self {
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
    pub fn documents(mut self, documents: UpdateAccountBankAccountDocuments<'a>) -> Self {
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
impl UpdateAccountBankAccount<'_> {
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

impl StripeRequest for UpdateAccountBankAccount<'_> {
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateCustomerBankAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_holder_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_holder_type: Option<UpdateCustomerBankAccountAccountHolderType>,
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
    exp_month: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exp_year: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<UpdateCustomerBankAccountOwner<'a>>,
}
impl<'a> UpdateCustomerBankAccountBuilder<'a> {
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateCustomerBankAccountOwner<'a> {
    /// Owner's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateCustomerBankAccountOwnerAddress<'a>>,
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
impl<'a> UpdateCustomerBankAccountOwner<'a> {
    pub fn new() -> Self {
        Self { address: None, email: None, name: None, phone: None }
    }
}
impl<'a> Default for UpdateCustomerBankAccountOwner<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Owner's address.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateCustomerBankAccountOwnerAddress<'a> {
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
impl<'a> UpdateCustomerBankAccountOwnerAddress<'a> {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl<'a> Default for UpdateCustomerBankAccountOwnerAddress<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Update a specified source for a given customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCustomerBankAccount<'a> {
    inner: UpdateCustomerBankAccountBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
    id: &'a str,
}
impl<'a> UpdateCustomerBankAccount<'a> {
    /// Construct a new `UpdateCustomerBankAccount`.
    pub fn new(customer: &'a stripe_shared::CustomerId, id: &'a str) -> Self {
        Self { customer, id, inner: UpdateCustomerBankAccountBuilder::new() }
    }
    /// The name of the person or business that owns the bank account.
    pub fn account_holder_name(mut self, account_holder_name: &'a str) -> Self {
        self.inner.account_holder_name = Some(account_holder_name);
        self
    }
    /// The type of entity that holds the account. This can be either `individual` or `company`.
    pub fn account_holder_type(
        mut self,
        account_holder_type: UpdateCustomerBankAccountAccountHolderType,
    ) -> Self {
        self.inner.account_holder_type = Some(account_holder_type);
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
    pub fn owner(mut self, owner: UpdateCustomerBankAccountOwner<'a>) -> Self {
        self.inner.owner = Some(owner);
        self
    }
}
impl UpdateCustomerBankAccount<'_> {
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

impl StripeRequest for UpdateCustomerBankAccount<'_> {
    type Output = UpdateCustomerBankAccountReturned;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        let id = self.id;
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
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "bank_account")
    )]
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

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

    impl<'a> Map for Builder<'a> {
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

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct VerifyBankAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amounts: Option<&'a [i64]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> VerifyBankAccountBuilder<'a> {
    fn new() -> Self {
        Self { amounts: None, expand: None }
    }
}
/// Verify a specified bank account for a given customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct VerifyBankAccount<'a> {
    inner: VerifyBankAccountBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
    id: &'a str,
}
impl<'a> VerifyBankAccount<'a> {
    /// Construct a new `VerifyBankAccount`.
    pub fn new(customer: &'a stripe_shared::CustomerId, id: &'a str) -> Self {
        Self { customer, id, inner: VerifyBankAccountBuilder::new() }
    }
    /// Two positive integers, in *cents*, equal to the values of the microdeposits sent to the bank account.
    pub fn amounts(mut self, amounts: &'a [i64]) -> Self {
        self.inner.amounts = Some(amounts);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl VerifyBankAccount<'_> {
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

impl StripeRequest for VerifyBankAccount<'_> {
    type Output = stripe_shared::BankAccount;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        let id = self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/customers/{customer}/sources/{id}/verify"),
        )
        .form(&self.inner)
    }
}
