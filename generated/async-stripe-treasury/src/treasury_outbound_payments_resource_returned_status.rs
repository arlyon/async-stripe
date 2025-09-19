#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryOutboundPaymentsResourceReturnedStatus {
    /// Reason for the return.
    pub code: TreasuryOutboundPaymentsResourceReturnedStatusCode,
    /// The Transaction associated with this object.
    pub transaction: stripe_types::Expandable<stripe_treasury::TreasuryTransaction>,
}
#[doc(hidden)]
pub struct TreasuryOutboundPaymentsResourceReturnedStatusBuilder {
    code: Option<TreasuryOutboundPaymentsResourceReturnedStatusCode>,
    transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
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
                builder: TreasuryOutboundPaymentsResourceReturnedStatusBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryOutboundPaymentsResourceReturnedStatusBuilder {
        type Out = TreasuryOutboundPaymentsResourceReturnedStatus;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.code),
                "transaction" => Deserialize::begin(&mut self.transaction),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { code: Deserialize::default(), transaction: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(code), Some(transaction)) = (self.code, self.transaction.take()) else {
                return None;
            };
            Some(Self::Out { code, transaction })
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

    impl ObjectDeser for TreasuryOutboundPaymentsResourceReturnedStatus {
        type Builder = TreasuryOutboundPaymentsResourceReturnedStatusBuilder;
    }

    impl FromValueOpt for TreasuryOutboundPaymentsResourceReturnedStatus {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryOutboundPaymentsResourceReturnedStatusBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "code" => b.code = FromValueOpt::from_value(v),
                    "transaction" => b.transaction = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Reason for the return.
#[derive(Copy, Clone, Eq, PartialEq)]
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
}
impl TreasuryOutboundPaymentsResourceReturnedStatusCode {
    pub fn as_str(self) -> &'static str {
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
        }
    }
}

impl std::str::FromStr for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    type Err = stripe_types::StripeParseError;
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
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TreasuryOutboundPaymentsResourceReturnedStatusCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryOutboundPaymentsResourceReturnedStatusCode::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TreasuryOutboundPaymentsResourceReturnedStatusCode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryOutboundPaymentsResourceReturnedStatusCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TreasuryOutboundPaymentsResourceReturnedStatusCode",
            )
        })
    }
}
