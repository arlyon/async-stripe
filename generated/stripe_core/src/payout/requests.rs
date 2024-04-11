#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListPayout<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// The ID of an external account - only return payouts sent to this external account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<&'a str>,
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Only return payouts that have the given status: `pending`, `paid`, `failed`, or `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
}
impl<'a> ListPayout<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListPayout<'a> {
    /// Returns a list of existing payouts sent to third-party bank accounts or payouts that Stripe sent to you.
    /// The payouts return in sorted order, with the most recently created payouts appearing first.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_shared::Payout>> {
        client.get_query("/payouts", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::List<stripe_shared::Payout>> {
        stripe::ListPaginator::from_list_params("/payouts", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePayout<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePayout<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrievePayout<'a> {
    /// Retrieves the details of an existing payout.
    /// Supply the unique payout ID from either a payout creation request or the payout list.
    /// Stripe returns the corresponding payout information.
    pub fn send(
        &self,
        client: &stripe::Client,
        payout: &stripe_shared::PayoutId,
    ) -> stripe::Response<stripe_shared::Payout> {
        client.get_query(&format!("/payouts/{payout}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePayout<'a> {
    /// A positive integer in cents representing how much to payout.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The ID of a bank account or a card to send the payout to.
    /// If you don't provide a destination, we use the default external account for the specified currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The method used to send this payout, which is `standard` or `instant`.
    /// We support `instant` for payouts to debit cards and bank accounts in certain countries.
    /// Learn more about [bank support for Instant Payouts](https://stripe.com/docs/payouts/instant-payouts-banks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<CreatePayoutMethod>,
    /// The balance type of your Stripe balance to draw this payout from.
    /// Balances for different payment sources are kept separately.
    /// You can find the amounts with the Balances API.
    /// One of `bank_account`, `card`, or `fpx`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<CreatePayoutSourceType>,
    /// A string that displays on the recipient's bank or card statement (up to 22 characters).
    /// A `statement_descriptor` that's longer than 22 characters return an error.
    /// Most banks truncate this information and display it inconsistently.
    /// Some banks might not display it at all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}
impl<'a> CreatePayout<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self {
            amount,
            currency,
            description: None,
            destination: None,
            expand: None,
            metadata: None,
            method: None,
            source_type: None,
            statement_descriptor: None,
        }
    }
}
/// The method used to send this payout, which is `standard` or `instant`.
/// We support `instant` for payouts to debit cards and bank accounts in certain countries.
/// Learn more about [bank support for Instant Payouts](https://stripe.com/docs/payouts/instant-payouts-banks).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePayoutMethod {
    Instant,
    Standard,
}
impl CreatePayoutMethod {
    pub fn as_str(self) -> &'static str {
        use CreatePayoutMethod::*;
        match self {
            Instant => "instant",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreatePayoutMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePayoutMethod::*;
        match s {
            "instant" => Ok(Instant),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePayoutMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePayoutMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePayoutMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePayoutMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreatePayoutMethod"))
    }
}
/// The balance type of your Stripe balance to draw this payout from.
/// Balances for different payment sources are kept separately.
/// You can find the amounts with the Balances API.
/// One of `bank_account`, `card`, or `fpx`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePayoutSourceType {
    BankAccount,
    Card,
    Fpx,
}
impl CreatePayoutSourceType {
    pub fn as_str(self) -> &'static str {
        use CreatePayoutSourceType::*;
        match self {
            BankAccount => "bank_account",
            Card => "card",
            Fpx => "fpx",
        }
    }
}

impl std::str::FromStr for CreatePayoutSourceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePayoutSourceType::*;
        match s {
            "bank_account" => Ok(BankAccount),
            "card" => Ok(Card),
            "fpx" => Ok(Fpx),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePayoutSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePayoutSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePayoutSourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePayoutSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreatePayoutSourceType"))
    }
}
impl<'a> CreatePayout<'a> {
    /// To send funds to your own bank account, create a new payout object.
    /// Your [Stripe balance](https://stripe.com/docs/api#balance) must cover the payout amount.
    /// If it doesn’t, you receive an “Insufficient Funds” error.
    ///
    /// If your API key is in test mode, money won’t actually be sent, though every other action occurs as if you’re in live mode.
    ///
    /// If you create a manual payout on a Stripe account that uses multiple payment source types, you need to specify the source type balance that the payout draws from.
    /// The [balance object](https://stripe.com/docs/api#balance_object) details available and pending amounts by source type.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_shared::Payout> {
        client.send_form("/payouts", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePayout<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdatePayout<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdatePayout<'a> {
    /// Updates the specified payout by setting the values of the parameters you pass.
    /// We don’t change parameters that you don’t provide.
    /// This request only accepts the metadata as arguments.
    pub fn send(
        &self,
        client: &stripe::Client,
        payout: &stripe_shared::PayoutId,
    ) -> stripe::Response<stripe_shared::Payout> {
        client.send_form(&format!("/payouts/{payout}"), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelPayout<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelPayout<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CancelPayout<'a> {
    /// You can cancel a previously created payout if its status is `pending`.
    /// Stripe refunds the funds to your available balance.
    /// You can’t cancel automatic Stripe payouts.
    pub fn send(
        &self,
        client: &stripe::Client,
        payout: &stripe_shared::PayoutId,
    ) -> stripe::Response<stripe_shared::Payout> {
        client.send_form(&format!("/payouts/{payout}/cancel"), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReversePayout<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> ReversePayout<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ReversePayout<'a> {
    /// Reverses a payout by debiting the destination bank account.
    /// At this time, you can only reverse payouts for connected accounts to US bank accounts.
    /// If the payout is manual and in the `pending` status, use `/v1/payouts/:id/cancel` instead.
    ///
    /// By requesting a reversal through `/v1/payouts/:id/reverse`, you confirm that the authorized signatory of the selected bank account authorizes the debit on the bank account and that no other authorization is required.
    pub fn send(
        &self,
        client: &stripe::Client,
        payout: &stripe_shared::PayoutId,
    ) -> stripe::Response<stripe_shared::Payout> {
        client.send_form(&format!("/payouts/{payout}/reverse"), self, http_types::Method::Post)
    }
}
