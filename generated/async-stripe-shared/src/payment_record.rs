/// A Payment Record is a resource that allows you to represent payments that occur on- or off-Stripe.
/// For example, you can create a Payment Record to model a payment made on a different payment processor,.
/// in order to mark an Invoice as paid and a Subscription as active. Payment Records consist of one or
/// more Payment Attempt Records, which represent individual attempts made on a payment network.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentRecord {
    pub amount: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount,
    pub amount_authorized: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount,
    pub amount_canceled: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount,
    pub amount_failed: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount,
    pub amount_guaranteed: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount,
    pub amount_refunded: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount,
    pub amount_requested: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceAmount,
    /// ID of the Connect application that created the PaymentRecord.
    pub application: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Customer information for this payment.
    pub customer_details:
        Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceCustomerDetails>,
    /// Indicates whether the customer was present in your checkout flow during this payment.
    pub customer_presence: Option<stripe_shared::PaymentRecordCustomerPresence>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::PaymentRecordId,
    /// ID of the latest Payment Attempt Record attached to this Payment Record.
    pub latest_payment_attempt_record: Option<String>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Information about the Payment Method debited for this payment.
    pub payment_method_details:
        Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails>,
    pub processor_details: stripe_shared::PaymentsPrimitivesPaymentRecordsResourceProcessorDetails,
    /// Indicates who reported the payment.
    pub reported_by: PaymentRecordReportedBy,
    /// Shipping information for this payment.
    pub shipping_details:
        Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceShippingDetails>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentRecord").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentRecordBuilder {
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
    customer_presence: Option<Option<stripe_shared::PaymentRecordCustomerPresence>>,
    description: Option<Option<String>>,
    id: Option<stripe_shared::PaymentRecordId>,
    latest_payment_attempt_record: Option<Option<String>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    payment_method_details:
        Option<Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails>>,
    processor_details:
        Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceProcessorDetails>,
    reported_by: Option<PaymentRecordReportedBy>,
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

    impl Deserialize for PaymentRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentRecord>,
        builder: PaymentRecordBuilder,
    }

    impl Visitor for Place<PaymentRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentRecordBuilder {
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
                    latest_payment_attempt_record: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    payment_method_details: Deserialize::default(),
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
                "latest_payment_attempt_record" => {
                    Deserialize::begin(&mut self.builder.latest_payment_attempt_record)
                }
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "payment_method_details" => {
                    Deserialize::begin(&mut self.builder.payment_method_details)
                }
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
                Some(latest_payment_attempt_record),
                Some(livemode),
                Some(metadata),
                Some(payment_method_details),
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
                self.builder.latest_payment_attempt_record.take(),
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.payment_method_details.take(),
                self.builder.processor_details.take(),
                self.builder.reported_by.take(),
                self.builder.shipping_details.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentRecord {
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
                latest_payment_attempt_record,
                livemode,
                metadata,
                payment_method_details,
                processor_details,
                reported_by,
                shipping_details,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentRecord {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("PaymentRecord", 21)?;
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
        s.serialize_field("latest_payment_attempt_record", &self.latest_payment_attempt_record)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("payment_method_details", &self.payment_method_details)?;
        s.serialize_field("processor_details", &self.processor_details)?;
        s.serialize_field("reported_by", &self.reported_by)?;
        s.serialize_field("shipping_details", &self.shipping_details)?;

        s.serialize_field("object", "payment_record")?;
        s.end()
    }
}
/// Indicates who reported the payment.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentRecordReportedBy {
    Self_,
    Stripe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentRecordReportedBy {
    pub fn as_str(&self) -> &str {
        use PaymentRecordReportedBy::*;
        match self {
            Self_ => "self",
            Stripe => "stripe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentRecordReportedBy {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentRecordReportedBy::*;
        match s {
            "self" => Ok(Self_),
            "stripe" => Ok(Stripe),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PaymentRecordReportedBy");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentRecordReportedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentRecordReportedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentRecordReportedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentRecordReportedBy)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentRecordReportedBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentRecordReportedBy {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentRecordReportedBy> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentRecordReportedBy::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentRecordReportedBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for PaymentRecord {
    type Id = stripe_shared::PaymentRecordId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(PaymentRecordId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentRecordCustomerPresence {
    OffSession,
    OnSession,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentRecordCustomerPresence {
    pub fn as_str(&self) -> &str {
        use PaymentRecordCustomerPresence::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentRecordCustomerPresence {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentRecordCustomerPresence::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentRecordCustomerPresence"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentRecordCustomerPresence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentRecordCustomerPresence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentRecordCustomerPresence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentRecordCustomerPresence)).finish_non_exhaustive()
    }
}
impl serde::Serialize for PaymentRecordCustomerPresence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentRecordCustomerPresence {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentRecordCustomerPresence> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentRecordCustomerPresence::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentRecordCustomerPresence {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
