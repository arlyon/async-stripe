#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupAttemptPaymentMethodDetailsNaverPay {
    /// Uniquely identifies this particular Naver Pay account.
    /// You can use this attribute to check whether two Naver Pay accounts are the same.
    pub buyer_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupAttemptPaymentMethodDetailsNaverPay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupAttemptPaymentMethodDetailsNaverPay").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupAttemptPaymentMethodDetailsNaverPayBuilder {
    buyer_id: Option<Option<String>>,
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

    impl Deserialize for SetupAttemptPaymentMethodDetailsNaverPay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupAttemptPaymentMethodDetailsNaverPay>,
        builder: SetupAttemptPaymentMethodDetailsNaverPayBuilder,
    }

    impl Visitor for Place<SetupAttemptPaymentMethodDetailsNaverPay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupAttemptPaymentMethodDetailsNaverPayBuilder {
                    buyer_id: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "buyer_id" => Deserialize::begin(&mut self.builder.buyer_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(buyer_id),) = (self.builder.buyer_id.take(),) else {
                return Ok(());
            };
            *self.out = Some(SetupAttemptPaymentMethodDetailsNaverPay { buyer_id });
            Ok(())
        }
    }
};
