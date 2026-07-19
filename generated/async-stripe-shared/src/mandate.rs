/// A Mandate is a record of the permission that your customer gives you to debit their payment method.
///
/// For more details see <<https://stripe.com/docs/api/mandates/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Mandate {
    pub customer_acceptance: stripe_shared::CustomerAcceptance,
    /// Unique identifier for the object.
    pub id: stripe_shared::MandateId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    pub multi_use: Option<stripe_shared::MandateMultiUse>,
    /// The account (if any) that the mandate is intended for.
    pub on_behalf_of: Option<String>,
    /// ID of the payment method associated with this mandate.
    pub payment_method: stripe_types::Expandable<stripe_shared::PaymentMethod>,
    pub payment_method_details: stripe_shared::MandatePaymentMethodDetails,
    pub single_use: Option<stripe_shared::MandateSingleUse>,
    /// The mandate status indicates whether or not you can use it to initiate a payment.
    pub status: MandateStatus,
    /// The type of the mandate.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: MandateType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Mandate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Mandate").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct MandateBuilder {
    customer_acceptance: Option<stripe_shared::CustomerAcceptance>,
    id: Option<stripe_shared::MandateId>,
    livemode: Option<bool>,
    multi_use: Option<Option<stripe_shared::MandateMultiUse>>,
    on_behalf_of: Option<Option<String>>,
    payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    payment_method_details: Option<stripe_shared::MandatePaymentMethodDetails>,
    single_use: Option<Option<stripe_shared::MandateSingleUse>>,
    status: Option<MandateStatus>,
    type_: Option<MandateType>,
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

    impl Deserialize for Mandate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Mandate>,
        builder: MandateBuilder,
    }

    impl Visitor for Place<Mandate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: MandateBuilder {
                    customer_acceptance: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    multi_use: Deserialize::default(),
                    on_behalf_of: Deserialize::default(),
                    payment_method: Deserialize::default(),
                    payment_method_details: Deserialize::default(),
                    single_use: Deserialize::default(),
                    status: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_acceptance" => Deserialize::begin(&mut self.builder.customer_acceptance),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "multi_use" => Deserialize::begin(&mut self.builder.multi_use),
                "on_behalf_of" => Deserialize::begin(&mut self.builder.on_behalf_of),
                "payment_method" => Deserialize::begin(&mut self.builder.payment_method),
                "payment_method_details" => {
                    Deserialize::begin(&mut self.builder.payment_method_details)
                }
                "single_use" => Deserialize::begin(&mut self.builder.single_use),
                "status" => Deserialize::begin(&mut self.builder.status),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(customer_acceptance),
                Some(id),
                Some(livemode),
                Some(multi_use),
                Some(on_behalf_of),
                Some(payment_method),
                Some(payment_method_details),
                Some(single_use),
                Some(status),
                Some(type_),
            ) = (
                self.builder.customer_acceptance.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.multi_use,
                self.builder.on_behalf_of.take(),
                self.builder.payment_method.take(),
                self.builder.payment_method_details.take(),
                self.builder.single_use.take(),
                self.builder.status.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(Mandate {
                customer_acceptance,
                id,
                livemode,
                multi_use,
                on_behalf_of,
                payment_method,
                payment_method_details,
                single_use,
                status,
                type_,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Mandate {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Mandate", 11)?;
        s.serialize_field("customer_acceptance", &self.customer_acceptance)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("multi_use", &self.multi_use)?;
        s.serialize_field("on_behalf_of", &self.on_behalf_of)?;
        s.serialize_field("payment_method", &self.payment_method)?;
        s.serialize_field("payment_method_details", &self.payment_method_details)?;
        s.serialize_field("single_use", &self.single_use)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "mandate")?;
        s.end()
    }
}
/// The mandate status indicates whether or not you can use it to initiate a payment.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandateStatus {
    Active,
    Inactive,
    Pending,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandateStatus {
    pub fn as_str(&self) -> &str {
        use MandateStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MandateStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "MandateStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for MandateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(MandateStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandateStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for MandateStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<MandateStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandateStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The type of the mandate.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandateType {
    MultiUse,
    SingleUse,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandateType {
    pub fn as_str(&self) -> &str {
        use MandateType::*;
        match self {
            MultiUse => "multi_use",
            SingleUse => "single_use",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MandateType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateType::*;
        match s {
            "multi_use" => Ok(MultiUse),
            "single_use" => Ok(SingleUse),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "MandateType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for MandateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(MandateType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for MandateType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<MandateType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandateType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for Mandate {
    type Id = stripe_shared::MandateId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(MandateId);
