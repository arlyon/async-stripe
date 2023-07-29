#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AfterExpiration {
    /// When set, configuration used to recover the Checkout Session on expiry.
    pub recovery: Option<stripe_checkout::checkout::session::after_expiration::recovery::Recovery>,
}
pub mod recovery;
pub use recovery::Recovery;
