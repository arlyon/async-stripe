#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodPayto {
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// The PayID alias for the bank account.
    pub pay_id: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodPaytoBuilder {
    bsb_number: Option<Option<String>>,
    last4: Option<Option<String>>,
    pay_id: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodPayto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodPayto>,
        builder: PaymentMethodPaytoBuilder,
    }

    impl Visitor for Place<PaymentMethodPayto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodPaytoBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodPaytoBuilder {
        type Out = PaymentMethodPayto;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bsb_number" => Deserialize::begin(&mut self.bsb_number),
                "last4" => Deserialize::begin(&mut self.last4),
                "pay_id" => Deserialize::begin(&mut self.pay_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bsb_number: Deserialize::default(),
                last4: Deserialize::default(),
                pay_id: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bsb_number), Some(last4), Some(pay_id)) =
                (self.bsb_number.take(), self.last4.take(), self.pay_id.take())
            else {
                return None;
            };
            Some(Self::Out { bsb_number, last4, pay_id })
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

    impl ObjectDeser for PaymentMethodPayto {
        type Builder = PaymentMethodPaytoBuilder;
    }

    impl FromValueOpt for PaymentMethodPayto {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodPaytoBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bsb_number" => b.bsb_number = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "pay_id" => b.pay_id = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
