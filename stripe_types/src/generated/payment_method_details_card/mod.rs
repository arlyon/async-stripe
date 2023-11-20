#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsCard {
    /// The authorized amount.
pub amount_authorized: Option<i64>,
    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `eftpos_au`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
pub brand: Option<String>,
    /// When using manual capture, a future timestamp at which the charge will be automatically refunded if uncaptured.
#[serde(skip_serializing_if = "Option::is_none")]
pub capture_before: Option<stripe_types::Timestamp>,
    /// Check results by Card networks on Card address and CVC at time of payment.
pub checks: Option<stripe_types::PaymentMethodDetailsCardChecks>,
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
#[serde(skip_serializing_if = "Option::is_none")]
pub extended_authorization: Option<stripe_types::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorization>,
    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *As of May 1, 2021, card fingerprint in India for Connect changed to allow two fingerprints for the same card---one for India and one for the rest of the world.*.
#[serde(skip_serializing_if = "Option::is_none")]
pub fingerprint: Option<String>,
    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
pub funding: Option<String>,
    /// Issuer identification number of the card.
    ///
    /// (For internal use only and not typically available in standard API requests.).
#[serde(skip_serializing_if = "Option::is_none")]
pub iin: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
pub incremental_authorization: Option<stripe_types::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorization>,
    /// Installment details for this payment (Mexico only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
pub installments: Option<stripe_types::PaymentMethodDetailsCardInstallments>,
    /// The name of the card's issuing bank.
    ///
    /// (For internal use only and not typically available in standard API requests.).
#[serde(skip_serializing_if = "Option::is_none")]
pub issuer: Option<String>,
    /// The last four digits of the card.
pub last4: Option<String>,
    /// ID of the mandate used to make this payment or created by it.
pub mandate: Option<String>,
    /// True if this payment was marked as MOTO and out of scope for SCA.
#[serde(skip_serializing_if = "Option::is_none")]
pub moto: Option<bool>,
#[serde(skip_serializing_if = "Option::is_none")]
pub multicapture: Option<stripe_types::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture>,
    /// Identifies which network this charge was processed on.
    ///
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `eftpos_au`, `interac`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
pub network: Option<String>,
    /// If this card has network token credentials, this contains the details of the network token credentials.
#[serde(skip_serializing_if = "Option::is_none")]
pub network_token: Option<stripe_types::PaymentMethodDetailsCardNetworkToken>,
#[serde(skip_serializing_if = "Option::is_none")]
pub overcapture: Option<stripe_types::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture>,
    /// Populated if this transaction used 3D Secure authentication.
pub three_d_secure: Option<stripe_types::ThreeDSecureDetailsCharge>,
    /// If this Card is part of a card wallet, this contains the details of the card wallet.
pub wallet: Option<stripe_types::PaymentMethodDetailsCardWallet>,

}
