#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListTreasuryDebitReversal<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Returns objects associated with this FinancialAccount.
    pub financial_account: &'a str,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only return DebitReversals for the ReceivedDebit ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_debit: Option<&'a str>,
    /// Only return DebitReversals for a given resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<ListTreasuryDebitReversalResolution>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Only return DebitReversals for a given status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListTreasuryDebitReversalStatus>,
}
impl<'a> ListTreasuryDebitReversal<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self {
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            received_debit: None,
            resolution: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Only return DebitReversals for a given resolution.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryDebitReversalResolution {
    Lost,
    Won,
}
impl ListTreasuryDebitReversalResolution {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryDebitReversalResolution::*;
        match self {
            Lost => "lost",
            Won => "won",
        }
    }
}

impl std::str::FromStr for ListTreasuryDebitReversalResolution {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryDebitReversalResolution::*;
        match s {
            "lost" => Ok(Lost),
            "won" => Ok(Won),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ListTreasuryDebitReversalResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryDebitReversalResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryDebitReversalResolution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListTreasuryDebitReversalResolution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ListTreasuryDebitReversalResolution")
        })
    }
}
/// Only return DebitReversals for a given status.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryDebitReversalStatus {
    Canceled,
    Completed,
    Processing,
}
impl ListTreasuryDebitReversalStatus {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryDebitReversalStatus::*;
        match self {
            Canceled => "canceled",
            Completed => "completed",
            Processing => "processing",
        }
    }
}

impl std::str::FromStr for ListTreasuryDebitReversalStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryDebitReversalStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "completed" => Ok(Completed),
            "processing" => Ok(Processing),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ListTreasuryDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryDebitReversalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListTreasuryDebitReversalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ListTreasuryDebitReversalStatus")
        })
    }
}
impl<'a> ListTreasuryDebitReversal<'a> {
    /// Returns a list of DebitReversals.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryDebitReversal>> {
        client.get_query("/treasury/debit_reversals", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_treasury::TreasuryDebitReversal>> {
        stripe::ListPaginator::from_list_params("/treasury/debit_reversals", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryDebitReversal<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryDebitReversal<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryDebitReversal<'a> {
    /// Retrieves a DebitReversal object.
    pub fn send(
        &self,
        client: &stripe::Client,
        debit_reversal: &stripe_treasury::TreasuryDebitReversalId,
    ) -> stripe::Response<stripe_treasury::TreasuryDebitReversal> {
        client.get_query(&format!("/treasury/debit_reversals/{debit_reversal}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryDebitReversal<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The ReceivedDebit to reverse.
    pub received_debit: &'a str,
}
impl<'a> CreateTreasuryDebitReversal<'a> {
    pub fn new(received_debit: &'a str) -> Self {
        Self { expand: None, metadata: None, received_debit }
    }
}
impl<'a> CreateTreasuryDebitReversal<'a> {
    /// Reverses a ReceivedDebit and creates a DebitReversal object.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_treasury::TreasuryDebitReversal> {
        client.send_form("/treasury/debit_reversals", self, http_types::Method::Post)
    }
}
