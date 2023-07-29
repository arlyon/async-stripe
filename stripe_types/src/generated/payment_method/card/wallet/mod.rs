#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Wallet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex_express_checkout: Option<
        stripe_types::payment_method::card::wallet::amex_express_checkout::AmexExpressCheckout,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<stripe_types::payment_method::card::wallet::apple_pay::ApplePay>,
    /// (For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<stripe_types::payment_method::card::wallet::google_pay::GooglePay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masterpass: Option<stripe_types::payment_method::card::wallet::masterpass::Masterpass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<stripe_types::payment_method::card::wallet::samsung_pay::SamsungPay>,
    /// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, or `visa_checkout`.
    ///
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[serde(rename = "type")]
    pub type_: WalletType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_checkout:
        Option<stripe_types::payment_method::card::wallet::visa_checkout::VisaCheckout>,
}
/// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, or `visa_checkout`.
///
/// An additional hash is included on the Wallet subhash with a name matching this value.
/// It contains additional information specific to the card wallet type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WalletType {
    AmexExpressCheckout,
    ApplePay,
    GooglePay,
    Masterpass,
    SamsungPay,
    VisaCheckout,
}

impl WalletType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AmexExpressCheckout => "amex_express_checkout",
            Self::ApplePay => "apple_pay",
            Self::GooglePay => "google_pay",
            Self::Masterpass => "masterpass",
            Self::SamsungPay => "samsung_pay",
            Self::VisaCheckout => "visa_checkout",
        }
    }
}

impl std::str::FromStr for WalletType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amex_express_checkout" => Ok(Self::AmexExpressCheckout),
            "apple_pay" => Ok(Self::ApplePay),
            "google_pay" => Ok(Self::GooglePay),
            "masterpass" => Ok(Self::Masterpass),
            "samsung_pay" => Ok(Self::SamsungPay),
            "visa_checkout" => Ok(Self::VisaCheckout),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for WalletType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for WalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for WalletType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WalletType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for WalletType"))
    }
}
pub mod amex_express_checkout;
pub use amex_express_checkout::AmexExpressCheckout;
pub mod apple_pay;
pub use apple_pay::ApplePay;
pub mod google_pay;
pub use google_pay::GooglePay;
pub mod masterpass;
pub use masterpass::Masterpass;
pub mod samsung_pay;
pub use samsung_pay::SamsungPay;
pub mod visa_checkout;
pub use visa_checkout::VisaCheckout;
