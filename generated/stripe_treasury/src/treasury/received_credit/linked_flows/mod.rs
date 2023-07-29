#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LinkedFlows {
    /// The CreditReversal created as a result of this ReceivedCredit being reversed.
pub credit_reversal: Option<String>,
    /// Set if the ReceivedCredit was created due to an [Issuing Authorization](https://stripe.com/docs/api#issuing_authorizations) object.
pub issuing_authorization: Option<String>,
    /// Set if the ReceivedCredit is also viewable as an [Issuing transaction](https://stripe.com/docs/api#issuing_transactions) object.
pub issuing_transaction: Option<String>,
    /// ID of the source flow.
    ///
    /// Set if `network` is `stripe` and the source flow is visible to the user.
    /// Examples of source flows include OutboundPayments, payouts, or CreditReversals.
pub source_flow: Option<String>,
    /// The expandable object of the source flow.
pub source_flow_details: Option<stripe_treasury::treasury::received_credit::linked_flows::source_flow_details::SourceFlowDetails>,
    /// The type of flow that originated the ReceivedCredit (for example, `outbound_payment`).
pub source_flow_type: Option<String>,

}
pub mod source_flow_details;
pub use source_flow_details::SourceFlowDetails;
