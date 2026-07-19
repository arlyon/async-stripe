/// Payment details attached to this payment evaluation.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationPaymentDetails {
    /// Amount intended to be collected by this payment.
    /// A positive integer representing how much to charge in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://docs.stripe.com/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Details about the payment's customer presence and type.
    pub money_movement_details:
        Option<stripe_fraud::InsightsResourcesPaymentEvaluationMoneyMovementDetails>,
    /// Details about the payment method used for the payment.
    pub payment_method_details:
        Option<stripe_fraud::InsightsResourcesPaymentEvaluationPaymentMethodDetails>,
    /// Shipping details for the payment evaluation.
    pub shipping_details: Option<stripe_fraud::InsightsResourcesPaymentEvaluationShipping>,
    /// Payment statement descriptor.
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationPaymentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationPaymentDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationPaymentDetailsBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    money_movement_details:
        Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationMoneyMovementDetails>>,
    payment_method_details:
        Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationPaymentMethodDetails>>,
    shipping_details: Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationShipping>>,
    statement_descriptor: Option<Option<String>>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationPaymentDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationPaymentDetails>,
        builder: InsightsResourcesPaymentEvaluationPaymentDetailsBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationPaymentDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationPaymentDetailsBuilder {
                    amount: Deserialize::default(),
                    currency: Deserialize::default(),
                    description: Deserialize::default(),
                    money_movement_details: Deserialize::default(),
                    payment_method_details: Deserialize::default(),
                    shipping_details: Deserialize::default(),
                    statement_descriptor: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "description" => Deserialize::begin(&mut self.builder.description),
                "money_movement_details" => {
                    Deserialize::begin(&mut self.builder.money_movement_details)
                }
                "payment_method_details" => {
                    Deserialize::begin(&mut self.builder.payment_method_details)
                }
                "shipping_details" => Deserialize::begin(&mut self.builder.shipping_details),
                "statement_descriptor" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(currency),
                Some(description),
                Some(money_movement_details),
                Some(payment_method_details),
                Some(shipping_details),
                Some(statement_descriptor),
            ) = (
                self.builder.amount,
                self.builder.currency.take(),
                self.builder.description.take(),
                self.builder.money_movement_details.take(),
                self.builder.payment_method_details.take(),
                self.builder.shipping_details.take(),
                self.builder.statement_descriptor.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(InsightsResourcesPaymentEvaluationPaymentDetails {
                amount,
                currency,
                description,
                money_movement_details,
                payment_method_details,
                shipping_details,
                statement_descriptor,
            });
            Ok(())
        }
    }
};
