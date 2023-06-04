#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedCoupon {
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: crate::coupon::CouponId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedCouponObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedCoupon {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum DeletedCouponObject {
    Coupon,
}

impl DeletedCouponObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Coupon => "coupon",
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
        self.as_str().fmt(f)
    }
}
impl crate::Object for DeletedCoupon {
    type Id = crate::coupon::CouponId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
