#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Treasury {
    /// The array of [ReceivedCredits](https://stripe.com/docs/api/treasury/received_credits) associated with this authorization.
    pub received_credits: Vec<String>,
    /// The array of [ReceivedDebits](https://stripe.com/docs/api/treasury/received_debits) associated with this authorization.
    pub received_debits: Vec<String>,
    /// The Treasury [Transaction](https://stripe.com/docs/api/treasury/transactions) associated with this authorization.
    pub transaction: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Treasury {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
