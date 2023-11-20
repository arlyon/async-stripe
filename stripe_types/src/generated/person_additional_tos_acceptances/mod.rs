#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PersonAdditionalTosAcceptances {
    pub account: stripe_types::PersonAdditionalTosAcceptance,
}
