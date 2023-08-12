#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTransfer<'a> {
    /// A positive integer in cents (or local equivalent) representing how much to transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// 3-letter [ISO code for currency](https://stripe.com/docs/payouts).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The ID of a connected Stripe account.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    pub destination: &'a str,
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
    /// You can use this parameter to transfer funds from a charge before they are added to your available balance.
    ///
    /// A pending balance will transfer immediately but the funds will not become available until the original charge becomes available.
    /// [See the Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-availability) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transaction: Option<&'a str>,
    /// The source balance to use for this transfer.
    ///
    /// One of `bank_account`, `card`, or `fpx`.
    /// For most users, this will default to `card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<CreateTransferSourceType>,
    /// A string that identifies this transaction as part of a group.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}
impl<'a> CreateTransfer<'a> {
    pub fn new(currency: stripe_types::Currency, destination: &'a str) -> Self {
        Self {
            amount: Default::default(),
            currency,
            description: Default::default(),
            destination,
            expand: Default::default(),
            metadata: Default::default(),
            source_transaction: Default::default(),
            source_type: Default::default(),
            transfer_group: Default::default(),
        }
    }
}
/// The source balance to use for this transfer.
///
/// One of `bank_account`, `card`, or `fpx`.
/// For most users, this will default to `card`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTransferSourceType {
    BankAccount,
    Card,
    Fpx,
}

impl CreateTransferSourceType {
    pub fn as_str(self) -> &'static str {
        use CreateTransferSourceType::*;
        match self {
            BankAccount => "bank_account",
            Card => "card",
            Fpx => "fpx",
        }
    }
}

impl std::str::FromStr for CreateTransferSourceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTransferSourceType::*;
        match s {
            "bank_account" => Ok(BankAccount),
            "card" => Ok(Card),
            "fpx" => Ok(Fpx),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTransferSourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTransferSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTransferSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTransferSourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreateTransfer<'a> {
    /// To send funds from your Stripe account to a connected account, you create a new transfer object.
    ///
    /// Your [Stripe balance](https://stripe.com/docs/api#balance) must be able to cover the transfer amount, or you’ll receive an “Insufficient Funds” error.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::Transfer> {
        client.send_form("/transfers", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListTransfer<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// Only return transfers for the destination specified by this account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
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
    pub starting_after: Option<&'a str>,
    /// Only return transfers with the specified transfer group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}
impl<'a> ListTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListTransfer<'a> {
    /// Returns a list of existing transfers sent to connected accounts.
    ///
    /// The transfers are returned in sorted order, with the most recently created transfers appearing first.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::Transfer>> {
        client.get_query("/transfers", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::Transfer> {
        stripe::ListPaginator::from_params("/transfers", self)
    }
}
impl<'a> stripe::PaginationParams for ListTransfer<'a> {}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTransfer<'a> {
    /// Retrieves the details of an existing transfer.
    ///
    /// Supply the unique transfer ID from either a transfer creation request or the transfer list, and Stripe will return the corresponding transfer information.
    pub fn send(
        &self,
        client: &stripe::Client,
        transfer: &stripe_types::transfer::TransferId,
    ) -> stripe::Response<stripe_types::Transfer> {
        client.get_query(&format!("/transfers/{transfer}", transfer = transfer), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTransfer<'a> {
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
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
}
impl<'a> UpdateTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateTransfer<'a> {
    /// Updates the specified transfer by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.  This request accepts only metadata as an argument.
    pub fn send(
        &self,
        client: &stripe::Client,
        transfer: &stripe_types::transfer::TransferId,
    ) -> stripe::Response<stripe_types::Transfer> {
        client.send_form(
            &format!("/transfers/{transfer}", transfer = transfer),
            self,
            http_types::Method::Post,
        )
    }
}
