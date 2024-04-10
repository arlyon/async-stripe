#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteAccountBankAccount {}
impl DeleteAccountBankAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteAccountBankAccount {
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
pub struct DeleteCustomerBankAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> DeleteCustomerBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> DeleteCustomerBankAccount<'a> {
    /// Delete a specified source for a given customer.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_shared::CustomerId,
        id: &str,
    ) -> stripe::Response<DeleteCustomerBankAccountReturned> {
        client.send_form(
            &format!("/customers/{customer}/sources/{id}"),
            self,
            http_types::Method::Delete,
        )
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

#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountBankAccount<'a> {
    /// The name of the person or business that owns the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<&'a str>,
    /// The type of entity that holds the account. This can be either `individual` or `company`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UpdateAccountBankAccountAccountHolderType>,
    /// The bank account type.
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<UpdateAccountBankAccountAccountType>,
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
    pub documents: Option<UpdateAccountBankAccountDocuments<'a>>,
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
impl<'a> UpdateAccountBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "futsu" => Ok(Futsu),
            "savings" => Ok(Savings),
            "toza" => Ok(Toza),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountBankAccountDocuments<'a> {
    /// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
    /// Must be a document associated with the bank account that displays the last 4 digits of the account number, either a statement or a voided check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification:
        Option<UpdateAccountBankAccountDocumentsBankAccountOwnershipVerification<'a>>,
}
impl<'a> UpdateAccountBankAccountDocuments<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
/// Must be a document associated with the bank account that displays the last 4 digits of the account number, either a statement or a voided check.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountBankAccountDocumentsBankAccountOwnershipVerification<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> UpdateAccountBankAccountDocumentsBankAccountOwnershipVerification<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateAccountBankAccount<'a> {
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerBankAccount<'a> {
    /// The name of the person or business that owns the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<&'a str>,
    /// The type of entity that holds the account. This can be either `individual` or `company`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UpdateCustomerBankAccountAccountHolderType>,
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
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<UpdateCustomerBankAccountOwner<'a>>,
}
impl<'a> UpdateCustomerBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCustomerBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
    }
}
/// Owner's address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
    }
}
impl<'a> UpdateCustomerBankAccount<'a> {
    /// Update a specified source for a given customer.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_shared::CustomerId,
        id: &str,
    ) -> stripe::Response<UpdateCustomerBankAccountReturned> {
        client.send_form(
            &format!("/customers/{customer}/sources/{id}"),
            self,
            http_types::Method::Post,
        )
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

#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct VerifyBankAccount<'a> {
    /// Two positive integers, in *cents*, equal to the values of the microdeposits sent to the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<&'a [i64]>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> VerifyBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> VerifyBankAccount<'a> {
    /// Verify a specified bank account for a given customer.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_shared::CustomerId,
        id: &str,
    ) -> stripe::Response<stripe_shared::BankAccount> {
        client.send_form(
            &format!("/customers/{customer}/sources/{id}/verify"),
            self,
            http_types::Method::Post,
        )
    }
}
