#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodCardGeneratedCard {
    /// The charge that created this object.
    pub charge: Option<String>,
    /// Transaction-specific details of the payment method used in the payment.
    pub payment_method_details: Option<stripe_shared::CardGeneratedFromPaymentMethodDetails>,
    /// The ID of the SetupAttempt that generated this PaymentMethod, if any.
    pub setup_attempt: Option<stripe_types::Expandable<stripe_shared::SetupAttempt>>,
}
#[doc(hidden)]
pub struct PaymentMethodCardGeneratedCardBuilder {
    charge: Option<Option<String>>,
    payment_method_details: Option<Option<stripe_shared::CardGeneratedFromPaymentMethodDetails>>,
    setup_attempt: Option<Option<stripe_types::Expandable<stripe_shared::SetupAttempt>>>,
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

    impl Deserialize for PaymentMethodCardGeneratedCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodCardGeneratedCard>,
        builder: PaymentMethodCardGeneratedCardBuilder,
    }

    impl Visitor for Place<PaymentMethodCardGeneratedCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodCardGeneratedCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodCardGeneratedCardBuilder {
        type Out = PaymentMethodCardGeneratedCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "charge" => Deserialize::begin(&mut self.charge),
                "payment_method_details" => Deserialize::begin(&mut self.payment_method_details),
                "setup_attempt" => Deserialize::begin(&mut self.setup_attempt),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                charge: Deserialize::default(),
                payment_method_details: Deserialize::default(),
                setup_attempt: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(charge), Some(payment_method_details), Some(setup_attempt)) =
                (self.charge.take(), self.payment_method_details.take(), self.setup_attempt.take())
            else {
                return None;
            };
            Some(Self::Out { charge, payment_method_details, setup_attempt })
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

    impl ObjectDeser for PaymentMethodCardGeneratedCard {
        type Builder = PaymentMethodCardGeneratedCardBuilder;
    }

    impl FromValueOpt for PaymentMethodCardGeneratedCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodCardGeneratedCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "charge" => b.charge = FromValueOpt::from_value(v),
                    "payment_method_details" => {
                        b.payment_method_details = FromValueOpt::from_value(v)
                    }
                    "setup_attempt" => b.setup_attempt = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
