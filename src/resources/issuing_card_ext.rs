use serde::{Deserialize, Serialize};

/// An enum representing the possible values of an `IssuingCardPin`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardPinStatus {
    Active,
    Blocked,
}

impl IssuingCardPinStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardPinStatus::Active => "active",
            IssuingCardPinStatus::Blocked => "blocked",
        }
    }
}

impl AsRef<str> for IssuingCardPinStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardPinStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCardShipping`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
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
        match self {
            IssuingCardShippingStatus::Canceled => "canceled",
            IssuingCardShippingStatus::Delivered => "delivered",
            IssuingCardShippingStatus::Failure => "failure",
            IssuingCardShippingStatus::Pending => "pending",
            IssuingCardShippingStatus::Returned => "returned",
            IssuingCardShippingStatus::Shipped => "shipped",
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
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCardShipping`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardShippingType {
    Bulk,
    Individual,
}

impl IssuingCardShippingType {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardShippingType::Bulk => "bulk",
            IssuingCardShippingType::Individual => "individual",
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
        self.as_str().fmt(f)
    }
}

impl std::default::Default for IssuingCardShippingType {
    fn default() -> Self {
        Self::Individual
    }
}

/// An enum representing the possible values of an `IssuingCard`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardType {
    Physical,
    Virtual,
}

impl IssuingCardType {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardType::Physical => "physical",
            IssuingCardType::Virtual => "virtual",
        }
    }
}

impl AsRef<str> for IssuingCardType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl std::default::Default for IssuingCardType {
    fn default() -> Self {
        Self::Physical
    }
}
