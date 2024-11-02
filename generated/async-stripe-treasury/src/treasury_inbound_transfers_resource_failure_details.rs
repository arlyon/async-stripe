#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryInboundTransfersResourceFailureDetails {
    /// Reason for the failure.
    pub code: TreasuryInboundTransfersResourceFailureDetailsCode,
}
#[doc(hidden)]
pub struct TreasuryInboundTransfersResourceFailureDetailsBuilder {
    code: Option<TreasuryInboundTransfersResourceFailureDetailsCode>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: TreasuryInboundTransfersResourceFailureDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryInboundTransfersResourceFailureDetailsBuilder {
        type Out = TreasuryInboundTransfersResourceFailureDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.code),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(code),) = (self.code.take(),) else {
                return None;
            };
            Some(Self::Out { code })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for TreasuryInboundTransfersResourceFailureDetails {
        type Builder = TreasuryInboundTransfersResourceFailureDetailsBuilder;
    }

    impl FromValueOpt for TreasuryInboundTransfersResourceFailureDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryInboundTransfersResourceFailureDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "code" => b.code = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for TreasuryInboundTransfersResourceFailureDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryInboundTransfersResourceFailureDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for TreasuryInboundTransfersResourceFailureDetailsCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TreasuryInboundTransfersResourceFailureDetailsCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryInboundTransfersResourceFailureDetailsCode::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TreasuryInboundTransfersResourceFailureDetailsCode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryInboundTransfersResourceFailureDetailsCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
