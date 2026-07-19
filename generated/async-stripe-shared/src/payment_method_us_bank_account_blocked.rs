#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodUsBankAccountBlocked {
    /// The ACH network code that resulted in this block.
    pub network_code: Option<PaymentMethodUsBankAccountBlockedNetworkCode>,
    /// The reason why this PaymentMethod's fingerprint has been blocked
    pub reason: Option<PaymentMethodUsBankAccountBlockedReason>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodUsBankAccountBlocked {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodUsBankAccountBlocked").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodUsBankAccountBlockedBuilder {
    network_code: Option<Option<PaymentMethodUsBankAccountBlockedNetworkCode>>,
    reason: Option<Option<PaymentMethodUsBankAccountBlockedReason>>,
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

    impl Deserialize for PaymentMethodUsBankAccountBlocked {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodUsBankAccountBlocked>,
        builder: PaymentMethodUsBankAccountBlockedBuilder,
    }

    impl Visitor for Place<PaymentMethodUsBankAccountBlocked> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodUsBankAccountBlockedBuilder {
                    network_code: Deserialize::default(),
                    reason: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "network_code" => Deserialize::begin(&mut self.builder.network_code),
                "reason" => Deserialize::begin(&mut self.builder.reason),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(network_code), Some(reason)) =
                (self.builder.network_code.take(), self.builder.reason.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodUsBankAccountBlocked { network_code, reason });
            Ok(())
        }
    }
};
/// The ACH network code that resulted in this block.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodUsBankAccountBlockedNetworkCode {
    R02,
    R03,
    R04,
    R05,
    R07,
    R08,
    R10,
    R11,
    R16,
    R20,
    R29,
    R31,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodUsBankAccountBlockedNetworkCode {
    pub fn as_str(&self) -> &str {
        use PaymentMethodUsBankAccountBlockedNetworkCode::*;
        match self {
            R02 => "R02",
            R03 => "R03",
            R04 => "R04",
            R05 => "R05",
            R07 => "R07",
            R08 => "R08",
            R10 => "R10",
            R11 => "R11",
            R16 => "R16",
            R20 => "R20",
            R29 => "R29",
            R31 => "R31",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodUsBankAccountBlockedNetworkCode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodUsBankAccountBlockedNetworkCode::*;
        match s {
            "R02" => Ok(R02),
            "R03" => Ok(R03),
            "R04" => Ok(R04),
            "R05" => Ok(R05),
            "R07" => Ok(R07),
            "R08" => Ok(R08),
            "R10" => Ok(R10),
            "R11" => Ok(R11),
            "R16" => Ok(R16),
            "R20" => Ok(R20),
            "R29" => Ok(R29),
            "R31" => Ok(R31),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodUsBankAccountBlockedNetworkCode"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodUsBankAccountBlockedNetworkCode))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentMethodUsBankAccountBlockedNetworkCode> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentMethodUsBankAccountBlockedNetworkCode::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The reason why this PaymentMethod's fingerprint has been blocked
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodUsBankAccountBlockedReason {
    BankAccountClosed,
    BankAccountFrozen,
    BankAccountInvalidDetails,
    BankAccountRestricted,
    BankAccountUnusable,
    DebitNotAuthorized,
    TokenizedAccountNumberDeactivated,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodUsBankAccountBlockedReason {
    pub fn as_str(&self) -> &str {
        use PaymentMethodUsBankAccountBlockedReason::*;
        match self {
            BankAccountClosed => "bank_account_closed",
            BankAccountFrozen => "bank_account_frozen",
            BankAccountInvalidDetails => "bank_account_invalid_details",
            BankAccountRestricted => "bank_account_restricted",
            BankAccountUnusable => "bank_account_unusable",
            DebitNotAuthorized => "debit_not_authorized",
            TokenizedAccountNumberDeactivated => "tokenized_account_number_deactivated",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodUsBankAccountBlockedReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodUsBankAccountBlockedReason::*;
        match s {
            "bank_account_closed" => Ok(BankAccountClosed),
            "bank_account_frozen" => Ok(BankAccountFrozen),
            "bank_account_invalid_details" => Ok(BankAccountInvalidDetails),
            "bank_account_restricted" => Ok(BankAccountRestricted),
            "bank_account_unusable" => Ok(BankAccountUnusable),
            "debit_not_authorized" => Ok(DebitNotAuthorized),
            "tokenized_account_number_deactivated" => Ok(TokenizedAccountNumberDeactivated),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodUsBankAccountBlockedReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodUsBankAccountBlockedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodUsBankAccountBlockedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodUsBankAccountBlockedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodUsBankAccountBlockedReason)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodUsBankAccountBlockedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentMethodUsBankAccountBlockedReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentMethodUsBankAccountBlockedReason> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodUsBankAccountBlockedReason::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodUsBankAccountBlockedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
