#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsPassthroughCard {
    /// Card brand.
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `eftpos_au`, `jcb`, `link`, `mastercard`, `unionpay`, `visa` or `unknown`.
    pub brand: Option<String>,
    /// Two-letter ISO code representing the country of the card.
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,
    /// Two-digit number representing the card's expiration month.
    pub exp_month: Option<i64>,
    /// Four-digit number representing the card's expiration year.
    pub exp_year: Option<i64>,
    /// Card funding type. Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: Option<String>,
    /// The last four digits of the card.
    pub last4: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsPassthroughCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsPassthroughCard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsPassthroughCardBuilder {
    brand: Option<Option<String>>,
    country: Option<Option<String>>,
    exp_month: Option<Option<i64>>,
    exp_year: Option<Option<i64>>,
    funding: Option<Option<String>>,
    last4: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsPassthroughCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsPassthroughCard>,
        builder: PaymentMethodDetailsPassthroughCardBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsPassthroughCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsPassthroughCardBuilder {
                    brand: Deserialize::default(),
                    country: Deserialize::default(),
                    exp_month: Deserialize::default(),
                    exp_year: Deserialize::default(),
                    funding: Deserialize::default(),
                    last4: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "brand" => Deserialize::begin(&mut self.builder.brand),
                "country" => Deserialize::begin(&mut self.builder.country),
                "exp_month" => Deserialize::begin(&mut self.builder.exp_month),
                "exp_year" => Deserialize::begin(&mut self.builder.exp_year),
                "funding" => Deserialize::begin(&mut self.builder.funding),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(brand),
                Some(country),
                Some(exp_month),
                Some(exp_year),
                Some(funding),
                Some(last4),
            ) = (
                self.builder.brand.take(),
                self.builder.country.take(),
                self.builder.exp_month,
                self.builder.exp_year,
                self.builder.funding.take(),
                self.builder.last4.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsPassthroughCard {
                brand,
                country,
                exp_month,
                exp_year,
                funding,
                last4,
            });
            Ok(())
        }
    }
};
