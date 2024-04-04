/// A customer session allows you to grant client access to Stripe's frontend SDKs (like StripeJs)
/// control over a customer.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomerSession {
    /// The client secret of this customer session.
    /// Used on the client to set up secure access to the given `customer`.
    ///
    /// The client secret can be used to provide access to `customer` from your frontend.
    /// It should not be stored, logged, or exposed to anyone other than the relevant customer.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<stripe_core::CustomerSessionResourceComponents>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The customer the customer session was created for.
    pub customer: stripe_types::Expandable<stripe_shared::Customer>,
    /// The timestamp at which this customer session will expire.
    pub expires_at: stripe_types::Timestamp,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
