#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MandatePaypal {
    /// The PayPal Billing Agreement ID (BAID).
    /// This is an ID generated by PayPal which represents the mandate between the merchant and the customer.
    pub billing_agreement_id: Option<String>,
    /// PayPal account PayerID. This identifier uniquely identifies the PayPal customer.
    pub payer_id: Option<String>,
}
