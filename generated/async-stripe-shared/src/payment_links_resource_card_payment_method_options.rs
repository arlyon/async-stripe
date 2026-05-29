#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceCardPaymentMethodOptions {
    /// Restrictions to apply to the card payment method. For example, you can block specific card brands.
    pub restrictions: Option<stripe_shared::PaymentLinksResourceCardRestrictions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceCardPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceCardPaymentMethodOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceCardPaymentMethodOptionsBuilder {
    restrictions: Option<Option<stripe_shared::PaymentLinksResourceCardRestrictions>>,
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

    impl Deserialize for PaymentLinksResourceCardPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCardPaymentMethodOptions>,
        builder: PaymentLinksResourceCardPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCardPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceCardPaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceCardPaymentMethodOptionsBuilder {
        type Out = PaymentLinksResourceCardPaymentMethodOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "restrictions" => Deserialize::begin(&mut self.restrictions),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { restrictions: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(restrictions),) = (self.restrictions.take(),) else {
                return None;
            };
            Some(Self::Out { restrictions })
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

    impl ObjectDeser for PaymentLinksResourceCardPaymentMethodOptions {
        type Builder = PaymentLinksResourceCardPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceCardPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceCardPaymentMethodOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "restrictions" => b.restrictions = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
