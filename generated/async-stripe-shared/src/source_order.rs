#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceOrder {
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount for the order.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The email address of the customer placing the order.
    pub email: Option<String>,
    /// List of items constituting the order.
    pub items: Option<Vec<stripe_shared::SourceOrderItem>>,
    pub shipping: Option<stripe_shared::Shipping>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceOrder").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceOrderBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
    email: Option<Option<String>>,
    items: Option<Option<Vec<stripe_shared::SourceOrderItem>>>,
    shipping: Option<Option<stripe_shared::Shipping>>,
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

    impl Deserialize for SourceOrder {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceOrder>,
        builder: SourceOrderBuilder,
    }

    impl Visitor for Place<SourceOrder> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceOrderBuilder {
                    amount: Deserialize::default(),
                    currency: Deserialize::default(),
                    email: Deserialize::default(),
                    items: Deserialize::default(),
                    shipping: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "email" => Deserialize::begin(&mut self.builder.email),
                "items" => Deserialize::begin(&mut self.builder.items),
                "shipping" => Deserialize::begin(&mut self.builder.shipping),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(currency), Some(email), Some(items), Some(shipping)) = (
                self.builder.amount,
                self.builder.currency.take(),
                self.builder.email.take(),
                self.builder.items.take(),
                self.builder.shipping.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(SourceOrder { amount, currency, email, items, shipping });
            Ok(())
        }
    }
};
