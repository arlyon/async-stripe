#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PromotionCodesResourceRestrictions {
    /// Promotion code restrictions defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    pub currency_options: Option<
        std::collections::HashMap<
            stripe_types::Currency,
            stripe_shared::PromotionCodeCurrencyOption,
        >,
    >,
    /// A Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or invoices.
    pub first_time_transaction: bool,
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    pub minimum_amount: Option<i64>,
    /// Three-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount
    pub minimum_amount_currency: Option<stripe_types::Currency>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PromotionCodesResourceRestrictions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PromotionCodesResourceRestrictions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PromotionCodesResourceRestrictionsBuilder {
    currency_options: Option<
        Option<
            std::collections::HashMap<
                stripe_types::Currency,
                stripe_shared::PromotionCodeCurrencyOption,
            >,
        >,
    >,
    first_time_transaction: Option<bool>,
    minimum_amount: Option<Option<i64>>,
    minimum_amount_currency: Option<Option<stripe_types::Currency>>,
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

    impl Deserialize for PromotionCodesResourceRestrictions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PromotionCodesResourceRestrictions>,
        builder: PromotionCodesResourceRestrictionsBuilder,
    }

    impl Visitor for Place<PromotionCodesResourceRestrictions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PromotionCodesResourceRestrictionsBuilder {
                    currency_options: Deserialize::default(),
                    first_time_transaction: Deserialize::default(),
                    minimum_amount: Deserialize::default(),
                    minimum_amount_currency: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "currency_options" => Deserialize::begin(&mut self.builder.currency_options),
                "first_time_transaction" => {
                    Deserialize::begin(&mut self.builder.first_time_transaction)
                }
                "minimum_amount" => Deserialize::begin(&mut self.builder.minimum_amount),
                "minimum_amount_currency" => {
                    Deserialize::begin(&mut self.builder.minimum_amount_currency)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(currency_options),
                Some(first_time_transaction),
                Some(minimum_amount),
                Some(minimum_amount_currency),
            ) = (
                self.builder.currency_options.take(),
                self.builder.first_time_transaction,
                self.builder.minimum_amount,
                self.builder.minimum_amount_currency.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PromotionCodesResourceRestrictions {
                currency_options,
                first_time_transaction,
                minimum_amount,
                minimum_amount_currency,
            });
            Ok(())
        }
    }
};
