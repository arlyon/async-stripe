#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Alipay {
    /// Uniquely identifies this particular Alipay account.
    ///
    /// You can use this attribute to check whether two Alipay accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
    /// Uniquely identifies this particular Alipay account.
    ///
    /// You can use this attribute to check whether two Alipay accounts are the same.
    pub fingerprint: Option<String>,
    /// Transaction ID of this particular Alipay transaction.
    pub transaction_id: Option<String>,
}
