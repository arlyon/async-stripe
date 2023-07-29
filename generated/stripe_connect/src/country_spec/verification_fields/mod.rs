#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VerificationFields {
    pub company: stripe_connect::country_spec::verification_fields::details::Details,
    pub individual: stripe_connect::country_spec::verification_fields::details::Details,
}
pub mod details;
pub use details::Details;
