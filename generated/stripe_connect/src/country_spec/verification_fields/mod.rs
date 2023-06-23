#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct VerificationFields {
    pub company: stripe_connect::country_spec::verification_fields::details::Details,
    pub individual: stripe_connect::country_spec::verification_fields::details::Details,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for VerificationFields {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod details;
pub use details::Details;
