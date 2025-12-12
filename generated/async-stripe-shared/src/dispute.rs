/// A dispute occurs when a customer questions your charge with their card issuer.
/// When this happens, you have the opportunity to respond to the dispute with
/// evidence that shows that the charge is legitimate.
///
/// Related guide: [Disputes and fraud](https://docs.stripe.com/disputes)
///
/// For more details see <<https://stripe.com/docs/api/disputes/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Dispute {
    /// Disputed amount.
    /// Usually the amount of the charge, but it can differ (usually because of currency fluctuation or because only part of the order is disputed).
    pub amount: i64,
    /// List of zero, one, or two balance transactions that show funds withdrawn and reinstated to your Stripe account as a result of this dispute.
    pub balance_transactions: Vec<stripe_shared::BalanceTransaction>,
    /// ID of the charge that's disputed.
    pub charge: stripe_types::Expandable<stripe_shared::Charge>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// List of eligibility types that are included in `enhanced_evidence`.
    pub enhanced_eligibility_types: Vec<DisputeEnhancedEligibilityTypes>,
    pub evidence: stripe_shared::DisputeEvidence,
    pub evidence_details: stripe_shared::DisputeEvidenceDetails,
    /// Unique identifier for the object.
    pub id: stripe_shared::DisputeId,
    /// If true, it's still possible to refund the disputed payment.
    /// After the payment has been fully refunded, no further funds are withdrawn from your Stripe account as a result of this dispute.
    pub is_charge_refundable: bool,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Network-dependent reason code for the dispute.
    pub network_reason_code: Option<String>,
    /// ID of the PaymentIntent that's disputed.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    pub payment_method_details: Option<stripe_shared::DisputePaymentMethodDetails>,
    /// Reason given by cardholder for dispute.
    /// Possible values are `bank_cannot_process`, `check_returned`, `credit_not_processed`, `customer_initiated`, `debit_not_authorized`, `duplicate`, `fraudulent`, `general`, `incorrect_account_details`, `insufficient_funds`, `noncompliant`, `product_not_received`, `product_unacceptable`, `subscription_canceled`, or `unrecognized`.
    /// Learn more about [dispute reasons](https://docs.stripe.com/disputes/categories).
    pub reason: String,
    /// The current status of a dispute.
    /// Possible values include:`warning_needs_response`, `warning_under_review`, `warning_closed`, `needs_response`, `under_review`, `won`, `lost`, or `prevented`.
    pub status: DisputeStatus,
}
#[doc(hidden)]
pub struct DisputeBuilder {
    amount: Option<i64>,
    balance_transactions: Option<Vec<stripe_shared::BalanceTransaction>>,
    charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    enhanced_eligibility_types: Option<Vec<DisputeEnhancedEligibilityTypes>>,
    evidence: Option<stripe_shared::DisputeEvidence>,
    evidence_details: Option<stripe_shared::DisputeEvidenceDetails>,
    id: Option<stripe_shared::DisputeId>,
    is_charge_refundable: Option<bool>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    network_reason_code: Option<Option<String>>,
    payment_intent: Option<Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>>,
    payment_method_details: Option<Option<stripe_shared::DisputePaymentMethodDetails>>,
    reason: Option<String>,
    status: Option<DisputeStatus>,
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

    impl Deserialize for Dispute {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Dispute>,
        builder: DisputeBuilder,
    }

