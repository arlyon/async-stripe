#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCardNetworkToken {
    /// Indicates if Stripe used a network token, either user provided or Stripe managed when processing the transaction.
    pub used: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsCardNetworkToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsCardNetworkToken").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsCardNetworkTokenBuilder {
    used: Option<bool>,
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

    impl Deserialize for PaymentMethodDetailsCardNetworkToken {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCardNetworkToken>,
        builder: PaymentMethodDetailsCardNetworkTokenBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCardNetworkToken> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsCardNetworkTokenBuilder {
                    used: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "used" => Deserialize::begin(&mut self.builder.used),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(used),) = (self.builder.used,) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsCardNetworkToken { used });
            Ok(())
        }
    }
};
