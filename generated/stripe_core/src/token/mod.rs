/// Tokenization is the process Stripe uses to collect sensitive card or bank
/// account details, or personally identifiable information (PII), directly from
/// your customers in a secure manner.
///
/// A token representing this information is returned to your server to use.
/// You should use our [recommended payments integrations](https://stripe.com/docs/payments) to perform this process client-side.
/// This ensures that no sensitive card data touches your server, and allows your integration to operate in a PCI-compliant way.  If you cannot use client-side tokenization, you can also create tokens using the API with either your publishable or secret API key.
/// Keep in mind that if your integration uses this method, you are responsible for any PCI compliance that may be required, and you must keep your secret API key safe.
/// Unlike with client-side tokenization, your customer's information is not sent directly to Stripe, so we cannot determine how it is handled or stored.  Tokens cannot be stored or used more than once.
/// To store card or bank account information for later use, you can create [Customer](https://stripe.com/docs/api#customers) objects or [Custom accounts](https://stripe.com/docs/api#external_accounts).
/// Note that [Radar](https://stripe.com/docs/radar), our integrated solution for automatic fraud protection, performs best with integrations that use client-side tokenization.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Token {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<stripe_types::BankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types::Card>,
    /// IP address of the client that generated the token.
    pub client_ip: Option<String>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_core::token::TokenId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Type of the token: `account`, `bank_account`, `card`, or `pii`.
    #[serde(rename = "type")]
    pub type_: String,
    /// Whether this token has already been used (tokens can be used only once).
    pub used: bool,
}
impl stripe_types::Object for Token {
    type Id = stripe_core::token::TokenId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(TokenId);
#[cfg(feature = "token")]
mod requests;
#[cfg(feature = "token")]
pub use requests::*;
