#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Card {
    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: String,
    /// Checks on Card address and CVC if provided.
    pub checks: Option<stripe_types::payment_method::card::checks::Checks>,
    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,
    /// A high-level description of the type of cards issued in this range.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,
    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *Starting May 1, 2021, card fingerprint in India for Connect will change to allow two fingerprints for the same card --- one for India and one for the rest of the world.*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: String,
    /// Issuer identification number of the card.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,
    /// The name of the card's issuing bank.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// The last four digits of the card.
    pub last4: String,
    /// Contains information about card networks that can be used to process the payment.
    pub networks: Option<stripe_types::payment_method::card::networks::Networks>,
    /// Contains details on how this Card maybe be used for 3D Secure authentication.
    pub three_d_secure_usage:
        Option<stripe_types::payment_method::card::three_d_secure_usage::ThreeDSecureUsage>,
    /// If this Card is part of a card wallet, this contains the details of the card wallet.
    pub wallet: Option<stripe_types::payment_method::card::wallet::Wallet>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Card {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod networks;
pub use networks::Networks;
pub mod checks;
pub use checks::Checks;
pub mod wallet;
pub use wallet::Wallet;
pub mod three_d_secure_usage;
pub use three_d_secure_usage::ThreeDSecureUsage;
