#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardShipping {
    pub address: stripe_shared::Address,
    /// Address validation details for the shipment.
    pub address_validation: Option<stripe_shared::IssuingCardShippingAddressValidation>,
    /// The delivery company that shipped a card.
    pub carrier: Option<IssuingCardShippingCarrier>,
    /// Additional information that may be required for clearing customs.
    pub customs: Option<stripe_shared::IssuingCardShippingCustoms>,
    /// A unix timestamp representing a best estimate of when the card will be delivered.
    pub eta: Option<stripe_types::Timestamp>,
    /// Recipient name.
    pub name: String,
    /// The phone number of the receiver of the shipment.
    /// Our courier partners will use this number to contact you in the event of card delivery issues.
    /// For individual shipments to the EU/UK, if this field is empty, we will provide them with the phone number provided when the cardholder was initially created.
    pub phone_number: Option<String>,
    /// Whether a signature is required for card delivery.
    /// This feature is only supported for US users.
    /// Standard shipping service does not support signature on delivery.
    /// The default value for standard shipping service is false and for express and priority services is true.
    pub require_signature: Option<bool>,
    /// Shipment service, such as `standard` or `express`.
    pub service: IssuingCardShippingService,
    /// The delivery status of the card.
    pub status: Option<IssuingCardShippingStatus>,
    /// A tracking number for a card shipment.
    pub tracking_number: Option<String>,
    /// A link to the shipping carrier's site where you can view detailed information about a card shipment.
    pub tracking_url: Option<String>,
    /// Packaging options.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: IssuingCardShippingType,
}
#[doc(hidden)]
pub struct IssuingCardShippingBuilder {
    address: Option<stripe_shared::Address>,
    address_validation: Option<Option<stripe_shared::IssuingCardShippingAddressValidation>>,
    carrier: Option<Option<IssuingCardShippingCarrier>>,
    customs: Option<Option<stripe_shared::IssuingCardShippingCustoms>>,
    eta: Option<Option<stripe_types::Timestamp>>,
    name: Option<String>,
    phone_number: Option<Option<String>>,
    require_signature: Option<Option<bool>>,
    service: Option<IssuingCardShippingService>,
    status: Option<Option<IssuingCardShippingStatus>>,
    tracking_number: Option<Option<String>>,
    tracking_url: Option<Option<String>>,
    type_: Option<IssuingCardShippingType>,
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

    impl Deserialize for IssuingCardShipping {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardShipping>,
        builder: IssuingCardShippingBuilder,
    }

