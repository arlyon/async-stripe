/// A Mandate is a record of the permission that your customer gives you to debit their payment method.
///
/// For more details see <<https://stripe.com/docs/api/mandates/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Mandate {
    pub customer_acceptance: stripe_shared::CustomerAcceptance,
    /// Unique identifier for the object.
    pub id: stripe_shared::MandateId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
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

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
            Ok(Box::new(Builder { out: &mut self.out, builder: MandateBuilder::deser_default() }))
        }
    }

    impl MapBuilder for MandateBuilder {
        type Out = Mandate;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_acceptance" => Deserialize::begin(&mut self.customer_acceptance),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "multi_use" => Deserialize::begin(&mut self.multi_use),
                "on_behalf_of" => Deserialize::begin(&mut self.on_behalf_of),
                "payment_method" => Deserialize::begin(&mut self.payment_method),
                "payment_method_details" => Deserialize::begin(&mut self.payment_method_details),
                "single_use" => Deserialize::begin(&mut self.single_use),
                "status" => Deserialize::begin(&mut self.status),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                customer_acceptance: self.customer_acceptance.take()?,
                id: self.id.take()?,
                livemode: self.livemode?,
                multi_use: self.multi_use?,
                on_behalf_of: self.on_behalf_of.take()?,
                payment_method: self.payment_method.take()?,
                payment_method_details: self.payment_method_details.take()?,
                single_use: self.single_use?,
                status: self.status?,
                type_: self.type_?,
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

    impl ObjectDeser for Mandate {
        type Builder = MandateBuilder;
    }

    impl FromValueOpt for Mandate {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = MandateBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "customer_acceptance" => {
                        b.customer_acceptance = Some(FromValueOpt::from_value(v)?)
                    }
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "multi_use" => b.multi_use = Some(FromValueOpt::from_value(v)?),
                    "on_behalf_of" => b.on_behalf_of = Some(FromValueOpt::from_value(v)?),
                    "payment_method" => b.payment_method = Some(FromValueOpt::from_value(v)?),
                    "payment_method_details" => {
                        b.payment_method_details = Some(FromValueOpt::from_value(v)?)
                    }
                    "single_use" => b.single_use = Some(FromValueOpt::from_value(v)?),
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MandateStatus {
    Active,
    Inactive,
    Pending,
}
impl MandateStatus {
    pub fn as_str(self) -> &'static str {
        use MandateStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for MandateStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for MandateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for MandateStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandateStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandateStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandateStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for MandateStatus"))
    }
}
/// The type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MandateType {
    MultiUse,
    SingleUse,
}
impl MandateType {
    pub fn as_str(self) -> &'static str {
        use MandateType::*;
        match self {
            MultiUse => "multi_use",
            SingleUse => "single_use",
        }
    }
}

impl std::str::FromStr for MandateType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateType::*;
        match s {
            "multi_use" => Ok(MultiUse),
            "single_use" => Ok(SingleUse),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for MandateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for MandateType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandateType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandateType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandateType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for MandateType"))
    }
}
impl stripe_types::Object for Mandate {
    type Id = stripe_shared::MandateId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(MandateId);
