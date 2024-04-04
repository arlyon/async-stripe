#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConnectEmbeddedAccountSessionCreateComponents {
    pub account_onboarding: stripe_connect::ConnectEmbeddedBaseConfigClaim,
    pub payment_details: stripe_connect::ConnectEmbeddedPaymentsConfig,
    pub payments: stripe_connect::ConnectEmbeddedPaymentsConfig,
    pub payouts: stripe_connect::ConnectEmbeddedPayoutsConfig,
}
