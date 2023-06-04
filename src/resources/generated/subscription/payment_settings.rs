#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentSettings {
    /// Payment-method-specific configuration to provide to invoices created by the subscription.
    pub payment_method_options:
        Option<crate::subscription::payment_method_options::PaymentMethodOptions>,
    /// The list of payment method types to provide to every invoice created by the subscription.
    ///
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    pub payment_method_types: Option<Vec<PaymentSettingsPaymentMethodTypes>>,
    /// Either `off`, or `on_subscription`.
    ///
    /// With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
    pub save_default_payment_method: Option<PaymentSettingsSaveDefaultPaymentMethod>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentSettings {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The list of payment method types to provide to every invoice created by the subscription.
///
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum PaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    CustomerBalance,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    Paynow,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl PaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AchCreditTransfer => "ach_credit_transfer",
            Self::AchDebit => "ach_debit",
            Self::AcssDebit => "acss_debit",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::CustomerBalance => "customer_balance",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Paynow => "paynow",
            Self::Promptpay => "promptpay",
            Self::SepaCreditTransfer => "sepa_credit_transfer",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for PaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Either `off`, or `on_subscription`.
///
/// With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum PaymentSettingsSaveDefaultPaymentMethod {
    Off,
    OnSubscription,
}

impl PaymentSettingsSaveDefaultPaymentMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::OnSubscription => "on_subscription",
        }
    }
}

impl AsRef<str> for PaymentSettingsSaveDefaultPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentSettingsSaveDefaultPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
