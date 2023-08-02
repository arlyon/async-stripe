#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Stores {
    /// FamilyMart instruction details.
    pub familymart: Option<stripe_types::familymart::Familymart>,
    /// Lawson instruction details.
    pub lawson: Option<stripe_types::lawson::Lawson>,
    /// Ministop instruction details.
    pub ministop: Option<stripe_types::ministop::Ministop>,
    /// Seicomart instruction details.
    pub seicomart: Option<stripe_types::seicomart::Seicomart>,
}
