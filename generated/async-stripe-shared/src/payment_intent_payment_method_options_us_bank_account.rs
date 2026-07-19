#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentPaymentMethodOptionsUsBankAccount {
    pub financial_connections: Option<stripe_shared::LinkedAccountOptionsCommon>,
    pub mandate_options: Option<stripe_shared::PaymentMethodOptionsUsBankAccountMandateOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    pub setup_future_usage: Option<PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage>,
    /// Controls when Stripe will attempt to debit the funds from the customer's account.
    /// The date must be a string in YYYY-MM-DD format.
    /// The date must be in the future and between 3 and 15 calendar days from now.
    pub target_date: Option<String>,
    /// The purpose of the transaction.
    pub transaction_purpose:
        Option<PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose>,
    /// Bank account verification method. The default value is `automatic`.
    pub verification_method:
        Option<PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentPaymentMethodOptionsUsBankAccount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentPaymentMethodOptionsUsBankAccountBuilder {
    financial_connections: Option<Option<stripe_shared::LinkedAccountOptionsCommon>>,
    mandate_options: Option<Option<stripe_shared::PaymentMethodOptionsUsBankAccountMandateOptions>>,
    setup_future_usage:
        Option<Option<PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage>>,
    target_date: Option<Option<String>>,
    transaction_purpose:
        Option<Option<PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose>>,
    verification_method:
        Option<Option<PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod>>,
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

    impl Deserialize for PaymentIntentPaymentMethodOptionsUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentPaymentMethodOptionsUsBankAccount>,
        builder: PaymentIntentPaymentMethodOptionsUsBankAccountBuilder,
    }

    impl Visitor for Place<PaymentIntentPaymentMethodOptionsUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentPaymentMethodOptionsUsBankAccountBuilder {
                    financial_connections: Deserialize::default(),
                    mandate_options: Deserialize::default(),
                    setup_future_usage: Deserialize::default(),
                    target_date: Deserialize::default(),
                    transaction_purpose: Deserialize::default(),
                    verification_method: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "financial_connections" => {
                    Deserialize::begin(&mut self.builder.financial_connections)
                }
                "mandate_options" => Deserialize::begin(&mut self.builder.mandate_options),
                "setup_future_usage" => Deserialize::begin(&mut self.builder.setup_future_usage),
                "target_date" => Deserialize::begin(&mut self.builder.target_date),
                "transaction_purpose" => Deserialize::begin(&mut self.builder.transaction_purpose),
                "verification_method" => Deserialize::begin(&mut self.builder.verification_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(financial_connections),
                Some(mandate_options),
                Some(setup_future_usage),
                Some(target_date),
                Some(transaction_purpose),
                Some(verification_method),
            ) = (
                self.builder.financial_connections.take(),
                self.builder.mandate_options.take(),
                self.builder.setup_future_usage.take(),
                self.builder.target_date.take(),
                self.builder.transaction_purpose.take(),
                self.builder.verification_method.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentPaymentMethodOptionsUsBankAccount {
                financial_connections,
                mandate_options,
                setup_future_usage,
                target_date,
                transaction_purpose,
                verification_method,
            });
            Ok(())
        }
    }
};
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    None,
    OffSession,
    OnSession,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    pub fn as_str(&self) -> &str {
        use PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The purpose of the transaction.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose {
    Goods,
    Other,
    Services,
    Unspecified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose {
    pub fn as_str(&self) -> &str {
        use PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose::*;
        match self {
            Goods => "goods",
            Other => "other",
            Services => "services",
            Unspecified => "unspecified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose::*;
        match s {
            "goods" => Ok(Goods),
            "other" => Ok(Other),
            "services" => Ok(Services),
            "unspecified" => Ok(Unspecified),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Bank account verification method. The default value is `automatic`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(&self) -> &str {
        use PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
