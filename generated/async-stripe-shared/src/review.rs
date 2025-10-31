/// Reviews can be used to supplement automated fraud detection with human expertise.
///
/// Learn more about [Radar](/radar) and reviewing payments
/// [here](https://stripe.com/docs/radar/reviews).
///
/// For more details see <<https://stripe.com/docs/api/radar/reviews/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Review {
    /// The ZIP or postal code of the card used, if applicable.
    pub billing_zip: Option<String>,
    /// The charge associated with this review.
    pub charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// The reason the review was closed, or null if it has not yet been closed.
    /// One of `approved`, `refunded`, `refunded_as_fraud`, `disputed`, `redacted`, `canceled`, `payment_never_settled`, or `acknowledged`.
    pub closed_reason: Option<ReviewClosedReason>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_shared::ReviewId,
    /// The IP address where the payment originated.
    pub ip_address: Option<String>,
    /// Information related to the location of the payment.
    /// Note that this information is an approximation and attempts to locate the nearest population center - it should not be used to determine a specific address.
    pub ip_address_location: Option<stripe_shared::RadarReviewResourceLocation>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// If `true`, the review needs action.
    pub open: bool,
    /// The reason the review was opened. One of `rule` or `manual`.
    pub opened_reason: ReviewOpenedReason,
    /// The PaymentIntent ID associated with this review, if one exists.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    /// The reason the review is currently open or closed.
    /// One of `rule`, `manual`, `approved`, `refunded`, `refunded_as_fraud`, `disputed`, `redacted`, `canceled`, `payment_never_settled`, or `acknowledged`.
    pub reason: String,
    /// Information related to the browsing session of the user who initiated the payment.
    pub session: Option<stripe_shared::RadarReviewResourceSession>,
}
#[doc(hidden)]
pub struct ReviewBuilder {
    billing_zip: Option<Option<String>>,
    charge: Option<Option<stripe_types::Expandable<stripe_shared::Charge>>>,
    closed_reason: Option<Option<ReviewClosedReason>>,
    created: Option<stripe_types::Timestamp>,
    id: Option<stripe_shared::ReviewId>,
    ip_address: Option<Option<String>>,
    ip_address_location: Option<Option<stripe_shared::RadarReviewResourceLocation>>,
    livemode: Option<bool>,
    open: Option<bool>,
    opened_reason: Option<ReviewOpenedReason>,
    payment_intent: Option<Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>>,
    reason: Option<String>,
    session: Option<Option<stripe_shared::RadarReviewResourceSession>>,
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

    impl Deserialize for Review {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Review>,
        builder: ReviewBuilder,
    }

