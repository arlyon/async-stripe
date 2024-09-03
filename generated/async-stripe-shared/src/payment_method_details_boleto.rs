#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsBoleto {
    /// The tax ID of the customer (CPF for individuals consumers or CNPJ for businesses consumers)
    pub tax_id: String,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsBoletoBuilder {
    tax_id: Option<String>,
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

    impl Deserialize for PaymentMethodDetailsBoleto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsBoleto>,
        builder: PaymentMethodDetailsBoletoBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsBoleto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsBoletoBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsBoletoBuilder {
        type Out = PaymentMethodDetailsBoleto;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "tax_id" => Deserialize::begin(&mut self.tax_id),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { tax_id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(tax_id),) = (self.tax_id.take(),) else {
                return None;
            };
            Some(Self::Out { tax_id })
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

    impl ObjectDeser for PaymentMethodDetailsBoleto {
        type Builder = PaymentMethodDetailsBoletoBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsBoleto {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsBoletoBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "tax_id" => b.tax_id = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
