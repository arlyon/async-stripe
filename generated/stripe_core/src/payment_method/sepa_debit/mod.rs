#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SepaDebit {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    /// Branch code of bank associated with the bank account.
    pub branch_code: Option<String>,
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Information about the object that generated this PaymentMethod.
    pub generated_from:
        Option<stripe_core::payment_method::sepa_debit::generated_from::GeneratedFrom>,
    /// Last four characters of the IBAN.
    pub last4: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SepaDebit {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod generated_from;
pub use generated_from::GeneratedFrom;
