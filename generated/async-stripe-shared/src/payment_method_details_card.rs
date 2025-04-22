#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCard {
    /// The authorized amount.
pub amount_authorized: Option<i64>,
    /// Authorization code on the charge.
pub authorization_code: Option<String>,
        /// Card brand.
    /// Can be `amex`, `diners`, `discover`, `eftpos_au`, `jcb`, `link`, `mastercard`, `unionpay`, `visa`, or `unknown`.
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
    /// Installment details for this payment (Mexico only).
    ///
        /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: PaymentMethodDetailsCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsCardBuilder {
        type Out = PaymentMethodDetailsCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_authorized" => Deserialize::begin(&mut self.amount_authorized),
                "authorization_code" => Deserialize::begin(&mut self.authorization_code),
                "brand" => Deserialize::begin(&mut self.brand),
                "capture_before" => Deserialize::begin(&mut self.capture_before),
                "checks" => Deserialize::begin(&mut self.checks),
                "country" => Deserialize::begin(&mut self.country),
                "description" => Deserialize::begin(&mut self.description),
                "exp_month" => Deserialize::begin(&mut self.exp_month),
                "exp_year" => Deserialize::begin(&mut self.exp_year),
                "extended_authorization" => Deserialize::begin(&mut self.extended_authorization),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "funding" => Deserialize::begin(&mut self.funding),
                "iin" => Deserialize::begin(&mut self.iin),
                "incremental_authorization" => {
                    Deserialize::begin(&mut self.incremental_authorization)
                }
                "installments" => Deserialize::begin(&mut self.installments),
                "issuer" => Deserialize::begin(&mut self.issuer),
                "last4" => Deserialize::begin(&mut self.last4),
                "mandate" => Deserialize::begin(&mut self.mandate),
                "moto" => Deserialize::begin(&mut self.moto),
                "multicapture" => Deserialize::begin(&mut self.multicapture),
                "network" => Deserialize::begin(&mut self.network),
                "network_token" => Deserialize::begin(&mut self.network_token),
                "network_transaction_id" => Deserialize::begin(&mut self.network_transaction_id),
                "overcapture" => Deserialize::begin(&mut self.overcapture),
                "regulated_status" => Deserialize::begin(&mut self.regulated_status),
                "three_d_secure" => Deserialize::begin(&mut self.three_d_secure),
                "wallet" => Deserialize::begin(&mut self.wallet),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.amount_authorized,
                self.authorization_code.take(),
                self.brand.take(),
                self.capture_before,
                self.checks.take(),
                self.country.take(),
                self.description.take(),
                self.exp_month,
                self.exp_year,
                self.extended_authorization,
                self.fingerprint.take(),
                self.funding.take(),
                self.iin.take(),
                self.incremental_authorization,
                self.installments,
                self.issuer.take(),
                self.last4.take(),
                self.mandate.take(),
                self.moto,
                self.multicapture,
                self.network.take(),
                self.network_token,
                self.network_transaction_id.take(),
                self.overcapture,
                self.regulated_status,
                self.three_d_secure.take(),
                self.wallet.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
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
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PaymentMethodDetailsCard {
        type Builder = PaymentMethodDetailsCardBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_authorized" => b.amount_authorized = FromValueOpt::from_value(v),
                    "authorization_code" => b.authorization_code = FromValueOpt::from_value(v),
                    "brand" => b.brand = FromValueOpt::from_value(v),
                    "capture_before" => b.capture_before = FromValueOpt::from_value(v),
                    "checks" => b.checks = FromValueOpt::from_value(v),
                    "country" => b.country = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "exp_month" => b.exp_month = FromValueOpt::from_value(v),
                    "exp_year" => b.exp_year = FromValueOpt::from_value(v),
                    "extended_authorization" => {
                        b.extended_authorization = FromValueOpt::from_value(v)
                    }
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "funding" => b.funding = FromValueOpt::from_value(v),
                    "iin" => b.iin = FromValueOpt::from_value(v),
                    "incremental_authorization" => {
                        b.incremental_authorization = FromValueOpt::from_value(v)
                    }
                    "installments" => b.installments = FromValueOpt::from_value(v),
                    "issuer" => b.issuer = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "mandate" => b.mandate = FromValueOpt::from_value(v),
                    "moto" => b.moto = FromValueOpt::from_value(v),
                    "multicapture" => b.multicapture = FromValueOpt::from_value(v),
                    "network" => b.network = FromValueOpt::from_value(v),
                    "network_token" => b.network_token = FromValueOpt::from_value(v),
                    "network_transaction_id" => {
                        b.network_transaction_id = FromValueOpt::from_value(v)
                    }
                    "overcapture" => b.overcapture = FromValueOpt::from_value(v),
                    "regulated_status" => b.regulated_status = FromValueOpt::from_value(v),
                    "three_d_secure" => b.three_d_secure = FromValueOpt::from_value(v),
                    "wallet" => b.wallet = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Status of a card based on the card issuer.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsCardRegulatedStatus {
    Regulated,
    Unregulated,
}
impl PaymentMethodDetailsCardRegulatedStatus {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsCardRegulatedStatus::*;
        match self {
            Regulated => "regulated",
            Unregulated => "unregulated",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCardRegulatedStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsCardRegulatedStatus::*;
        match s {
            "regulated" => Ok(Regulated),
            "unregulated" => Ok(Unregulated),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsCardRegulatedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsCardRegulatedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for PaymentMethodDetailsCardRegulatedStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsCardRegulatedStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodDetailsCardRegulatedStatus::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodDetailsCardRegulatedStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCardRegulatedStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentMethodDetailsCardRegulatedStatus")
        })
    }
}
