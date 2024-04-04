#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentPagesCheckoutSessionAfterExpirationRecovery {
    /// Enables user redeemable promotion codes on the recovered Checkout Sessions. Defaults to `false`
    pub allow_promotion_codes: bool,
    /// If `true`, a recovery url will be generated to recover this Checkout Session if it
    /// expires before a transaction is completed. It will be attached to the
    /// Checkout Session object upon expiration.
    pub enabled: bool,
    /// The timestamp at which the recovery URL will expire.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// URL that creates a new Checkout Session when clicked that is a copy of this expired Checkout Session.
    pub url: Option<String>,
}
