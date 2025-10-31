#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsCustomerBalance {
    pub bank_transfer: Option<stripe_shared::PaymentMethodOptionsCustomerBalanceBankTransfer>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    /// Permitted values include: `bank_transfer`.
    pub funding_type: Option<PaymentMethodOptionsCustomerBalanceFundingType>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    pub setup_future_usage: Option<PaymentMethodOptionsCustomerBalanceSetupFutureUsage>,
}
#[doc(hidden)]
pub struct PaymentMethodOptionsCustomerBalanceBuilder {
    bank_transfer: Option<Option<stripe_shared::PaymentMethodOptionsCustomerBalanceBankTransfer>>,
    funding_type: Option<Option<PaymentMethodOptionsCustomerBalanceFundingType>>,
    setup_future_usage: Option<Option<PaymentMethodOptionsCustomerBalanceSetupFutureUsage>>,
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

    impl Deserialize for PaymentMethodOptionsCustomerBalance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsCustomerBalance>,
        builder: PaymentMethodOptionsCustomerBalanceBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsCustomerBalance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsCustomerBalanceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsCustomerBalanceBuilder {
        type Out = PaymentMethodOptionsCustomerBalance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_transfer" => Deserialize::begin(&mut self.bank_transfer),
                "funding_type" => Deserialize::begin(&mut self.funding_type),
                "setup_future_usage" => Deserialize::begin(&mut self.setup_future_usage),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bank_transfer: Deserialize::default(),
                funding_type: Deserialize::default(),
                setup_future_usage: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bank_transfer), Some(funding_type), Some(setup_future_usage)) =
                (self.bank_transfer.take(), self.funding_type, self.setup_future_usage)
            else {
                return None;
            };
            Some(Self::Out { bank_transfer, funding_type, setup_future_usage })
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

    impl ObjectDeser for PaymentMethodOptionsCustomerBalance {
        type Builder = PaymentMethodOptionsCustomerBalanceBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsCustomerBalance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodOptionsCustomerBalanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_transfer" => b.bank_transfer = FromValueOpt::from_value(v),
                    "funding_type" => b.funding_type = FromValueOpt::from_value(v),
                    "setup_future_usage" => b.setup_future_usage = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The funding method type to be used when there are not enough funds in the customer balance.
/// Permitted values include: `bank_transfer`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}
impl PaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsCustomerBalanceFundingType::*;
        match self {
            BankTransfer => "bank_transfer",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsCustomerBalanceFundingType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsCustomerBalanceFundingType::*;
        match s {
            "bank_transfer" => Ok(BankTransfer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsCustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsCustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsCustomerBalanceFundingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodOptionsCustomerBalanceFundingType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsCustomerBalanceFundingType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodOptionsCustomerBalanceFundingType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodOptionsCustomerBalanceFundingType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsCustomerBalanceFundingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodOptionsCustomerBalanceFundingType",
            )
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    None,
}
impl PaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsCustomerBalanceSetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsCustomerBalanceSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsCustomerBalanceSetupFutureUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodOptionsCustomerBalanceSetupFutureUsage::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodOptionsCustomerBalanceSetupFutureUsage);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodOptionsCustomerBalanceSetupFutureUsage",
            )
        })
    }
}
