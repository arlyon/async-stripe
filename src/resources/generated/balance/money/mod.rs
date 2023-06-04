#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Money {
    /// Balance amount.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_types: Option<crate::balance::money::source_types::SourceTypes>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Money {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod source_types;
pub use source_types::SourceTypes;
