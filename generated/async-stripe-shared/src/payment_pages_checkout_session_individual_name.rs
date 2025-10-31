#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionIndividualName {
    /// Indicates whether individual name collection is enabled for the session
    pub enabled: bool,
    /// Whether the customer is required to complete the field before completing the Checkout Session.
    /// Defaults to `false`.
    pub optional: bool,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionIndividualNameBuilder {
    enabled: Option<bool>,
    optional: Option<bool>,
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

    impl Deserialize for PaymentPagesCheckoutSessionIndividualName {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionIndividualName>,
        builder: PaymentPagesCheckoutSessionIndividualNameBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionIndividualName> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionIndividualNameBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionIndividualNameBuilder {
        type Out = PaymentPagesCheckoutSessionIndividualName;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.enabled),
                "optional" => Deserialize::begin(&mut self.optional),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { enabled: Deserialize::default(), optional: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(enabled), Some(optional)) = (self.enabled, self.optional) else {
                return None;
            };
            Some(Self::Out { enabled, optional })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionIndividualName {
        type Builder = PaymentPagesCheckoutSessionIndividualNameBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionIndividualName {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionIndividualNameBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    "optional" => b.optional = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
