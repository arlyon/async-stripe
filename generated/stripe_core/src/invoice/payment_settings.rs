#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentSettings {
    /// ID of the mandate to be used for this invoice.
    ///
    /// It must correspond to the payment method used to pay the invoice, including the invoice's default_payment_method or default_source, if set.
    pub default_mandate: Option<String>,
    /// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
    pub payment_method_options:
        Option<stripe_core::invoice::payment_method_options::PaymentMethodOptions>,
    /// The list of payment method types (e.g.
    ///
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    pub payment_method_types: Option<Vec<PaymentSettingsPaymentMethodTypes>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentSettings {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The list of payment method types (e.g.
///
/// card) to provide to the invoice’s PaymentIntent.
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for PaymentSettingsPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ach_credit_transfer" => Ok(Self::AchCreditTransfer),
            "ach_debit" => Ok(Self::AchDebit),
            "acss_debit" => Ok(Self::AcssDebit),
            "au_becs_debit" => Ok(Self::AuBecsDebit),
            "bacs_debit" => Ok(Self::BacsDebit),
            "bancontact" => Ok(Self::Bancontact),
            "boleto" => Ok(Self::Boleto),
            "card" => Ok(Self::Card),
            "customer_balance" => Ok(Self::CustomerBalance),
            "fpx" => Ok(Self::Fpx),
            "giropay" => Ok(Self::Giropay),
            "grabpay" => Ok(Self::Grabpay),
            "ideal" => Ok(Self::Ideal),
            "konbini" => Ok(Self::Konbini),
            "link" => Ok(Self::Link),
            "paynow" => Ok(Self::Paynow),
            "promptpay" => Ok(Self::Promptpay),
            "sepa_credit_transfer" => Ok(Self::SepaCreditTransfer),
            "sepa_debit" => Ok(Self::SepaDebit),
            "sofort" => Ok(Self::Sofort),
            "us_bank_account" => Ok(Self::UsBankAccount),
            "wechat_pay" => Ok(Self::WechatPay),

            _ => Err(()),
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
impl serde::Serialize for PaymentSettingsPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentSettingsPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentSettingsPaymentMethodTypes")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentSettingsPaymentMethodTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<PaymentSettingsPaymentMethodTypes> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentSettingsPaymentMethodTypes::from_str(s)?);
        Ok(())
    }
}
