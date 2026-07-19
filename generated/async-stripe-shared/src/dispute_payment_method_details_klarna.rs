#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputePaymentMethodDetailsKlarna {
    /// Chargeback loss reason mapped by Stripe from Klarna's chargeback loss reason
    pub chargeback_loss_reason_code: Option<String>,
    /// The reason for the dispute as defined by Klarna
    pub reason_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputePaymentMethodDetailsKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisputePaymentMethodDetailsKlarna").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct DisputePaymentMethodDetailsKlarnaBuilder {
    chargeback_loss_reason_code: Option<Option<String>>,
    reason_code: Option<Option<String>>,
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

    impl Deserialize for DisputePaymentMethodDetailsKlarna {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputePaymentMethodDetailsKlarna>,
        builder: DisputePaymentMethodDetailsKlarnaBuilder,
    }

    impl Visitor for Place<DisputePaymentMethodDetailsKlarna> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputePaymentMethodDetailsKlarnaBuilder {
                    chargeback_loss_reason_code: Deserialize::default(),
                    reason_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "chargeback_loss_reason_code" => {
                    Deserialize::begin(&mut self.builder.chargeback_loss_reason_code)
                }
                "reason_code" => Deserialize::begin(&mut self.builder.reason_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(chargeback_loss_reason_code), Some(reason_code)) =
                (self.builder.chargeback_loss_reason_code.take(), self.builder.reason_code.take())
            else {
                return Ok(());
            };
            *self.out = Some(DisputePaymentMethodDetailsKlarna {
                chargeback_loss_reason_code,
                reason_code,
            });
            Ok(())
        }
    }
};
