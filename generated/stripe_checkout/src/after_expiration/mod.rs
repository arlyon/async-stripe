#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AfterExpiration {
    /// When set, configuration used to recover the Checkout Session on expiry.
    pub recovery: Option<stripe_checkout::recovery::Recovery>,
}
