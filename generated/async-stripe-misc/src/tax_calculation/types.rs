/// A Tax Calculation allows you to calculate the tax to collect from your customer.
///
/// Related guide: [Calculate tax in your custom payment flow](https://stripe.com/docs/tax/custom)
///
/// For more details see <<https://stripe.com/docs/api/tax/calculations/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxCalculation {
    /// Total amount after taxes in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_total: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of an existing [Customer](https://stripe.com/docs/api/customers/object) used for the resource.
    pub customer: Option<String>,
    pub customer_details: stripe_misc::TaxProductResourceCustomerDetails,
    /// Timestamp of date at which the tax calculation will expire.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the calculation.
    pub id: Option<stripe_misc::TaxCalculationId>,
    /// The list of items the customer is purchasing.
    pub line_items: Option<stripe_types::List<stripe_misc::TaxCalculationLineItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The details of the ship from location, such as the address.
    pub ship_from_details: Option<stripe_misc::TaxProductResourceShipFromDetails>,
    /// The shipping cost details for the calculation.
    pub shipping_cost: Option<stripe_misc::TaxProductResourceTaxCalculationShippingCost>,
    /// The amount of tax to be collected on top of the line item prices.
    pub tax_amount_exclusive: i64,
    /// The amount of tax already included in the line item prices.
    pub tax_amount_inclusive: i64,
    /// Breakdown of individual tax amounts that add up to the total.
    pub tax_breakdown: Vec<stripe_misc::TaxProductResourceTaxBreakdown>,
    /// Timestamp of date at which the tax rules and rates in effect applies for the calculation.
    pub tax_date: stripe_types::Timestamp,
}
#[doc(hidden)]
pub struct TaxCalculationBuilder {
    amount_total: Option<i64>,
    currency: Option<stripe_types::Currency>,
    customer: Option<Option<String>>,
    customer_details: Option<stripe_misc::TaxProductResourceCustomerDetails>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    id: Option<Option<stripe_misc::TaxCalculationId>>,
    line_items: Option<Option<stripe_types::List<stripe_misc::TaxCalculationLineItem>>>,
    livemode: Option<bool>,
    ship_from_details: Option<Option<stripe_misc::TaxProductResourceShipFromDetails>>,
    shipping_cost: Option<Option<stripe_misc::TaxProductResourceTaxCalculationShippingCost>>,
    tax_amount_exclusive: Option<i64>,
    tax_amount_inclusive: Option<i64>,
    tax_breakdown: Option<Vec<stripe_misc::TaxProductResourceTaxBreakdown>>,
    tax_date: Option<stripe_types::Timestamp>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxCalculation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxCalculation>,
        builder: TaxCalculationBuilder,
    }

    impl Visitor for Place<TaxCalculation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxCalculationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxCalculationBuilder {
        type Out = TaxCalculation;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_total" => Deserialize::begin(&mut self.amount_total),
                "currency" => Deserialize::begin(&mut self.currency),
                "customer" => Deserialize::begin(&mut self.customer),
                "customer_details" => Deserialize::begin(&mut self.customer_details),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "id" => Deserialize::begin(&mut self.id),
                "line_items" => Deserialize::begin(&mut self.line_items),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "ship_from_details" => Deserialize::begin(&mut self.ship_from_details),
                "shipping_cost" => Deserialize::begin(&mut self.shipping_cost),
                "tax_amount_exclusive" => Deserialize::begin(&mut self.tax_amount_exclusive),
                "tax_amount_inclusive" => Deserialize::begin(&mut self.tax_amount_inclusive),
                "tax_breakdown" => Deserialize::begin(&mut self.tax_breakdown),
                "tax_date" => Deserialize::begin(&mut self.tax_date),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_total: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                customer_details: Deserialize::default(),
                expires_at: Deserialize::default(),
                id: Deserialize::default(),
                line_items: Deserialize::default(),
                livemode: Deserialize::default(),
                ship_from_details: Deserialize::default(),
                shipping_cost: Deserialize::default(),
                tax_amount_exclusive: Deserialize::default(),
                tax_amount_inclusive: Deserialize::default(),
                tax_breakdown: Deserialize::default(),
                tax_date: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount_total),
                Some(currency),
                Some(customer),
                Some(customer_details),
                Some(expires_at),
                Some(id),
                Some(line_items),
                Some(livemode),
                Some(ship_from_details),
                Some(shipping_cost),
                Some(tax_amount_exclusive),
                Some(tax_amount_inclusive),
                Some(tax_breakdown),
                Some(tax_date),
            ) = (
                self.amount_total,
                self.currency,
                self.customer.take(),
                self.customer_details.take(),
                self.expires_at,
                self.id.take(),
                self.line_items.take(),
                self.livemode,
                self.ship_from_details.take(),
                self.shipping_cost.take(),
                self.tax_amount_exclusive,
                self.tax_amount_inclusive,
                self.tax_breakdown.take(),
                self.tax_date,
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount_total,
                currency,
                customer,
                customer_details,
                expires_at,
                id,
                line_items,
                livemode,
                ship_from_details,
                shipping_cost,
                tax_amount_exclusive,
                tax_amount_inclusive,
                tax_breakdown,
                tax_date,
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

    impl ObjectDeser for TaxCalculation {
        type Builder = TaxCalculationBuilder;
    }

    impl FromValueOpt for TaxCalculation {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxCalculationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_total" => b.amount_total = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "customer_details" => b.customer_details = FromValueOpt::from_value(v),
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "line_items" => b.line_items = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "ship_from_details" => b.ship_from_details = FromValueOpt::from_value(v),
                    "shipping_cost" => b.shipping_cost = FromValueOpt::from_value(v),
                    "tax_amount_exclusive" => b.tax_amount_exclusive = FromValueOpt::from_value(v),
                    "tax_amount_inclusive" => b.tax_amount_inclusive = FromValueOpt::from_value(v),
                    "tax_breakdown" => b.tax_breakdown = FromValueOpt::from_value(v),
                    "tax_date" => b.tax_date = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxCalculation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TaxCalculation", 15)?;
        s.serialize_field("amount_total", &self.amount_total)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("customer_details", &self.customer_details)?;
        s.serialize_field("expires_at", &self.expires_at)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("line_items", &self.line_items)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("ship_from_details", &self.ship_from_details)?;
        s.serialize_field("shipping_cost", &self.shipping_cost)?;
        s.serialize_field("tax_amount_exclusive", &self.tax_amount_exclusive)?;
        s.serialize_field("tax_amount_inclusive", &self.tax_amount_inclusive)?;
        s.serialize_field("tax_breakdown", &self.tax_breakdown)?;
        s.serialize_field("tax_date", &self.tax_date)?;

        s.serialize_field("object", "tax.calculation")?;
        s.end()
    }
}
impl stripe_types::Object for TaxCalculation {
    type Id = Option<stripe_misc::TaxCalculationId>;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TaxCalculationId);
