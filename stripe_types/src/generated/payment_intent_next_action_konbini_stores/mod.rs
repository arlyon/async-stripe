#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentNextActionKonbiniStores {
    /// FamilyMart instruction details.
    pub familymart: Option<stripe_types::PaymentIntentNextActionKonbiniFamilymart>,
    /// Lawson instruction details.
    pub lawson: Option<stripe_types::PaymentIntentNextActionKonbiniLawson>,
    /// Ministop instruction details.
    pub ministop: Option<stripe_types::PaymentIntentNextActionKonbiniMinistop>,
    /// Seicomart instruction details.
    pub seicomart: Option<stripe_types::PaymentIntentNextActionKonbiniSeicomart>,
}
