#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCard {
    /// The authorized amount.
pub amount_authorized: Option<i64>,
    /// Authorization code on the charge.
pub authorization_code: Option<String>,
        /// Card brand.
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `eftpos_au`, `jcb`, `link`, `mastercard`, `unionpay`, `visa` or `unknown`.
pub brand: Option<String>,
        /// When using manual capture, a future timestamp at which the charge will be automatically refunded if uncaptured.
pub capture_before: Option<stripe_types::Timestamp>,
    /// Check results by Card networks on Card address and CVC at time of payment.
pub checks: Option<stripe_shared::PaymentMethodDetailsCardChecks>,
        /// Two-letter ISO code representing the country of the card.
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
pub country: Option<String>,
        /// A high-level description of the type of cards issued in this range.
    /// (For internal use only and not typically available in standard API requests.).
pub description: Option<String>,
    /// Two-digit number representing the card's expiration month.
pub exp_month: i64,
    /// Four-digit number representing the card's expiration year.
pub exp_year: i64,
pub extended_authorization: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorization>,
        /// Uniquely identifies this particular card number.
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.
    ///
        /// *As of May 1, 2021, card fingerprint in India for Connect changed to allow two fingerprints for the same card---one for India and one for the rest of the world.*.
pub fingerprint: Option<String>,
    /// Card funding type. Can be `credit`, `debit`, `prepaid`, or `unknown`.
pub funding: Option<String>,
        /// Issuer identification number of the card.
    /// (For internal use only and not typically available in standard API requests.).
pub iin: Option<String>,
pub incremental_authorization: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorization>,
    /// Installment details for this payment.
    ///
        /// For more information, see the [installments integration guide](https://docs.stripe.com/payments/installments).
pub installments: Option<stripe_shared::PaymentMethodDetailsCardInstallments>,
        /// The name of the card's issuing bank.
    /// (For internal use only and not typically available in standard API requests.).
pub issuer: Option<String>,
    /// The last four digits of the card.
pub last4: Option<String>,
    /// ID of the mandate used to make this payment or created by it.
pub mandate: Option<String>,
    /// True if this payment was marked as MOTO and out of scope for SCA.
pub moto: Option<bool>,
pub multicapture: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture>,
        /// Identifies which network this charge was processed on.
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `eftpos_au`, `interac`, `jcb`, `link`, `mastercard`, `unionpay`, `visa`, or `unknown`.
pub network: Option<String>,
        /// If this card has network token credentials, this contains the details of the network token credentials.
pub network_token: Option<stripe_shared::PaymentMethodDetailsCardNetworkToken>,
        /// This is used by the financial networks to identify a transaction.
    /// Visa calls this the Transaction ID, Mastercard calls this the Trace ID, and American Express calls this the Acquirer Reference Data.
    /// This value will be present if it is returned by the financial network in the authorization response, and null otherwise.
pub network_transaction_id: Option<String>,
pub overcapture: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture>,
    /// Status of a card based on the card issuer.
pub regulated_status: Option<PaymentMethodDetailsCardRegulatedStatus>,
    /// Populated if this transaction used 3D Secure authentication.
pub three_d_secure: Option<stripe_shared::ThreeDSecureDetailsCharge>,
    /// If this Card is part of a card wallet, this contains the details of the card wallet.
