#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodCard {
    /// Card brand.
    /// Can be `amex`, `diners`, `discover`, `eftpos_au`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: String,
    /// Checks on Card address and CVC if provided.
    pub checks: Option<stripe_shared::PaymentMethodCardChecks>,
    /// Two-letter ISO code representing the country of the card.
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,
    /// A high-level description of the type of cards issued in this range.
    /// (For internal use only and not typically available in standard API requests.).
    pub description: Option<String>,
    /// The brand to use when displaying the card, this accounts for customer's brand choice on dual-branded cards.
    /// Can be `american_express`, `cartes_bancaires`, `diners_club`, `discover`, `eftpos_australia`, `interac`, `jcb`, `mastercard`, `union_pay`, `visa`, or `other` and may contain more values in the future.
    pub display_brand: Option<String>,
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
    pub funding: String,
    /// Issuer identification number of the card.
    /// (For internal use only and not typically available in standard API requests.).
    pub iin: Option<String>,
    /// The name of the card's issuing bank.
    /// (For internal use only and not typically available in standard API requests.).
    pub issuer: Option<String>,
    /// The last four digits of the card.
    pub last4: String,
    /// Contains information about card networks that can be used to process the payment.
    pub networks: Option<stripe_shared::Networks>,
    /// Contains details on how this Card may be used for 3D Secure authentication.
    pub three_d_secure_usage: Option<stripe_shared::ThreeDSecureUsage>,
    /// If this Card is part of a card wallet, this contains the details of the card wallet.
    pub wallet: Option<stripe_shared::PaymentMethodCardWallet>,
}
#[doc(hidden)]
pub struct PaymentMethodCardBuilder {
    brand: Option<String>,
    checks: Option<Option<stripe_shared::PaymentMethodCardChecks>>,
    country: Option<Option<String>>,
    description: Option<Option<String>>,
    display_brand: Option<Option<String>>,
    exp_month: Option<i64>,
    exp_year: Option<i64>,
    fingerprint: Option<Option<String>>,
    funding: Option<String>,
    iin: Option<Option<String>>,
    issuer: Option<Option<String>>,
    last4: Option<String>,
    networks: Option<Option<stripe_shared::Networks>>,
    three_d_secure_usage: Option<Option<stripe_shared::ThreeDSecureUsage>>,
    wallet: Option<Option<stripe_shared::PaymentMethodCardWallet>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodCard>,
        builder: PaymentMethodCardBuilder,
    }

    impl Visitor for Place<PaymentMethodCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodCardBuilder {
        type Out = PaymentMethodCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "brand" => Deserialize::begin(&mut self.brand),
                "checks" => Deserialize::begin(&mut self.checks),
                "country" => Deserialize::begin(&mut self.country),
                "description" => Deserialize::begin(&mut self.description),
                "display_brand" => Deserialize::begin(&mut self.display_brand),
                "exp_month" => Deserialize::begin(&mut self.exp_month),
                "exp_year" => Deserialize::begin(&mut self.exp_year),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "funding" => Deserialize::begin(&mut self.funding),
                "iin" => Deserialize::begin(&mut self.iin),
                "issuer" => Deserialize::begin(&mut self.issuer),
                "last4" => Deserialize::begin(&mut self.last4),
                "networks" => Deserialize::begin(&mut self.networks),
                "three_d_secure_usage" => Deserialize::begin(&mut self.three_d_secure_usage),
                "wallet" => Deserialize::begin(&mut self.wallet),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                brand: Deserialize::default(),
                checks: Deserialize::default(),
                country: Deserialize::default(),
                description: Deserialize::default(),
                display_brand: Deserialize::default(),
                exp_month: Deserialize::default(),
                exp_year: Deserialize::default(),
                fingerprint: Deserialize::default(),
                funding: Deserialize::default(),
                iin: Deserialize::default(),
                issuer: Deserialize::default(),
                last4: Deserialize::default(),
                networks: Deserialize::default(),
                three_d_secure_usage: Deserialize::default(),
                wallet: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                brand: self.brand.take()?,
                checks: self.checks.take()?,
                country: self.country.take()?,
                description: self.description.take()?,
                display_brand: self.display_brand.take()?,
                exp_month: self.exp_month?,
                exp_year: self.exp_year?,
                fingerprint: self.fingerprint.take()?,
                funding: self.funding.take()?,
                iin: self.iin.take()?,
                issuer: self.issuer.take()?,
                last4: self.last4.take()?,
                networks: self.networks.take()?,
                three_d_secure_usage: self.three_d_secure_usage?,
                wallet: self.wallet.take()?,
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

    impl ObjectDeser for PaymentMethodCard {
        type Builder = PaymentMethodCardBuilder;
    }

    impl FromValueOpt for PaymentMethodCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "brand" => b.brand = Some(FromValueOpt::from_value(v)?),
                    "checks" => b.checks = Some(FromValueOpt::from_value(v)?),
                    "country" => b.country = Some(FromValueOpt::from_value(v)?),
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "display_brand" => b.display_brand = Some(FromValueOpt::from_value(v)?),
                    "exp_month" => b.exp_month = Some(FromValueOpt::from_value(v)?),
                    "exp_year" => b.exp_year = Some(FromValueOpt::from_value(v)?),
                    "fingerprint" => b.fingerprint = Some(FromValueOpt::from_value(v)?),
                    "funding" => b.funding = Some(FromValueOpt::from_value(v)?),
                    "iin" => b.iin = Some(FromValueOpt::from_value(v)?),
                    "issuer" => b.issuer = Some(FromValueOpt::from_value(v)?),
                    "last4" => b.last4 = Some(FromValueOpt::from_value(v)?),
                    "networks" => b.networks = Some(FromValueOpt::from_value(v)?),
                    "three_d_secure_usage" => {
                        b.three_d_secure_usage = Some(FromValueOpt::from_value(v)?)
                    }
                    "wallet" => b.wallet = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
