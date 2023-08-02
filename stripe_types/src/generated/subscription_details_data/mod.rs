#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionDetailsData {
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will reflect the metadata of the subscription at the time of invoice creation.
    ///
    /// *Note: This attribute is populated only for invoices created on or after June 29, 2023.*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}
