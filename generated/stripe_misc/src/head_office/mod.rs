#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HeadOffice {
    pub address: stripe_types::address::Address,
}
