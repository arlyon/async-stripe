#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentNextActionVerifyWithMicrodeposits {
    /// The timestamp when the microdeposits are expected to land.
    pub arrival_date: stripe_types::Timestamp,
    /// The URL for the hosted verification page, which allows customers to verify their bank account.
    pub hosted_verification_url: String,
    /// The type of the microdeposit sent to the customer.
    /// Used to distinguish between different verification methods.
    pub microdeposit_type: Option<stripe_shared::MicrodepositType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentNextActionVerifyWithMicrodeposits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentNextActionVerifyWithMicrodeposits").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupIntentNextActionVerifyWithMicrodepositsBuilder {
    arrival_date: Option<stripe_types::Timestamp>,
    hosted_verification_url: Option<String>,
    microdeposit_type: Option<Option<stripe_shared::MicrodepositType>>,
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

    impl Deserialize for SetupIntentNextActionVerifyWithMicrodeposits {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentNextActionVerifyWithMicrodeposits>,
        builder: SetupIntentNextActionVerifyWithMicrodepositsBuilder,
    }

    impl Visitor for Place<SetupIntentNextActionVerifyWithMicrodeposits> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentNextActionVerifyWithMicrodepositsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentNextActionVerifyWithMicrodepositsBuilder {
        type Out = SetupIntentNextActionVerifyWithMicrodeposits;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "arrival_date" => Deserialize::begin(&mut self.arrival_date),
                "hosted_verification_url" => Deserialize::begin(&mut self.hosted_verification_url),
                "microdeposit_type" => Deserialize::begin(&mut self.microdeposit_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                arrival_date: None,
                hosted_verification_url: None,
                microdeposit_type: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(arrival_date), Some(hosted_verification_url), Some(microdeposit_type)) = (
                self.arrival_date,
                self.hosted_verification_url.take(),
                self.microdeposit_type.take(),
            ) else {
                return None;
            };
            Some(Self::Out { arrival_date, hosted_verification_url, microdeposit_type })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for SetupIntentNextActionVerifyWithMicrodeposits {
        type Builder = SetupIntentNextActionVerifyWithMicrodepositsBuilder;
    }

    impl FromValueOpt for SetupIntentNextActionVerifyWithMicrodeposits {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupIntentNextActionVerifyWithMicrodepositsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "arrival_date" => b.arrival_date = FromValueOpt::from_value(v),
                    "hosted_verification_url" => {
                        b.hosted_verification_url = FromValueOpt::from_value(v)
                    }
                    "microdeposit_type" => b.microdeposit_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of the microdeposit sent to the customer.
/// Used to distinguish between different verification methods.
pub type SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType =
    stripe_shared::MicrodepositType;
