#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputePaymentMethodDetailsPaypal {
    /// The ID of the dispute in PayPal.
    pub case_id: Option<String>,
    /// The reason for the dispute as defined by PayPal
    pub reason_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputePaymentMethodDetailsPaypal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisputePaymentMethodDetailsPaypal").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct DisputePaymentMethodDetailsPaypalBuilder {
    case_id: Option<Option<String>>,
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

    impl Deserialize for DisputePaymentMethodDetailsPaypal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputePaymentMethodDetailsPaypal>,
        builder: DisputePaymentMethodDetailsPaypalBuilder,
    }

    impl Visitor for Place<DisputePaymentMethodDetailsPaypal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputePaymentMethodDetailsPaypalBuilder {
                    case_id: Deserialize::default(),
                    reason_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "case_id" => Deserialize::begin(&mut self.builder.case_id),
                "reason_code" => Deserialize::begin(&mut self.builder.reason_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(case_id), Some(reason_code)) =
                (self.builder.case_id.take(), self.builder.reason_code.take())
            else {
                return Ok(());
            };
            *self.out = Some(DisputePaymentMethodDetailsPaypal { case_id, reason_code });
            Ok(())
        }
    }
};
