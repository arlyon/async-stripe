#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Konbini {
    /// If the payment succeeded, this contains the details of the convenience store where the payment was completed.
    pub store: Option<stripe_types::charge::payment_method_details::store::Store>,
}
