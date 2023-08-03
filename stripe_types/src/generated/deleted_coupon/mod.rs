#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedCoupon {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::coupon::CouponId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedCouponObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DeletedCouponObject {
    Coupon,
}

impl DeletedCouponObject {
    pub fn as_str(self) -> &'static str {
        use DeletedCouponObject::*;
        match self {
            Coupon => "coupon",
        }
    }
}

impl std::str::FromStr for DeletedCouponObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeletedCouponObject::*;
        match s {
            "coupon" => Ok(Coupon),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedCouponObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedCouponObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DeletedCouponObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for DeletedCouponObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedCouponObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for DeletedCouponObject"))
    }
}
impl stripe_types::Object for DeletedCoupon {
    type Id = stripe_types::coupon::CouponId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
