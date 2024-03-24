/// A Tax Calculation allows you to calculate the tax to collect from your customer.
///
/// Related guide: [Calculate tax in your custom payment flow](https://stripe.com/docs/tax/custom)
///
/// For more details see <<https://stripe.com/docs/api/tax/calculations/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxCalculation {
    /// Total after taxes.
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
#[cfg(feature = "min-ser")]
pub struct TaxCalculationBuilder {
    amount_total: Option<i64>,
    currency: Option<stripe_types::Currency>,
    customer: Option<Option<String>>,
    customer_details: Option<stripe_misc::TaxProductResourceCustomerDetails>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    id: Option<Option<stripe_misc::TaxCalculationId>>,
    line_items: Option<Option<stripe_types::List<stripe_misc::TaxCalculationLineItem>>>,
    livemode: Option<bool>,
    shipping_cost: Option<Option<stripe_misc::TaxProductResourceTaxCalculationShippingCost>>,
    tax_amount_exclusive: Option<i64>,
    tax_amount_inclusive: Option<i64>,
    tax_breakdown: Option<Vec<stripe_misc::TaxProductResourceTaxBreakdown>>,
    tax_date: Option<stripe_types::Timestamp>,
}

#[cfg(feature = "min-ser")]
#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
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
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxCalculationBuilder::deser_default() }))
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
                shipping_cost: Deserialize::default(),
                tax_amount_exclusive: Deserialize::default(),
                tax_amount_inclusive: Deserialize::default(),
                tax_breakdown: Deserialize::default(),
                tax_date: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_total = self.amount_total.take()?;
            let currency = self.currency.take()?;
            let customer = self.customer.take()?;
            let customer_details = self.customer_details.take()?;
            let expires_at = self.expires_at.take()?;
            let id = self.id.take()?;
            let line_items = self.line_items.take()?;
            let livemode = self.livemode.take()?;
            let shipping_cost = self.shipping_cost.take()?;
            let tax_amount_exclusive = self.tax_amount_exclusive.take()?;
            let tax_amount_inclusive = self.tax_amount_inclusive.take()?;
            let tax_breakdown = self.tax_breakdown.take()?;
            let tax_date = self.tax_date.take()?;

            Some(Self::Out {
                amount_total,
                currency,
                customer,
                customer_details,
                expires_at,
                id,
                line_items,
                livemode,
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
                    "amount_total" => b.amount_total = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "customer" => b.customer = Some(FromValueOpt::from_value(v)?),
                    "customer_details" => b.customer_details = Some(FromValueOpt::from_value(v)?),
                    "expires_at" => b.expires_at = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "line_items" => b.line_items = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "shipping_cost" => b.shipping_cost = Some(FromValueOpt::from_value(v)?),
                    "tax_amount_exclusive" => b.tax_amount_exclusive = Some(FromValueOpt::from_value(v)?),
                    "tax_amount_inclusive" => b.tax_amount_inclusive = Some(FromValueOpt::from_value(v)?),
                    "tax_breakdown" => b.tax_breakdown = Some(FromValueOpt::from_value(v)?),
                    "tax_date" => b.tax_date = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
impl stripe_types::Object for TaxCalculation {
    type Id = Option<stripe_misc::TaxCalculationId>;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TaxCalculationId);
