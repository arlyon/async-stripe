/// As a [card issuer](https://stripe.com/docs/issuing), you can dispute transactions that the cardholder does not recognize, suspects to be fraudulent, or has other issues with.
///
/// Related guide: [Issuing disputes](https://stripe.com/docs/issuing/purchases/disputes)
///
/// For more details see <<https://stripe.com/docs/api/issuing/disputes/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingDispute {
    /// Disputed amount in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// Usually the amount of the `transaction`, but can differ (usually because of currency fluctuation).
    pub amount: i64,
    /// List of balance transactions associated with the dispute.
    pub balance_transactions: Option<Vec<stripe_shared::BalanceTransaction>>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The currency the `transaction` was made in.
    pub currency: stripe_types::Currency,
    pub evidence: stripe_shared::IssuingDisputeEvidence,
    /// Unique identifier for the object.
    pub id: stripe_shared::IssuingDisputeId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The enum that describes the dispute loss outcome.
    /// If the dispute is not lost, this field will be absent.
    /// New enum values may be added in the future, so be sure to handle unknown values.
    pub loss_reason: Option<IssuingDisputeLossReason>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Current status of the dispute.
    pub status: stripe_shared::IssuingDisputeStatus,
    /// The transaction being disputed.
    pub transaction: stripe_types::Expandable<stripe_shared::IssuingTransaction>,
    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this dispute if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts.
    pub treasury: Option<stripe_shared::IssuingDisputeTreasury>,
}
#[doc(hidden)]
pub struct IssuingDisputeBuilder {
    amount: Option<i64>,
    balance_transactions: Option<Option<Vec<stripe_shared::BalanceTransaction>>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    evidence: Option<stripe_shared::IssuingDisputeEvidence>,
    id: Option<stripe_shared::IssuingDisputeId>,
    livemode: Option<bool>,
    loss_reason: Option<Option<IssuingDisputeLossReason>>,
    metadata: Option<std::collections::HashMap<String, String>>,
    status: Option<stripe_shared::IssuingDisputeStatus>,
    transaction: Option<stripe_types::Expandable<stripe_shared::IssuingTransaction>>,
    treasury: Option<Option<stripe_shared::IssuingDisputeTreasury>>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingDispute {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDispute>,
        builder: IssuingDisputeBuilder,
    }

