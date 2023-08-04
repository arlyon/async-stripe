#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingCardShipping {
    pub address: stripe_types::Address,
    /// The delivery company that shipped a card.
    pub carrier: Option<IssuingCardShippingCarrier>,
    /// Additional information that may be required for clearing customs.
    pub customs: Option<stripe_types::IssuingCardShippingCustoms>,
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
    pub service: IssuingCardShippingService,
    /// The delivery status of the card.
    pub status: Option<IssuingCardShippingStatus>,
    /// A tracking number for a card shipment.
    pub tracking_number: Option<String>,
    /// A link to the shipping carrier's site where you can view detailed information about a card shipment.
    pub tracking_url: Option<String>,
    /// Packaging options.
    #[serde(rename = "type")]
    pub type_: IssuingCardShippingType,
}
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingCarrier::*;
        match s {
            "dhl" => Ok(Dhl),
            "fedex" => Ok(Fedex),
            "royal_mail" => Ok(RoyalMail),
            "usps" => Ok(Usps),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingCardShippingCarrier {
    fn as_ref(&self) -> &str {
        self.as_str()
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
impl serde::Serialize for IssuingCardShippingCarrier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardShippingCarrier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingService::*;
        match s {
            "express" => Ok(Express),
            "priority" => Ok(Priority),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingCardShippingService {
    fn as_ref(&self) -> &str {
        self.as_str()
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
impl serde::Serialize for IssuingCardShippingService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardShippingService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
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
        }
    }
}

impl std::str::FromStr for IssuingCardShippingStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "delivered" => Ok(Delivered),
            "failure" => Ok(Failure),
            "pending" => Ok(Pending),
            "returned" => Ok(Returned),
            "shipped" => Ok(Shipped),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingCardShippingStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
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
impl serde::Serialize for IssuingCardShippingStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardShippingStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingType::*;
        match s {
            "bulk" => Ok(Bulk),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingCardShippingType {
    fn as_ref(&self) -> &str {
        self.as_str()
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
impl serde::Serialize for IssuingCardShippingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardShippingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardShippingType"))
    }
}
