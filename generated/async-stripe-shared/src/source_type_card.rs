#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeCard {
    pub address_line1_check: Option<String>,
    pub address_zip_check: Option<String>,
    pub brand: Option<String>,
    pub country: Option<String>,
    pub cvc_check: Option<String>,
    pub description: Option<String>,
    pub dynamic_last4: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
    pub fingerprint: Option<String>,
    pub funding: Option<String>,
    pub iin: Option<String>,
    pub issuer: Option<String>,
    pub last4: Option<String>,
    pub name: Option<String>,
    pub three_d_secure: Option<String>,
    pub tokenization_method: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTypeCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTypeCard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceTypeCardBuilder {
    address_line1_check: Option<Option<String>>,
    address_zip_check: Option<Option<String>>,
    brand: Option<Option<String>>,
    country: Option<Option<String>>,
    cvc_check: Option<Option<String>>,
    description: Option<Option<String>>,
    dynamic_last4: Option<Option<String>>,
    exp_month: Option<Option<i64>>,
    exp_year: Option<Option<i64>>,
    fingerprint: Option<Option<String>>,
    funding: Option<Option<String>>,
    iin: Option<Option<String>>,
    issuer: Option<Option<String>>,
    last4: Option<Option<String>>,
    name: Option<Option<String>>,
    three_d_secure: Option<Option<String>>,
    tokenization_method: Option<Option<String>>,
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

    impl Deserialize for SourceTypeCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeCard>,
        builder: SourceTypeCardBuilder,
    }

    impl Visitor for Place<SourceTypeCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeCardBuilder {
                    address_line1_check: Deserialize::default(),
                    address_zip_check: Deserialize::default(),
                    brand: Deserialize::default(),
                    country: Deserialize::default(),
                    cvc_check: Deserialize::default(),
                    description: Deserialize::default(),
                    dynamic_last4: Deserialize::default(),
                    exp_month: Deserialize::default(),
                    exp_year: Deserialize::default(),
                    fingerprint: Deserialize::default(),
                    funding: Deserialize::default(),
                    iin: Deserialize::default(),
                    issuer: Deserialize::default(),
                    last4: Deserialize::default(),
                    name: Deserialize::default(),
                    three_d_secure: Deserialize::default(),
                    tokenization_method: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address_line1_check" => Deserialize::begin(&mut self.builder.address_line1_check),
                "address_zip_check" => Deserialize::begin(&mut self.builder.address_zip_check),
                "brand" => Deserialize::begin(&mut self.builder.brand),
                "country" => Deserialize::begin(&mut self.builder.country),
                "cvc_check" => Deserialize::begin(&mut self.builder.cvc_check),
                "description" => Deserialize::begin(&mut self.builder.description),
                "dynamic_last4" => Deserialize::begin(&mut self.builder.dynamic_last4),
                "exp_month" => Deserialize::begin(&mut self.builder.exp_month),
                "exp_year" => Deserialize::begin(&mut self.builder.exp_year),
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "funding" => Deserialize::begin(&mut self.builder.funding),
                "iin" => Deserialize::begin(&mut self.builder.iin),
                "issuer" => Deserialize::begin(&mut self.builder.issuer),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "name" => Deserialize::begin(&mut self.builder.name),
                "three_d_secure" => Deserialize::begin(&mut self.builder.three_d_secure),
                "tokenization_method" => Deserialize::begin(&mut self.builder.tokenization_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(address_line1_check),
                Some(address_zip_check),
                Some(brand),
                Some(country),
                Some(cvc_check),
                Some(description),
                Some(dynamic_last4),
                Some(exp_month),
                Some(exp_year),
                Some(fingerprint),
                Some(funding),
                Some(iin),
                Some(issuer),
                Some(last4),
                Some(name),
                Some(three_d_secure),
                Some(tokenization_method),
            ) = (
                self.builder.address_line1_check.take(),
                self.builder.address_zip_check.take(),
                self.builder.brand.take(),
                self.builder.country.take(),
                self.builder.cvc_check.take(),
                self.builder.description.take(),
                self.builder.dynamic_last4.take(),
                self.builder.exp_month,
                self.builder.exp_year,
                self.builder.fingerprint.take(),
                self.builder.funding.take(),
                self.builder.iin.take(),
                self.builder.issuer.take(),
                self.builder.last4.take(),
                self.builder.name.take(),
                self.builder.three_d_secure.take(),
                self.builder.tokenization_method.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SourceTypeCard {
                address_line1_check,
                address_zip_check,
                brand,
                country,
                cvc_check,
                description,
                dynamic_last4,
                exp_month,
                exp_year,
                fingerprint,
                funding,
                iin,
                issuer,
                last4,
                name,
                three_d_secure,
                tokenization_method,
            });
            Ok(())
        }
    }
};
