#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionFuelData {
    /// [Conexxus Payment System Product Code](https://www.conexxus.org/conexxus-payment-system-product-codes) identifying the primary fuel product purchased.
    pub industry_product_code: Option<String>,
    /// The quantity of `unit`s of fuel that was dispensed, represented as a decimal string with at most 12 decimal places.
    pub quantity_decimal: Option<String>,
    /// The type of fuel that was purchased.
    /// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    /// The units for `quantity_decimal`.
    /// One of `charging_minute`, `imperial_gallon`, `kilogram`, `kilowatt_hour`, `liter`, `pound`, `us_gallon`, or `other`.
    pub unit: String,
    /// The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places.
    pub unit_cost_decimal: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingTransactionFuelData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingTransactionFuelData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingTransactionFuelDataBuilder {
    industry_product_code: Option<Option<String>>,
    quantity_decimal: Option<Option<String>>,
    type_: Option<String>,
    unit: Option<String>,
    unit_cost_decimal: Option<String>,
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

    impl Deserialize for IssuingTransactionFuelData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionFuelData>,
        builder: IssuingTransactionFuelDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionFuelData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionFuelDataBuilder {
                    industry_product_code: Deserialize::default(),
                    quantity_decimal: Deserialize::default(),
                    type_: Deserialize::default(),
                    unit: Deserialize::default(),
                    unit_cost_decimal: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "industry_product_code" => {
                    Deserialize::begin(&mut self.builder.industry_product_code)
                }
                "quantity_decimal" => Deserialize::begin(&mut self.builder.quantity_decimal),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "unit" => Deserialize::begin(&mut self.builder.unit),
                "unit_cost_decimal" => Deserialize::begin(&mut self.builder.unit_cost_decimal),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(industry_product_code),
                Some(quantity_decimal),
                Some(type_),
                Some(unit),
                Some(unit_cost_decimal),
            ) = (
                self.builder.industry_product_code.take(),
                self.builder.quantity_decimal.take(),
                self.builder.type_.take(),
                self.builder.unit.take(),
                self.builder.unit_cost_decimal.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingTransactionFuelData {
                industry_product_code,
                quantity_decimal,
                type_,
                unit,
                unit_cost_decimal,
            });
            Ok(())
        }
    }
};
