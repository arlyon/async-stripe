#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsUsBankAccount {
    pub financial_connections: Option<stripe_shared::LinkedAccountOptionsCommon>,
    pub mandate_options: Option<stripe_shared::PaymentMethodOptionsUsBankAccountMandateOptions>,
    /// Bank account verification method.
    pub verification_method: Option<SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsUsBankAccountBuilder {
    financial_connections: Option<Option<stripe_shared::LinkedAccountOptionsCommon>>,
    mandate_options: Option<Option<stripe_shared::PaymentMethodOptionsUsBankAccountMandateOptions>>,
    verification_method:
        Option<Option<SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>>,
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

    impl Deserialize for SetupIntentPaymentMethodOptionsUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsUsBankAccount>,
        builder: SetupIntentPaymentMethodOptionsUsBankAccountBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentPaymentMethodOptionsUsBankAccountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsUsBankAccountBuilder {
        type Out = SetupIntentPaymentMethodOptionsUsBankAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "financial_connections" => Deserialize::begin(&mut self.financial_connections),
                "mandate_options" => Deserialize::begin(&mut self.mandate_options),
                "verification_method" => Deserialize::begin(&mut self.verification_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                financial_connections: Deserialize::default(),
                mandate_options: Deserialize::default(),
                verification_method: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(financial_connections), Some(mandate_options), Some(verification_method)) =
                (self.financial_connections.take(), self.mandate_options, self.verification_method)
            else {
                return None;
            };
            Some(Self::Out { financial_connections, mandate_options, verification_method })
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

    impl ObjectDeser for SetupIntentPaymentMethodOptionsUsBankAccount {
        type Builder = SetupIntentPaymentMethodOptionsUsBankAccountBuilder;
    }

    impl FromValueOpt for SetupIntentPaymentMethodOptionsUsBankAccount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupIntentPaymentMethodOptionsUsBankAccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "financial_connections" => {
                        b.financial_connections = FromValueOpt::from_value(v)
                    }
                    "mandate_options" => b.mandate_options = FromValueOpt::from_value(v),
                    "verification_method" => b.verification_method = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Bank account verification method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod",
            )
        })
    }
}