pub wallet: Option<stripe_shared::PaymentMethodDetailsCardWallet>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsCard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsCardBuilder {
    amount_authorized: Option<Option<i64>>,
authorization_code: Option<Option<String>>,
brand: Option<Option<String>>,
capture_before: Option<Option<stripe_types::Timestamp>>,
checks: Option<Option<stripe_shared::PaymentMethodDetailsCardChecks>>,
country: Option<Option<String>>,
description: Option<Option<String>>,
exp_month: Option<i64>,
exp_year: Option<i64>,
extended_authorization: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorization>>,
fingerprint: Option<Option<String>>,
funding: Option<Option<String>>,
iin: Option<Option<String>>,
incremental_authorization: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorization>>,
installments: Option<Option<stripe_shared::PaymentMethodDetailsCardInstallments>>,
issuer: Option<Option<String>>,
last4: Option<Option<String>>,
mandate: Option<Option<String>>,
moto: Option<Option<bool>>,
multicapture: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture>>,
network: Option<Option<String>>,
network_token: Option<Option<stripe_shared::PaymentMethodDetailsCardNetworkToken>>,
network_transaction_id: Option<Option<String>>,
overcapture: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture>>,
regulated_status: Option<Option<PaymentMethodDetailsCardRegulatedStatus>>,
three_d_secure: Option<Option<stripe_shared::ThreeDSecureDetailsCharge>>,
wallet: Option<Option<stripe_shared::PaymentMethodDetailsCardWallet>>,

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

    impl Deserialize for PaymentMethodDetailsCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCard>,
        builder: PaymentMethodDetailsCardBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsCardBuilder {
                    amount_authorized: Deserialize::default(),
                    authorization_code: Deserialize::default(),
                    brand: Deserialize::default(),
                    capture_before: Deserialize::default(),
                    checks: Deserialize::default(),
                    country: Deserialize::default(),
                    description: Deserialize::default(),
                    exp_month: Deserialize::default(),
                    exp_year: Deserialize::default(),
                    extended_authorization: Deserialize::default(),
                    fingerprint: Deserialize::default(),
                    funding: Deserialize::default(),
                    iin: Deserialize::default(),
                    incremental_authorization: Deserialize::default(),
                    installments: Deserialize::default(),
                    issuer: Deserialize::default(),
                    last4: Deserialize::default(),
                    mandate: Deserialize::default(),
                    moto: Deserialize::default(),
                    multicapture: Deserialize::default(),
                    network: Deserialize::default(),
                    network_token: Deserialize::default(),
                    network_transaction_id: Deserialize::default(),
                    overcapture: Deserialize::default(),
                    regulated_status: Deserialize::default(),
                    three_d_secure: Deserialize::default(),
                    wallet: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_authorized" => Deserialize::begin(&mut self.builder.amount_authorized),
                "authorization_code" => Deserialize::begin(&mut self.builder.authorization_code),
                "brand" => Deserialize::begin(&mut self.builder.brand),
                "capture_before" => Deserialize::begin(&mut self.builder.capture_before),
                "checks" => Deserialize::begin(&mut self.builder.checks),
                "country" => Deserialize::begin(&mut self.builder.country),
                "description" => Deserialize::begin(&mut self.builder.description),
                "exp_month" => Deserialize::begin(&mut self.builder.exp_month),
                "exp_year" => Deserialize::begin(&mut self.builder.exp_year),
                "extended_authorization" => {
                    Deserialize::begin(&mut self.builder.extended_authorization)
                }
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "funding" => Deserialize::begin(&mut self.builder.funding),
                "iin" => Deserialize::begin(&mut self.builder.iin),
                "incremental_authorization" => {
                    Deserialize::begin(&mut self.builder.incremental_authorization)
                }
                "installments" => Deserialize::begin(&mut self.builder.installments),
                "issuer" => Deserialize::begin(&mut self.builder.issuer),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "mandate" => Deserialize::begin(&mut self.builder.mandate),
                "moto" => Deserialize::begin(&mut self.builder.moto),
                "multicapture" => Deserialize::begin(&mut self.builder.multicapture),
                "network" => Deserialize::begin(&mut self.builder.network),
                "network_token" => Deserialize::begin(&mut self.builder.network_token),
                "network_transaction_id" => {
                    Deserialize::begin(&mut self.builder.network_transaction_id)
                }
                "overcapture" => Deserialize::begin(&mut self.builder.overcapture),
                "regulated_status" => Deserialize::begin(&mut self.builder.regulated_status),
                "three_d_secure" => Deserialize::begin(&mut self.builder.three_d_secure),
                "wallet" => Deserialize::begin(&mut self.builder.wallet),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount_authorized),
                Some(authorization_code),
                Some(brand),
                Some(capture_before),
                Some(checks),
                Some(country),
                Some(description),
                Some(exp_month),
                Some(exp_year),
                Some(extended_authorization),
                Some(fingerprint),
                Some(funding),
                Some(iin),
                Some(incremental_authorization),
                Some(installments),
                Some(issuer),
                Some(last4),
                Some(mandate),
                Some(moto),
                Some(multicapture),
                Some(network),
                Some(network_token),
                Some(network_transaction_id),
                Some(overcapture),
                Some(regulated_status),
                Some(three_d_secure),
                Some(wallet),
            ) = (
                self.builder.amount_authorized,
                self.builder.authorization_code.take(),
                self.builder.brand.take(),
                self.builder.capture_before,
                self.builder.checks.take(),
                self.builder.country.take(),
                self.builder.description.take(),
                self.builder.exp_month,
                self.builder.exp_year,
                self.builder.extended_authorization.take(),
                self.builder.fingerprint.take(),
                self.builder.funding.take(),
                self.builder.iin.take(),
                self.builder.incremental_authorization.take(),
                self.builder.installments.take(),
                self.builder.issuer.take(),
                self.builder.last4.take(),
                self.builder.mandate.take(),
                self.builder.moto,
                self.builder.multicapture.take(),
                self.builder.network.take(),
                self.builder.network_token,
                self.builder.network_transaction_id.take(),
                self.builder.overcapture.take(),
                self.builder.regulated_status.take(),
                self.builder.three_d_secure.take(),
                self.builder.wallet.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsCard {
                amount_authorized,
                authorization_code,
                brand,
                capture_before,
                checks,
                country,
                description,
                exp_month,
                exp_year,
                extended_authorization,
                fingerprint,
                funding,
                iin,
                incremental_authorization,
                installments,
                issuer,
                last4,
                mandate,
                moto,
                multicapture,
                network,
                network_token,
                network_transaction_id,
                overcapture,
                regulated_status,
                three_d_secure,
                wallet,
            });
            Ok(())
        }
    }
};
/// Status of a card based on the card issuer.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodDetailsCardRegulatedStatus {
    Regulated,
    Unregulated,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodDetailsCardRegulatedStatus {
    pub fn as_str(&self) -> &str {
        use PaymentMethodDetailsCardRegulatedStatus::*;
        match self {
            Regulated => "regulated",
            Unregulated => "unregulated",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCardRegulatedStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsCardRegulatedStatus::*;
        match s {
            "regulated" => Ok(Regulated),
            "unregulated" => Ok(Unregulated),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodDetailsCardRegulatedStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsCardRegulatedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodDetailsCardRegulatedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsCardRegulatedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodDetailsCardRegulatedStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDetailsCardRegulatedStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentMethodDetailsCardRegulatedStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentMethodDetailsCardRegulatedStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodDetailsCardRegulatedStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCardRegulatedStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
