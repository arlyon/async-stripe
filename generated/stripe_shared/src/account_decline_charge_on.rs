#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountDeclineChargeOn {
    /// Whether Stripe automatically declines charges with an incorrect ZIP or postal code.
    /// This setting only applies when a ZIP or postal code is provided and they fail bank verification.
    pub avs_failure: bool,
    /// Whether Stripe automatically declines charges with an incorrect CVC.
    /// This setting only applies when a CVC is provided and it fails bank verification.
    pub cvc_failure: bool,
}
