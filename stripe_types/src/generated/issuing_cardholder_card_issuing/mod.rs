#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IssuingCardholderCardIssuing {
    /// Information about cardholder acceptance of [Authorized User Terms](https://stripe.com/docs/issuing/cards).
    pub user_terms_acceptance: Option<stripe_types::IssuingCardholderUserTermsAcceptance>,
}
