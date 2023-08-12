#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListIssuingTransaction<'a> {
    /// Only return transactions that belong to the given card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<&'a str>,
    /// Only return transactions that belong to the given cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<&'a str>,
    /// Only return transactions that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
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
    /// Only return transactions that have the given type.
    ///
    /// One of `capture` or `refund`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ListIssuingTransactionType>,
}
impl<'a> ListIssuingTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Only return transactions that have the given type.
///
/// One of `capture` or `refund`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListIssuingTransactionType {
    Capture,
    Refund,
}

impl ListIssuingTransactionType {
    pub fn as_str(self) -> &'static str {
        use ListIssuingTransactionType::*;
        match self {
            Capture => "capture",
            Refund => "refund",
        }
    }
}

impl std::str::FromStr for ListIssuingTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListIssuingTransactionType::*;
        match s {
            "capture" => Ok(Capture),
            "refund" => Ok(Refund),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListIssuingTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListIssuingTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListIssuingTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListIssuingTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> ListIssuingTransaction<'a> {
    /// Returns a list of Issuing `Transaction` objects.
    ///
    /// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::IssuingTransaction>> {
        client.get_query("/issuing/transactions", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::IssuingTransaction> {
        stripe::ListPaginator::from_params("/issuing/transactions", self)
    }
}
impl<'a> stripe::PaginationParams for ListIssuingTransaction<'a> {}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveIssuingTransaction<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveIssuingTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveIssuingTransaction<'a> {
    /// Retrieves an Issuing `Transaction` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        transaction: &stripe_types::issuing_transaction::IssuingTransactionId,
    ) -> stripe::Response<stripe_types::IssuingTransaction> {
        client.get_query(
            &format!("/issuing/transactions/{transaction}", transaction = transaction),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateIssuingTransaction<'a> {
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
impl<'a> UpdateIssuingTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateIssuingTransaction<'a> {
    /// Updates the specified Issuing `Transaction` object by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    pub fn send(
        &self,
        client: &stripe::Client,
        transaction: &stripe_types::issuing_transaction::IssuingTransactionId,
    ) -> stripe::Response<stripe_types::IssuingTransaction> {
        client.send_form(
            &format!("/issuing/transactions/{transaction}", transaction = transaction),
            self,
            http_types::Method::Post,
        )
    }
}
