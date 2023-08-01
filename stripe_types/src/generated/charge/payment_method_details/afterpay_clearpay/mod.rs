#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AfterpayClearpay {
    /// Order identifier shown to the merchant in Afterpay’s online portal.
    pub reference: Option<String>,
}
