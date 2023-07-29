/// To top up your Stripe balance, you create a top-up object.
///
/// You can retrieve individual top-ups, as well as list all top-ups.
/// Top-ups are identified by a unique, random ID.  Related guide: [Topping Up your Platform Account](https://stripe.com/docs/connect/top-ups).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Topup {
    /// Amount transferred.
    pub amount: i64,
    /// ID of the balance transaction that describes the impact of this top-up on your account balance.
    ///
    /// May not be specified depending on status of top-up.
    pub balance_transaction:
        Option<stripe_types::Expandable<stripe_types::balance_transaction::BalanceTransaction>>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// Date the funds are expected to arrive in your Stripe account for payouts.
    ///
    /// This factors in delays like weekends or bank holidays.
    /// May not be specified depending on status of top-up.
    pub expected_availability_date: Option<stripe_types::Timestamp>,
    /// Error code explaining reason for top-up failure if available (see [the errors section](https://stripe.com/docs/api#errors) for a list of codes).
    pub failure_code: Option<String>,
    /// Message to user further explaining reason for top-up failure if available.
    pub failure_message: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_types::topup::TopupId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TopupObject,
    /// For most Stripe users, the source of every top-up is a bank account.
    ///
    /// This hash is then the [source object](https://stripe.com/docs/api#source_object) describing that bank account.
    pub source: Option<stripe_types::source::Source>,
    /// Extra information about a top-up.
    ///
    /// This will appear on your source's bank statement.
    /// It must contain at least one letter.
    pub statement_descriptor: Option<String>,
    /// The status of the top-up is either `canceled`, `failed`, `pending`, `reversed`, or `succeeded`.
    pub status: TopupStatus,
    /// A string that identifies this top-up as part of a group.
    pub transfer_group: Option<String>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TopupObject {
    Topup,
}

impl TopupObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Topup => "topup",
        }
    }
}

impl std::str::FromStr for TopupObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "topup" => Ok(Self::Topup),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for TopupObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TopupObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TopupObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TopupObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TopupObject"))
    }
}
/// The status of the top-up is either `canceled`, `failed`, `pending`, `reversed`, or `succeeded`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TopupStatus {
    Canceled,
    Failed,
    Pending,
    Reversed,
    Succeeded,
}

impl TopupStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Failed => "failed",
            Self::Pending => "pending",
            Self::Reversed => "reversed",
            Self::Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TopupStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "canceled" => Ok(Self::Canceled),
            "failed" => Ok(Self::Failed),
            "pending" => Ok(Self::Pending),
            "reversed" => Ok(Self::Reversed),
            "succeeded" => Ok(Self::Succeeded),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for TopupStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TopupStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TopupStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TopupStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TopupStatus"))
    }
}
impl stripe_types::Object for Topup {
    type Id = stripe_types::topup::TopupId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TopupId, "tu_");
