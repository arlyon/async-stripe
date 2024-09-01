/// Usage records allow you to report customer usage and metrics to Stripe for
/// metered billing of subscription prices.
///
/// Related guide: [Metered billing](https://stripe.com/docs/billing/subscriptions/metered-billing)
///
/// This is our legacy usage-based billing API.
/// See the [updated usage-based billing docs](https://docs.stripe.com/billing/subscriptions/usage-based).
///
/// For more details see <<https://stripe.com/docs/api/usage_records/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct UsageRecord {
    /// Unique identifier for the object.
    pub id: stripe_billing::UsageRecordId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The usage quantity for the specified date.
    pub quantity: u64,
    /// The ID of the subscription item this usage record contains data for.
    pub subscription_item: String,
    /// The timestamp when this usage occurred.
    pub timestamp: stripe_types::Timestamp,
}
#[doc(hidden)]
pub struct UsageRecordBuilder {
    id: Option<stripe_billing::UsageRecordId>,
    livemode: Option<bool>,
    quantity: Option<u64>,
    subscription_item: Option<String>,
    timestamp: Option<stripe_types::Timestamp>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for UsageRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<UsageRecord>,
        builder: UsageRecordBuilder,
    }

    impl Visitor for Place<UsageRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: UsageRecordBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for UsageRecordBuilder {
        type Out = UsageRecord;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "quantity" => Deserialize::begin(&mut self.quantity),
                "subscription_item" => Deserialize::begin(&mut self.subscription_item),
                "timestamp" => Deserialize::begin(&mut self.timestamp),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                quantity: Deserialize::default(),
                subscription_item: Deserialize::default(),
                timestamp: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(id),
                Some(livemode),
                Some(quantity),
                Some(subscription_item),
                Some(timestamp),
            ) = (
                self.id.take(),
                self.livemode,
                self.quantity,
                self.subscription_item.take(),
                self.timestamp,
            )
            else {
                return None;
            };
            Some(Self::Out { id, livemode, quantity, subscription_item, timestamp })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for UsageRecord {
        type Builder = UsageRecordBuilder;
    }

    impl FromValueOpt for UsageRecord {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = UsageRecordBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "quantity" => b.quantity = FromValueOpt::from_value(v),
                    "subscription_item" => b.subscription_item = FromValueOpt::from_value(v),
                    "timestamp" => b.timestamp = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for UsageRecord {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("UsageRecord", 6)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("quantity", &self.quantity)?;
        s.serialize_field("subscription_item", &self.subscription_item)?;
        s.serialize_field("timestamp", &self.timestamp)?;

        s.serialize_field("object", "usage_record")?;
        s.end()
    }
}
impl stripe_types::Object for UsageRecord {
    type Id = stripe_billing::UsageRecordId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(UsageRecordId);
