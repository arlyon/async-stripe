#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationMerchantData {
    /// A categorization of the seller's type of business.
    /// See our [merchant categories guide](https://docs.stripe.com/issuing/merchant-categories) for a list of possible values.
    pub category: String,
    /// The merchant category code for the seller’s business
    pub category_code: String,
    /// City where the seller is located
    pub city: Option<String>,
    /// Country where the seller is located
    pub country: Option<String>,
    /// Name of the seller
    pub name: Option<String>,
    /// Identifier assigned to the seller by the card network.
    /// Different card networks may assign different network_id fields to the same merchant.
    pub network_id: String,
    /// Postal code where the seller is located
    pub postal_code: Option<String>,
    /// State where the seller is located
    pub state: Option<String>,
    /// The seller's tax identification number. Currently populated for French merchants only.
    pub tax_id: Option<String>,
    /// An ID assigned by the seller to the location of the sale.
    pub terminal_id: Option<String>,
    /// URL provided by the merchant on a 3DS request
    pub url: Option<String>,
}
#[doc(hidden)]
pub struct IssuingAuthorizationMerchantDataBuilder {
    category: Option<String>,
    category_code: Option<String>,
    city: Option<Option<String>>,
    country: Option<Option<String>>,
    name: Option<Option<String>>,
    network_id: Option<String>,
    postal_code: Option<Option<String>>,
    state: Option<Option<String>>,
    tax_id: Option<Option<String>>,
    terminal_id: Option<Option<String>>,
    url: Option<Option<String>>,
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

    impl Deserialize for IssuingAuthorizationMerchantData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationMerchantData>,
        builder: IssuingAuthorizationMerchantDataBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationMerchantData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationMerchantDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationMerchantDataBuilder {
        type Out = IssuingAuthorizationMerchantData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "category" => Deserialize::begin(&mut self.category),
                "category_code" => Deserialize::begin(&mut self.category_code),
                "city" => Deserialize::begin(&mut self.city),
                "country" => Deserialize::begin(&mut self.country),
                "name" => Deserialize::begin(&mut self.name),
                "network_id" => Deserialize::begin(&mut self.network_id),
                "postal_code" => Deserialize::begin(&mut self.postal_code),
                "state" => Deserialize::begin(&mut self.state),
                "tax_id" => Deserialize::begin(&mut self.tax_id),
                "terminal_id" => Deserialize::begin(&mut self.terminal_id),
                "url" => Deserialize::begin(&mut self.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                category: Deserialize::default(),
                category_code: Deserialize::default(),
                city: Deserialize::default(),
                country: Deserialize::default(),
                name: Deserialize::default(),
                network_id: Deserialize::default(),
                postal_code: Deserialize::default(),
                state: Deserialize::default(),
                tax_id: Deserialize::default(),
                terminal_id: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(category),
                Some(category_code),
                Some(city),
                Some(country),
                Some(name),
                Some(network_id),
                Some(postal_code),
                Some(state),
                Some(tax_id),
                Some(terminal_id),
                Some(url),
            ) = (
                self.category.take(),
                self.category_code.take(),
                self.city.take(),
                self.country.take(),
                self.name.take(),
                self.network_id.take(),
                self.postal_code.take(),
                self.state.take(),
                self.tax_id.take(),
                self.terminal_id.take(),
                self.url.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                category,
                category_code,
                city,
                country,
                name,
                network_id,
                postal_code,
                state,
                tax_id,
                terminal_id,
                url,
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

    impl ObjectDeser for IssuingAuthorizationMerchantData {
        type Builder = IssuingAuthorizationMerchantDataBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationMerchantData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingAuthorizationMerchantDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "category" => b.category = FromValueOpt::from_value(v),
                    "category_code" => b.category_code = FromValueOpt::from_value(v),
                    "city" => b.city = FromValueOpt::from_value(v),
                    "country" => b.country = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "network_id" => b.network_id = FromValueOpt::from_value(v),
                    "postal_code" => b.postal_code = FromValueOpt::from_value(v),
                    "state" => b.state = FromValueOpt::from_value(v),
                    "tax_id" => b.tax_id = FromValueOpt::from_value(v),
                    "terminal_id" => b.terminal_id = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
