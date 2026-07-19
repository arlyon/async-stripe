#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodKlarna {
    /// The customer's date of birth, if provided.
    pub dob: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsKlarnaDob>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodKlarna").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodKlarnaBuilder {
    dob: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsKlarnaDob>>,
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

    impl Deserialize for PaymentMethodKlarna {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodKlarna>,
        builder: PaymentMethodKlarnaBuilder,
    }

    impl Visitor for Place<PaymentMethodKlarna> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodKlarnaBuilder { dob: Deserialize::default() },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "dob" => Deserialize::begin(&mut self.builder.dob),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(dob),) = (self.builder.dob,) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodKlarna { dob });
            Ok(())
        }
    }
};
