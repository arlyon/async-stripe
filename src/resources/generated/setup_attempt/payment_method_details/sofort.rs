#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Sofort {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit: Option<crate::Expandable<crate::payment_method::PaymentMethod>>,
    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit_mandate: Option<crate::Expandable<crate::mandate::Mandate>>,
    /// Last four characters of the IBAN.
    pub iban_last4: Option<String>,
    /// Preferred language of the Sofort authorization page that the customer is redirected to.
    /// Can be one of `en`, `de`, `fr`, or `nl`.
    pub preferred_language: Option<SofortPreferredLanguage>,
    /// Owner's verified full name.
    ///
    /// Values are verified or provided by Sofort directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_name: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Sofort {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Preferred language of the Sofort authorization page that the customer is redirected to.
/// Can be one of `en`, `de`, `fr`, or `nl`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SofortPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl SofortPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Fr => "fr",
            Self::Nl => "nl",
        }
    }
}

impl AsRef<str> for SofortPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}