#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct StatusDetails {
    /// Details related to the closure of this FinancialAccount.
    pub closed: Option<crate::treasury::financial_account::status_details::closed::Closed>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for StatusDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod closed;
pub use closed::Closed;
