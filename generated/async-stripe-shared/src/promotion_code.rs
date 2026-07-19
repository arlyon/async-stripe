/// A Promotion Code represents a customer-redeemable code for an underlying promotion.
/// You can create multiple codes for a single promotion.
///
/// If you enable promotion codes in your [customer portal configuration](https://docs.stripe.com/customer-management/configure-portal), then customers can redeem a code themselves when updating a subscription in the portal.
/// Customers can also view the currently active promotion codes and coupons on each of their subscriptions in the portal.
///
/// For more details see <<https://stripe.com/docs/api/promotion_codes/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PromotionCode {
    /// Whether the promotion code is currently active.
    /// A promotion code is only active if the coupon is also valid.
    pub active: bool,
    /// The customer-facing code.
    /// Regardless of case, this code must be unique across all active promotion codes for each customer.
    /// Valid characters are lower case letters (a-z), upper case letters (A-Z), digits (0-9), and dashes (-).
    pub code: String,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The customer who can use this promotion code.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// The account representing the customer who can use this promotion code.
    pub customer_account: Option<String>,
    /// Date at which the promotion code can no longer be redeemed.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_shared::PromotionCodeId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Maximum number of times this promotion code can be redeemed.
    pub max_redemptions: Option<i64>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub promotion: stripe_shared::PromotionCodesResourcePromotion,
    pub restrictions: stripe_shared::PromotionCodesResourceRestrictions,
    /// Number of times this promotion code has been used.
    pub times_redeemed: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PromotionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PromotionCode").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PromotionCodeBuilder {
    active: Option<bool>,
    code: Option<String>,
    created: Option<stripe_types::Timestamp>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    customer_account: Option<Option<String>>,
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PromotionCodeBuilder {
                    active: Deserialize::default(),
                    code: Deserialize::default(),
                    created: Deserialize::default(),
                    customer: Deserialize::default(),
                    customer_account: Deserialize::default(),
                    expires_at: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    max_redemptions: Deserialize::default(),
                    metadata: Deserialize::default(),
                    promotion: Deserialize::default(),
                    restrictions: Deserialize::default(),
                    times_redeemed: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.builder.active),
                "code" => Deserialize::begin(&mut self.builder.code),
                "created" => Deserialize::begin(&mut self.builder.created),
                "customer" => Deserialize::begin(&mut self.builder.customer),
                "customer_account" => Deserialize::begin(&mut self.builder.customer_account),
                "expires_at" => Deserialize::begin(&mut self.builder.expires_at),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "max_redemptions" => Deserialize::begin(&mut self.builder.max_redemptions),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "promotion" => Deserialize::begin(&mut self.builder.promotion),
                "restrictions" => Deserialize::begin(&mut self.builder.restrictions),
                "times_redeemed" => Deserialize::begin(&mut self.builder.times_redeemed),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(active),
                Some(code),
                Some(created),
                Some(customer),
                Some(customer_account),
                Some(expires_at),
                Some(id),
                Some(livemode),
                Some(max_redemptions),
                Some(metadata),
                Some(promotion),
                Some(restrictions),
                Some(times_redeemed),
            ) = (
                self.builder.active,
                self.builder.code.take(),
                self.builder.created,
                self.builder.customer.take(),
                self.builder.customer_account.take(),
                self.builder.expires_at,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.max_redemptions,
                self.builder.metadata.take(),
                self.builder.promotion.take(),
                self.builder.restrictions.take(),
                self.builder.times_redeemed,
            )
            else {
                return Ok(());
            };
            *self.out = Some(PromotionCode {
                active,
                code,
                created,
                customer,
                customer_account,
                expires_at,
                id,
                livemode,
                max_redemptions,
                metadata,
                promotion,
                restrictions,
                times_redeemed,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for PromotionCode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("PromotionCode", 14)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("code", &self.code)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("customer_account", &self.customer_account)?;
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
