#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Familymart {
    /// The confirmation number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<String>,
    /// The payment code.
    pub payment_code: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Familymart {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
