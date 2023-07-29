#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedDiscount {
    /// The Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode.
    ///
    /// Will not be present for subscription mode.
    pub checkout_session: Option<String>,
    pub coupon: stripe_types::coupon::Coupon,
    /// The ID of the customer associated with this discount.
    pub customer: Option<stripe_types::Expandable<stripe_types::customer::Customer>>,
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// The ID of the discount object.
    ///
    /// Discounts cannot be fetched by ID.
    /// Use `expand[]=discounts` in API calls to expand discount IDs in an array.
    pub id: stripe_types::discount::DiscountId,
    /// The invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice.
    pub invoice: Option<String>,
    /// The invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's coupon was applied to, if it was applied directly to a particular invoice item or invoice line item.
    pub invoice_item: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedDiscountObject,
    /// The promotion code applied to create this discount.
    pub promotion_code:
        Option<stripe_types::Expandable<stripe_types::promotion_code::PromotionCode>>,
    /// Date that the coupon was applied.
    pub start: stripe_types::Timestamp,
    /// The subscription that this coupon is applied to, if it is applied to a particular subscription.
    pub subscription: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedDiscount {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedDiscountObject {
    Discount,
}

impl DeletedDiscountObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Discount => "discount",
        }
    }
}

impl std::str::FromStr for DeletedDiscountObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "discount" => Ok(Self::Discount),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedDiscountObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedDiscountObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedDiscountObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedDiscountObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedDiscountObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedDiscountObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<DeletedDiscountObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DeletedDiscountObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for DeletedDiscount {
    type Id = stripe_types::discount::DiscountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
