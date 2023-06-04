#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Stores {
    /// FamilyMart instruction details.
    pub familymart: Option<
        crate::payment_intent::next_action_konbini_display_details::stores::familymart::Familymart,
    >,
    /// Lawson instruction details.
    pub lawson:
        Option<crate::payment_intent::next_action_konbini_display_details::stores::lawson::Lawson>,
    /// Ministop instruction details.
    pub ministop: Option<
        crate::payment_intent::next_action_konbini_display_details::stores::ministop::Ministop,
    >,
    /// Seicomart instruction details.
    pub seicomart: Option<
        crate::payment_intent::next_action_konbini_display_details::stores::seicomart::Seicomart,
    >,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Stores {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod familymart;
pub use familymart::Familymart;
pub mod lawson;
pub use lawson::Lawson;
pub mod ministop;
pub use ministop::Ministop;
pub mod seicomart;
pub use seicomart::Seicomart;
