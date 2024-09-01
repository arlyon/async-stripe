/// Use OutboundTransfers to transfer funds from a [FinancialAccount](https://stripe.com/docs/api#financial_accounts) to a PaymentMethod belonging to the same entity.
/// To send funds to a different party, use [OutboundPayments](https://stripe.com/docs/api#outbound_payments) instead.
/// You can send funds over ACH rails or through a domestic wire transfer to a user's own external bank account.
///
/// Simulate OutboundTransfer state changes with the `/v1/test_helpers/treasury/outbound_transfers` endpoints.
/// These methods can only be called on test mode objects.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryOutboundTransfer {
    /// Amount (in cents) transferred.
    pub amount: i64,
    /// Returns `true` if the object can be canceled, and `false` otherwise.
    pub cancelable: bool,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// The PaymentMethod used as the payment instrument for an OutboundTransfer.
    pub destination_payment_method: Option<String>,
    pub destination_payment_method_details: stripe_treasury::OutboundTransfersPaymentMethodDetails,
    /// The date when funds are expected to arrive in the destination account.
    pub expected_arrival_date: stripe_types::Timestamp,
    /// The FinancialAccount that funds were pulled from.
    pub financial_account: String,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryOutboundTransferId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Details about a returned OutboundTransfer. Only set when the status is `returned`.
    pub returned_details: Option<stripe_treasury::TreasuryOutboundTransfersResourceReturnedDetails>,
    /// Information about the OutboundTransfer to be sent to the recipient account.
    pub statement_descriptor: String,
    /// Current status of the OutboundTransfer: `processing`, `failed`, `canceled`, `posted`, `returned`.
    /// An OutboundTransfer is `processing` if it has been created and is pending.
    /// The status changes to `posted` once the OutboundTransfer has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
    /// If an OutboundTransfer fails to arrive at its destination, its status will change to `returned`.
    pub status: stripe_treasury::TreasuryOutboundTransferStatus,
    pub status_transitions: stripe_treasury::TreasuryOutboundTransfersResourceStatusTransitions,
    /// The Transaction associated with this object.
    pub transaction: stripe_types::Expandable<stripe_treasury::TreasuryTransaction>,
}
#[doc(hidden)]
pub struct TreasuryOutboundTransferBuilder {
    amount: Option<i64>,
    cancelable: Option<bool>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    destination_payment_method: Option<Option<String>>,
    destination_payment_method_details:
        Option<stripe_treasury::OutboundTransfersPaymentMethodDetails>,
    expected_arrival_date: Option<stripe_types::Timestamp>,
    financial_account: Option<String>,
    hosted_regulatory_receipt_url: Option<Option<String>>,
    id: Option<stripe_treasury::TreasuryOutboundTransferId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    returned_details:
        Option<Option<stripe_treasury::TreasuryOutboundTransfersResourceReturnedDetails>>,
    statement_descriptor: Option<String>,
    status: Option<stripe_treasury::TreasuryOutboundTransferStatus>,
    status_transitions: Option<stripe_treasury::TreasuryOutboundTransfersResourceStatusTransitions>,
    transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryOutboundTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryOutboundTransfer>,
        builder: TreasuryOutboundTransferBuilder,
    }

    impl Visitor for Place<TreasuryOutboundTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryOutboundTransferBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryOutboundTransferBuilder {
        type Out = TreasuryOutboundTransfer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "cancelable" => Deserialize::begin(&mut self.cancelable),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "description" => Deserialize::begin(&mut self.description),
                "destination_payment_method" => {
                    Deserialize::begin(&mut self.destination_payment_method)
                }
                "destination_payment_method_details" => {
                    Deserialize::begin(&mut self.destination_payment_method_details)
                }
                "expected_arrival_date" => Deserialize::begin(&mut self.expected_arrival_date),
                "financial_account" => Deserialize::begin(&mut self.financial_account),
                "hosted_regulatory_receipt_url" => {
                    Deserialize::begin(&mut self.hosted_regulatory_receipt_url)
                }
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "returned_details" => Deserialize::begin(&mut self.returned_details),
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
                destination_payment_method: Deserialize::default(),
                destination_payment_method_details: Deserialize::default(),
                expected_arrival_date: Deserialize::default(),
                financial_account: Deserialize::default(),
                hosted_regulatory_receipt_url: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                returned_details: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                status: Deserialize::default(),
                status_transitions: Deserialize::default(),
                transaction: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(cancelable),
                Some(created),
                Some(currency),
                Some(description),
                Some(destination_payment_method),
                Some(destination_payment_method_details),
                Some(expected_arrival_date),
                Some(financial_account),
                Some(hosted_regulatory_receipt_url),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(returned_details),
                Some(statement_descriptor),
                Some(status),
                Some(status_transitions),
                Some(transaction),
            ) = (
                self.amount,
                self.cancelable,
                self.created,
                self.currency,
                self.description.take(),
                self.destination_payment_method.take(),
                self.destination_payment_method_details.take(),
                self.expected_arrival_date,
                self.financial_account.take(),
                self.hosted_regulatory_receipt_url.take(),
                self.id.take(),
                self.livemode,
                self.metadata.take(),
                self.returned_details.take(),
                self.statement_descriptor.take(),
                self.status,
                self.status_transitions,
                self.transaction.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                cancelable,
                created,
                currency,
                description,
                destination_payment_method,
                destination_payment_method_details,
                expected_arrival_date,
                financial_account,
                hosted_regulatory_receipt_url,
                id,
                livemode,
                metadata,
                returned_details,
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

    impl ObjectDeser for TreasuryOutboundTransfer {
        type Builder = TreasuryOutboundTransferBuilder;
    }

    impl FromValueOpt for TreasuryOutboundTransfer {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryOutboundTransferBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "cancelable" => b.cancelable = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "destination_payment_method" => {
                        b.destination_payment_method = FromValueOpt::from_value(v)
                    }
                    "destination_payment_method_details" => {
                        b.destination_payment_method_details = FromValueOpt::from_value(v)
                    }
                    "expected_arrival_date" => {
                        b.expected_arrival_date = FromValueOpt::from_value(v)
                    }
                    "financial_account" => b.financial_account = FromValueOpt::from_value(v),
                    "hosted_regulatory_receipt_url" => {
                        b.hosted_regulatory_receipt_url = FromValueOpt::from_value(v)
                    }
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "returned_details" => b.returned_details = FromValueOpt::from_value(v),
                    "statement_descriptor" => b.statement_descriptor = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "status_transitions" => b.status_transitions = FromValueOpt::from_value(v),
                    "transaction" => b.transaction = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryOutboundTransfer {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TreasuryOutboundTransfer", 19)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("cancelable", &self.cancelable)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("destination_payment_method", &self.destination_payment_method)?;
        s.serialize_field(
            "destination_payment_method_details",
            &self.destination_payment_method_details,
        )?;
        s.serialize_field("expected_arrival_date", &self.expected_arrival_date)?;
        s.serialize_field("financial_account", &self.financial_account)?;
        s.serialize_field("hosted_regulatory_receipt_url", &self.hosted_regulatory_receipt_url)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("returned_details", &self.returned_details)?;
        s.serialize_field("statement_descriptor", &self.statement_descriptor)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("status_transitions", &self.status_transitions)?;
        s.serialize_field("transaction", &self.transaction)?;

        s.serialize_field("object", "treasury.outbound_transfer")?;
        s.end()
    }
}
impl stripe_types::Object for TreasuryOutboundTransfer {
    type Id = stripe_treasury::TreasuryOutboundTransferId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TreasuryOutboundTransferId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryOutboundTransferStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}
impl TreasuryOutboundTransferStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryOutboundTransferStatus::*;
        match self {
            Canceled => "canceled",
            Failed => "failed",
            Posted => "posted",
            Processing => "processing",
            Returned => "returned",
        }
    }
}

impl std::str::FromStr for TreasuryOutboundTransferStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryOutboundTransferStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "failed" => Ok(Failed),
            "posted" => Ok(Posted),
            "processing" => Ok(Processing),
            "returned" => Ok(Returned),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TreasuryOutboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryOutboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryOutboundTransferStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryOutboundTransferStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TreasuryOutboundTransferStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryOutboundTransferStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TreasuryOutboundTransferStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryOutboundTransferStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TreasuryOutboundTransferStatus")
        })
    }
}
