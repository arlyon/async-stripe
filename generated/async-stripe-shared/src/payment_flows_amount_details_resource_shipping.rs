#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsAmountDetailsResourceShipping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsAmountDetailsResourceShipping").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentFlowsAmountDetailsResourceShippingBuilder {
                    amount: Deserialize::default(),
                    from_postal_code: Deserialize::default(),
                    to_postal_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "from_postal_code" => Deserialize::begin(&mut self.builder.from_postal_code),
                "to_postal_code" => Deserialize::begin(&mut self.builder.to_postal_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(from_postal_code), Some(to_postal_code)) = (
                self.builder.amount,
                self.builder.from_postal_code.take(),
                self.builder.to_postal_code.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentFlowsAmountDetailsResourceShipping {
                amount,
                from_postal_code,
                to_postal_code,
            });
            Ok(())
        }
    }
};
