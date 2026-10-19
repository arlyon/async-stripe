#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutAcssDebitPaymentMethodOptions {
    /// Currency supported by the bank account. Returned when the Session is in `setup` mode.
    pub currency: Option<stripe_shared::Currency>,
    pub mandate_options: Option<stripe_shared::CheckoutAcssDebitMandateOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    pub setup_future_usage: Option<stripe_shared::SetupFutureUsage>,
    /// Controls when Stripe will attempt to debit the funds from the customer's account.
    /// The date must be a string in YYYY-MM-DD format.
    /// The date must be in the future and between 3 and 15 calendar days from now.
    pub target_date: Option<String>,
    /// Bank account verification method. The default value is `automatic`.
    pub verification_method: Option<stripe_shared::VerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutAcssDebitPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CheckoutAcssDebitPaymentMethodOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CheckoutAcssDebitPaymentMethodOptionsBuilder {
    currency: Option<Option<stripe_shared::Currency>>,
    mandate_options: Option<Option<stripe_shared::CheckoutAcssDebitMandateOptions>>,
    setup_future_usage: Option<Option<stripe_shared::SetupFutureUsage>>,
    target_date: Option<Option<String>>,
    verification_method: Option<Option<stripe_shared::VerificationMethod>>,
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

    impl Deserialize for CheckoutAcssDebitPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutAcssDebitPaymentMethodOptions>,
        builder: CheckoutAcssDebitPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<CheckoutAcssDebitPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CheckoutAcssDebitPaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CheckoutAcssDebitPaymentMethodOptionsBuilder {
        type Out = CheckoutAcssDebitPaymentMethodOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "currency" => Deserialize::begin(&mut self.currency),
                "mandate_options" => Deserialize::begin(&mut self.mandate_options),
                "setup_future_usage" => Deserialize::begin(&mut self.setup_future_usage),
                "target_date" => Deserialize::begin(&mut self.target_date),
                "verification_method" => Deserialize::begin(&mut self.verification_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                currency: Some(None),
                mandate_options: Some(None),
                setup_future_usage: Some(None),
                target_date: Some(None),
                verification_method: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(currency),
                Some(mandate_options),
                Some(setup_future_usage),
                Some(target_date),
                Some(verification_method),
            ) = (
                self.currency.take(),
                self.mandate_options.take(),
                self.setup_future_usage.take(),
                self.target_date.take(),
                self.verification_method.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                currency,
                mandate_options,
                setup_future_usage,
                target_date,
                verification_method,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for CheckoutAcssDebitPaymentMethodOptions {
        type Builder = CheckoutAcssDebitPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for CheckoutAcssDebitPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CheckoutAcssDebitPaymentMethodOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "mandate_options" => b.mandate_options = FromValueOpt::from_value(v),
                    "setup_future_usage" => b.setup_future_usage = FromValueOpt::from_value(v),
                    "target_date" => b.target_date = FromValueOpt::from_value(v),
                    "verification_method" => b.verification_method = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Currency supported by the bank account. Returned when the Session is in `setup` mode.
pub type CheckoutAcssDebitPaymentMethodOptionsCurrency = stripe_shared::Currency;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage = stripe_shared::SetupFutureUsage;
/// Bank account verification method. The default value is `automatic`.
pub type CheckoutAcssDebitPaymentMethodOptionsVerificationMethod =
    stripe_shared::VerificationMethod;
