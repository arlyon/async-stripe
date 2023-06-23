#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CashBalance {
    /// The funds available to the account holder.
    ///
    /// Typically this is the current balance less any holds.  Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.  Each value is a integer amount.
    /// A positive amount indicates money owed to the account holder.
    /// A negative amount indicates money owed by the account holder.
    pub available: Option<i64>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CashBalance {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
