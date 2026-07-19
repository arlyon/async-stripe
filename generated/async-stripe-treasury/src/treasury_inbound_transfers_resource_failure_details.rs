#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryInboundTransfersResourceFailureDetails {
    /// Reason for the failure.
    pub code: TreasuryInboundTransfersResourceFailureDetailsCode,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryInboundTransfersResourceFailureDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryInboundTransfersResourceFailureDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryInboundTransfersResourceFailureDetailsBuilder {
    code: Option<TreasuryInboundTransfersResourceFailureDetailsCode>,
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

    impl Deserialize for TreasuryInboundTransfersResourceFailureDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryInboundTransfersResourceFailureDetails>,
        builder: TreasuryInboundTransfersResourceFailureDetailsBuilder,
    }

    impl Visitor for Place<TreasuryInboundTransfersResourceFailureDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryInboundTransfersResourceFailureDetailsBuilder {
                    code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.builder.code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(code),) = (self.builder.code.take(),) else {
                return Ok(());
            };
            *self.out = Some(TreasuryInboundTransfersResourceFailureDetails { code });
            Ok(())
        }
    }
};
/// Reason for the failure.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryInboundTransfersResourceFailureDetailsCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    DebitNotAuthorized,
    IncorrectAccountHolderAddress,
    IncorrectAccountHolderName,
    IncorrectAccountHolderTaxId,
    InsufficientFunds,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryInboundTransfersResourceFailureDetailsCode {
    pub fn as_str(&self) -> &str {
        use TreasuryInboundTransfersResourceFailureDetailsCode::*;
        match self {
            AccountClosed => "account_closed",
            AccountFrozen => "account_frozen",
            BankAccountRestricted => "bank_account_restricted",
            BankOwnershipChanged => "bank_ownership_changed",
            DebitNotAuthorized => "debit_not_authorized",
            IncorrectAccountHolderAddress => "incorrect_account_holder_address",
            IncorrectAccountHolderName => "incorrect_account_holder_name",
            IncorrectAccountHolderTaxId => "incorrect_account_holder_tax_id",
            InsufficientFunds => "insufficient_funds",
            InvalidAccountNumber => "invalid_account_number",
            InvalidCurrency => "invalid_currency",
            NoAccount => "no_account",
            Other => "other",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryInboundTransfersResourceFailureDetailsCode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryInboundTransfersResourceFailureDetailsCode::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "account_frozen" => Ok(AccountFrozen),
            "bank_account_restricted" => Ok(BankAccountRestricted),
            "bank_ownership_changed" => Ok(BankOwnershipChanged),
            "debit_not_authorized" => Ok(DebitNotAuthorized),
            "incorrect_account_holder_address" => Ok(IncorrectAccountHolderAddress),
            "incorrect_account_holder_name" => Ok(IncorrectAccountHolderName),
            "incorrect_account_holder_tax_id" => Ok(IncorrectAccountHolderTaxId),
            "insufficient_funds" => Ok(InsufficientFunds),
            "invalid_account_number" => Ok(InvalidAccountNumber),
            "invalid_currency" => Ok(InvalidCurrency),
            "no_account" => Ok(NoAccount),
            "other" => Ok(Other),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryInboundTransfersResourceFailureDetailsCode"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryInboundTransfersResourceFailureDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryInboundTransfersResourceFailureDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryInboundTransfersResourceFailureDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryInboundTransfersResourceFailureDetailsCode))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryInboundTransfersResourceFailureDetailsCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TreasuryInboundTransfersResourceFailureDetailsCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TreasuryInboundTransfersResourceFailureDetailsCode> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryInboundTransfersResourceFailureDetailsCode::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryInboundTransfersResourceFailureDetailsCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
