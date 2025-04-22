#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceCustomTextPosition {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}
#[doc(hidden)]
pub struct PaymentLinksResourceCustomTextPositionBuilder {
    message: Option<String>,
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

    impl Deserialize for PaymentLinksResourceCustomTextPosition {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCustomTextPosition>,
        builder: PaymentLinksResourceCustomTextPositionBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCustomTextPosition> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceCustomTextPositionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceCustomTextPositionBuilder {
        type Out = PaymentLinksResourceCustomTextPosition;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "message" => Deserialize::begin(&mut self.message),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { message: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(message),) = (self.message.take(),) else {
                return None;
            };
            Some(Self::Out { message })
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

    impl ObjectDeser for PaymentLinksResourceCustomTextPosition {
        type Builder = PaymentLinksResourceCustomTextPositionBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceCustomTextPosition {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceCustomTextPositionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "message" => b.message = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
