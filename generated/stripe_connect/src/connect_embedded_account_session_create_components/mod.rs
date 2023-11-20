#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConnectEmbeddedAccountSessionCreateComponents {
    pub account_onboarding: stripe_connect::ConnectEmbeddedBaseConfigClaim,
}
