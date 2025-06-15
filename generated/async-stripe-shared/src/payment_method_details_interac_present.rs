#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsInteracPresent {
    /// Card brand. Can be `interac`, `mastercard` or `visa`.
    pub brand: Option<String>,
    /// The cardholder name as read from the card, in [ISO 7813](https://en.wikipedia.org/wiki/ISO/IEC_7813) format.
    /// May include alphanumeric characters, special characters and first/last name separator (`/`).
    /// In some cases, the cardholder name may not be available depending on how the issuer has configured the card.
    /// Cardholder name is typically not available on swipe or contactless payments, such as those made with Apple Pay and Google Pay.
    pub cardholder_name: Option<String>,
    /// Two-letter ISO code representing the country of the card.
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,
    /// A high-level description of the type of cards issued in this range.
    /// (For internal use only and not typically available in standard API requests.).
    pub description: Option<String>,
    /// Authorization response cryptogram.
    pub emv_auth_data: Option<String>,
    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,
    /// Uniquely identifies this particular card number.
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.
    ///
    /// *As of May 1, 2021, card fingerprint in India for Connect changed to allow two fingerprints for the same card---one for India and one for the rest of the world.*.
    pub fingerprint: Option<String>,
    /// Card funding type. Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: Option<String>,
    /// ID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions.
    /// Only present if it was possible to generate a card PaymentMethod.
    pub generated_card: Option<String>,
    /// Issuer identification number of the card.
    /// (For internal use only and not typically available in standard API requests.).
    pub iin: Option<String>,
    /// The name of the card's issuing bank.
    /// (For internal use only and not typically available in standard API requests.).
    pub issuer: Option<String>,
    /// The last four digits of the card.
    pub last4: Option<String>,
    /// Identifies which network this charge was processed on.
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `eftpos_au`, `interac`, `jcb`, `link`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub network: Option<String>,
    /// This is used by the financial networks to identify a transaction.
    /// Visa calls this the Transaction ID, Mastercard calls this the Trace ID, and American Express calls this the Acquirer Reference Data.
    /// This value will be present if it is returned by the financial network in the authorization response, and null otherwise.
    pub network_transaction_id: Option<String>,
    /// EMV tag 5F2D. Preferred languages specified by the integrated circuit chip.
    pub preferred_locales: Option<Vec<String>>,
    /// How card details were read in this transaction.
    pub read_method: Option<PaymentMethodDetailsInteracPresentReadMethod>,
    /// A collection of fields required to be displayed on receipts. Only required for EMV transactions.
    pub receipt: Option<stripe_shared::PaymentMethodDetailsInteracPresentReceipt>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsInteracPresentBuilder {
    brand: Option<Option<String>>,
    cardholder_name: Option<Option<String>>,
    country: Option<Option<String>>,
    description: Option<Option<String>>,
    emv_auth_data: Option<Option<String>>,
    exp_month: Option<i64>,
    exp_year: Option<i64>,
    fingerprint: Option<Option<String>>,
    funding: Option<Option<String>>,
    generated_card: Option<Option<String>>,
    iin: Option<Option<String>>,
    issuer: Option<Option<String>>,
    last4: Option<Option<String>>,
    network: Option<Option<String>>,
    network_transaction_id: Option<Option<String>>,
    preferred_locales: Option<Option<Vec<String>>>,
    read_method: Option<Option<PaymentMethodDetailsInteracPresentReadMethod>>,
    receipt: Option<Option<stripe_shared::PaymentMethodDetailsInteracPresentReceipt>>,
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

    impl Deserialize for PaymentMethodDetailsInteracPresent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsInteracPresent>,
        builder: PaymentMethodDetailsInteracPresentBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsInteracPresent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsInteracPresentBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsInteracPresentBuilder {
        type Out = PaymentMethodDetailsInteracPresent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "brand" => Deserialize::begin(&mut self.brand),
                "cardholder_name" => Deserialize::begin(&mut self.cardholder_name),
                "country" => Deserialize::begin(&mut self.country),
                "description" => Deserialize::begin(&mut self.description),
                "emv_auth_data" => Deserialize::begin(&mut self.emv_auth_data),
                "exp_month" => Deserialize::begin(&mut self.exp_month),
                "exp_year" => Deserialize::begin(&mut self.exp_year),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "funding" => Deserialize::begin(&mut self.funding),
                "generated_card" => Deserialize::begin(&mut self.generated_card),
                "iin" => Deserialize::begin(&mut self.iin),
                "issuer" => Deserialize::begin(&mut self.issuer),
                "last4" => Deserialize::begin(&mut self.last4),
                "network" => Deserialize::begin(&mut self.network),
                "network_transaction_id" => Deserialize::begin(&mut self.network_transaction_id),
                "preferred_locales" => Deserialize::begin(&mut self.preferred_locales),
                "read_method" => Deserialize::begin(&mut self.read_method),
                "receipt" => Deserialize::begin(&mut self.receipt),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                brand: Deserialize::default(),
                cardholder_name: Deserialize::default(),
                country: Deserialize::default(),
                description: Deserialize::default(),
                emv_auth_data: Deserialize::default(),
                exp_month: Deserialize::default(),
                exp_year: Deserialize::default(),
                fingerprint: Deserialize::default(),
                funding: Deserialize::default(),
                generated_card: Deserialize::default(),
                iin: Deserialize::default(),
                issuer: Deserialize::default(),
                last4: Deserialize::default(),
                network: Deserialize::default(),
                network_transaction_id: Deserialize::default(),
                preferred_locales: Deserialize::default(),
                read_method: Deserialize::default(),
                receipt: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(brand),
                Some(cardholder_name),
                Some(country),
                Some(description),
                Some(emv_auth_data),
                Some(exp_month),
                Some(exp_year),
                Some(fingerprint),
                Some(funding),
                Some(generated_card),
                Some(iin),
                Some(issuer),
                Some(last4),
                Some(network),
                Some(network_transaction_id),
                Some(preferred_locales),
                Some(read_method),
                Some(receipt),
            ) = (
                self.brand.take(),
                self.cardholder_name.take(),
                self.country.take(),
                self.description.take(),
                self.emv_auth_data.take(),
                self.exp_month,
                self.exp_year,
                self.fingerprint.take(),
                self.funding.take(),
                self.generated_card.take(),
                self.iin.take(),
                self.issuer.take(),
                self.last4.take(),
                self.network.take(),
                self.network_transaction_id.take(),
                self.preferred_locales.take(),
                self.read_method,
                self.receipt.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                brand,
                cardholder_name,
                country,
                description,
                emv_auth_data,
                exp_month,
                exp_year,
                fingerprint,
                funding,
                generated_card,
                iin,
                issuer,
                last4,
                network,
                network_transaction_id,
                preferred_locales,
                read_method,
                receipt,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PaymentMethodDetailsInteracPresent {
        type Builder = PaymentMethodDetailsInteracPresentBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsInteracPresent {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsInteracPresentBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "brand" => b.brand = FromValueOpt::from_value(v),
                    "cardholder_name" => b.cardholder_name = FromValueOpt::from_value(v),
                    "country" => b.country = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "emv_auth_data" => b.emv_auth_data = FromValueOpt::from_value(v),
                    "exp_month" => b.exp_month = FromValueOpt::from_value(v),
                    "exp_year" => b.exp_year = FromValueOpt::from_value(v),
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "funding" => b.funding = FromValueOpt::from_value(v),
                    "generated_card" => b.generated_card = FromValueOpt::from_value(v),
                    "iin" => b.iin = FromValueOpt::from_value(v),
                    "issuer" => b.issuer = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "network" => b.network = FromValueOpt::from_value(v),
                    "network_transaction_id" => {
                        b.network_transaction_id = FromValueOpt::from_value(v)
                    }
                    "preferred_locales" => b.preferred_locales = FromValueOpt::from_value(v),
                    "read_method" => b.read_method = FromValueOpt::from_value(v),
                    "receipt" => b.receipt = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// How card details were read in this transaction.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsInteracPresentReadMethod {
    ContactEmv,
    ContactlessEmv,
    ContactlessMagstripeMode,
    MagneticStripeFallback,
    MagneticStripeTrack2,
}
impl PaymentMethodDetailsInteracPresentReadMethod {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsInteracPresentReadMethod::*;
        match self {
            ContactEmv => "contact_emv",
            ContactlessEmv => "contactless_emv",
            ContactlessMagstripeMode => "contactless_magstripe_mode",
            MagneticStripeFallback => "magnetic_stripe_fallback",
            MagneticStripeTrack2 => "magnetic_stripe_track2",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsInteracPresentReadMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsInteracPresentReadMethod::*;
        match s {
            "contact_emv" => Ok(ContactEmv),
            "contactless_emv" => Ok(ContactlessEmv),
            "contactless_magstripe_mode" => Ok(ContactlessMagstripeMode),
            "magnetic_stripe_fallback" => Ok(MagneticStripeFallback),
            "magnetic_stripe_track2" => Ok(MagneticStripeTrack2),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsInteracPresentReadMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsInteracPresentReadMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDetailsInteracPresentReadMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodDetailsInteracPresentReadMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsInteracPresentReadMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodDetailsInteracPresentReadMethod::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodDetailsInteracPresentReadMethod);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsInteracPresentReadMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodDetailsInteracPresentReadMethod",
            )
        })
    }
}
