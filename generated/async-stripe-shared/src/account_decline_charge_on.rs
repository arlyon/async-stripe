#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountDeclineChargeOn {
    /// Whether Stripe automatically declines charges with an incorrect ZIP or postal code.
    /// This setting only applies when a ZIP or postal code is provided and they fail bank verification.
    pub avs_failure: bool,
    /// Whether Stripe automatically declines charges with an incorrect CVC.
    /// This setting only applies when a CVC is provided and it fails bank verification.
    pub cvc_failure: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountDeclineChargeOn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountDeclineChargeOn").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountDeclineChargeOnBuilder {
    avs_failure: Option<bool>,
    cvc_failure: Option<bool>,
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

    impl Deserialize for AccountDeclineChargeOn {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountDeclineChargeOn>,
        builder: AccountDeclineChargeOnBuilder,
    }

    impl Visitor for Place<AccountDeclineChargeOn> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountDeclineChargeOnBuilder {
                    avs_failure: Deserialize::default(),
                    cvc_failure: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "avs_failure" => Deserialize::begin(&mut self.builder.avs_failure),
                "cvc_failure" => Deserialize::begin(&mut self.builder.cvc_failure),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(avs_failure), Some(cvc_failure)) =
                (self.builder.avs_failure, self.builder.cvc_failure)
            else {
                return Ok(());
            };
            *self.out = Some(AccountDeclineChargeOn { avs_failure, cvc_failure });
            Ok(())
        }
    }
};
