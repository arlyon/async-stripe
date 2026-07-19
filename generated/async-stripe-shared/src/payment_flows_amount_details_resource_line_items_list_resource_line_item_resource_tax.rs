#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourceTax {
    /// The total amount of tax on the transaction represented in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    /// Required for L2 rates.
    /// An integer greater than or equal to 0.
    ///
    /// This field is mutually exclusive with the `amount_details[line_items][#][tax][total_tax_amount]` field.
    pub total_tax_amount: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourceTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourceTax")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourceTaxBuilder {
    total_tax_amount: Option<i64>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourceTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourceTax,
        >,
        builder: PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourceTaxBuilder,
    }

    impl Visitor for Place<PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourceTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourceTaxBuilder { total_tax_amount: Deserialize::default(),
 },
        }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "total_tax_amount" => Deserialize::begin(&mut self.builder.total_tax_amount),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(total_tax_amount),) = (self.builder.total_tax_amount,) else {
                return Ok(());
            };
            *self.out =
                Some(PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourceTax {
                    total_tax_amount,
                });
            Ok(())
        }
    }
};
