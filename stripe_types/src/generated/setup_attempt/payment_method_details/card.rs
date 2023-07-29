#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Card {
    /// Populated if this authorization used 3D Secure authentication.
    pub three_d_secure:
        Option<stripe_types::charge::payment_method_details::card::three_d_secure::ThreeDSecure>,
}
