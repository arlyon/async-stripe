#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCardNetworkToken {
    /// Indicates if Stripe used a network token, either user provided or Stripe managed when processing the transaction.
    pub used: bool,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsCardNetworkTokenBuilder {
    used: Option<bool>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsCardNetworkToken {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCardNetworkToken>,
        builder: PaymentMethodDetailsCardNetworkTokenBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCardNetworkToken> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsCardNetworkTokenBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsCardNetworkTokenBuilder {
        type Out = PaymentMethodDetailsCardNetworkToken;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "used" => Deserialize::begin(&mut self.used),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { used: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { used: self.used? })
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

    impl ObjectDeser for PaymentMethodDetailsCardNetworkToken {
        type Builder = PaymentMethodDetailsCardNetworkTokenBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsCardNetworkToken {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsCardNetworkTokenBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "used" => b.used = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
