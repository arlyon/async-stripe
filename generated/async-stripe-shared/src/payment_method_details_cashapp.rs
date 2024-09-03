#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCashapp {
    /// A unique and immutable identifier assigned by Cash App to every buyer.
    pub buyer_id: Option<String>,
    /// A public identifier for buyers using Cash App.
    pub cashtag: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsCashappBuilder {
    buyer_id: Option<Option<String>>,
    cashtag: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsCashapp {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCashapp>,
        builder: PaymentMethodDetailsCashappBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCashapp> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsCashappBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsCashappBuilder {
        type Out = PaymentMethodDetailsCashapp;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "buyer_id" => Deserialize::begin(&mut self.buyer_id),
                "cashtag" => Deserialize::begin(&mut self.cashtag),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { buyer_id: Deserialize::default(), cashtag: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(buyer_id), Some(cashtag)) = (self.buyer_id.take(), self.cashtag.take())
            else {
                return None;
            };
            Some(Self::Out { buyer_id, cashtag })
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

    impl ObjectDeser for PaymentMethodDetailsCashapp {
        type Builder = PaymentMethodDetailsCashappBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsCashapp {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsCashappBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "buyer_id" => b.buyer_id = FromValueOpt::from_value(v),
                    "cashtag" => b.cashtag = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
