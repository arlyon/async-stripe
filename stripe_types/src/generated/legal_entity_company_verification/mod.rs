#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LegalEntityCompanyVerification {
    pub document: stripe_types::LegalEntityCompanyVerificationDocument,
}
