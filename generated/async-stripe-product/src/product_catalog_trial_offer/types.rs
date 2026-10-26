/// Trial offers let you define free or paid introductory pricing for a subscription item.
/// A TrialOffer specifies the price to charge during the trial, how many billing intervals
/// the trial lasts, and what price the subscription item transitions to when the trial ends.
/// You attach a TrialOffer to a subscription item
/// using `items[current_trial][trial_offer]` when creating or updating a subscription.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ProductCatalogTrialOffer {
    /// Whether the trial offer is active. Set to false to archive the trial offer.
    pub active: bool,
    pub duration: stripe_product::ProductCatalogImplEndpointsTrialOfferResourceTrialOfferDuration,
    pub end_behavior:
        stripe_product::ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehavior,
    /// Unique identifier for the object.
    pub id: stripe_product::ProductCatalogTrialOfferId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// A brief description of the trial offer, hidden from customers.
    pub nickname: Option<String>,
    /// The price during the trial offer.
    pub price: stripe_types::Expandable<stripe_shared::Price>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ProductCatalogTrialOffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ProductCatalogTrialOffer").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ProductCatalogTrialOfferBuilder {
    active: Option<bool>,
    duration:
        Option<stripe_product::ProductCatalogImplEndpointsTrialOfferResourceTrialOfferDuration>,
    end_behavior:
        Option<stripe_product::ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehavior>,
    id: Option<stripe_product::ProductCatalogTrialOfferId>,
    livemode: Option<bool>,
    nickname: Option<Option<String>>,
    price: Option<stripe_types::Expandable<stripe_shared::Price>>,
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

    impl Deserialize for ProductCatalogTrialOffer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ProductCatalogTrialOffer>,
        builder: ProductCatalogTrialOfferBuilder,
    }

    impl Visitor for Place<ProductCatalogTrialOffer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ProductCatalogTrialOfferBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ProductCatalogTrialOfferBuilder {
        type Out = ProductCatalogTrialOffer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.active),
                "duration" => Deserialize::begin(&mut self.duration),
                "end_behavior" => Deserialize::begin(&mut self.end_behavior),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "nickname" => Deserialize::begin(&mut self.nickname),
                "price" => Deserialize::begin(&mut self.price),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                active: None,
                duration: None,
                end_behavior: None,
                id: None,
                livemode: None,
                nickname: Some(None),
                price: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(active),
                Some(duration),
                Some(end_behavior),
                Some(id),
                Some(livemode),
                Some(nickname),
                Some(price),
            ) = (
                self.active,
                self.duration.take(),
                self.end_behavior.take(),
                self.id.take(),
                self.livemode,
                self.nickname.take(),
                self.price.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { active, duration, end_behavior, id, livemode, nickname, price })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for ProductCatalogTrialOffer {
        type Builder = ProductCatalogTrialOfferBuilder;
    }

    impl FromValueOpt for ProductCatalogTrialOffer {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ProductCatalogTrialOfferBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "active" => b.active = FromValueOpt::from_value(v),
                    "duration" => b.duration = FromValueOpt::from_value(v),
                    "end_behavior" => b.end_behavior = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "nickname" => b.nickname = FromValueOpt::from_value(v),
                    "price" => b.price = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ProductCatalogTrialOffer {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ProductCatalogTrialOffer", 8)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("duration", &self.duration)?;
        s.serialize_field("end_behavior", &self.end_behavior)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("nickname", &self.nickname)?;
        s.serialize_field("price", &self.price)?;

        s.serialize_field("object", "product_catalog.trial_offer")?;
        s.end()
    }
}
impl stripe_types::Object for ProductCatalogTrialOffer {
    type Id = stripe_product::ProductCatalogTrialOfferId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ProductCatalogTrialOfferId);
