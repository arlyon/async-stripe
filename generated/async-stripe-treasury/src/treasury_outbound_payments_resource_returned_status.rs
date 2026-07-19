#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryOutboundPaymentsResourceReturnedStatus {
    /// Reason for the return.
    pub code: TreasuryOutboundPaymentsResourceReturnedStatusCode,
    /// The Transaction associated with this object.
    pub transaction: stripe_types::Expandable<stripe_treasury::TreasuryTransaction>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryOutboundPaymentsResourceReturnedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryOutboundPaymentsResourceReturnedStatus").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryOutboundPaymentsResourceReturnedStatusBuilder {
    code: Option<TreasuryOutboundPaymentsResourceReturnedStatusCode>,
    transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
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

    impl Deserialize for TreasuryOutboundPaymentsResourceReturnedStatus {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryOutboundPaymentsResourceReturnedStatus>,
        builder: TreasuryOutboundPaymentsResourceReturnedStatusBuilder,
    }

    impl Visitor for Place<TreasuryOutboundPaymentsResourceReturnedStatus> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryOutboundPaymentsResourceReturnedStatusBuilder {
                    code: Deserialize::default(),
                    transaction: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.builder.code),
                "transaction" => Deserialize::begin(&mut self.builder.transaction),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(code), Some(transaction)) =
                (self.builder.code.take(), self.builder.transaction.take())
            else {
                return Ok(());
            };
            *self.out = Some(TreasuryOutboundPaymentsResourceReturnedStatus { code, transaction });
            Ok(())
        }
    }
};
/// Reason for the return.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryOutboundPaymentsResourceReturnedStatusCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    Declined,
    IncorrectAccountHolderName,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryOutboundPaymentsResourceReturnedStatusCode {
    pub fn as_str(&self) -> &str {
        use TreasuryOutboundPaymentsResourceReturnedStatusCode::*;
        match self {
            AccountClosed => "account_closed",
            AccountFrozen => "account_frozen",
            BankAccountRestricted => "bank_account_restricted",
            BankOwnershipChanged => "bank_ownership_changed",
            Declined => "declined",
            IncorrectAccountHolderName => "incorrect_account_holder_name",
            InvalidAccountNumber => "invalid_account_number",
            InvalidCurrency => "invalid_currency",
            NoAccount => "no_account",
            Other => "other",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryOutboundPaymentsResourceReturnedStatusCode::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "account_frozen" => Ok(AccountFrozen),
            "bank_account_restricted" => Ok(BankAccountRestricted),
            "bank_ownership_changed" => Ok(BankOwnershipChanged),
            "declined" => Ok(Declined),
            "incorrect_account_holder_name" => Ok(IncorrectAccountHolderName),
            "invalid_account_number" => Ok(InvalidAccountNumber),
            "invalid_currency" => Ok(InvalidCurrency),
            "no_account" => Ok(NoAccount),
            "other" => Ok(Other),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryOutboundPaymentsResourceReturnedStatusCode"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryOutboundPaymentsResourceReturnedStatusCode))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TreasuryOutboundPaymentsResourceReturnedStatusCode> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryOutboundPaymentsResourceReturnedStatusCode::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
