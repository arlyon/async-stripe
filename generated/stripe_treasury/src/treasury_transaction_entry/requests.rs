#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListTreasuryTransactionEntry<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<stripe_types::RangeQueryTs>,
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
    /// The results are in reverse chronological order by `created` or `effective_at`.
    /// The default is `created`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<ListTreasuryTransactionEntryOrderBy>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Only return TransactionEntries associated with this Transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<&'a str>,
}
impl<'a> ListTreasuryTransactionEntry<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self {
            created: None,
            effective_at: None,
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            order_by: None,
            starting_after: None,
            transaction: None,
        }
    }
}
/// The results are in reverse chronological order by `created` or `effective_at`.
/// The default is `created`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryTransactionEntryOrderBy {
    Created,
    EffectiveAt,
}
impl ListTreasuryTransactionEntryOrderBy {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryTransactionEntryOrderBy::*;
        match self {
            Created => "created",
            EffectiveAt => "effective_at",
        }
    }
}

impl std::str::FromStr for ListTreasuryTransactionEntryOrderBy {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryTransactionEntryOrderBy::*;
        match s {
            "created" => Ok(Created),
            "effective_at" => Ok(EffectiveAt),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ListTreasuryTransactionEntryOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryTransactionEntryOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryTransactionEntryOrderBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListTreasuryTransactionEntryOrderBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ListTreasuryTransactionEntryOrderBy")
        })
    }
}
impl<'a> ListTreasuryTransactionEntry<'a> {
    /// Retrieves a list of TransactionEntry objects.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryTransactionEntry>> {
        client.get_query("/treasury/transaction_entries", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_treasury::TreasuryTransactionEntry>> {
        stripe::ListPaginator::from_list_params("/treasury/transaction_entries", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryTransactionEntry<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryTransactionEntry<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryTransactionEntry<'a> {
    /// Retrieves a TransactionEntry object.
    pub fn send(
        &self,
        client: &stripe::Client,
        id: &stripe_treasury::TreasuryTransactionEntryId,
    ) -> stripe::Response<stripe_treasury::TreasuryTransactionEntry> {
        client.get_query(&format!("/treasury/transaction_entries/{id}"), self)
    }
}
