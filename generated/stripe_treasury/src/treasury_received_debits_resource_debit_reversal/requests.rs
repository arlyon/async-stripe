#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryReceivedDebitsResourceDebitReversal<'a> {
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
    /// The ReceivedDebit to reverse.
    pub received_debit: &'a str,
}
impl<'a> CreateTreasuryReceivedDebitsResourceDebitReversal<'a> {
    pub fn new(received_debit: &'a str) -> Self {
        Self { expand: Default::default(), metadata: Default::default(), received_debit }
    }
}
impl<'a> CreateTreasuryReceivedDebitsResourceDebitReversal<'a> {
    /// Reverses a ReceivedDebit and creates a DebitReversal object.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_treasury::TreasuryReceivedDebitsResourceDebitReversal> {
        client.send_form("/treasury/debit_reversals", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryReceivedDebitsResourceDebitReversal<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryReceivedDebitsResourceDebitReversal<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryReceivedDebitsResourceDebitReversal<'a> {
    /// Retrieves a DebitReversal object.
    pub fn send(&self, client: &stripe::Client, debit_reversal: &stripe_treasury::treasury_received_debits_resource_debit_reversal::TreasuryDebitReversalId) -> stripe::Response<stripe_treasury::TreasuryReceivedDebitsResourceDebitReversal> {
        client.get_query(&format!("/treasury/debit_reversals/{debit_reversal}", debit_reversal = debit_reversal), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListTreasuryReceivedDebitsResourceDebitReversal<'a> {
    /// A cursor for use in pagination.
    ///
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
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only return DebitReversals for the ReceivedDebit ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_debit: Option<&'a str>,
    /// Only return DebitReversals for a given resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<ListTreasuryReceivedDebitsResourceDebitReversalResolution>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Only return DebitReversals for a given status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListTreasuryReceivedDebitsResourceDebitReversalStatus>,
}
impl<'a> ListTreasuryReceivedDebitsResourceDebitReversal<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self { ending_before: Default::default(), expand: Default::default(), financial_account, limit: Default::default(), received_debit: Default::default(), resolution: Default::default(), starting_after: Default::default(), status: Default::default() }
    }
}
/// Only return DebitReversals for a given resolution.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryReceivedDebitsResourceDebitReversalResolution {
    Lost,
    Won,
}

impl ListTreasuryReceivedDebitsResourceDebitReversalResolution {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryReceivedDebitsResourceDebitReversalResolution::*;
        match self {
            Lost => "lost",
            Won => "won",
        }
    }
}

impl std::str::FromStr for ListTreasuryReceivedDebitsResourceDebitReversalResolution {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryReceivedDebitsResourceDebitReversalResolution::*;
        match s {
            "lost" => Ok(Lost),
            "won" => Ok(Won),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListTreasuryReceivedDebitsResourceDebitReversalResolution {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTreasuryReceivedDebitsResourceDebitReversalResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryReceivedDebitsResourceDebitReversalResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryReceivedDebitsResourceDebitReversalResolution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Only return DebitReversals for a given status.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryReceivedDebitsResourceDebitReversalStatus {
    Canceled,
    Completed,
    Processing,
}

impl ListTreasuryReceivedDebitsResourceDebitReversalStatus {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryReceivedDebitsResourceDebitReversalStatus::*;
        match self {
            Canceled => "canceled",
            Completed => "completed",
            Processing => "processing",
        }
    }
}

impl std::str::FromStr for ListTreasuryReceivedDebitsResourceDebitReversalStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryReceivedDebitsResourceDebitReversalStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "completed" => Ok(Completed),
            "processing" => Ok(Processing),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListTreasuryReceivedDebitsResourceDebitReversalStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTreasuryReceivedDebitsResourceDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryReceivedDebitsResourceDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryReceivedDebitsResourceDebitReversalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> ListTreasuryReceivedDebitsResourceDebitReversal<'a> {
    /// Returns a list of DebitReversals.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryReceivedDebitsResourceDebitReversal>> {
        client.get_query("/treasury/debit_reversals", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_treasury::TreasuryReceivedDebitsResourceDebitReversal> {
        stripe::ListPaginator::from_params("/treasury/debit_reversals", self)
    }
}
impl<'a> stripe::PaginationParams for ListTreasuryReceivedDebitsResourceDebitReversal<'a> {}
