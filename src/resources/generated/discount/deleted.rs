#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedDiscount {
    /// The Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode.
    ///
    /// Will not be present for subscription mode.
    pub checkout_session: Option<String>,
    pub coupon: crate::coupon::Coupon,
    /// The ID of the customer associated with this discount.
    pub customer: Option<crate::Expandable<crate::customer::Customer>>,
    /// Always true for a deleted object.
    pub deleted: bool,
    /// The ID of the discount object.
    ///
    /// Discounts cannot be fetched by ID.
    /// Use `expand[]=discounts` in API calls to expand discount IDs in an array.
    pub id: crate::discount::DiscountId,
    /// The invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice.
    pub invoice: Option<String>,
    /// The invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's coupon was applied to, if it was applied directly to a particular invoice item or invoice line item.
    pub invoice_item: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedDiscountObject,
    /// The promotion code applied to create this discount.
    pub promotion_code: Option<crate::Expandable<crate::promotion_code::PromotionCode>>,
    /// Date that the coupon was applied.
    pub start: crate::Timestamp,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl crate::Object for DeletedDiscount {
    type Id = crate::discount::DiscountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
