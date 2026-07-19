#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeThreeDSecure {
    pub address_line1_check: Option<String>,
    pub address_zip_check: Option<String>,
    pub authenticated: Option<bool>,
    pub brand: Option<String>,
    pub card: Option<String>,
    pub country: Option<String>,
    pub customer: Option<String>,
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
impl std::fmt::Debug for SourceTypeThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTypeThreeDSecure").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceTypeThreeDSecureBuilder {
    address_line1_check: Option<Option<String>>,
    address_zip_check: Option<Option<String>>,
    authenticated: Option<Option<bool>>,
    brand: Option<Option<String>>,
    card: Option<Option<String>>,
    country: Option<Option<String>>,
    customer: Option<Option<String>>,
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

    impl Deserialize for SourceTypeThreeDSecure {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeThreeDSecure>,
        builder: SourceTypeThreeDSecureBuilder,
    }

    impl Visitor for Place<SourceTypeThreeDSecure> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeThreeDSecureBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeThreeDSecureBuilder {
        type Out = SourceTypeThreeDSecure;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address_line1_check" => Deserialize::begin(&mut self.address_line1_check),
                "address_zip_check" => Deserialize::begin(&mut self.address_zip_check),
                "authenticated" => Deserialize::begin(&mut self.authenticated),
                "brand" => Deserialize::begin(&mut self.brand),
                "card" => Deserialize::begin(&mut self.card),
                "country" => Deserialize::begin(&mut self.country),
                "customer" => Deserialize::begin(&mut self.customer),
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
                address_line1_check: Some(None),
                address_zip_check: Some(None),
                authenticated: Some(None),
                brand: Some(None),
                card: Some(None),
                country: Some(None),
                customer: Some(None),
                cvc_check: Some(None),
                description: Some(None),
                dynamic_last4: Some(None),
                exp_month: Some(None),
                exp_year: Some(None),
                fingerprint: Some(None),
                funding: Some(None),
                iin: Some(None),
                issuer: Some(None),
                last4: Some(None),
                name: Some(None),
                three_d_secure: Some(None),
                tokenization_method: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                address_line1_check: self.address_line1_check.take().flatten(),
                address_zip_check: self.address_zip_check.take().flatten(),
                authenticated: self.authenticated.flatten(),
                brand: self.brand.take().flatten(),
                card: self.card.take().flatten(),
                country: self.country.take().flatten(),
                customer: self.customer.take().flatten(),
                cvc_check: self.cvc_check.take().flatten(),
                description: self.description.take().flatten(),
                dynamic_last4: self.dynamic_last4.take().flatten(),
                exp_month: self.exp_month.flatten(),
                exp_year: self.exp_year.flatten(),
                fingerprint: self.fingerprint.take().flatten(),
                funding: self.funding.take().flatten(),
                iin: self.iin.take().flatten(),
                issuer: self.issuer.take().flatten(),
                last4: self.last4.take().flatten(),
                name: self.name.take().flatten(),
                three_d_secure: self.three_d_secure.take().flatten(),
                tokenization_method: self.tokenization_method.take().flatten(),
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

    impl ObjectDeser for SourceTypeThreeDSecure {
        type Builder = SourceTypeThreeDSecureBuilder;
    }

    impl FromValueOpt for SourceTypeThreeDSecure {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTypeThreeDSecureBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address_line1_check" => b.address_line1_check = FromValueOpt::from_value(v),
                    "address_zip_check" => b.address_zip_check = FromValueOpt::from_value(v),
                    "authenticated" => b.authenticated = FromValueOpt::from_value(v),
                    "brand" => b.brand = FromValueOpt::from_value(v),
                    "card" => b.card = FromValueOpt::from_value(v),
                    "country" => b.country = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
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
