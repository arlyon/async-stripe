/// Stripe needs to collect certain pieces of information about each account
/// created. These requirements can differ depending on the account's country. The
/// Country Specs API makes these rules available to your integration.
///
/// You can also view the information from this API call as [an online
/// guide](/docs/connect/required-verification-information).
///
/// For more details see <<https://stripe.com/docs/api/country_specs/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CountrySpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CountrySpec").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: CountrySpecBuilder {
                    default_currency: Deserialize::default(),
                    id: Deserialize::default(),
                    supported_bank_account_currencies: Deserialize::default(),
                    supported_payment_currencies: Deserialize::default(),
                    supported_payment_methods: Deserialize::default(),
                    supported_transfer_countries: Deserialize::default(),
                    verification_fields: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "default_currency" => Deserialize::begin(&mut self.builder.default_currency),
                "id" => Deserialize::begin(&mut self.builder.id),
                "supported_bank_account_currencies" => {
                    Deserialize::begin(&mut self.builder.supported_bank_account_currencies)
                }
                "supported_payment_currencies" => {
                    Deserialize::begin(&mut self.builder.supported_payment_currencies)
                }
                "supported_payment_methods" => {
                    Deserialize::begin(&mut self.builder.supported_payment_methods)
                }
                "supported_transfer_countries" => {
                    Deserialize::begin(&mut self.builder.supported_transfer_countries)
                }
                "verification_fields" => Deserialize::begin(&mut self.builder.verification_fields),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(default_currency),
                Some(id),
                Some(supported_bank_account_currencies),
                Some(supported_payment_currencies),
                Some(supported_payment_methods),
                Some(supported_transfer_countries),
                Some(verification_fields),
            ) = (
                self.builder.default_currency.take(),
                self.builder.id.take(),
                self.builder.supported_bank_account_currencies.take(),
                self.builder.supported_payment_currencies.take(),
                self.builder.supported_payment_methods.take(),
                self.builder.supported_transfer_countries.take(),
                self.builder.verification_fields.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(CountrySpec {
                default_currency,
                id,
                supported_bank_account_currencies,
                supported_payment_currencies,
                supported_payment_methods,
                supported_transfer_countries,
                verification_fields,
            });
            Ok(())
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
