#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct NextActionKonbiniDisplayDetails {
    /// The timestamp at which the pending Konbini payment expires.
    pub expires_at: crate::Timestamp,
    /// The URL for the Konbini payment instructions page, which allows customers to view and print a Konbini voucher.
    pub hosted_voucher_url: Option<String>,
    pub stores: crate::payment_intent::next_action_konbini_display_details::stores::Stores,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for NextActionKonbiniDisplayDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod stores;
pub use stores::Stores;
