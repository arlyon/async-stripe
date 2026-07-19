/// A Payment Attempt Record represents an individual attempt at making a payment, on or off Stripe.
/// Each payment attempt tries to collect a fixed amount of money from a fixed customer and payment
/// method.
/// Payment Attempt Records are attached to Payment Records.
/// Only one attempt per Payment Record.
/// can have guaranteed funds.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentAttemptRecord {
    pub amount: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount,
    pub amount_authorized: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount,
    pub amount_canceled: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount,
    pub amount_failed: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount,
    pub amount_guaranteed: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount,
    pub amount_refunded: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount,
    pub amount_requested: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount,
    /// ID of the Connect application that created the PaymentAttemptRecord.
    pub application: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Customer information for this payment.
    pub customer_details:
        Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceCustomerDetails>,
    /// Indicates whether the customer was present in your checkout flow during this payment.
    pub customer_presence: Option<PaymentAttemptRecordCustomerPresence>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_payment::PaymentAttemptRecordId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Information about the Payment Method debited for this payment.
    pub payment_method_details:
        Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails>,
    /// ID of the Payment Record this Payment Attempt Record belongs to.
    pub payment_record: Option<String>,
    pub processor_details: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceProcessorDetails,
    /// Indicates who reported the payment.
    pub reported_by: PaymentAttemptRecordReportedBy,
    /// Shipping information for this payment.
    pub shipping_details:
        Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceShippingDetails>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentAttemptRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentAttemptRecord").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentAttemptRecordBuilder {
    amount: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    amount_authorized: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    amount_canceled: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    amount_failed: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    amount_guaranteed: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    amount_refunded: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    amount_requested: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    application: Option<Option<String>>,
    created: Option<stripe_types::Timestamp>,
    customer_details:
        Option<Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceCustomerDetails>>,
    customer_presence: Option<Option<PaymentAttemptRecordCustomerPresence>>,
    description: Option<Option<String>>,
    id: Option<stripe_payment::PaymentAttemptRecordId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    payment_method_details:
        Option<Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails>>,
    payment_record: Option<Option<String>>,
    processor_details:
        Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceProcessorDetails>,
    reported_by: Option<PaymentAttemptRecordReportedBy>,
    shipping_details:
        Option<Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceShippingDetails>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for PaymentAttemptRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentAttemptRecord>,
        builder: PaymentAttemptRecordBuilder,
    }

    impl Visitor for Place<PaymentAttemptRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentAttemptRecordBuilder {
                    amount: Deserialize::default(),
                    amount_authorized: Deserialize::default(),
                    amount_canceled: Deserialize::default(),
                    amount_failed: Deserialize::default(),
                    amount_guaranteed: Deserialize::default(),
                    amount_refunded: Deserialize::default(),
                    amount_requested: Deserialize::default(),
                    application: Deserialize::default(),
                    created: Deserialize::default(),
                    customer_details: Deserialize::default(),
                    customer_presence: Deserialize::default(),
                    description: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    payment_method_details: Deserialize::default(),
                    payment_record: Deserialize::default(),
                    processor_details: Deserialize::default(),
                    reported_by: Deserialize::default(),
                    shipping_details: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "amount_authorized" => Deserialize::begin(&mut self.builder.amount_authorized),
                "amount_canceled" => Deserialize::begin(&mut self.builder.amount_canceled),
                "amount_failed" => Deserialize::begin(&mut self.builder.amount_failed),
                "amount_guaranteed" => Deserialize::begin(&mut self.builder.amount_guaranteed),
                "amount_refunded" => Deserialize::begin(&mut self.builder.amount_refunded),
                "amount_requested" => Deserialize::begin(&mut self.builder.amount_requested),
                "application" => Deserialize::begin(&mut self.builder.application),
                "created" => Deserialize::begin(&mut self.builder.created),
                "customer_details" => Deserialize::begin(&mut self.builder.customer_details),
                "customer_presence" => Deserialize::begin(&mut self.builder.customer_presence),
                "description" => Deserialize::begin(&mut self.builder.description),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "payment_method_details" => {
                    Deserialize::begin(&mut self.builder.payment_method_details)
                }
                "payment_record" => Deserialize::begin(&mut self.builder.payment_record),
                "processor_details" => Deserialize::begin(&mut self.builder.processor_details),
                "reported_by" => Deserialize::begin(&mut self.builder.reported_by),
                "shipping_details" => Deserialize::begin(&mut self.builder.shipping_details),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(amount_authorized),
                Some(amount_canceled),
                Some(amount_failed),
                Some(amount_guaranteed),
                Some(amount_refunded),
                Some(amount_requested),
                Some(application),
                Some(created),
                Some(customer_details),
                Some(customer_presence),
                Some(description),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(payment_method_details),
                Some(payment_record),
                Some(processor_details),
                Some(reported_by),
                Some(shipping_details),
            ) = (
                self.builder.amount.take(),
                self.builder.amount_authorized.take(),
                self.builder.amount_canceled.take(),
                self.builder.amount_failed.take(),
                self.builder.amount_guaranteed.take(),
                self.builder.amount_refunded.take(),
                self.builder.amount_requested.take(),
                self.builder.application.take(),
                self.builder.created,
                self.builder.customer_details.take(),
                self.builder.customer_presence.take(),
                self.builder.description.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.payment_method_details.take(),
                self.builder.payment_record.take(),
                self.builder.processor_details.take(),
                self.builder.reported_by.take(),
                self.builder.shipping_details.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentAttemptRecord {
                amount,
                amount_authorized,
                amount_canceled,
                amount_failed,
                amount_guaranteed,
                amount_refunded,
                amount_requested,
                application,
                created,
                customer_details,
                customer_presence,
                description,
                id,
                livemode,
                metadata,
                payment_method_details,
                payment_record,
                processor_details,
                reported_by,
                shipping_details,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentAttemptRecord {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("PaymentAttemptRecord", 21)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("amount_authorized", &self.amount_authorized)?;
        s.serialize_field("amount_canceled", &self.amount_canceled)?;
        s.serialize_field("amount_failed", &self.amount_failed)?;
        s.serialize_field("amount_guaranteed", &self.amount_guaranteed)?;
        s.serialize_field("amount_refunded", &self.amount_refunded)?;
        s.serialize_field("amount_requested", &self.amount_requested)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("customer_details", &self.customer_details)?;
        s.serialize_field("customer_presence", &self.customer_presence)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("payment_method_details", &self.payment_method_details)?;
        s.serialize_field("payment_record", &self.payment_record)?;
        s.serialize_field("processor_details", &self.processor_details)?;
        s.serialize_field("reported_by", &self.reported_by)?;
        s.serialize_field("shipping_details", &self.shipping_details)?;

        s.serialize_field("object", "payment_attempt_record")?;
        s.end()
    }
}
/// Indicates whether the customer was present in your checkout flow during this payment.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentAttemptRecordCustomerPresence {
    OffSession,
    OnSession,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentAttemptRecordCustomerPresence {
    pub fn as_str(&self) -> &str {
        use PaymentAttemptRecordCustomerPresence::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentAttemptRecordCustomerPresence {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentAttemptRecordCustomerPresence::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentAttemptRecordCustomerPresence"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentAttemptRecordCustomerPresence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentAttemptRecordCustomerPresence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentAttemptRecordCustomerPresence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentAttemptRecordCustomerPresence)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentAttemptRecordCustomerPresence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentAttemptRecordCustomerPresence {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentAttemptRecordCustomerPresence> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentAttemptRecordCustomerPresence::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentAttemptRecordCustomerPresence {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Indicates who reported the payment.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentAttemptRecordReportedBy {
    Self_,
    Stripe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentAttemptRecordReportedBy {
    pub fn as_str(&self) -> &str {
        use PaymentAttemptRecordReportedBy::*;
        match self {
            Self_ => "self",
            Stripe => "stripe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentAttemptRecordReportedBy {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentAttemptRecordReportedBy::*;
        match s {
            "self" => Ok(Self_),
            "stripe" => Ok(Stripe),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentAttemptRecordReportedBy"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentAttemptRecordReportedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentAttemptRecordReportedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentAttemptRecordReportedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentAttemptRecordReportedBy)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentAttemptRecordReportedBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentAttemptRecordReportedBy {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentAttemptRecordReportedBy> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentAttemptRecordReportedBy::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentAttemptRecordReportedBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for PaymentAttemptRecord {
    type Id = stripe_payment::PaymentAttemptRecordId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(PaymentAttemptRecordId);
