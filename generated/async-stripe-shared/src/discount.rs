/// A discount represents the actual application of a [coupon](https://stripe.com/docs/api#coupons) or [promotion code](https://stripe.com/docs/api#promotion_codes).
/// It contains information about when the discount began, when it will end, and what it is applied to.
///
/// Related guide: [Applying discounts to subscriptions](https://stripe.com/docs/billing/subscriptions/discounts).
///
/// For more details see <<https://stripe.com/docs/api/discounts/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Discount {
    /// The Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode.
    /// Will not be present for subscription mode.
    pub checkout_session: Option<String>,
    pub coupon: stripe_shared::Coupon,
    /// The ID of the customer associated with this discount.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// If the coupon has a duration of `repeating`, the date that this discount will end.
    /// If the coupon has a duration of `once` or `forever`, this attribute will be null.
    pub end: Option<stripe_types::Timestamp>,
    /// The ID of the discount object.
    /// Discounts cannot be fetched by ID.
    /// Use `expand[]=discounts` in API calls to expand discount IDs in an array.
    pub id: stripe_shared::DiscountId,
    /// The invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice.
    pub invoice: Option<String>,
    /// The invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's coupon was applied to, if it was applied directly to a particular invoice item or invoice line item.
    pub invoice_item: Option<String>,
    /// The promotion code applied to create this discount.
    pub promotion_code: Option<stripe_types::Expandable<stripe_shared::PromotionCode>>,
    /// Date that the coupon was applied.
    pub start: stripe_types::Timestamp,
    /// The subscription that this coupon is applied to, if it is applied to a particular subscription.
    pub subscription: Option<String>,
    /// The subscription item that this coupon is applied to, if it is applied to a particular subscription item.
    pub subscription_item: Option<String>,
}
#[doc(hidden)]
pub struct DiscountBuilder {
    checkout_session: Option<Option<String>>,
    coupon: Option<stripe_shared::Coupon>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    end: Option<Option<stripe_types::Timestamp>>,
    id: Option<stripe_shared::DiscountId>,
    invoice: Option<Option<String>>,
    invoice_item: Option<Option<String>>,
    promotion_code: Option<Option<stripe_types::Expandable<stripe_shared::PromotionCode>>>,
    start: Option<stripe_types::Timestamp>,
    subscription: Option<Option<String>>,
    subscription_item: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Discount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Discount>,
        builder: DiscountBuilder,
    }

    impl Visitor for Place<Discount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: DiscountBuilder::deser_default() }))
        }
    }

    impl MapBuilder for DiscountBuilder {
        type Out = Discount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "checkout_session" => Deserialize::begin(&mut self.checkout_session),
                "coupon" => Deserialize::begin(&mut self.coupon),
                "customer" => Deserialize::begin(&mut self.customer),
                "end" => Deserialize::begin(&mut self.end),
                "id" => Deserialize::begin(&mut self.id),
                "invoice" => Deserialize::begin(&mut self.invoice),
                "invoice_item" => Deserialize::begin(&mut self.invoice_item),
                "promotion_code" => Deserialize::begin(&mut self.promotion_code),
                "start" => Deserialize::begin(&mut self.start),
                "subscription" => Deserialize::begin(&mut self.subscription),
                "subscription_item" => Deserialize::begin(&mut self.subscription_item),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                checkout_session: Deserialize::default(),
                coupon: Deserialize::default(),
                customer: Deserialize::default(),
                end: Deserialize::default(),
                id: Deserialize::default(),
                invoice: Deserialize::default(),
                invoice_item: Deserialize::default(),
                promotion_code: Deserialize::default(),
                start: Deserialize::default(),
                subscription: Deserialize::default(),
                subscription_item: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(checkout_session),
                Some(coupon),
                Some(customer),
                Some(end),
                Some(id),
                Some(invoice),
                Some(invoice_item),
                Some(promotion_code),
                Some(start),
                Some(subscription),
                Some(subscription_item),
            ) = (
                self.checkout_session.take(),
                self.coupon.take(),
                self.customer.take(),
                self.end,
                self.id.take(),
                self.invoice.take(),
                self.invoice_item.take(),
                self.promotion_code.take(),
                self.start,
                self.subscription.take(),
                self.subscription_item.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                checkout_session,
                coupon,
                customer,
                end,
                id,
                invoice,
                invoice_item,
                promotion_code,
                start,
                subscription,
                subscription_item,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for Discount {
        type Builder = DiscountBuilder;
    }

    impl FromValueOpt for Discount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DiscountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "checkout_session" => b.checkout_session = FromValueOpt::from_value(v),
                    "coupon" => b.coupon = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "end" => b.end = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "invoice" => b.invoice = FromValueOpt::from_value(v),
                    "invoice_item" => b.invoice_item = FromValueOpt::from_value(v),
                    "promotion_code" => b.promotion_code = FromValueOpt::from_value(v),
                    "start" => b.start = FromValueOpt::from_value(v),
                    "subscription" => b.subscription = FromValueOpt::from_value(v),
                    "subscription_item" => b.subscription_item = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Discount {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Discount", 12)?;
        s.serialize_field("checkout_session", &self.checkout_session)?;
        s.serialize_field("coupon", &self.coupon)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("end", &self.end)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("invoice", &self.invoice)?;
        s.serialize_field("invoice_item", &self.invoice_item)?;
        s.serialize_field("promotion_code", &self.promotion_code)?;
        s.serialize_field("start", &self.start)?;
        s.serialize_field("subscription", &self.subscription)?;
        s.serialize_field("subscription_item", &self.subscription_item)?;

        s.serialize_field("object", "discount")?;
        s.end()
    }
}
impl stripe_types::Object for Discount {
    type Id = stripe_shared::DiscountId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(DiscountId);
