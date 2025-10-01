#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DeletedDiscount {
    /// The Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode.
    /// Will not be present for subscription mode.
    pub checkout_session: Option<String>,
    /// The ID of the customer associated with this discount.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    #[allow(dead_code)]
    deleted: stripe_types::AlwaysTrue,
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
    pub source: stripe_shared::DiscountSource,
    /// Date that the coupon was applied.
    pub start: stripe_types::Timestamp,
    /// The subscription that this coupon is applied to, if it is applied to a particular subscription.
    pub subscription: Option<String>,
    /// The subscription item that this coupon is applied to, if it is applied to a particular subscription item.
    pub subscription_item: Option<String>,
}
#[doc(hidden)]
pub struct DeletedDiscountBuilder {
    checkout_session: Option<Option<String>>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    deleted: Option<stripe_types::AlwaysTrue>,
    id: Option<stripe_shared::DiscountId>,
    invoice: Option<Option<String>>,
    invoice_item: Option<Option<String>>,
    promotion_code: Option<Option<stripe_types::Expandable<stripe_shared::PromotionCode>>>,
    source: Option<stripe_shared::DiscountSource>,
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

    impl Deserialize for DeletedDiscount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DeletedDiscount>,
        builder: DeletedDiscountBuilder,
    }

    impl Visitor for Place<DeletedDiscount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DeletedDiscountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DeletedDiscountBuilder {
        type Out = DeletedDiscount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "checkout_session" => Deserialize::begin(&mut self.checkout_session),
                "customer" => Deserialize::begin(&mut self.customer),
                "deleted" => Deserialize::begin(&mut self.deleted),
                "id" => Deserialize::begin(&mut self.id),
                "invoice" => Deserialize::begin(&mut self.invoice),
                "invoice_item" => Deserialize::begin(&mut self.invoice_item),
                "promotion_code" => Deserialize::begin(&mut self.promotion_code),
                "source" => Deserialize::begin(&mut self.source),
                "start" => Deserialize::begin(&mut self.start),
                "subscription" => Deserialize::begin(&mut self.subscription),
                "subscription_item" => Deserialize::begin(&mut self.subscription_item),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                checkout_session: Deserialize::default(),
                customer: Deserialize::default(),
                deleted: Deserialize::default(),
                id: Deserialize::default(),
                invoice: Deserialize::default(),
                invoice_item: Deserialize::default(),
                promotion_code: Deserialize::default(),
                source: Deserialize::default(),
                start: Deserialize::default(),
                subscription: Deserialize::default(),
                subscription_item: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(checkout_session),
                Some(customer),
                Some(deleted),
                Some(id),
                Some(invoice),
                Some(invoice_item),
                Some(promotion_code),
                Some(source),
                Some(start),
                Some(subscription),
                Some(subscription_item),
            ) = (
                self.checkout_session.take(),
                self.customer.take(),
                self.deleted,
                self.id.take(),
                self.invoice.take(),
                self.invoice_item.take(),
                self.promotion_code.take(),
                self.source.take(),
                self.start,
                self.subscription.take(),
                self.subscription_item.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                checkout_session,
                customer,
                deleted,
                id,
                invoice,
                invoice_item,
                promotion_code,
                source,
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

    impl ObjectDeser for DeletedDiscount {
        type Builder = DeletedDiscountBuilder;
    }

    impl FromValueOpt for DeletedDiscount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DeletedDiscountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "checkout_session" => b.checkout_session = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "deleted" => b.deleted = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "invoice" => b.invoice = FromValueOpt::from_value(v),
                    "invoice_item" => b.invoice_item = FromValueOpt::from_value(v),
                    "promotion_code" => b.promotion_code = FromValueOpt::from_value(v),
                    "source" => b.source = FromValueOpt::from_value(v),
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
impl serde::Serialize for DeletedDiscount {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("DeletedDiscount", 12)?;
        s.serialize_field("checkout_session", &self.checkout_session)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("deleted", &self.deleted)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("invoice", &self.invoice)?;
        s.serialize_field("invoice_item", &self.invoice_item)?;
        s.serialize_field("promotion_code", &self.promotion_code)?;
        s.serialize_field("source", &self.source)?;
        s.serialize_field("start", &self.start)?;
        s.serialize_field("subscription", &self.subscription)?;
        s.serialize_field("subscription_item", &self.subscription_item)?;

        s.serialize_field("object", "discount")?;
        s.end()
    }
}
impl stripe_types::Object for DeletedDiscount {
    type Id = stripe_shared::DiscountId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
