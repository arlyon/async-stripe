#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsAmountDetailsResourceShipping {
    /// If a physical good is being shipped, the cost of shipping represented in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    /// An integer greater than or equal to 0.
    pub amount: Option<i64>,
    /// If a physical good is being shipped, the postal code of where it is being shipped from.
    /// At most 10 alphanumeric characters long, hyphens are allowed.
    pub from_postal_code: Option<String>,
    /// If a physical good is being shipped, the postal code of where it is being shipped to.
    /// At most 10 alphanumeric characters long, hyphens are allowed.
    pub to_postal_code: Option<String>,
}
#[doc(hidden)]
pub struct PaymentFlowsAmountDetailsResourceShippingBuilder {
    amount: Option<Option<i64>>,
    from_postal_code: Option<Option<String>>,
    to_postal_code: Option<Option<String>>,
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

    impl Deserialize for PaymentFlowsAmountDetailsResourceShipping {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsAmountDetailsResourceShipping>,
        builder: PaymentFlowsAmountDetailsResourceShippingBuilder,
    }

    impl Visitor for Place<PaymentFlowsAmountDetailsResourceShipping> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsAmountDetailsResourceShippingBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentFlowsAmountDetailsResourceShippingBuilder {
        type Out = PaymentFlowsAmountDetailsResourceShipping;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "from_postal_code" => Deserialize::begin(&mut self.from_postal_code),
                "to_postal_code" => Deserialize::begin(&mut self.to_postal_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                from_postal_code: Deserialize::default(),
                to_postal_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(from_postal_code), Some(to_postal_code)) =
                (self.amount, self.from_postal_code.take(), self.to_postal_code.take())
            else {
                return None;
            };
            Some(Self::Out { amount, from_postal_code, to_postal_code })
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

    impl ObjectDeser for PaymentFlowsAmountDetailsResourceShipping {
        type Builder = PaymentFlowsAmountDetailsResourceShippingBuilder;
    }

    impl FromValueOpt for PaymentFlowsAmountDetailsResourceShipping {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsAmountDetailsResourceShippingBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "from_postal_code" => b.from_postal_code = FromValueOpt::from_value(v),
                    "to_postal_code" => b.to_postal_code = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
