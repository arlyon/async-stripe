#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CardPresent {
    /// The authorized amount.
    pub amount_authorized: Option<i64>,
    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: Option<String>,
    /// When using manual capture, a future timestamp after which the charge will be automatically refunded if uncaptured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_before: Option<stripe_types::Timestamp>,
    /// The cardholder name as read from the card, in [ISO 7813](https://en.wikipedia.org/wiki/ISO/IEC_7813) format.
    ///
    /// May include alphanumeric characters, special characters and first/last name separator (`/`).
    /// In some cases, the cardholder name may not be available depending on how the issuer has configured the card.
    /// Cardholder name is typically not available on swipe or contactless payments, such as those made with Apple Pay and Google Pay.
    pub cardholder_name: Option<String>,
    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,
    /// A high-level description of the type of cards issued in this range.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Authorization response cryptogram.
    pub emv_auth_data: Option<String>,
    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,
    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *Starting May 1, 2021, card fingerprint in India for Connect will change to allow two fingerprints for the same card --- one for India and one for the rest of the world.*.
    pub fingerprint: Option<String>,
    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: Option<String>,
    /// ID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions.
    ///
    /// Only present if it was possible to generate a card PaymentMethod.
    pub generated_card: Option<String>,
    /// Issuer identification number of the card.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,
    /// Whether this [PaymentIntent](https://stripe.com/docs/api/payment_intents) is eligible for incremental authorizations.
    ///
    /// Request support using [request_incremental_authorization_support](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-payment_method_options-card_present-request_incremental_authorization_support).
    pub incremental_authorization_supported: bool,
    /// The name of the card's issuing bank.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// The last four digits of the card.
    pub last4: Option<String>,
    /// Identifies which network this charge was processed on.
    ///
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `interac`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub network: Option<String>,
    /// Defines whether the authorized amount can be over-captured or not.
    pub overcapture_supported: bool,
    /// How card details were read in this transaction.
    pub read_method: Option<CardPresentReadMethod>,
    /// A collection of fields required to be displayed on receipts.
    ///
    /// Only required for EMV transactions.
    pub receipt:
        Option<stripe_core::charge::payment_method_details::card_present::receipt::Receipt>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CardPresent {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// How card details were read in this transaction.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardPresentReadMethod {
    ContactEmv,
    ContactlessEmv,
    ContactlessMagstripeMode,
    MagneticStripeFallback,
    MagneticStripeTrack2,
}

impl CardPresentReadMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ContactEmv => "contact_emv",
            Self::ContactlessEmv => "contactless_emv",
            Self::ContactlessMagstripeMode => "contactless_magstripe_mode",
            Self::MagneticStripeFallback => "magnetic_stripe_fallback",
            Self::MagneticStripeTrack2 => "magnetic_stripe_track2",
        }
    }
}

impl std::str::FromStr for CardPresentReadMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "contact_emv" => Ok(Self::ContactEmv),
            "contactless_emv" => Ok(Self::ContactlessEmv),
            "contactless_magstripe_mode" => Ok(Self::ContactlessMagstripeMode),
            "magnetic_stripe_fallback" => Ok(Self::MagneticStripeFallback),
            "magnetic_stripe_track2" => Ok(Self::MagneticStripeTrack2),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CardPresentReadMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardPresentReadMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CardPresentReadMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CardPresentReadMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CardPresentReadMethod"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CardPresentReadMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CardPresentReadMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CardPresentReadMethod::from_str(s)?);
        Ok(())
    }
}
pub mod receipt;
pub use receipt::Receipt;
