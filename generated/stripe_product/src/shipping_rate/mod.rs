/// Shipping rates describe the price of shipping presented to your customers and can be
/// applied to [Checkout Sessions](https://stripe.com/docs/payments/checkout/shipping)
/// and [Orders](https://stripe.com/docs/orders/shipping) to collect shipping costs.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ShippingRate {
    /// Whether the shipping rate can be used for new purchases.
    ///
    /// Defaults to `true`.
    pub active: bool,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    pub delivery_estimate:
        Option<stripe_product::shipping_rate::delivery_estimate::DeliveryEstimate>,
    /// The name of the shipping rate, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<stripe_product::shipping_rate::fixed_amount::FixedAmount>,
    /// Unique identifier for the object.
    pub id: stripe_product::shipping_rate::ShippingRateId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ShippingRateObject,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    pub tax_behavior: Option<ShippingRateTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    ///
    /// The Shipping tax code is `txcd_92010001`.
    pub tax_code: Option<stripe_types::Expandable<stripe_types::tax_code::TaxCode>>,
    /// The type of calculation to use on the shipping rate.
    ///
    /// Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    pub type_: ShippingRateType,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShippingRateObject {
    ShippingRate,
}

impl ShippingRateObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ShippingRate => "shipping_rate",
        }
    }
}

impl std::str::FromStr for ShippingRateObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "shipping_rate" => Ok(Self::ShippingRate),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ShippingRateObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingRateObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ShippingRateObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShippingRateObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ShippingRateObject"))
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShippingRateTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl ShippingRateTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for ShippingRateTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exclusive" => Ok(Self::Exclusive),
            "inclusive" => Ok(Self::Inclusive),
            "unspecified" => Ok(Self::Unspecified),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ShippingRateTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingRateTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ShippingRateTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShippingRateTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ShippingRateTaxBehavior"))
    }
}
/// The type of calculation to use on the shipping rate.
///
/// Can only be `fixed_amount` for now.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShippingRateType {
    FixedAmount,
}

impl ShippingRateType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FixedAmount => "fixed_amount",
        }
    }
}

impl std::str::FromStr for ShippingRateType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fixed_amount" => Ok(Self::FixedAmount),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ShippingRateType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingRateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ShippingRateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShippingRateType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ShippingRateType"))
    }
}
impl stripe_types::Object for ShippingRate {
    type Id = stripe_product::shipping_rate::ShippingRateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ShippingRateId, "shr_");
pub mod currency_option;
pub use currency_option::CurrencyOption;
pub mod delivery_estimate;
pub use delivery_estimate::DeliveryEstimate;
pub mod fixed_amount;
pub use fixed_amount::FixedAmount;
