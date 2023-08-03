#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsCardWallet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex_express_checkout: Option<stripe_types::PaymentMethodDetailsCardWalletAmexExpressCheckout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<stripe_types::PaymentMethodDetailsCardWalletApplePay>,
    /// (For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<stripe_types::PaymentMethodDetailsCardWalletGooglePay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types::PaymentMethodDetailsCardWalletLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masterpass: Option<stripe_types::PaymentMethodDetailsCardWalletMasterpass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<stripe_types::PaymentMethodDetailsCardWalletSamsungPay>,
    /// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, `visa_checkout`, or `link`.
    ///
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[serde(rename = "type")]
    pub type_: PaymentMethodDetailsCardWalletType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_checkout: Option<stripe_types::PaymentMethodDetailsCardWalletVisaCheckout>,
}
/// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, `visa_checkout`, or `link`.
///
/// An additional hash is included on the Wallet subhash with a name matching this value.
/// It contains additional information specific to the card wallet type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsCardWalletType {
    AmexExpressCheckout,
    ApplePay,
    GooglePay,
    Link,
    Masterpass,
    SamsungPay,
    VisaCheckout,
}

impl PaymentMethodDetailsCardWalletType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsCardWalletType::*;
        match self {
            AmexExpressCheckout => "amex_express_checkout",
            ApplePay => "apple_pay",
            GooglePay => "google_pay",
            Link => "link",
            Masterpass => "masterpass",
            SamsungPay => "samsung_pay",
            VisaCheckout => "visa_checkout",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCardWalletType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsCardWalletType::*;
        match s {
            "amex_express_checkout" => Ok(AmexExpressCheckout),
            "apple_pay" => Ok(ApplePay),
            "google_pay" => Ok(GooglePay),
            "link" => Ok(Link),
            "masterpass" => Ok(Masterpass),
            "samsung_pay" => Ok(SamsungPay),
            "visa_checkout" => Ok(VisaCheckout),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsCardWalletType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodDetailsCardWalletType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCardWalletType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodDetailsCardWalletType"))
    }
}