    impl Visitor for Place<Review> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ReviewBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ReviewBuilder {
        type Out = Review;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_zip" => Deserialize::begin(&mut self.billing_zip),
                "charge" => Deserialize::begin(&mut self.charge),
                "closed_reason" => Deserialize::begin(&mut self.closed_reason),
                "created" => Deserialize::begin(&mut self.created),
                "id" => Deserialize::begin(&mut self.id),
                "ip_address" => Deserialize::begin(&mut self.ip_address),
                "ip_address_location" => Deserialize::begin(&mut self.ip_address_location),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "open" => Deserialize::begin(&mut self.open),
                "opened_reason" => Deserialize::begin(&mut self.opened_reason),
                "payment_intent" => Deserialize::begin(&mut self.payment_intent),
                "reason" => Deserialize::begin(&mut self.reason),
                "session" => Deserialize::begin(&mut self.session),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                billing_zip: Deserialize::default(),
                charge: Deserialize::default(),
                closed_reason: Deserialize::default(),
                created: Deserialize::default(),
                id: Deserialize::default(),
                ip_address: Deserialize::default(),
                ip_address_location: Deserialize::default(),
                livemode: Deserialize::default(),
                open: Deserialize::default(),
                opened_reason: Deserialize::default(),
                payment_intent: Deserialize::default(),
                reason: Deserialize::default(),
                session: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(billing_zip),
                Some(charge),
                Some(closed_reason),
                Some(created),
                Some(id),
                Some(ip_address),
                Some(ip_address_location),
                Some(livemode),
                Some(open),
                Some(opened_reason),
                Some(payment_intent),
                Some(reason),
                Some(session),
            ) = (
                self.billing_zip.take(),
                self.charge.take(),
                self.closed_reason,
                self.created,
                self.id.take(),
                self.ip_address.take(),
                self.ip_address_location.take(),
                self.livemode,
                self.open,
                self.opened_reason,
                self.payment_intent.take(),
                self.reason.take(),
                self.session.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                billing_zip,
                charge,
                closed_reason,
                created,
                id,
                ip_address,
                ip_address_location,
                livemode,
                open,
                opened_reason,
                payment_intent,
                reason,
                session,
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

    impl ObjectDeser for Review {
        type Builder = ReviewBuilder;
    }

    impl FromValueOpt for Review {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ReviewBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "billing_zip" => b.billing_zip = FromValueOpt::from_value(v),
                    "charge" => b.charge = FromValueOpt::from_value(v),
                    "closed_reason" => b.closed_reason = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "ip_address" => b.ip_address = FromValueOpt::from_value(v),
                    "ip_address_location" => b.ip_address_location = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "open" => b.open = FromValueOpt::from_value(v),
                    "opened_reason" => b.opened_reason = FromValueOpt::from_value(v),
                    "payment_intent" => b.payment_intent = FromValueOpt::from_value(v),
                    "reason" => b.reason = FromValueOpt::from_value(v),
                    "session" => b.session = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Review {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Review", 14)?;
        s.serialize_field("billing_zip", &self.billing_zip)?;
        s.serialize_field("charge", &self.charge)?;
        s.serialize_field("closed_reason", &self.closed_reason)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("ip_address", &self.ip_address)?;
        s.serialize_field("ip_address_location", &self.ip_address_location)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("open", &self.open)?;
        s.serialize_field("opened_reason", &self.opened_reason)?;
        s.serialize_field("payment_intent", &self.payment_intent)?;
        s.serialize_field("reason", &self.reason)?;
        s.serialize_field("session", &self.session)?;

        s.serialize_field("object", "review")?;
        s.end()
    }
}
/// The reason the review was closed, or null if it has not yet been closed.
/// One of `approved`, `refunded`, `refunded_as_fraud`, `disputed`, `redacted`, `canceled`, `payment_never_settled`, or `acknowledged`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReviewClosedReason {
    Acknowledged,
    Approved,
    Canceled,
    Disputed,
    PaymentNeverSettled,
    Redacted,
    Refunded,
    RefundedAsFraud,
}
impl ReviewClosedReason {
    pub fn as_str(self) -> &'static str {
        use ReviewClosedReason::*;
        match self {
            Acknowledged => "acknowledged",
            Approved => "approved",
            Canceled => "canceled",
            Disputed => "disputed",
            PaymentNeverSettled => "payment_never_settled",
            Redacted => "redacted",
            Refunded => "refunded",
            RefundedAsFraud => "refunded_as_fraud",
        }
    }
}

impl std::str::FromStr for ReviewClosedReason {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReviewClosedReason::*;
        match s {
            "acknowledged" => Ok(Acknowledged),
            "approved" => Ok(Approved),
            "canceled" => Ok(Canceled),
            "disputed" => Ok(Disputed),
            "payment_never_settled" => Ok(PaymentNeverSettled),
            "redacted" => Ok(Redacted),
            "refunded" => Ok(Refunded),
            "refunded_as_fraud" => Ok(RefundedAsFraud),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ReviewClosedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReviewClosedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ReviewClosedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ReviewClosedReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ReviewClosedReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReviewClosedReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ReviewClosedReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReviewClosedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReviewClosedReason"))
    }
}
/// The reason the review was opened. One of `rule` or `manual`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReviewOpenedReason {
    Manual,
    Rule,
}
impl ReviewOpenedReason {
    pub fn as_str(self) -> &'static str {
        use ReviewOpenedReason::*;
        match self {
            Manual => "manual",
            Rule => "rule",
        }
    }
}

impl std::str::FromStr for ReviewOpenedReason {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReviewOpenedReason::*;
        match s {
            "manual" => Ok(Manual),
            "rule" => Ok(Rule),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ReviewOpenedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReviewOpenedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ReviewOpenedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ReviewOpenedReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ReviewOpenedReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReviewOpenedReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ReviewOpenedReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReviewOpenedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReviewOpenedReason"))
    }
}
impl stripe_types::Object for Review {
    type Id = stripe_shared::ReviewId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ReviewId);
