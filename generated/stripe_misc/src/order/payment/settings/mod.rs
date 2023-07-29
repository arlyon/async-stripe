#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Settings {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    pub application_fee_amount: Option<i64>,
    /// Indicates whether order has been opted into using [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods) to manage payment method types.
    pub automatic_payment_methods: Option<
        stripe_misc::order::payment::settings::automatic_payment_methods::AutomaticPaymentMethods,
    >,
    /// PaymentMethod-specific configuration to provide to the order's PaymentIntent.
    pub payment_method_options:
        Option<stripe_misc::order::payment::settings::payment_method_options::PaymentMethodOptions>,
    /// The list of [payment method types](https://stripe.com/docs/payments/payment-methods/overview) to provide to the order's PaymentIntent.
    ///
    /// Do not include this attribute if you prefer to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    pub payment_method_types: Option<Vec<SettingsPaymentMethodTypes>>,
    /// The URL to redirect the customer to after they authenticate their payment.
    pub return_url: Option<String>,
    /// For non-card charges, you can use this value as the complete description that appears on your customers' statements.
    ///
    /// Must contain at least one letter, maximum 22 characters.
    pub statement_descriptor: Option<String>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    pub statement_descriptor_suffix: Option<String>,
    /// Provides configuration for completing a transfer for the order after it is paid.
    pub transfer_data: Option<stripe_misc::order::payment::settings::transfer_data::TransferData>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Settings {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The list of [payment method types](https://stripe.com/docs/payments/payment-methods/overview) to provide to the order's PaymentIntent.
///
/// Do not include this attribute if you prefer to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SettingsPaymentMethodTypes {
    AcssDebit,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Card,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Link,
    Oxxo,
    P24,
    SepaDebit,
    Sofort,
    WechatPay,
}

impl SettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Card => "card",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl std::str::FromStr for SettingsPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acss_debit" => Ok(Self::AcssDebit),
            "afterpay_clearpay" => Ok(Self::AfterpayClearpay),
            "alipay" => Ok(Self::Alipay),
            "au_becs_debit" => Ok(Self::AuBecsDebit),
            "bacs_debit" => Ok(Self::BacsDebit),
            "bancontact" => Ok(Self::Bancontact),
            "card" => Ok(Self::Card),
            "customer_balance" => Ok(Self::CustomerBalance),
            "eps" => Ok(Self::Eps),
            "fpx" => Ok(Self::Fpx),
            "giropay" => Ok(Self::Giropay),
            "grabpay" => Ok(Self::Grabpay),
            "ideal" => Ok(Self::Ideal),
            "klarna" => Ok(Self::Klarna),
            "link" => Ok(Self::Link),
            "oxxo" => Ok(Self::Oxxo),
            "p24" => Ok(Self::P24),
            "sepa_debit" => Ok(Self::SepaDebit),
            "sofort" => Ok(Self::Sofort),
            "wechat_pay" => Ok(Self::WechatPay),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SettingsPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SettingsPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SettingsPaymentMethodTypes"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SettingsPaymentMethodTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SettingsPaymentMethodTypes> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SettingsPaymentMethodTypes::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
pub mod automatic_payment_methods;
pub use automatic_payment_methods::AutomaticPaymentMethods;
pub mod payment_method_options;
pub use payment_method_options::PaymentMethodOptions;
pub mod transfer_data;
pub use transfer_data::TransferData;
