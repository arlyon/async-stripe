/// Iban Records contain E.U.
///
/// bank account details per the SEPA format.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Iban {
    /// The name of the person or business that owns the bank account.
    pub account_holder_name: String,
    /// The BIC/SWIFT code of the account.
    pub bic: String,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// The IBAN of the account.
    pub iban: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Iban {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
