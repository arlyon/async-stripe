#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSourceSourceOrderItemsType {
    Discount,
    Shipping,
    Sku,
    Tax,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateSourceSourceOrderItemsType {
    pub fn as_str(&self) -> &str {
        use CreateSourceSourceOrderItemsType::*;
        match self {
            Discount => "discount",
            Shipping => "shipping",
            Sku => "sku",
            Tax => "tax",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateSourceSourceOrderItemsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceSourceOrderItemsType::*;
        match s {
            "discount" => Ok(Discount),
            "shipping" => Ok(Shipping),
            "sku" => Ok(Sku),
            "tax" => Ok(Tax),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateSourceSourceOrderItemsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateSourceSourceOrderItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateSourceSourceOrderItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSourceSourceOrderItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateSourceSourceOrderItemsType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateSourceSourceOrderItemsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSourceSourceOrderItemsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
