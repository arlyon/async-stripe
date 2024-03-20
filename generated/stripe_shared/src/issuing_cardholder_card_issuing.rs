#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IssuingCardholderCardIssuing {
    /// Information about cardholder acceptance of Celtic [Authorized User Terms](https://stripe.com/docs/issuing/cards#accept-authorized-user-terms).
    /// Required for cards backed by a Celtic program.
    pub user_terms_acceptance: Option<stripe_shared::IssuingCardholderUserTermsAcceptance>,
}
