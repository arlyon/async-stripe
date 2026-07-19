#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationMerchantData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationMerchantData").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: IssuingAuthorizationMerchantDataBuilder {
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
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "category" => Deserialize::begin(&mut self.builder.category),
                "category_code" => Deserialize::begin(&mut self.builder.category_code),
                "city" => Deserialize::begin(&mut self.builder.city),
                "country" => Deserialize::begin(&mut self.builder.country),
                "name" => Deserialize::begin(&mut self.builder.name),
                "network_id" => Deserialize::begin(&mut self.builder.network_id),
                "postal_code" => Deserialize::begin(&mut self.builder.postal_code),
                "state" => Deserialize::begin(&mut self.builder.state),
                "tax_id" => Deserialize::begin(&mut self.builder.tax_id),
                "terminal_id" => Deserialize::begin(&mut self.builder.terminal_id),
                "url" => Deserialize::begin(&mut self.builder.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.category.take(),
                self.builder.category_code.take(),
                self.builder.city.take(),
                self.builder.country.take(),
                self.builder.name.take(),
                self.builder.network_id.take(),
                self.builder.postal_code.take(),
                self.builder.state.take(),
                self.builder.tax_id.take(),
                self.builder.terminal_id.take(),
                self.builder.url.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingAuthorizationMerchantData {
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
            });
            Ok(())
        }
    }
};
