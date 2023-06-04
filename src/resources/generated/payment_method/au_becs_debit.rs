#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AuBecsDebit {
    /// Six-digit number identifying bank and branch associated with this bank account.
    pub bsb_number: Option<String>,
    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AuBecsDebit {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
