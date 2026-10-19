#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsPix {
    /// Determines if the amount includes the IOF tax.
    pub amount_includes_iof: Option<stripe_shared::MandatePixAmountIncludesIof>,
    /// The number of seconds (between 10 and 1209600) after which Pix payment will expire.
    pub expires_after_seconds: Option<i64>,
    /// The timestamp at which the Pix expires.
    pub expires_at: Option<i64>,
    pub mandate_options: Option<stripe_shared::PaymentMethodOptionsMandateOptionsPix>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsPixSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsPix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodOptionsPix").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodOptionsPixBuilder {
    amount_includes_iof: Option<Option<stripe_shared::MandatePixAmountIncludesIof>>,
    expires_after_seconds: Option<Option<i64>>,
    expires_at: Option<Option<i64>>,
    mandate_options: Option<Option<stripe_shared::PaymentMethodOptionsMandateOptionsPix>>,
    setup_future_usage: Option<Option<stripe_shared::PaymentMethodOptionsPixSetupFutureUsage>>,
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

    impl Deserialize for PaymentMethodOptionsPix {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsPix>,
        builder: PaymentMethodOptionsPixBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsPix> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsPixBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsPixBuilder {
        type Out = PaymentMethodOptionsPix;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_includes_iof" => Deserialize::begin(&mut self.amount_includes_iof),
                "expires_after_seconds" => Deserialize::begin(&mut self.expires_after_seconds),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "mandate_options" => Deserialize::begin(&mut self.mandate_options),
                "setup_future_usage" => Deserialize::begin(&mut self.setup_future_usage),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_includes_iof: Some(None),
                expires_after_seconds: Some(None),
                expires_at: Some(None),
                mandate_options: Some(None),
                setup_future_usage: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount_includes_iof),
                Some(expires_after_seconds),
                Some(expires_at),
                Some(mandate_options),
                Some(setup_future_usage),
            ) = (
                self.amount_includes_iof.take(),
                self.expires_after_seconds,
                self.expires_at,
                self.mandate_options.take(),
                self.setup_future_usage.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount_includes_iof,
                expires_after_seconds,
                expires_at,
                mandate_options,
                setup_future_usage,
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

    impl ObjectDeser for PaymentMethodOptionsPix {
        type Builder = PaymentMethodOptionsPixBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsPix {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodOptionsPixBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_includes_iof" => b.amount_includes_iof = FromValueOpt::from_value(v),
                    "expires_after_seconds" => {
                        b.expires_after_seconds = FromValueOpt::from_value(v)
                    }
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "mandate_options" => b.mandate_options = FromValueOpt::from_value(v),
                    "setup_future_usage" => b.setup_future_usage = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Determines if the amount includes the IOF tax.
pub type PaymentMethodOptionsPixAmountIncludesIof = stripe_shared::MandatePixAmountIncludesIof;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type PaymentMethodOptionsPixSetupFutureUsage =
    stripe_shared::PaymentMethodOptionsPixSetupFutureUsage;
