#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Fee {
    /// Amount of the fee, in cents.
    pub amount: i64,
    /// ID of the Connect application that earned the fee.
    pub application: Option<String>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// Type of the fee, one of: `application_fee`, `stripe_fee` or `tax`.
    #[serde(rename = "type")]
    pub type_: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Fee {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
