#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedDiscount {
    /// The Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode.
    ///
    /// Will not be present for subscription mode.
    pub checkout_session: Option<String>,
    pub coupon: stripe_types::Coupon,
    /// The ID of the customer associated with this discount.
    pub customer: Option<stripe_types::Expandable<stripe_types::Customer>>,
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
    /// The promotion code applied to create this discount.
    pub promotion_code: Option<stripe_types::Expandable<stripe_types::PromotionCode>>,
    /// Date that the coupon was applied.
    pub start: stripe_types::Timestamp,
    /// The subscription that this coupon is applied to, if it is applied to a particular subscription.
    pub subscription: Option<String>,
}
impl stripe_types::Object for DeletedDiscount {
    type Id = stripe_types::discount::DiscountId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
