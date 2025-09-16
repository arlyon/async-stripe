/// A summary of a customer's active entitlements.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct EntitlementsActiveEntitlementSummary {
    /// The customer that is entitled to this feature.
    pub customer: String,
    /// The list of entitlements this customer has.
    pub entitlements: stripe_types::List<stripe_misc::EntitlementsActiveEntitlement>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
#[doc(hidden)]
pub struct EntitlementsActiveEntitlementSummaryBuilder {
    customer: Option<String>,
    entitlements: Option<stripe_types::List<stripe_misc::EntitlementsActiveEntitlement>>,
    livemode: Option<bool>,
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

    impl Deserialize for EntitlementsActiveEntitlementSummary {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<EntitlementsActiveEntitlementSummary>,
        builder: EntitlementsActiveEntitlementSummaryBuilder,
    }

    impl Visitor for Place<EntitlementsActiveEntitlementSummary> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: EntitlementsActiveEntitlementSummaryBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for EntitlementsActiveEntitlementSummaryBuilder {
        type Out = EntitlementsActiveEntitlementSummary;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer" => Deserialize::begin(&mut self.customer),
                "entitlements" => Deserialize::begin(&mut self.entitlements),
                "livemode" => Deserialize::begin(&mut self.livemode),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                customer: Deserialize::default(),
                entitlements: Deserialize::default(),
                livemode: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(customer), Some(entitlements), Some(livemode)) =
                (self.customer.take(), self.entitlements.take(), self.livemode)
            else {
                return None;
            };
            Some(Self::Out { customer, entitlements, livemode })
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

    impl ObjectDeser for EntitlementsActiveEntitlementSummary {
        type Builder = EntitlementsActiveEntitlementSummaryBuilder;
    }

    impl FromValueOpt for EntitlementsActiveEntitlementSummary {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = EntitlementsActiveEntitlementSummaryBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "entitlements" => b.entitlements = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for EntitlementsActiveEntitlementSummary {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("EntitlementsActiveEntitlementSummary", 4)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("entitlements", &self.entitlements)?;
        s.serialize_field("livemode", &self.livemode)?;

        s.serialize_field("object", "entitlements.active_entitlement_summary")?;
        s.end()
    }
}
