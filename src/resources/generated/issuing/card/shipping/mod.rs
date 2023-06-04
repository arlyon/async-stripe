#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Shipping {
    pub address: crate::address::Address,
    /// The delivery company that shipped a card.
    pub carrier: Option<ShippingCarrier>,
    /// Additional information that may be required for clearing customs.
    pub customs: Option<crate::issuing::card::shipping::customs::Customs>,
    /// A unix timestamp representing a best estimate of when the card will be delivered.
    pub eta: Option<crate::Timestamp>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// Shipment service, such as `standard` or `express`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// The delivery status of the card.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// Packaging options.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
pub mod customs;
pub use customs::Customs;
