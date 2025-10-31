#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsAmountDetailsResourceTax {
    /// The total amount of tax on the transaction represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// Required for L2 rates.
    /// An integer greater than or equal to 0.
    ///
    /// This field is mutually exclusive with the `amount_details[line_items][#][tax][total_tax_amount]` field.
    pub total_tax_amount: Option<i64>,
}
#[doc(hidden)]
pub struct PaymentFlowsAmountDetailsResourceTaxBuilder {
    total_tax_amount: Option<Option<i64>>,
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

    impl Deserialize for PaymentFlowsAmountDetailsResourceTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsAmountDetailsResourceTax>,
        builder: PaymentFlowsAmountDetailsResourceTaxBuilder,
    }

    impl Visitor for Place<PaymentFlowsAmountDetailsResourceTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsAmountDetailsResourceTaxBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentFlowsAmountDetailsResourceTaxBuilder {
        type Out = PaymentFlowsAmountDetailsResourceTax;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "total_tax_amount" => Deserialize::begin(&mut self.total_tax_amount),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { total_tax_amount: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(total_tax_amount),) = (self.total_tax_amount,) else {
                return None;
            };
            Some(Self::Out { total_tax_amount })
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

    impl ObjectDeser for PaymentFlowsAmountDetailsResourceTax {
        type Builder = PaymentFlowsAmountDetailsResourceTaxBuilder;
    }

    impl FromValueOpt for PaymentFlowsAmountDetailsResourceTax {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsAmountDetailsResourceTaxBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "total_tax_amount" => b.total_tax_amount = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
