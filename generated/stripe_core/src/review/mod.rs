/// Reviews can be used to supplement automated fraud detection with human expertise.
///
/// Learn more about [Radar](/radar) and reviewing payments
/// [here](https://stripe.com/docs/radar/reviews).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Review {
    /// The ZIP or postal code of the card used, if applicable.
    pub billing_zip: Option<String>,
    /// The charge associated with this review.
    pub charge: Option<stripe_types::Expandable<stripe_core::charge::Charge>>,
    /// The reason the review was closed, or null if it has not yet been closed.
    ///
    /// One of `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
    pub closed_reason: Option<ReviewClosedReason>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_core::review::ReviewId,
    /// The IP address where the payment originated.
    pub ip_address: Option<String>,
    /// Information related to the location of the payment.
    ///
    /// Note that this information is an approximation and attempts to locate the nearest population center - it should not be used to determine a specific address.
    pub ip_address_location: Option<stripe_core::review::location::Location>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ReviewObject,
    /// If `true`, the review needs action.
    pub open: bool,
    /// The reason the review was opened.
    ///
    /// One of `rule` or `manual`.
    pub opened_reason: ReviewOpenedReason,
    /// The PaymentIntent ID associated with this review, if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent:
        Option<stripe_types::Expandable<stripe_core::payment_intent::PaymentIntent>>,
    /// The reason the review is currently open or closed.
    ///
    /// One of `rule`, `manual`, `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
    pub reason: String,
    /// Information related to the browsing session of the user who initiated the payment.
    pub session: Option<stripe_core::review::session::Session>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Review {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The reason the review was closed, or null if it has not yet been closed.
///
/// One of `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReviewClosedReason {
    Approved,
    Disputed,
    Redacted,
    Refunded,
    RefundedAsFraud,
}

impl ReviewClosedReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::Disputed => "disputed",
            Self::Redacted => "redacted",
            Self::Refunded => "refunded",
            Self::RefundedAsFraud => "refunded_as_fraud",
        }
    }
}

impl std::str::FromStr for ReviewClosedReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "approved" => Ok(Self::Approved),
            "disputed" => Ok(Self::Disputed),
            "redacted" => Ok(Self::Redacted),
            "refunded" => Ok(Self::Refunded),
            "refunded_as_fraud" => Ok(Self::RefundedAsFraud),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ReviewClosedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReviewClosedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ReviewClosedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReviewClosedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReviewClosedReason"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReviewClosedReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ReviewClosedReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReviewClosedReason::from_str(s)?);
        Ok(())
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReviewObject {
    Review,
}

impl ReviewObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Review => "review",
        }
    }
}

impl std::str::FromStr for ReviewObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "review" => Ok(Self::Review),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ReviewObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReviewObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ReviewObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReviewObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ReviewObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReviewObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ReviewObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReviewObject::from_str(s)?);
        Ok(())
    }
}
/// The reason the review was opened.
///
/// One of `rule` or `manual`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReviewOpenedReason {
    Manual,
    Rule,
}

impl ReviewOpenedReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::Rule => "rule",
        }
    }
}

impl std::str::FromStr for ReviewOpenedReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "manual" => Ok(Self::Manual),
            "rule" => Ok(Self::Rule),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ReviewOpenedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReviewOpenedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ReviewOpenedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReviewOpenedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReviewOpenedReason"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReviewOpenedReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ReviewOpenedReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReviewOpenedReason::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for Review {
    type Id = stripe_core::review::ReviewId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ReviewId, "prv_");
pub mod location;
pub mod requests;
pub use location::Location;
pub mod session;
pub use session::Session;
