#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThresholdsResourceUsageAlertFilter {
    /// Limit the scope of the alert to this customer ID
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::ThresholdsResourceUsageAlertFilterType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThresholdsResourceUsageAlertFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ThresholdsResourceUsageAlertFilter").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ThresholdsResourceUsageAlertFilterBuilder {
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    type_: Option<stripe_shared::ThresholdsResourceUsageAlertFilterType>,
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

    impl Deserialize for ThresholdsResourceUsageAlertFilter {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThresholdsResourceUsageAlertFilter>,
        builder: ThresholdsResourceUsageAlertFilterBuilder,
    }

    impl Visitor for Place<ThresholdsResourceUsageAlertFilter> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThresholdsResourceUsageAlertFilterBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThresholdsResourceUsageAlertFilterBuilder {
        type Out = ThresholdsResourceUsageAlertFilter;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer" => Deserialize::begin(&mut self.customer),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { customer: Some(None), type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(customer), Some(type_)) = (self.customer.take(), self.type_.take()) else {
                return None;
            };
            Some(Self::Out { customer, type_ })
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

    impl ObjectDeser for ThresholdsResourceUsageAlertFilter {
        type Builder = ThresholdsResourceUsageAlertFilterBuilder;
    }

    impl FromValueOpt for ThresholdsResourceUsageAlertFilter {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThresholdsResourceUsageAlertFilterBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
pub type ThresholdsResourceUsageAlertFilterType =
    stripe_shared::ThresholdsResourceUsageAlertFilterType;
