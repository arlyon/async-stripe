#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Paypal {
    /// PayPal account PayerID.
    ///
    /// This identifier uniquely identifies the PayPal customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_id: Option<String>,
}
