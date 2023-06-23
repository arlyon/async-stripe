/// When an [issued card](https://stripe.com/docs/issuing) is used to make a purchase, an Issuing `Authorization`
/// object is created.
///
/// [Authorizations](https://stripe.com/docs/issuing/purchases/authorizations) must be approved for the purchase to be completed successfully.  Related guide: [Issued Card Authorizations](https://stripe.com/docs/issuing/purchases/authorizations).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Authorization {
    /// The total amount that was authorized or rejected.
    ///
    /// This amount is in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    /// Detailed breakdown of amount components.
    ///
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<stripe_core::issuing::authorization::amount_details::AmountDetails>,
    /// Whether the authorization has been approved.
    pub approved: bool,
    /// How the card details were provided.
    pub authorization_method: AuthorizationAuthorizationMethod,
    /// List of balance transactions associated with this authorization.
    pub balance_transactions: Vec<stripe_core::balance_transaction::BalanceTransaction>,
    pub card: stripe_core::issuing::card::Card,
    /// The cardholder to whom this authorization belongs.
    pub cardholder: Option<stripe_types::Expandable<stripe_core::issuing::cardholder::Cardholder>>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Unique identifier for the object.
    pub id: stripe_core::issuing::authorization::IssuingAuthorizationId,
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
    pub merchant_data: stripe_core::issuing::authorization::merchant_data::MerchantData,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Details about the authorization, such as identifiers, set by the card network.
    pub network_data: Option<stripe_core::issuing::authorization::network_data::NetworkData>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: AuthorizationObject,
    /// The pending authorization request.
    ///
    /// This field will only be non-null during an `issuing_authorization.request` webhook.
    pub pending_request:
        Option<stripe_core::issuing::authorization::pending_request::PendingRequest>,
    /// History of every time `pending_request` was approved/denied, either by you directly or by Stripe (e.g.
    ///
    /// based on your `spending_controls`).
    /// If the merchant changes the authorization by performing an [incremental authorization](https://stripe.com/docs/issuing/purchases/authorizations), you can look at this field to see the previous requests for the authorization.
    pub request_history: Vec<stripe_core::issuing::authorization::request_history::RequestHistory>,
    /// The current status of the authorization in its lifecycle.
    pub status: AuthorizationStatus,
    /// List of [transactions](https://stripe.com/docs/api/issuing/transactions) associated with this authorization.
    pub transactions: Vec<stripe_core::issuing::transaction::Transaction>,
    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this authorization if it was created on a [FinancialAccount](https://stripe.com/docs/api/treasury/financial_accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_core::issuing::authorization::treasury::Treasury>,
    pub verification_data: stripe_core::issuing::authorization::verification_data::VerificationData,
    /// The digital wallet used for this authorization.
    ///
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    pub wallet: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Authorization {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// How the card details were provided.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationAuthorizationMethod {
    Chip,
    Contactless,
    KeyedIn,
    Online,
    Swipe,
}

impl AuthorizationAuthorizationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Chip => "chip",
            Self::Contactless => "contactless",
            Self::KeyedIn => "keyed_in",
            Self::Online => "online",
            Self::Swipe => "swipe",
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
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationObject {
    #[serde(rename = "issuing.authorization")]
    IssuingAuthorization,
}

impl AuthorizationObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IssuingAuthorization => "issuing.authorization",
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
/// The current status of the authorization in its lifecycle.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationStatus {
    Closed,
    Pending,
    Reversed,
}

impl AuthorizationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Pending => "pending",
            Self::Reversed => "reversed",
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
impl stripe_types::Object for Authorization {
    type Id = stripe_core::issuing::authorization::IssuingAuthorizationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(IssuingAuthorizationId, "iauth_");
pub mod amount_details;
pub mod requests;
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
