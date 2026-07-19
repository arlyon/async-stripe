#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsAfterpayClearpay {
    /// The Afterpay order ID associated with this payment intent.
    pub order_id: Option<String>,
    /// Order identifier shown to the merchant in Afterpay’s online portal.
    pub reference: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsAfterpayClearpay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsAfterpayClearpay").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsAfterpayClearpayBuilder {
    order_id: Option<Option<String>>,
    reference: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsAfterpayClearpay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsAfterpayClearpay>,
        builder: PaymentMethodDetailsAfterpayClearpayBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsAfterpayClearpay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsAfterpayClearpayBuilder {
                    order_id: Deserialize::default(),
                    reference: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "order_id" => Deserialize::begin(&mut self.builder.order_id),
                "reference" => Deserialize::begin(&mut self.builder.reference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(order_id), Some(reference)) =
                (self.builder.order_id.take(), self.builder.reference.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsAfterpayClearpay { order_id, reference });
            Ok(())
        }
    }
};
