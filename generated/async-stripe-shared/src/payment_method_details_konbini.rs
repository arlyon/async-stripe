#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsKonbini {
    /// If the payment succeeded, this contains the details of the convenience store where the payment was completed.
    pub store: Option<stripe_shared::PaymentMethodDetailsKonbiniStore>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsKonbini {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsKonbini").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsKonbiniBuilder {
    store: Option<Option<stripe_shared::PaymentMethodDetailsKonbiniStore>>,
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

    impl Deserialize for PaymentMethodDetailsKonbini {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsKonbini>,
        builder: PaymentMethodDetailsKonbiniBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsKonbini> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsKonbiniBuilder { store: Deserialize::default() },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "store" => Deserialize::begin(&mut self.builder.store),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(store),) = (self.builder.store.take(),) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsKonbini { store });
            Ok(())
        }
    }
};
