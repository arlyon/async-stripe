#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentAmountDetailsLineItem {
        /// The discount applied on this line item represented in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    /// An integer greater than 0.
    ///
    /// This field is mutually exclusive with the `amount_details[discount_amount]` field.
pub discount_amount: Option<i64>,
    /// Unique identifier for the object.
pub id: stripe_shared::PaymentIntentAmountDetailsLineItemId,
    /// Payment method-specific information for line items.
pub payment_method_options: Option<stripe_shared::PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions>,
        /// The product code of the line item, such as an SKU.
    /// Required for L3 rates.
    /// At most 12 characters long.
pub product_code: Option<String>,
    /// The product name of the line item. Required for L3 rates. At most 1024 characters long.
    ///
        /// For Cards, this field is truncated to 26 alphanumeric characters before being sent to the card networks.
    /// For PayPal, this field is truncated to 127 characters.
pub product_name: String,
    /// The quantity of items. Required for L3 rates. An integer greater than 0.
pub quantity: u64,
    /// Contains information about the tax on the item.
pub tax: Option<stripe_shared::PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourceTax>,
        /// The unit cost of the line item represented in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    /// Required for L3 rates.
    /// An integer greater than or equal to 0.
pub unit_cost: i64,
        /// A unit of measure for the line item, such as gallons, feet, meters, etc.
    /// Required for L3 rates.
    /// At most 12 alphanumeric characters long.
pub unit_of_measure: Option<String>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentAmountDetailsLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentAmountDetailsLineItem").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentAmountDetailsLineItemBuilder {
    discount_amount: Option<Option<i64>>,
id: Option<stripe_shared::PaymentIntentAmountDetailsLineItemId>,
payment_method_options: Option<Option<stripe_shared::PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions>>,
product_code: Option<Option<String>>,
product_name: Option<String>,
quantity: Option<u64>,
tax: Option<Option<stripe_shared::PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourceTax>>,
unit_cost: Option<i64>,
unit_of_measure: Option<Option<String>>,

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

    impl Deserialize for PaymentIntentAmountDetailsLineItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentAmountDetailsLineItem>,
        builder: PaymentIntentAmountDetailsLineItemBuilder,
    }

    impl Visitor for Place<PaymentIntentAmountDetailsLineItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentAmountDetailsLineItemBuilder {
                    discount_amount: Deserialize::default(),
                    id: Deserialize::default(),
                    payment_method_options: Deserialize::default(),
                    product_code: Deserialize::default(),
                    product_name: Deserialize::default(),
                    quantity: Deserialize::default(),
                    tax: Deserialize::default(),
                    unit_cost: Deserialize::default(),
                    unit_of_measure: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "discount_amount" => Deserialize::begin(&mut self.builder.discount_amount),
                "id" => Deserialize::begin(&mut self.builder.id),
                "payment_method_options" => {
                    Deserialize::begin(&mut self.builder.payment_method_options)
                }
                "product_code" => Deserialize::begin(&mut self.builder.product_code),
                "product_name" => Deserialize::begin(&mut self.builder.product_name),
                "quantity" => Deserialize::begin(&mut self.builder.quantity),
                "tax" => Deserialize::begin(&mut self.builder.tax),
                "unit_cost" => Deserialize::begin(&mut self.builder.unit_cost),
                "unit_of_measure" => Deserialize::begin(&mut self.builder.unit_of_measure),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(discount_amount),
                Some(id),
                Some(payment_method_options),
                Some(product_code),
                Some(product_name),
                Some(quantity),
                Some(tax),
                Some(unit_cost),
                Some(unit_of_measure),
            ) = (
                self.builder.discount_amount,
                self.builder.id.take(),
                self.builder.payment_method_options.take(),
                self.builder.product_code.take(),
                self.builder.product_name.take(),
                self.builder.quantity,
                self.builder.tax,
                self.builder.unit_cost,
                self.builder.unit_of_measure.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentAmountDetailsLineItem {
                discount_amount,
                id,
                payment_method_options,
                product_code,
                product_name,
                quantity,
                tax,
                unit_cost,
                unit_of_measure,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentIntentAmountDetailsLineItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("PaymentIntentAmountDetailsLineItem", 10)?;
        s.serialize_field("discount_amount", &self.discount_amount)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("payment_method_options", &self.payment_method_options)?;
        s.serialize_field("product_code", &self.product_code)?;
        s.serialize_field("product_name", &self.product_name)?;
        s.serialize_field("quantity", &self.quantity)?;
        s.serialize_field("tax", &self.tax)?;
        s.serialize_field("unit_cost", &self.unit_cost)?;
        s.serialize_field("unit_of_measure", &self.unit_of_measure)?;

        s.serialize_field("object", "payment_intent_amount_details_line_item")?;
        s.end()
    }
}
impl stripe_types::Object for PaymentIntentAmountDetailsLineItem {
    type Id = stripe_shared::PaymentIntentAmountDetailsLineItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(PaymentIntentAmountDetailsLineItemId);
