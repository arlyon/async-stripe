#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountUnificationAccountControllerFees {
    /// A value indicating the responsible payer of a bundle of Stripe fees for pricing-control eligible products on this account.
    /// Learn more about [fee behavior on connected accounts](https://docs.stripe.com/connect/direct-charges-fee-payer-behavior).
    pub payer: AccountUnificationAccountControllerFeesPayer,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountUnificationAccountControllerFees {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountUnificationAccountControllerFees").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountUnificationAccountControllerFeesBuilder {
    payer: Option<AccountUnificationAccountControllerFeesPayer>,
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

    impl Deserialize for AccountUnificationAccountControllerFees {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountUnificationAccountControllerFees>,
        builder: AccountUnificationAccountControllerFeesBuilder,
    }

    impl Visitor for Place<AccountUnificationAccountControllerFees> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountUnificationAccountControllerFeesBuilder {
                    payer: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payer" => Deserialize::begin(&mut self.builder.payer),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(payer),) = (self.builder.payer.take(),) else {
                return Ok(());
            };
            *self.out = Some(AccountUnificationAccountControllerFees { payer });
            Ok(())
        }
    }
};
/// A value indicating the responsible payer of a bundle of Stripe fees for pricing-control eligible products on this account.
/// Learn more about [fee behavior on connected accounts](https://docs.stripe.com/connect/direct-charges-fee-payer-behavior).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountUnificationAccountControllerFeesPayer {
    Account,
    Application,
    ApplicationCustom,
    ApplicationExpress,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AccountUnificationAccountControllerFeesPayer {
    pub fn as_str(&self) -> &str {
        use AccountUnificationAccountControllerFeesPayer::*;
        match self {
            Account => "account",
            Application => "application",
            ApplicationCustom => "application_custom",
            ApplicationExpress => "application_express",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AccountUnificationAccountControllerFeesPayer {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountUnificationAccountControllerFeesPayer::*;
        match s {
            "account" => Ok(Account),
            "application" => Ok(Application),
            "application_custom" => Ok(ApplicationCustom),
            "application_express" => Ok(ApplicationExpress),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "AccountUnificationAccountControllerFeesPayer"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AccountUnificationAccountControllerFeesPayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AccountUnificationAccountControllerFeesPayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountUnificationAccountControllerFeesPayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AccountUnificationAccountControllerFeesPayer))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountUnificationAccountControllerFeesPayer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for AccountUnificationAccountControllerFeesPayer {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<AccountUnificationAccountControllerFeesPayer> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(AccountUnificationAccountControllerFeesPayer::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountUnificationAccountControllerFeesPayer {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
