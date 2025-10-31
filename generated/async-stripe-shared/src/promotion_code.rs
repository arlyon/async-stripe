/// A Promotion Code represents a customer-redeemable code for an underlying promotion.
/// You can create multiple codes for a single promotion.
///
/// If you enable promotion codes in your [customer portal configuration](https://stripe.com/docs/customer-management/configure-portal), then customers can redeem a code themselves when updating a subscription in the portal.
/// Customers can also view the currently active promotion codes and coupons on each of their subscriptions in the portal.
///
/// For more details see <<https://stripe.com/docs/api/promotion_codes/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PromotionCode {
    /// Whether the promotion code is currently active.
    /// A promotion code is only active if the coupon is also valid.
    pub active: bool,
    /// The customer-facing code.
    /// Regardless of case, this code must be unique across all active promotion codes for each customer.
    /// Valid characters are lower case letters (a-z), upper case letters (A-Z), and digits (0-9).
    pub code: String,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The customer that this promotion code can be used by.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// Date at which the promotion code can no longer be redeemed.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_shared::PromotionCodeId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Maximum number of times this promotion code can be redeemed.
    pub max_redemptions: Option<i64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub promotion: stripe_shared::PromotionCodesResourcePromotion,
    pub restrictions: stripe_shared::PromotionCodesResourceRestrictions,
    /// Number of times this promotion code has been used.
    pub times_redeemed: i64,
}
#[doc(hidden)]
pub struct PromotionCodeBuilder {
    active: Option<bool>,
    code: Option<String>,
    created: Option<stripe_types::Timestamp>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    id: Option<stripe_shared::PromotionCodeId>,
    livemode: Option<bool>,
    max_redemptions: Option<Option<i64>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    promotion: Option<stripe_shared::PromotionCodesResourcePromotion>,
    restrictions: Option<stripe_shared::PromotionCodesResourceRestrictions>,
    times_redeemed: Option<i64>,
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

    impl Deserialize for PromotionCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PromotionCode>,
        builder: PromotionCodeBuilder,
    }

    impl Visitor for Place<PromotionCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PromotionCodeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PromotionCodeBuilder {
        type Out = PromotionCode;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.active),
                "code" => Deserialize::begin(&mut self.code),
                "created" => Deserialize::begin(&mut self.created),
                "customer" => Deserialize::begin(&mut self.customer),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "max_redemptions" => Deserialize::begin(&mut self.max_redemptions),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "promotion" => Deserialize::begin(&mut self.promotion),
                "restrictions" => Deserialize::begin(&mut self.restrictions),
                "times_redeemed" => Deserialize::begin(&mut self.times_redeemed),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                active: Deserialize::default(),
                code: Deserialize::default(),
                created: Deserialize::default(),
                customer: Deserialize::default(),
                expires_at: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                max_redemptions: Deserialize::default(),
                metadata: Deserialize::default(),
                promotion: Deserialize::default(),
                restrictions: Deserialize::default(),
                times_redeemed: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(active),
                Some(code),
                Some(created),
                Some(customer),
                Some(expires_at),
                Some(id),
                Some(livemode),
                Some(max_redemptions),
                Some(metadata),
                Some(promotion),
                Some(restrictions),
                Some(times_redeemed),
            ) = (
                self.active,
                self.code.take(),
                self.created,
                self.customer.take(),
                self.expires_at,
                self.id.take(),
                self.livemode,
                self.max_redemptions,
                self.metadata.take(),
                self.promotion.take(),
                self.restrictions.take(),
                self.times_redeemed,
            )
            else {
                return None;
            };
            Some(Self::Out {
                active,
                code,
                created,
                customer,
                expires_at,
                id,
                livemode,
                max_redemptions,
                metadata,
                promotion,
                restrictions,
                times_redeemed,
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

    impl ObjectDeser for PromotionCode {
        type Builder = PromotionCodeBuilder;
    }

    impl FromValueOpt for PromotionCode {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PromotionCodeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "active" => b.active = FromValueOpt::from_value(v),
                    "code" => b.code = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "max_redemptions" => b.max_redemptions = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "promotion" => b.promotion = FromValueOpt::from_value(v),
                    "restrictions" => b.restrictions = FromValueOpt::from_value(v),
                    "times_redeemed" => b.times_redeemed = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for PromotionCode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("PromotionCode", 13)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("code", &self.code)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("expires_at", &self.expires_at)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("max_redemptions", &self.max_redemptions)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("promotion", &self.promotion)?;
        s.serialize_field("restrictions", &self.restrictions)?;
        s.serialize_field("times_redeemed", &self.times_redeemed)?;

        s.serialize_field("object", "promotion_code")?;
        s.end()
    }
}
impl stripe_types::Object for PromotionCode {
    type Id = stripe_shared::PromotionCodeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(PromotionCodeId);
