/// A summary of a customer's active entitlements.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct EntitlementsActiveEntitlementSummary {
    /// The customer that is entitled to this feature.
    pub customer: String,
    /// The list of entitlements this customer has.
    pub entitlements: stripe_types::List<stripe_misc::EntitlementsActiveEntitlement>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for EntitlementsActiveEntitlementSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("EntitlementsActiveEntitlementSummary").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: EntitlementsActiveEntitlementSummaryBuilder {
                    customer: Deserialize::default(),
                    entitlements: Deserialize::default(),
                    livemode: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer" => Deserialize::begin(&mut self.builder.customer),
                "entitlements" => Deserialize::begin(&mut self.builder.entitlements),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(customer), Some(entitlements), Some(livemode)) = (
                self.builder.customer.take(),
                self.builder.entitlements.take(),
                self.builder.livemode,
            ) else {
                return Ok(());
            };
            *self.out =
                Some(EntitlementsActiveEntitlementSummary { customer, entitlements, livemode });
            Ok(())
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
