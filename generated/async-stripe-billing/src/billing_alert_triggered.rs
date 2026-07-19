#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingAlertTriggered {
    pub alert: stripe_billing::BillingAlert,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// ID of customer for which the alert triggered
    pub customer: String,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The value triggering the alert
    pub value: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingAlertTriggered {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingAlertTriggered").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingAlertTriggeredBuilder {
    alert: Option<stripe_billing::BillingAlert>,
    created: Option<stripe_types::Timestamp>,
    customer: Option<String>,
    livemode: Option<bool>,
    value: Option<i64>,
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

    impl Deserialize for BillingAlertTriggered {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingAlertTriggered>,
        builder: BillingAlertTriggeredBuilder,
    }

    impl Visitor for Place<BillingAlertTriggered> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingAlertTriggeredBuilder {
                    alert: Deserialize::default(),
                    created: Deserialize::default(),
                    customer: Deserialize::default(),
                    livemode: Deserialize::default(),
                    value: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "alert" => Deserialize::begin(&mut self.builder.alert),
                "created" => Deserialize::begin(&mut self.builder.created),
                "customer" => Deserialize::begin(&mut self.builder.customer),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "value" => Deserialize::begin(&mut self.builder.value),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(alert), Some(created), Some(customer), Some(livemode), Some(value)) = (
                self.builder.alert.take(),
                self.builder.created,
                self.builder.customer.take(),
                self.builder.livemode,
                self.builder.value,
            ) else {
                return Ok(());
            };
            *self.out = Some(BillingAlertTriggered { alert, created, customer, livemode, value });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingAlertTriggered {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BillingAlertTriggered", 6)?;
        s.serialize_field("alert", &self.alert)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("value", &self.value)?;

        s.serialize_field("object", "billing.alert_triggered")?;
        s.end()
    }
}
