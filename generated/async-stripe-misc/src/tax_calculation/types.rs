/// A Tax Calculation allows you to calculate the tax to collect from your customer.
///
/// Related guide: [Calculate tax in your custom payment flow](https://docs.stripe.com/tax/custom)
///
/// For more details see <<https://stripe.com/docs/api/tax/calculations/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxCalculation {
    /// Total amount after taxes in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units).
    pub amount_total: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of an existing [Customer](https://docs.stripe.com/api/customers/object) used for the resource.
    pub customer: Option<String>,
    pub customer_details: stripe_misc::TaxProductResourceCustomerDetails,
    /// Timestamp of date at which the tax calculation will expire.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the calculation.
    pub id: Option<stripe_misc::TaxCalculationId>,
    /// The list of items the customer is purchasing.
    pub line_items: Option<stripe_types::List<stripe_misc::TaxCalculationLineItem>>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
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
    /// The calculation uses the tax rules and rates that are in effect at this timestamp.
    /// You can use a date up to 31 days in the past or up to 31 days in the future.
    /// If you use a future date, Stripe doesn't guarantee that the expected tax rules and rate being used match the actual rules and rate that will be in effect on that date.
    /// We deploy tax changes before their effective date, but not within a fixed window.
    pub tax_date: stripe_types::Timestamp,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxCalculation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxCalculation").finish_non_exhaustive()
    }
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
    ship_from_details: Option<stripe_misc::TaxProductResourceShipFromDetails>,
    shipping_cost: Option<stripe_misc::TaxProductResourceTaxCalculationShippingCost>,
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
    use miniserde::{Deserialize, Result, make_place};
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
                "ship_from_details" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.ship_from_details)
                }
                "shipping_cost" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.shipping_cost)
                }
                "tax_amount_exclusive" => Deserialize::begin(&mut self.tax_amount_exclusive),
                "tax_amount_inclusive" => Deserialize::begin(&mut self.tax_amount_inclusive),
                "tax_breakdown" => Deserialize::begin(&mut self.tax_breakdown),
                "tax_date" => Deserialize::begin(&mut self.tax_date),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_total: None,
                currency: None,
                customer: Some(None),
                customer_details: None,
                expires_at: Some(None),
                id: Some(None),
                line_items: Some(None),
                livemode: None,
                ship_from_details: None,
                shipping_cost: None,
                tax_amount_exclusive: None,
                tax_amount_inclusive: None,
                tax_breakdown: None,
                tax_date: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    amount_total: Some(amount_total),
                    currency: Some(currency),
                    customer: Some(customer),
                    customer_details: Some(customer_details),
                    expires_at: Some(expires_at),
                    id: Some(id),
                    line_items: Some(line_items),
                    livemode: Some(livemode),
                    ship_from_details,
                    shipping_cost,
                    tax_amount_exclusive: Some(tax_amount_exclusive),
                    tax_amount_inclusive: Some(tax_amount_inclusive),
                    tax_breakdown: Some(tax_breakdown),
                    tax_date: Some(tax_date),
                } => Some(Self::Out {
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
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for TaxCalculation {
        type Builder = TaxCalculationBuilder;
    }

    impl FromValueOpt for TaxCalculation {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
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
