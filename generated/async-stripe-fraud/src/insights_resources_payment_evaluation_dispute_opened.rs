/// Dispute opened event details attached to this payment evaluation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationDisputeOpened {
    /// Amount to dispute for this payment.
    /// A positive integer representing how much to charge in [the smallest currency unit](https://docs.stripe.com/currencies#zero-decimal) (for example, 100 cents to charge 1.00 USD or 100 to charge 100 Yen, a zero-decimal currency).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Reason given by cardholder for dispute.
    pub reason: InsightsResourcesPaymentEvaluationDisputeOpenedReason,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationDisputeOpenedBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
    reason: Option<InsightsResourcesPaymentEvaluationDisputeOpenedReason>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationDisputeOpened {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationDisputeOpened>,
        builder: InsightsResourcesPaymentEvaluationDisputeOpenedBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationDisputeOpened> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationDisputeOpenedBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationDisputeOpenedBuilder {
        type Out = InsightsResourcesPaymentEvaluationDisputeOpened;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "currency" => Deserialize::begin(&mut self.currency),
                "reason" => Deserialize::begin(&mut self.reason),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                currency: Deserialize::default(),
                reason: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(currency), Some(reason)) =
                (self.amount, self.currency.take(), self.reason.take())
            else {
                return None;
            };
            Some(Self::Out { amount, currency, reason })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationDisputeOpened {
        type Builder = InsightsResourcesPaymentEvaluationDisputeOpenedBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationDisputeOpened {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationDisputeOpenedBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "reason" => b.reason = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Reason given by cardholder for dispute.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationDisputeOpenedReason {
    AccountNotAvailable,
    CreditNotProcessed,
    CustomerInitiated,
    Duplicate,
    Fraudulent,
    General,
    Noncompliant,
    ProductNotReceived,
    ProductUnacceptable,
    SubscriptionCanceled,
    Unrecognized,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationDisputeOpenedReason {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationDisputeOpenedReason::*;
        match self {
            AccountNotAvailable => "account_not_available",
            CreditNotProcessed => "credit_not_processed",
            CustomerInitiated => "customer_initiated",
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            General => "general",
            Noncompliant => "noncompliant",
            ProductNotReceived => "product_not_received",
            ProductUnacceptable => "product_unacceptable",
            SubscriptionCanceled => "subscription_canceled",
            Unrecognized => "unrecognized",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationDisputeOpenedReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationDisputeOpenedReason::*;
        match s {
            "account_not_available" => Ok(AccountNotAvailable),
            "credit_not_processed" => Ok(CreditNotProcessed),
            "customer_initiated" => Ok(CustomerInitiated),
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "general" => Ok(General),
            "noncompliant" => Ok(Noncompliant),
            "product_not_received" => Ok(ProductNotReceived),
            "product_unacceptable" => Ok(ProductUnacceptable),
            "subscription_canceled" => Ok(SubscriptionCanceled),
            "unrecognized" => Ok(Unrecognized),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationDisputeOpenedReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationDisputeOpenedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationDisputeOpenedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationDisputeOpenedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InsightsResourcesPaymentEvaluationDisputeOpenedReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InsightsResourcesPaymentEvaluationDisputeOpenedReason>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationDisputeOpenedReason::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InsightsResourcesPaymentEvaluationDisputeOpenedReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InsightsResourcesPaymentEvaluationDisputeOpenedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
