#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsKonbini {
    /// An optional 10 to 11 digit numeric-only string determining the confirmation code at applicable convenience stores.
    pub confirmation_number: Option<String>,
    /// The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire.
    /// For example, if a PaymentIntent is confirmed with Konbini and `expires_after_days` set to 2 on Monday JST, the instructions will expire on Wednesday 23:59:59 JST.
    pub expires_after_days: Option<u32>,
    /// The timestamp at which the Konbini payment instructions will expire.
    /// Only one of `expires_after_days` or `expires_at` may be set.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// A product descriptor of up to 22 characters, which will appear to customers at the convenience store.
    pub product_description: Option<String>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    pub setup_future_usage: Option<PaymentMethodOptionsKonbiniSetupFutureUsage>,
}
#[doc(hidden)]
pub struct PaymentMethodOptionsKonbiniBuilder {
    confirmation_number: Option<Option<String>>,
    expires_after_days: Option<Option<u32>>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    product_description: Option<Option<String>>,
    setup_future_usage: Option<Option<PaymentMethodOptionsKonbiniSetupFutureUsage>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodOptionsKonbini {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsKonbini>,
        builder: PaymentMethodOptionsKonbiniBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsKonbini> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsKonbiniBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsKonbiniBuilder {
        type Out = PaymentMethodOptionsKonbini;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "confirmation_number" => Deserialize::begin(&mut self.confirmation_number),
                "expires_after_days" => Deserialize::begin(&mut self.expires_after_days),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "product_description" => Deserialize::begin(&mut self.product_description),
                "setup_future_usage" => Deserialize::begin(&mut self.setup_future_usage),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                confirmation_number: Deserialize::default(),
                expires_after_days: Deserialize::default(),
                expires_at: Deserialize::default(),
                product_description: Deserialize::default(),
                setup_future_usage: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                confirmation_number: self.confirmation_number.take()?,
                expires_after_days: self.expires_after_days?,
                expires_at: self.expires_at?,
                product_description: self.product_description.take()?,
                setup_future_usage: self.setup_future_usage?,
            })
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

    impl ObjectDeser for PaymentMethodOptionsKonbini {
        type Builder = PaymentMethodOptionsKonbiniBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsKonbini {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodOptionsKonbiniBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "confirmation_number" => {
                        b.confirmation_number = Some(FromValueOpt::from_value(v)?)
                    }
                    "expires_after_days" => {
                        b.expires_after_days = Some(FromValueOpt::from_value(v)?)
                    }
                    "expires_at" => b.expires_at = Some(FromValueOpt::from_value(v)?),
                    "product_description" => {
                        b.product_description = Some(FromValueOpt::from_value(v)?)
                    }
                    "setup_future_usage" => {
                        b.setup_future_usage = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsKonbiniSetupFutureUsage {
    None,
}
impl PaymentMethodOptionsKonbiniSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsKonbiniSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsKonbiniSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsKonbiniSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsKonbiniSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsKonbiniSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsKonbiniSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodOptionsKonbiniSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsKonbiniSetupFutureUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodOptionsKonbiniSetupFutureUsage::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodOptionsKonbiniSetupFutureUsage);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsKonbiniSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodOptionsKonbiniSetupFutureUsage",
            )
        })
    }
}
