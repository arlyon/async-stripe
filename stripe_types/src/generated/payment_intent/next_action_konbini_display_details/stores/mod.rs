#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Stores {
    /// FamilyMart instruction details.
pub familymart: Option<stripe_types::payment_intent::next_action_konbini_display_details::stores::familymart::Familymart>,
    /// Lawson instruction details.
pub lawson: Option<stripe_types::payment_intent::next_action_konbini_display_details::stores::lawson::Lawson>,
    /// Ministop instruction details.
pub ministop: Option<stripe_types::payment_intent::next_action_konbini_display_details::stores::ministop::Ministop>,
    /// Seicomart instruction details.
pub seicomart: Option<stripe_types::payment_intent::next_action_konbini_display_details::stores::seicomart::Seicomart>,

}
pub mod familymart;
pub use familymart::Familymart;
pub mod lawson;
pub use lawson::Lawson;
pub mod ministop;
pub use ministop::Ministop;
pub mod seicomart;
pub use seicomart::Seicomart;
