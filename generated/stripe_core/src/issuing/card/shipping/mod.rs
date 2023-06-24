#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Shipping {
    pub address: stripe_types::address::Address,
    /// The delivery company that shipped a card.
    pub carrier: Option<ShippingCarrier>,
    /// Additional information that may be required for clearing customs.
    pub customs: Option<stripe_core::issuing::card::shipping::customs::Customs>,
    /// A unix timestamp representing a best estimate of when the card will be delivered.
    pub eta: Option<stripe_types::Timestamp>,
    /// Recipient name.
    pub name: String,
    /// The phone number of the receiver of the bulk shipment.
    ///
    /// This phone number will be provided to the shipping company, who might use it to contact the receiver in case of delivery issues.
    pub phone_number: Option<String>,
    /// Whether a signature is required for card delivery.
    ///
    /// This feature is only supported for US users.
    /// Standard shipping service does not support signature on delivery.
    /// The default value for standard shipping service is false and for express and priority services is true.
    pub require_signature: Option<bool>,
    /// Shipment service, such as `standard` or `express`.
    pub service: ShippingService,
    /// The delivery status of the card.
    pub status: Option<ShippingStatus>,
    /// A tracking number for a card shipment.
    pub tracking_number: Option<String>,
    /// A link to the shipping carrier's site where you can view detailed information about a card shipment.
    pub tracking_url: Option<String>,
    /// Packaging options.
    #[serde(rename = "type")]
    pub type_: ShippingType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Shipping {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The delivery company that shipped a card.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShippingCarrier {
    Dhl,
    Fedex,
    RoyalMail,
    Usps,
}

impl ShippingCarrier {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dhl => "dhl",
            Self::Fedex => "fedex",
            Self::RoyalMail => "royal_mail",
            Self::Usps => "usps",
        }
    }
}

impl std::str::FromStr for ShippingCarrier {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dhl" => Ok(Self::Dhl),
            "fedex" => Ok(Self::Fedex),
            "royal_mail" => Ok(Self::RoyalMail),
            "usps" => Ok(Self::Usps),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ShippingCarrier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ShippingCarrier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShippingCarrier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ShippingCarrier"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ShippingCarrier {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ShippingCarrier> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ShippingCarrier::from_str(s)?);
        Ok(())
    }
}
/// Shipment service, such as `standard` or `express`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShippingService {
    Express,
    Priority,
    Standard,
}

impl ShippingService {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Express => "express",
            Self::Priority => "priority",
            Self::Standard => "standard",
        }
    }
}

impl std::str::FromStr for ShippingService {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "express" => Ok(Self::Express),
            "priority" => Ok(Self::Priority),
            "standard" => Ok(Self::Standard),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ShippingService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ShippingService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShippingService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ShippingService"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ShippingService {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ShippingService> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ShippingService::from_str(s)?);
        Ok(())
    }
}
/// The delivery status of the card.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShippingStatus {
    Canceled,
    Delivered,
    Failure,
    Pending,
    Returned,
    Shipped,
}

impl ShippingStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Delivered => "delivered",
            Self::Failure => "failure",
            Self::Pending => "pending",
            Self::Returned => "returned",
            Self::Shipped => "shipped",
        }
    }
}

impl std::str::FromStr for ShippingStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "canceled" => Ok(Self::Canceled),
            "delivered" => Ok(Self::Delivered),
            "failure" => Ok(Self::Failure),
            "pending" => Ok(Self::Pending),
            "returned" => Ok(Self::Returned),
            "shipped" => Ok(Self::Shipped),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ShippingStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ShippingStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShippingStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ShippingStatus"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ShippingStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ShippingStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ShippingStatus::from_str(s)?);
        Ok(())
    }
}
/// Packaging options.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShippingType {
    Bulk,
    Individual,
}

impl ShippingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bulk => "bulk",
            Self::Individual => "individual",
        }
    }
}

impl std::str::FromStr for ShippingType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bulk" => Ok(Self::Bulk),
            "individual" => Ok(Self::Individual),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ShippingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ShippingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShippingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ShippingType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ShippingType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ShippingType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ShippingType::from_str(s)?);
        Ok(())
    }
}
pub mod customs;
pub use customs::Customs;