    impl Visitor for Place<Dispute> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: DisputeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for DisputeBuilder {
        type Out = Dispute;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "balance_transactions" => Deserialize::begin(&mut self.balance_transactions),
                "charge" => Deserialize::begin(&mut self.charge),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "enhanced_eligibility_types" => {
                    Deserialize::begin(&mut self.enhanced_eligibility_types)
                }
                "evidence" => Deserialize::begin(&mut self.evidence),
                "evidence_details" => Deserialize::begin(&mut self.evidence_details),
                "id" => Deserialize::begin(&mut self.id),
                "is_charge_refundable" => Deserialize::begin(&mut self.is_charge_refundable),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "network_reason_code" => Deserialize::begin(&mut self.network_reason_code),
                "payment_intent" => Deserialize::begin(&mut self.payment_intent),
                "payment_method_details" => Deserialize::begin(&mut self.payment_method_details),
                "reason" => Deserialize::begin(&mut self.reason),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                balance_transactions: Deserialize::default(),
                charge: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                enhanced_eligibility_types: Deserialize::default(),
                evidence: Deserialize::default(),
                evidence_details: Deserialize::default(),
                id: Deserialize::default(),
                is_charge_refundable: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                network_reason_code: Deserialize::default(),
                payment_intent: Deserialize::default(),
                payment_method_details: Deserialize::default(),
                reason: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(balance_transactions),
                Some(charge),
                Some(created),
                Some(currency),
                Some(enhanced_eligibility_types),
                Some(evidence),
                Some(evidence_details),
                Some(id),
                Some(is_charge_refundable),
                Some(livemode),
                Some(metadata),
                Some(network_reason_code),
                Some(payment_intent),
                Some(payment_method_details),
                Some(reason),
                Some(status),
            ) = (
                self.amount,
                self.balance_transactions.take(),
                self.charge.take(),
                self.created,
                self.currency.take(),
                self.enhanced_eligibility_types.take(),
                self.evidence.take(),
                self.evidence_details.take(),
                self.id.take(),
                self.is_charge_refundable,
                self.livemode,
                self.metadata.take(),
                self.network_reason_code.take(),
                self.payment_intent.take(),
                self.payment_method_details.take(),
                self.reason.take(),
                self.status.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                balance_transactions,
                charge,
                created,
                currency,
                enhanced_eligibility_types,
                evidence,
                evidence_details,
                id,
                is_charge_refundable,
                livemode,
                metadata,
                network_reason_code,
                payment_intent,
                payment_method_details,
                reason,
                status,
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

    impl ObjectDeser for Dispute {
        type Builder = DisputeBuilder;
    }

    impl FromValueOpt for Dispute {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DisputeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "balance_transactions" => b.balance_transactions = FromValueOpt::from_value(v),
                    "charge" => b.charge = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "enhanced_eligibility_types" => {
                        b.enhanced_eligibility_types = FromValueOpt::from_value(v)
                    }
                    "evidence" => b.evidence = FromValueOpt::from_value(v),
                    "evidence_details" => b.evidence_details = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "is_charge_refundable" => b.is_charge_refundable = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "network_reason_code" => b.network_reason_code = FromValueOpt::from_value(v),
                    "payment_intent" => b.payment_intent = FromValueOpt::from_value(v),
                    "payment_method_details" => {
                        b.payment_method_details = FromValueOpt::from_value(v)
                    }
                    "reason" => b.reason = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Dispute {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Dispute", 18)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("balance_transactions", &self.balance_transactions)?;
        s.serialize_field("charge", &self.charge)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("enhanced_eligibility_types", &self.enhanced_eligibility_types)?;
        s.serialize_field("evidence", &self.evidence)?;
        s.serialize_field("evidence_details", &self.evidence_details)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("is_charge_refundable", &self.is_charge_refundable)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("network_reason_code", &self.network_reason_code)?;
        s.serialize_field("payment_intent", &self.payment_intent)?;
        s.serialize_field("payment_method_details", &self.payment_method_details)?;
        s.serialize_field("reason", &self.reason)?;
        s.serialize_field("status", &self.status)?;

        s.serialize_field("object", "dispute")?;
        s.end()
    }
}
/// List of eligibility types that are included in `enhanced_evidence`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum DisputeEnhancedEligibilityTypes {
    VisaCompellingEvidence3,
    VisaCompliance,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl DisputeEnhancedEligibilityTypes {
    pub fn as_str(&self) -> &str {
        use DisputeEnhancedEligibilityTypes::*;
        match self {
            VisaCompellingEvidence3 => "visa_compelling_evidence_3",
            VisaCompliance => "visa_compliance",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for DisputeEnhancedEligibilityTypes {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputeEnhancedEligibilityTypes::*;
        match s {
            "visa_compelling_evidence_3" => Ok(VisaCompellingEvidence3),
            "visa_compliance" => Ok(VisaCompliance),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "DisputeEnhancedEligibilityTypes"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for DisputeEnhancedEligibilityTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DisputeEnhancedEligibilityTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DisputeEnhancedEligibilityTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for DisputeEnhancedEligibilityTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<DisputeEnhancedEligibilityTypes> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DisputeEnhancedEligibilityTypes::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(DisputeEnhancedEligibilityTypes);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DisputeEnhancedEligibilityTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The current status of a dispute.
/// Possible values include:`warning_needs_response`, `warning_under_review`, `warning_closed`, `needs_response`, `under_review`, `won`, `lost`, or `prevented`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum DisputeStatus {
    Lost,
    NeedsResponse,
    Prevented,
    UnderReview,
    WarningClosed,
    WarningNeedsResponse,
    WarningUnderReview,
    Won,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl DisputeStatus {
    pub fn as_str(&self) -> &str {
        use DisputeStatus::*;
        match self {
            Lost => "lost",
            NeedsResponse => "needs_response",
            Prevented => "prevented",
            UnderReview => "under_review",
            WarningClosed => "warning_closed",
            WarningNeedsResponse => "warning_needs_response",
            WarningUnderReview => "warning_under_review",
            Won => "won",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for DisputeStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputeStatus::*;
        match s {
            "lost" => Ok(Lost),
            "needs_response" => Ok(NeedsResponse),
            "prevented" => Ok(Prevented),
            "under_review" => Ok(UnderReview),
            "warning_closed" => Ok(WarningClosed),
            "warning_needs_response" => Ok(WarningNeedsResponse),
            "warning_under_review" => Ok(WarningUnderReview),
            "won" => Ok(Won),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "DisputeStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for DisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DisputeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for DisputeStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<DisputeStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DisputeStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(DisputeStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DisputeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for Dispute {
    type Id = stripe_shared::DisputeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(DisputeId);
