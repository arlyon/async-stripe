#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
#[doc(hidden)]
pub struct AccountDeclineChargeOnBuilder {
    avs_failure: Option<bool>,
    cvc_failure: Option<bool>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: AccountDeclineChargeOnBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountDeclineChargeOnBuilder {
        type Out = AccountDeclineChargeOn;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "avs_failure" => Deserialize::begin(&mut self.avs_failure),
                "cvc_failure" => Deserialize::begin(&mut self.cvc_failure),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { avs_failure: Deserialize::default(), cvc_failure: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(avs_failure), Some(cvc_failure)) = (self.avs_failure, self.cvc_failure)
            else {
                return None;
            };
            Some(Self::Out { avs_failure, cvc_failure })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for AccountDeclineChargeOn {
        type Builder = AccountDeclineChargeOnBuilder;
    }

    impl FromValueOpt for AccountDeclineChargeOn {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountDeclineChargeOnBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "avs_failure" => b.avs_failure = FromValueOpt::from_value(v),
                    "cvc_failure" => b.cvc_failure = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