    impl Visitor for Place<IssuingCardShipping> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardShippingBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardShippingBuilder {
        type Out = IssuingCardShipping;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "address_validation" => Deserialize::begin(&mut self.address_validation),
                "carrier" => Deserialize::begin(&mut self.carrier),
                "customs" => Deserialize::begin(&mut self.customs),
                "eta" => Deserialize::begin(&mut self.eta),
                "name" => Deserialize::begin(&mut self.name),
                "phone_number" => Deserialize::begin(&mut self.phone_number),
                "require_signature" => Deserialize::begin(&mut self.require_signature),
                "service" => Deserialize::begin(&mut self.service),
                "status" => Deserialize::begin(&mut self.status),
                "tracking_number" => Deserialize::begin(&mut self.tracking_number),
                "tracking_url" => Deserialize::begin(&mut self.tracking_url),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                address_validation: Deserialize::default(),
                carrier: Deserialize::default(),
                customs: Deserialize::default(),
                eta: Deserialize::default(),
                name: Deserialize::default(),
                phone_number: Deserialize::default(),
                require_signature: Deserialize::default(),
                service: Deserialize::default(),
                status: Deserialize::default(),
                tracking_number: Deserialize::default(),
                tracking_url: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(address),
                Some(address_validation),
                Some(carrier),
                Some(customs),
                Some(eta),
                Some(name),
                Some(phone_number),
                Some(require_signature),
                Some(service),
                Some(status),
                Some(tracking_number),
                Some(tracking_url),
                Some(type_),
            ) = (
                self.address.take(),
                self.address_validation.take(),
                self.carrier,
                self.customs.take(),
                self.eta,
                self.name.take(),
                self.phone_number.take(),
                self.require_signature,
                self.service,
                self.status,
                self.tracking_number.take(),
                self.tracking_url.take(),
                self.type_,
            )
            else {
                return None;
            };
            Some(Self::Out {
                address,
                address_validation,
                carrier,
                customs,
                eta,
                name,
                phone_number,
                require_signature,
                service,
                status,
                tracking_number,
                tracking_url,
                type_,
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

    impl ObjectDeser for IssuingCardShipping {
        type Builder = IssuingCardShippingBuilder;
    }

    impl FromValueOpt for IssuingCardShipping {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingCardShippingBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = FromValueOpt::from_value(v),
                    "address_validation" => b.address_validation = FromValueOpt::from_value(v),
                    "carrier" => b.carrier = FromValueOpt::from_value(v),
                    "customs" => b.customs = FromValueOpt::from_value(v),
                    "eta" => b.eta = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "phone_number" => b.phone_number = FromValueOpt::from_value(v),
                    "require_signature" => b.require_signature = FromValueOpt::from_value(v),
                    "service" => b.service = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "tracking_number" => b.tracking_number = FromValueOpt::from_value(v),
                    "tracking_url" => b.tracking_url = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The delivery company that shipped a card.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardShippingCarrier {
    Dhl,
    Fedex,
    RoyalMail,
    Usps,
}
impl IssuingCardShippingCarrier {
    pub fn as_str(self) -> &'static str {
        use IssuingCardShippingCarrier::*;
        match self {
            Dhl => "dhl",
            Fedex => "fedex",
            RoyalMail => "royal_mail",
            Usps => "usps",
        }
    }
}

impl std::str::FromStr for IssuingCardShippingCarrier {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingCarrier::*;
        match s {
            "dhl" => Ok(Dhl),
            "fedex" => Ok(Fedex),
            "royal_mail" => Ok(RoyalMail),
            "usps" => Ok(Usps),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingCardShippingCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardShippingCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingCardShippingCarrier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingCardShippingCarrier {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardShippingCarrier> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardShippingCarrier::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardShippingCarrier);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardShippingCarrier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardShippingCarrier"))
    }
}
/// Shipment service, such as `standard` or `express`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardShippingService {
    Express,
    Priority,
    Standard,
}
impl IssuingCardShippingService {
    pub fn as_str(self) -> &'static str {
        use IssuingCardShippingService::*;
        match self {
            Express => "express",
            Priority => "priority",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for IssuingCardShippingService {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingService::*;
        match s {
            "express" => Ok(Express),
            "priority" => Ok(Priority),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingCardShippingService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardShippingService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingCardShippingService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingCardShippingService {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardShippingService> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardShippingService::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardShippingService);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardShippingService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardShippingService"))
    }
}
/// The delivery status of the card.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardShippingStatus {
    Canceled,
    Delivered,
    Failure,
    Pending,
    Returned,
    Shipped,
    Submitted,
}
impl IssuingCardShippingStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingCardShippingStatus::*;
        match self {
            Canceled => "canceled",
            Delivered => "delivered",
            Failure => "failure",
            Pending => "pending",
            Returned => "returned",
            Shipped => "shipped",
            Submitted => "submitted",
        }
    }
}

impl std::str::FromStr for IssuingCardShippingStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "delivered" => Ok(Delivered),
            "failure" => Ok(Failure),
            "pending" => Ok(Pending),
            "returned" => Ok(Returned),
            "shipped" => Ok(Shipped),
            "submitted" => Ok(Submitted),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingCardShippingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardShippingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingCardShippingStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingCardShippingStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardShippingStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardShippingStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardShippingStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardShippingStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardShippingStatus"))
    }
}
/// Packaging options.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardShippingType {
    Bulk,
    Individual,
}
impl IssuingCardShippingType {
    pub fn as_str(self) -> &'static str {
        use IssuingCardShippingType::*;
        match self {
            Bulk => "bulk",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for IssuingCardShippingType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingType::*;
        match s {
            "bulk" => Ok(Bulk),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingCardShippingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardShippingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingCardShippingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingCardShippingType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardShippingType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardShippingType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardShippingType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardShippingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardShippingType"))
    }
}
