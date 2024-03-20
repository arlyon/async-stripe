/// Tokenization is the process Stripe uses to collect sensitive card or bank
/// account details, or personally identifiable information (PII), directly from
/// your customers in a secure manner. A token representing this information is
/// returned to your server to use. Use our
/// [recommended payments integrations](https://stripe.com/docs/payments) to perform this process
/// on the client-side. This guarantees that no sensitive card data touches your server,
/// and allows your integration to operate in a PCI-compliant way.
///
/// If you can't use client-side tokenization, you can also create tokens using
/// the API with either your publishable or secret API key. If
/// your integration uses this method, you're responsible for any PCI compliance
/// that it might require, and you must keep your secret API key safe. Unlike with
/// client-side tokenization, your customer's information isn't sent directly to
/// Stripe, so we can't determine how it's handled or stored.
///
/// You can't store or use tokens more than once. To store card or bank account
/// information for later use, create [Customer](https://stripe.com/docs/api#customers)
/// objects or [Custom accounts](https://stripe.com/docs/api#external_accounts).
/// [Radar](https://stripe.com/docs/radar), our integrated solution for automatic fraud protection,
/// performs best with integrations that use client-side tokenization.
///
/// For more details see <<https://stripe.com/docs/api/tokens/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Token {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<stripe_shared::BankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_shared::Card>,
    /// IP address of the client that generates the token.
    pub client_ip: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_core::TokenId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Type of the token: `account`, `bank_account`, `card`, or `pii`.
    #[serde(rename = "type")]
    pub type_: String,
    /// Determines if you have already used this token (you can only use tokens once).
    pub used: bool,
}
impl stripe_types::Object for Token {
    type Id = stripe_core::TokenId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TokenId);