    impl Visitor for Place<IssuingDispute> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingDisputeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingDisputeBuilder {
        type Out = IssuingDispute;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "balance_transactions" => Deserialize::begin(&mut self.balance_transactions),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "evidence" => Deserialize::begin(&mut self.evidence),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "loss_reason" => Deserialize::begin(&mut self.loss_reason),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "status" => Deserialize::begin(&mut self.status),
                "transaction" => Deserialize::begin(&mut self.transaction),
                "treasury" => Deserialize::begin(&mut self.treasury),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                balance_transactions: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                evidence: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                loss_reason: Deserialize::default(),
                metadata: Deserialize::default(),
                status: Deserialize::default(),
                transaction: Deserialize::default(),
                treasury: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(balance_transactions),
                Some(created),
                Some(currency),
                Some(evidence),
                Some(id),
                Some(livemode),
                Some(loss_reason),
                Some(metadata),
                Some(status),
                Some(transaction),
                Some(treasury),
            ) = (
                self.amount,
                self.balance_transactions.take(),
                self.created,
                self.currency.take(),
                self.evidence.take(),
                self.id.take(),
                self.livemode,
                self.loss_reason.take(),
                self.metadata.take(),
                self.status,
                self.transaction.take(),
                self.treasury.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                balance_transactions,
                created,
                currency,
                evidence,
                id,
                livemode,
                loss_reason,
                metadata,
                status,
                transaction,
                treasury,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for IssuingDispute {
        type Builder = IssuingDisputeBuilder;
    }

    impl FromValueOpt for IssuingDispute {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingDisputeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "balance_transactions" => b.balance_transactions = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "evidence" => b.evidence = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "loss_reason" => b.loss_reason = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "transaction" => b.transaction = FromValueOpt::from_value(v),
                    "treasury" => b.treasury = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingDispute {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("IssuingDispute", 13)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("balance_transactions", &self.balance_transactions)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("evidence", &self.evidence)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("loss_reason", &self.loss_reason)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("transaction", &self.transaction)?;
        s.serialize_field("treasury", &self.treasury)?;

        s.serialize_field("object", "issuing.dispute")?;
        s.end()
    }
}
/// The enum that describes the dispute loss outcome.
/// If the dispute is not lost, this field will be absent.
/// New enum values may be added in the future, so be sure to handle unknown values.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingDisputeLossReason {
    CardholderAuthenticationIssuerLiability,
    Eci5TokenTransactionWithTavv,
    ExcessDisputesInTimeframe,
    HasNotMetTheMinimumDisputeAmountRequirements,
    InvalidDuplicateDispute,
    InvalidIncorrectAmountDispute,
    InvalidNoAuthorization,
    InvalidUseOfDisputes,
    MerchandiseDeliveredOrShipped,
    MerchandiseOrServiceAsDescribed,
    NotCancelled,
    Other,
    RefundIssued,
    SubmittedBeyondAllowableTimeLimit,
    Transaction3dsRequired,
    TransactionApprovedAfterPriorFraudDispute,
    TransactionAuthorized,
    TransactionElectronicallyRead,
    TransactionQualifiesForVisaEasyPaymentService,
    TransactionUnattended,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingDisputeLossReason {
    pub fn as_str(&self) -> &str {
        use IssuingDisputeLossReason::*;
        match self {
            CardholderAuthenticationIssuerLiability => "cardholder_authentication_issuer_liability",
            Eci5TokenTransactionWithTavv => "eci5_token_transaction_with_tavv",
            ExcessDisputesInTimeframe => "excess_disputes_in_timeframe",
            HasNotMetTheMinimumDisputeAmountRequirements => {
                "has_not_met_the_minimum_dispute_amount_requirements"
            }
            InvalidDuplicateDispute => "invalid_duplicate_dispute",
            InvalidIncorrectAmountDispute => "invalid_incorrect_amount_dispute",
            InvalidNoAuthorization => "invalid_no_authorization",
            InvalidUseOfDisputes => "invalid_use_of_disputes",
            MerchandiseDeliveredOrShipped => "merchandise_delivered_or_shipped",
            MerchandiseOrServiceAsDescribed => "merchandise_or_service_as_described",
            NotCancelled => "not_cancelled",
            Other => "other",
            RefundIssued => "refund_issued",
            SubmittedBeyondAllowableTimeLimit => "submitted_beyond_allowable_time_limit",
            Transaction3dsRequired => "transaction_3ds_required",
            TransactionApprovedAfterPriorFraudDispute => {
                "transaction_approved_after_prior_fraud_dispute"
            }
            TransactionAuthorized => "transaction_authorized",
            TransactionElectronicallyRead => "transaction_electronically_read",
            TransactionQualifiesForVisaEasyPaymentService => {
                "transaction_qualifies_for_visa_easy_payment_service"
            }
            TransactionUnattended => "transaction_unattended",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingDisputeLossReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingDisputeLossReason::*;
        match s {
            "cardholder_authentication_issuer_liability" => {
                Ok(CardholderAuthenticationIssuerLiability)
            }
            "eci5_token_transaction_with_tavv" => Ok(Eci5TokenTransactionWithTavv),
            "excess_disputes_in_timeframe" => Ok(ExcessDisputesInTimeframe),
            "has_not_met_the_minimum_dispute_amount_requirements" => {
                Ok(HasNotMetTheMinimumDisputeAmountRequirements)
            }
            "invalid_duplicate_dispute" => Ok(InvalidDuplicateDispute),
            "invalid_incorrect_amount_dispute" => Ok(InvalidIncorrectAmountDispute),
            "invalid_no_authorization" => Ok(InvalidNoAuthorization),
            "invalid_use_of_disputes" => Ok(InvalidUseOfDisputes),
            "merchandise_delivered_or_shipped" => Ok(MerchandiseDeliveredOrShipped),
            "merchandise_or_service_as_described" => Ok(MerchandiseOrServiceAsDescribed),
            "not_cancelled" => Ok(NotCancelled),
            "other" => Ok(Other),
            "refund_issued" => Ok(RefundIssued),
            "submitted_beyond_allowable_time_limit" => Ok(SubmittedBeyondAllowableTimeLimit),
            "transaction_3ds_required" => Ok(Transaction3dsRequired),
            "transaction_approved_after_prior_fraud_dispute" => {
                Ok(TransactionApprovedAfterPriorFraudDispute)
            }
            "transaction_authorized" => Ok(TransactionAuthorized),
            "transaction_electronically_read" => Ok(TransactionElectronicallyRead),
            "transaction_qualifies_for_visa_easy_payment_service" => {
                Ok(TransactionQualifiesForVisaEasyPaymentService)
            }
            "transaction_unattended" => Ok(TransactionUnattended),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for IssuingDisputeLossReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingDisputeLossReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingDisputeLossReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingDisputeLossReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingDisputeLossReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingDisputeLossReason::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingDisputeLossReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingDisputeLossReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
impl stripe_types::Object for IssuingDispute {
    type Id = stripe_shared::IssuingDisputeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(IssuingDisputeId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingDisputeStatus {
    Expired,
    Lost,
    Submitted,
    Unsubmitted,
    Won,
}
impl IssuingDisputeStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingDisputeStatus::*;
        match self {
            Expired => "expired",
            Lost => "lost",
            Submitted => "submitted",
            Unsubmitted => "unsubmitted",
            Won => "won",
        }
    }
}

impl std::str::FromStr for IssuingDisputeStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingDisputeStatus::*;
        match s {
            "expired" => Ok(Expired),
            "lost" => Ok(Lost),
            "submitted" => Ok(Submitted),
            "unsubmitted" => Ok(Unsubmitted),
            "won" => Ok(Won),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingDisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingDisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingDisputeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingDisputeStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingDisputeStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingDisputeStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingDisputeStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingDisputeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingDisputeStatus"))
    }
}
