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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: SourceTypeThreeDSecureBuilder {
                    address_line1_check: Deserialize::default(),
                    address_zip_check: Deserialize::default(),
                    authenticated: Deserialize::default(),
                    brand: Deserialize::default(),
                    card: Deserialize::default(),
                    country: Deserialize::default(),
                    customer: Deserialize::default(),
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
                "authenticated" => Deserialize::begin(&mut self.builder.authenticated),
                "brand" => Deserialize::begin(&mut self.builder.brand),
                "card" => Deserialize::begin(&mut self.builder.card),
                "country" => Deserialize::begin(&mut self.builder.country),
                "customer" => Deserialize::begin(&mut self.builder.customer),
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
                Some(authenticated),
                Some(brand),
                Some(card),
                Some(country),
                Some(customer),
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
                self.builder.authenticated,
                self.builder.brand.take(),
                self.builder.card.take(),
                self.builder.country.take(),
                self.builder.customer.take(),
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
            *self.out = Some(SourceTypeThreeDSecure {
                address_line1_check,
                address_zip_check,
                authenticated,
                brand,
                card,
                country,
                customer,
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
