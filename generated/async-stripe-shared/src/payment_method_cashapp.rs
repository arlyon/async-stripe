#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodCashapp {
    /// A unique and immutable identifier assigned by Cash App to every buyer.
    pub buyer_id: Option<String>,
    /// A public identifier for buyers using Cash App.
    pub cashtag: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodCashapp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodCashapp").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodCashappBuilder {
    buyer_id: Option<Option<String>>,
    cashtag: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodCashapp {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodCashapp>,
        builder: PaymentMethodCashappBuilder,
    }

    impl Visitor for Place<PaymentMethodCashapp> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodCashappBuilder {
                    buyer_id: Deserialize::default(),
                    cashtag: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "buyer_id" => Deserialize::begin(&mut self.builder.buyer_id),
                "cashtag" => Deserialize::begin(&mut self.builder.cashtag),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(buyer_id), Some(cashtag)) =
                (self.builder.buyer_id.take(), self.builder.cashtag.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodCashapp { buyer_id, cashtag });
            Ok(())
        }
    }
};
