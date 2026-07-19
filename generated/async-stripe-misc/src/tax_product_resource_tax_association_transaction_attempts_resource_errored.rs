#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxAssociationTransactionAttemptsResourceErrored {
    /// Details on why we couldn't commit the tax transaction.
    pub reason: TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceTaxAssociationTransactionAttemptsResourceErrored {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourceTaxAssociationTransactionAttemptsResourceErrored")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredBuilder {
    reason: Option<TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason>,
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

    impl Deserialize for TaxProductResourceTaxAssociationTransactionAttemptsResourceErrored {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxAssociationTransactionAttemptsResourceErrored>,
        builder: TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxAssociationTransactionAttemptsResourceErrored> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredBuilder {
                        reason: Deserialize::default(),
                    },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "reason" => Deserialize::begin(&mut self.builder.reason),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(reason),) = (self.builder.reason.take(),) else {
                return Ok(());
            };
            *self.out =
                Some(TaxProductResourceTaxAssociationTransactionAttemptsResourceErrored { reason });
            Ok(())
        }
    }
};
/// Details on why we couldn't commit the tax transaction.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason {
    AnotherPaymentAssociatedWithCalculation,
    CalculationExpired,
    CurrencyMismatch,
    OriginalTransactionVoided,
    UniqueReferenceViolation,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason {
    pub fn as_str(&self) -> &str {
        use TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::*;
        match self {
            AnotherPaymentAssociatedWithCalculation => {
                "another_payment_associated_with_calculation"
            }
            CalculationExpired => "calculation_expired",
            CurrencyMismatch => "currency_mismatch",
            OriginalTransactionVoided => "original_transaction_voided",
            UniqueReferenceViolation => "unique_reference_violation",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::*;
        match s {
            "another_payment_associated_with_calculation" => {
                Ok(AnotherPaymentAssociatedWithCalculation)
            }
            "calculation_expired" => Ok(CalculationExpired),
            "currency_mismatch" => Ok(CurrencyMismatch),
            "original_transaction_voided" => Ok(OriginalTransactionVoided),
            "unique_reference_violation" => Ok(UniqueReferenceViolation),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
