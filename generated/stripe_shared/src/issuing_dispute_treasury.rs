#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingDisputeTreasury {
    /// The Treasury [DebitReversal](https://stripe.com/docs/api/treasury/debit_reversals) representing this Issuing dispute.
    pub debit_reversal: Option<String>,
    /// The Treasury [ReceivedDebit](https://stripe.com/docs/api/treasury/received_debits) that is being disputed.
    pub received_debit: String,
}
