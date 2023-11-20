#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsCard {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<PaymentIntentPaymentMethodOptionsCardCaptureMethod>,
    /// Installment details for this payment (Mexico only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
    pub installments: Option<stripe_types::PaymentMethodOptionsCardInstallments>,
    /// Configuration options for setting up an eMandate for cards issued in India.
    pub mandate_options: Option<stripe_types::PaymentMethodOptionsCardMandateOptions>,
    /// Selected network to process this payment intent on.
    ///
    /// Depends on the available networks of the card attached to the payment intent.
    /// Can be only set confirm-time.
    pub network: Option<PaymentIntentPaymentMethodOptionsCardNetwork>,
    /// Request ability to [capture beyond the standard authorization validity window](https://stripe.com/docs/payments/extended-authorization) for this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_extended_authorization:
        Option<PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization>,
    /// Request ability to [increment](https://stripe.com/docs/payments/incremental-authorization) for this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_incremental_authorization:
        Option<PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization>,
    /// Request ability to make [multiple captures](https://stripe.com/docs/payments/multicapture) for this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_multicapture: Option<PaymentIntentPaymentMethodOptionsCardRequestMulticapture>,
    /// Request ability to [overcapture](https://stripe.com/docs/payments/overcapture) for this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_overcapture: Option<PaymentIntentPaymentMethodOptionsCardRequestOvercapture>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Permitted values include: `automatic` or `any`.
    /// If not provided, defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: Option<PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<PaymentIntentPaymentMethodOptionsCardSetupFutureUsage>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kana: Option<String>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 17 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 17 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kanji: Option<String>,
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    Manual,
}

impl PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentIntentPaymentMethodOptionsCardCaptureMethod",
            )
        })
    }
}
/// Selected network to process this payment intent on.
///
/// Depends on the available networks of the card attached to the payment intent.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl PaymentIntentPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardNetwork::*;
        match self {
            Amex => "amex",
            CartesBancaires => "cartes_bancaires",
            Diners => "diners",
            Discover => "discover",
            EftposAu => "eftpos_au",
            Interac => "interac",
            Jcb => "jcb",
            Mastercard => "mastercard",
            Unionpay => "unionpay",
            Unknown => "unknown",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardNetwork::*;
        match s {
            "amex" => Ok(Amex),
            "cartes_bancaires" => Ok(CartesBancaires),
            "diners" => Ok(Diners),
            "discover" => Ok(Discover),
            "eftpos_au" => Ok(EftposAu),
            "interac" => Ok(Interac),
            "jcb" => Ok(Jcb),
            "mastercard" => Ok(Mastercard),
            "unionpay" => Ok(Unionpay),
            "unknown" => Ok(Unknown),
            "visa" => Ok(Visa),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsCardNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentIntentPaymentMethodOptionsCardNetwork",
            )
        })
    }
}
/// Request ability to [capture beyond the standard authorization validity window](https://stripe.com/docs/payments/extended-authorization) for this PaymentIntent.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    IfAvailable,
    Never,
}

impl PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization"))
    }
}
/// Request ability to [increment](https://stripe.com/docs/payments/incremental-authorization) for this PaymentIntent.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    IfAvailable,
    Never,
}

impl PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization"))
    }
}
/// Request ability to make [multiple captures](https://stripe.com/docs/payments/multicapture) for this PaymentIntent.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    IfAvailable,
    Never,
}

impl PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardRequestMulticapture::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardRequestMulticapture::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentIntentPaymentMethodOptionsCardRequestMulticapture",
            )
        })
    }
}
/// Request ability to [overcapture](https://stripe.com/docs/payments/overcapture) for this PaymentIntent.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    IfAvailable,
    Never,
}

impl PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardRequestOvercapture::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardRequestOvercapture::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentIntentPaymentMethodOptionsCardRequestOvercapture",
            )
        })
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
///
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Permitted values include: `automatic` or `any`.
/// If not provided, defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    ChallengeOnly,
}

impl PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure::*;
        match self {
            Any => "any",
            Automatic => "automatic",
            ChallengeOnly => "challenge_only",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure::*;
        match s {
            "any" => Ok(Any),
            "automatic" => Ok(Automatic),
            "challenge_only" => Ok(ChallengeOnly),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure",
            )
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsCardSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsCardSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentIntentPaymentMethodOptionsCardSetupFutureUsage",
            )
        })
    }
}
