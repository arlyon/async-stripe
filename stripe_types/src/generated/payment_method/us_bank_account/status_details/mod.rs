#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StatusDetails {
#[serde(skip_serializing_if = "Option::is_none")]
pub blocked: Option<stripe_types::payment_method::us_bank_account::status_details::status_details_blocked::StatusDetailsBlocked>,

}
pub mod status_details_blocked;
pub use status_details_blocked::StatusDetailsBlocked;
