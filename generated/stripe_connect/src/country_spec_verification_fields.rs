#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CountrySpecVerificationFields {
    pub company: stripe_connect::CountrySpecVerificationFieldDetails,
    pub individual: stripe_connect::CountrySpecVerificationFieldDetails,
}
