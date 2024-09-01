/// Stripe needs to collect certain pieces of information about each account
/// created. These requirements can differ depending on the account's country. The
/// Country Specs API makes these rules available to your integration.
///
/// You can also view the information from this API call as [an online
/// guide](/docs/connect/required-verification-information).
///
/// For more details see <<https://stripe.com/docs/api/country_specs/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CountrySpec {
    /// The default currency for this country. This applies to both payment methods and bank accounts.
    pub default_currency: stripe_types::Currency,
    /// Unique identifier for the object. Represented as the ISO country code for this country.
    pub id: stripe_connect::CountrySpecId,
    /// Currencies that can be accepted in the specific country (for transfers).
    pub supported_bank_account_currencies: std::collections::HashMap<String, Vec<String>>,
    /// Currencies that can be accepted in the specified country (for payments).
    pub supported_payment_currencies: Vec<String>,
    /// Payment methods available in the specified country.
    /// You may need to enable some payment methods (e.g., [ACH](https://stripe.com/docs/ach)) on your account before they appear in this list.
    /// The `stripe` payment method refers to [charging through your platform](https://stripe.com/docs/connect/destination-charges).
    pub supported_payment_methods: Vec<String>,
    /// Countries that can accept transfers from the specified country.
    pub supported_transfer_countries: Vec<String>,
    pub verification_fields: stripe_connect::CountrySpecVerificationFields,
}
#[doc(hidden)]
pub struct CountrySpecBuilder {
    default_currency: Option<stripe_types::Currency>,
    id: Option<stripe_connect::CountrySpecId>,
    supported_bank_account_currencies: Option<std::collections::HashMap<String, Vec<String>>>,
    supported_payment_currencies: Option<Vec<String>>,
    supported_payment_methods: Option<Vec<String>>,
    supported_transfer_countries: Option<Vec<String>>,
    verification_fields: Option<stripe_connect::CountrySpecVerificationFields>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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

    impl Deserialize for CountrySpec {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CountrySpec>,
        builder: CountrySpecBuilder,
    }

    impl Visitor for Place<CountrySpec> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CountrySpecBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CountrySpecBuilder {
        type Out = CountrySpec;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "default_currency" => Deserialize::begin(&mut self.default_currency),
                "id" => Deserialize::begin(&mut self.id),
                "supported_bank_account_currencies" => {
                    Deserialize::begin(&mut self.supported_bank_account_currencies)
                }
                "supported_payment_currencies" => {
                    Deserialize::begin(&mut self.supported_payment_currencies)
                }
                "supported_payment_methods" => {
                    Deserialize::begin(&mut self.supported_payment_methods)
                }
                "supported_transfer_countries" => {
                    Deserialize::begin(&mut self.supported_transfer_countries)
                }
                "verification_fields" => Deserialize::begin(&mut self.verification_fields),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                default_currency: Deserialize::default(),
                id: Deserialize::default(),
                supported_bank_account_currencies: Deserialize::default(),
                supported_payment_currencies: Deserialize::default(),
                supported_payment_methods: Deserialize::default(),
                supported_transfer_countries: Deserialize::default(),
                verification_fields: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(default_currency),
                Some(id),
                Some(supported_bank_account_currencies),
                Some(supported_payment_currencies),
                Some(supported_payment_methods),
                Some(supported_transfer_countries),
                Some(verification_fields),
            ) = (
                self.default_currency,
                self.id.take(),
                self.supported_bank_account_currencies.take(),
                self.supported_payment_currencies.take(),
                self.supported_payment_methods.take(),
                self.supported_transfer_countries.take(),
                self.verification_fields.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                default_currency,
                id,
                supported_bank_account_currencies,
                supported_payment_currencies,
                supported_payment_methods,
                supported_transfer_countries,
                verification_fields,
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

    impl ObjectDeser for CountrySpec {
        type Builder = CountrySpecBuilder;
    }

    impl FromValueOpt for CountrySpec {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CountrySpecBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "default_currency" => b.default_currency = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "supported_bank_account_currencies" => {
                        b.supported_bank_account_currencies = FromValueOpt::from_value(v)
                    }
                    "supported_payment_currencies" => {
                        b.supported_payment_currencies = FromValueOpt::from_value(v)
                    }
                    "supported_payment_methods" => {
                        b.supported_payment_methods = FromValueOpt::from_value(v)
                    }
                    "supported_transfer_countries" => {
                        b.supported_transfer_countries = FromValueOpt::from_value(v)
                    }
                    "verification_fields" => b.verification_fields = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for CountrySpec {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("CountrySpec", 8)?;
        s.serialize_field("default_currency", &self.default_currency)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field(
            "supported_bank_account_currencies",
            &self.supported_bank_account_currencies,
        )?;
        s.serialize_field("supported_payment_currencies", &self.supported_payment_currencies)?;
        s.serialize_field("supported_payment_methods", &self.supported_payment_methods)?;
        s.serialize_field("supported_transfer_countries", &self.supported_transfer_countries)?;
        s.serialize_field("verification_fields", &self.verification_fields)?;

        s.serialize_field("object", "country_spec")?;
        s.end()
    }
}
impl stripe_types::Object for CountrySpec {
    type Id = stripe_connect::CountrySpecId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(CountrySpecId);
