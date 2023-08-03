#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AccountPaymentsSettings {
    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    pub statement_descriptor: Option<String>,
    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    pub statement_descriptor_kana: Option<String>,
    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    pub statement_descriptor_kanji: Option<String>,
    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    ///
    /// This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge.
    /// `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix_kana: Option<String>,
    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    ///
    /// This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge.
    /// `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix_kanji: Option<String>,
}
