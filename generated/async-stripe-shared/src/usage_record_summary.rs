#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct UsageRecordSummary {
    /// Unique identifier for the object.
    pub id: stripe_shared::UsageRecordSummaryId,
    /// The invoice in which this usage period has been billed for.
    pub invoice: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub period: stripe_shared::Period,
    /// The ID of the subscription item this summary is describing.
    pub subscription_item: String,
    /// The total usage within this usage period.
    pub total_usage: i64,
}
#[doc(hidden)]
pub struct UsageRecordSummaryBuilder {
    id: Option<stripe_shared::UsageRecordSummaryId>,
    invoice: Option<Option<String>>,
    livemode: Option<bool>,
    period: Option<stripe_shared::Period>,
    subscription_item: Option<String>,
    total_usage: Option<i64>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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

    impl Deserialize for UsageRecordSummary {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<UsageRecordSummary>,
        builder: UsageRecordSummaryBuilder,
    }

    impl Visitor for Place<UsageRecordSummary> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: UsageRecordSummaryBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for UsageRecordSummaryBuilder {
        type Out = UsageRecordSummary;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.id),
                "invoice" => Deserialize::begin(&mut self.invoice),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "period" => Deserialize::begin(&mut self.period),
                "subscription_item" => Deserialize::begin(&mut self.subscription_item),
                "total_usage" => Deserialize::begin(&mut self.total_usage),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                id: Deserialize::default(),
                invoice: Deserialize::default(),
                livemode: Deserialize::default(),
                period: Deserialize::default(),
                subscription_item: Deserialize::default(),
                total_usage: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(id),
                Some(invoice),
                Some(livemode),
                Some(period),
                Some(subscription_item),
                Some(total_usage),
            ) = (
                self.id.take(),
                self.invoice.take(),
                self.livemode,
                self.period,
                self.subscription_item.take(),
                self.total_usage,
            )
            else {
                return None;
            };
            Some(Self::Out { id, invoice, livemode, period, subscription_item, total_usage })
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

    impl ObjectDeser for UsageRecordSummary {
        type Builder = UsageRecordSummaryBuilder;
    }

    impl FromValueOpt for UsageRecordSummary {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = UsageRecordSummaryBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "id" => b.id = FromValueOpt::from_value(v),
                    "invoice" => b.invoice = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "period" => b.period = FromValueOpt::from_value(v),
                    "subscription_item" => b.subscription_item = FromValueOpt::from_value(v),
                    "total_usage" => b.total_usage = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for UsageRecordSummary {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("UsageRecordSummary", 7)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("invoice", &self.invoice)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("period", &self.period)?;
        s.serialize_field("subscription_item", &self.subscription_item)?;
        s.serialize_field("total_usage", &self.total_usage)?;

        s.serialize_field("object", "usage_record_summary")?;
        s.end()
    }
}
impl stripe_types::Object for UsageRecordSummary {
    type Id = stripe_shared::UsageRecordSummaryId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(UsageRecordSummaryId);
