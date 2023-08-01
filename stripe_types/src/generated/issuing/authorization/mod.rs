/// When an [issued card](https://stripe.com/docs/issuing) is used to make a purchase, an Issuing `Authorization`
/// object is created.
///
/// [Authorizations](https://stripe.com/docs/issuing/purchases/authorizations) must be approved for the purchase to be completed successfully.  Related guide: [Issued card authorizations](https://stripe.com/docs/issuing/purchases/authorizations).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Authorization {
    /// The total amount that was authorized or rejected.
    ///
    /// This amount is in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    /// Detailed breakdown of amount components.
    ///
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<stripe_types::issuing::authorization::amount_details::AmountDetails>,
    /// Whether the authorization has been approved.
    pub approved: bool,
    /// How the card details were provided.
    pub authorization_method: AuthorizationAuthorizationMethod,
    /// List of balance transactions associated with this authorization.
    pub balance_transactions: Vec<stripe_types::balance_transaction::BalanceTransaction>,
    pub card: stripe_types::issuing::card::Card,
    /// The cardholder to whom this authorization belongs.
    pub cardholder: Option<stripe_types::Expandable<stripe_types::issuing::cardholder::Cardholder>>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Unique identifier for the object.
    pub id: stripe_types::issuing::authorization::IssuingAuthorizationId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The total amount that was authorized or rejected.
    ///
    /// This amount is in the `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub merchant_amount: i64,
    /// The currency that was presented to the cardholder for the authorization.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub merchant_currency: stripe_types::Currency,
    pub merchant_data: stripe_types::issuing::authorization::merchant_data::MerchantData,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Details about the authorization, such as identifiers, set by the card network.
    pub network_data: Option<stripe_types::issuing::authorization::network_data::NetworkData>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: AuthorizationObject,
    /// The pending authorization request.
    ///
    /// This field will only be non-null during an `issuing_authorization.request` webhook.
    pub pending_request:
        Option<stripe_types::issuing::authorization::pending_request::PendingRequest>,
    /// History of every time a `pending_request` authorization was approved/declined, either by you directly or by Stripe (e.g.
    ///
    /// based on your spending_controls).
    /// If the merchant changes the authorization by performing an incremental authorization, you can look at this field to see the previous requests for the authorization.
    /// This field can be helpful in determining why a given authorization was approved/declined.
    pub request_history: Vec<stripe_types::issuing::authorization::request_history::RequestHistory>,
    /// The current status of the authorization in its lifecycle.
    pub status: AuthorizationStatus,
    /// List of [transactions](https://stripe.com/docs/api/issuing/transactions) associated with this authorization.
    pub transactions: Vec<stripe_types::issuing::transaction::Transaction>,
    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this authorization if it was created on a [FinancialAccount](https://stripe.com/docs/api/treasury/financial_accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_types::issuing::authorization::treasury::Treasury>,
    pub verification_data:
        stripe_types::issuing::authorization::verification_data::VerificationData,
    /// The digital wallet used for this transaction.
    ///
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    /// Will populate as `null` when no digital wallet was utilized.
    pub wallet: Option<String>,
}
/// How the card details were provided.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AuthorizationAuthorizationMethod {
    Chip,
    Contactless,
    KeyedIn,
    Online,
    Swipe,
}

impl AuthorizationAuthorizationMethod {
    pub fn as_str(self) -> &'static str {
        use AuthorizationAuthorizationMethod::*;
        match self {
            Chip => "chip",
            Contactless => "contactless",
            KeyedIn => "keyed_in",
            Online => "online",
            Swipe => "swipe",
        }
    }
}

impl std::str::FromStr for AuthorizationAuthorizationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AuthorizationAuthorizationMethod::*;
        match s {
            "chip" => Ok(Chip),
            "contactless" => Ok(Contactless),
            "keyed_in" => Ok(KeyedIn),
            "online" => Ok(Online),
            "swipe" => Ok(Swipe),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AuthorizationAuthorizationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AuthorizationAuthorizationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AuthorizationAuthorizationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AuthorizationAuthorizationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AuthorizationAuthorizationMethod")
        })
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AuthorizationObject {
    IssuingAuthorization,
}

impl AuthorizationObject {
    pub fn as_str(self) -> &'static str {
        use AuthorizationObject::*;
        match self {
            IssuingAuthorization => "issuing.authorization",
        }
    }
}

impl std::str::FromStr for AuthorizationObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AuthorizationObject::*;
        match s {
            "issuing.authorization" => Ok(IssuingAuthorization),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AuthorizationObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AuthorizationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AuthorizationObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AuthorizationObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AuthorizationObject"))
    }
}
/// The current status of the authorization in its lifecycle.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AuthorizationStatus {
    Closed,
    Pending,
    Reversed,
}

impl AuthorizationStatus {
    pub fn as_str(self) -> &'static str {
        use AuthorizationStatus::*;
        match self {
            Closed => "closed",
            Pending => "pending",
            Reversed => "reversed",
        }
    }
}

impl std::str::FromStr for AuthorizationStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AuthorizationStatus::*;
        match s {
            "closed" => Ok(Closed),
            "pending" => Ok(Pending),
            "reversed" => Ok(Reversed),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AuthorizationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AuthorizationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AuthorizationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AuthorizationStatus"))
    }
}
impl stripe_types::Object for Authorization {
    type Id = stripe_types::issuing::authorization::IssuingAuthorizationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(IssuingAuthorizationId, "iauth_");
pub mod amount_details;
pub use amount_details::AmountDetails;
pub mod merchant_data;
pub use merchant_data::MerchantData;
pub mod network_data;
pub use network_data::NetworkData;
pub mod pending_request;
pub use pending_request::PendingRequest;
pub mod request_history;
pub use request_history::RequestHistory;
pub mod treasury;
pub use treasury::Treasury;
pub mod verification_data;
pub use verification_data::VerificationData;
