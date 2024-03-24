/// Use [InboundTransfers](https://stripe.com/docs/treasury/moving-money/financial-accounts/into/inbound-transfers) to add funds to your [FinancialAccount](https://stripe.com/docs/api#financial_accounts) via a PaymentMethod that is owned by you.
/// The funds will be transferred via an ACH debit.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryInboundTransfer {
    /// Amount (in cents) transferred.
    pub amount: i64,
    /// Returns `true` if the InboundTransfer is able to be canceled.
    pub cancelable: bool,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Details about this InboundTransfer's failure. Only set when status is `failed`.
    pub failure_details: Option<stripe_treasury::TreasuryInboundTransfersResourceFailureDetails>,
    /// The FinancialAccount that received the funds.
    pub financial_account: String,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryInboundTransferId,
    pub linked_flows: stripe_treasury::TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The origin payment method to be debited for an InboundTransfer.
    pub origin_payment_method: String,
    /// Details about the PaymentMethod for an InboundTransfer.
    pub origin_payment_method_details: Option<stripe_treasury::InboundTransfers>,
    /// Returns `true` if the funds for an InboundTransfer were returned after the InboundTransfer went to the `succeeded` state.
    pub returned: Option<bool>,
    /// Statement descriptor shown when funds are debited from the source.
    /// Not all payment networks support `statement_descriptor`.
    pub statement_descriptor: String,
    /// Status of the InboundTransfer: `processing`, `succeeded`, `failed`, and `canceled`.
    /// An InboundTransfer is `processing` if it is created and pending.
    /// The status changes to `succeeded` once the funds have been "confirmed" and a `transaction` is created and posted.
    /// The status changes to `failed` if the transfer fails.
    pub status: stripe_treasury::TreasuryInboundTransferStatus,
    pub status_transitions: stripe_treasury::TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions,
    /// The Transaction associated with this object.
    pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryInboundTransferBuilder {
    amount: Option<i64>,
    cancelable: Option<bool>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    failure_details: Option<Option<stripe_treasury::TreasuryInboundTransfersResourceFailureDetails>>,
    financial_account: Option<String>,
    hosted_regulatory_receipt_url: Option<Option<String>>,
    id: Option<stripe_treasury::TreasuryInboundTransferId>,
    linked_flows: Option<stripe_treasury::TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    origin_payment_method: Option<String>,
    origin_payment_method_details: Option<Option<stripe_treasury::InboundTransfers>>,
    returned: Option<Option<bool>>,
    statement_descriptor: Option<String>,
    status: Option<stripe_treasury::TreasuryInboundTransferStatus>,
    status_transitions: Option<stripe_treasury::TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions>,
    transaction: Option<Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>>,
}

#[cfg(feature = "min-ser")]
#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryInboundTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryInboundTransfer>,
        builder: TreasuryInboundTransferBuilder,
    }

    impl Visitor for Place<TreasuryInboundTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryInboundTransferBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryInboundTransferBuilder {
        type Out = TreasuryInboundTransfer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "cancelable" => Deserialize::begin(&mut self.cancelable),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "description" => Deserialize::begin(&mut self.description),
                "failure_details" => Deserialize::begin(&mut self.failure_details),
                "financial_account" => Deserialize::begin(&mut self.financial_account),
                "hosted_regulatory_receipt_url" => Deserialize::begin(&mut self.hosted_regulatory_receipt_url),
                "id" => Deserialize::begin(&mut self.id),
                "linked_flows" => Deserialize::begin(&mut self.linked_flows),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "origin_payment_method" => Deserialize::begin(&mut self.origin_payment_method),
                "origin_payment_method_details" => Deserialize::begin(&mut self.origin_payment_method_details),
                "returned" => Deserialize::begin(&mut self.returned),
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),
                "status" => Deserialize::begin(&mut self.status),
                "status_transitions" => Deserialize::begin(&mut self.status_transitions),
                "transaction" => Deserialize::begin(&mut self.transaction),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                cancelable: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                failure_details: Deserialize::default(),
                financial_account: Deserialize::default(),
                hosted_regulatory_receipt_url: Deserialize::default(),
                id: Deserialize::default(),
                linked_flows: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                origin_payment_method: Deserialize::default(),
                origin_payment_method_details: Deserialize::default(),
                returned: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                status: Deserialize::default(),
                status_transitions: Deserialize::default(),
                transaction: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let cancelable = self.cancelable.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let description = self.description.take()?;
            let failure_details = self.failure_details.take()?;
            let financial_account = self.financial_account.take()?;
            let hosted_regulatory_receipt_url = self.hosted_regulatory_receipt_url.take()?;
            let id = self.id.take()?;
            let linked_flows = self.linked_flows.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let origin_payment_method = self.origin_payment_method.take()?;
            let origin_payment_method_details = self.origin_payment_method_details.take()?;
            let returned = self.returned.take()?;
            let statement_descriptor = self.statement_descriptor.take()?;
            let status = self.status.take()?;
            let status_transitions = self.status_transitions.take()?;
            let transaction = self.transaction.take()?;

            Some(Self::Out {
                amount,
                cancelable,
                created,
                currency,
                description,
                failure_details,
                financial_account,
                hosted_regulatory_receipt_url,
                id,
                linked_flows,
                livemode,
                metadata,
                origin_payment_method,
                origin_payment_method_details,
                returned,
                statement_descriptor,
                status,
                status_transitions,
                transaction,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for TreasuryInboundTransfer {
        type Builder = TreasuryInboundTransferBuilder;
    }

    impl FromValueOpt for TreasuryInboundTransfer {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryInboundTransferBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "cancelable" => b.cancelable = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "failure_details" => b.failure_details = Some(FromValueOpt::from_value(v)?),
                    "financial_account" => b.financial_account = Some(FromValueOpt::from_value(v)?),
                    "hosted_regulatory_receipt_url" => b.hosted_regulatory_receipt_url = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "linked_flows" => b.linked_flows = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "origin_payment_method" => b.origin_payment_method = Some(FromValueOpt::from_value(v)?),
                    "origin_payment_method_details" => b.origin_payment_method_details = Some(FromValueOpt::from_value(v)?),
                    "returned" => b.returned = Some(FromValueOpt::from_value(v)?),
                    "statement_descriptor" => b.statement_descriptor = Some(FromValueOpt::from_value(v)?),
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "status_transitions" => b.status_transitions = Some(FromValueOpt::from_value(v)?),
                    "transaction" => b.transaction = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
impl stripe_types::Object for TreasuryInboundTransfer {
    type Id = stripe_treasury::TreasuryInboundTransferId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryInboundTransferId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryInboundTransferStatus {
    Canceled,
    Failed,
    Processing,
    Succeeded,
}
impl TreasuryInboundTransferStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryInboundTransferStatus::*;
        match self {
            Canceled => "canceled",
            Failed => "failed",
            Processing => "processing",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TreasuryInboundTransferStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryInboundTransferStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "failed" => Ok(Failed),
            "processing" => Ok(Processing),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TreasuryInboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryInboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryInboundTransferStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryInboundTransferStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryInboundTransferStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TreasuryInboundTransferStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TreasuryInboundTransferStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryInboundTransferStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

#[cfg(feature = "min-ser")]
stripe_types::impl_from_val_with_from_str!(TreasuryInboundTransferStatus);
