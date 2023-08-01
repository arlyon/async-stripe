#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CardIssuing {
    /// Information about cardholder acceptance of [Authorized User Terms](https://stripe.com/docs/issuing/cards).
pub user_terms_acceptance: Option<stripe_types::issuing::cardholder::individual::card_issuing::user_terms_acceptance::UserTermsAcceptance>,

}
pub mod user_terms_acceptance;
pub use user_terms_acceptance::UserTermsAcceptance;
