#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentNextActionKonbiniStores {
    /// FamilyMart instruction details.
    pub familymart: Option<stripe_shared::PaymentIntentNextActionKonbiniFamilymart>,
    /// Lawson instruction details.
    pub lawson: Option<stripe_shared::PaymentIntentNextActionKonbiniLawson>,
    /// Ministop instruction details.
    pub ministop: Option<stripe_shared::PaymentIntentNextActionKonbiniMinistop>,
    /// Seicomart instruction details.
    pub seicomart: Option<stripe_shared::PaymentIntentNextActionKonbiniSeicomart>,
}
