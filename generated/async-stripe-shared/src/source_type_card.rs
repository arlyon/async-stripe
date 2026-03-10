#[derive(Clone, Debug, Eq, PartialEq)]
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: SourceTypeCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeCardBuilder {
        type Out = SourceTypeCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address_line1_check" => Deserialize::begin(&mut self.address_line1_check),
                "address_zip_check" => Deserialize::begin(&mut self.address_zip_check),
                "brand" => Deserialize::begin(&mut self.brand),
                "country" => Deserialize::begin(&mut self.country),
                "cvc_check" => Deserialize::begin(&mut self.cvc_check),
                "description" => Deserialize::begin(&mut self.description),
                "dynamic_last4" => Deserialize::begin(&mut self.dynamic_last4),
                "exp_month" => Deserialize::begin(&mut self.exp_month),
                "exp_year" => Deserialize::begin(&mut self.exp_year),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "funding" => Deserialize::begin(&mut self.funding),
                "iin" => Deserialize::begin(&mut self.iin),
                "issuer" => Deserialize::begin(&mut self.issuer),
                "last4" => Deserialize::begin(&mut self.last4),
                "name" => Deserialize::begin(&mut self.name),
                "three_d_secure" => Deserialize::begin(&mut self.three_d_secure),
                "tokenization_method" => Deserialize::begin(&mut self.tokenization_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.address_line1_check.take(),
                self.address_zip_check.take(),
                self.brand.take(),
                self.country.take(),
                self.cvc_check.take(),
                self.description.take(),
                self.dynamic_last4.take(),
                self.exp_month,
                self.exp_year,
                self.fingerprint.take(),
                self.funding.take(),
                self.iin.take(),
                self.issuer.take(),
                self.last4.take(),
                self.name.take(),
                self.three_d_secure.take(),
                self.tokenization_method.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
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

    impl ObjectDeser for SourceTypeCard {
        type Builder = SourceTypeCardBuilder;
    }

    impl FromValueOpt for SourceTypeCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTypeCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address_line1_check" => b.address_line1_check = FromValueOpt::from_value(v),
                    "address_zip_check" => b.address_zip_check = FromValueOpt::from_value(v),
                    "brand" => b.brand = FromValueOpt::from_value(v),
                    "country" => b.country = FromValueOpt::from_value(v),
                    "cvc_check" => b.cvc_check = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "dynamic_last4" => b.dynamic_last4 = FromValueOpt::from_value(v),
                    "exp_month" => b.exp_month = FromValueOpt::from_value(v),
                    "exp_year" => b.exp_year = FromValueOpt::from_value(v),
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "funding" => b.funding = FromValueOpt::from_value(v),
                    "iin" => b.iin = FromValueOpt::from_value(v),
                    "issuer" => b.issuer = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "three_d_secure" => b.three_d_secure = FromValueOpt::from_value(v),
                    "tokenization_method" => b.tokenization_method = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
