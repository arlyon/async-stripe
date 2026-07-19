#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodPaypal {
    /// Two-letter ISO code representing the buyer's country.
    /// Values are provided by PayPal directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub country: Option<String>,
    /// Owner's email. Values are provided by PayPal directly
    /// (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub payer_email: Option<String>,
    /// PayPal account PayerID. This identifier uniquely identifies the PayPal customer.
    pub payer_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodPaypal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodPaypal").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodPaypalBuilder {
    country: Option<Option<String>>,
    payer_email: Option<Option<String>>,
    payer_id: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodPaypal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodPaypal>,
        builder: PaymentMethodPaypalBuilder,
    }

    impl Visitor for Place<PaymentMethodPaypal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodPaypalBuilder {
                    country: Deserialize::default(),
                    payer_email: Deserialize::default(),
                    payer_id: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "country" => Deserialize::begin(&mut self.builder.country),
                "payer_email" => Deserialize::begin(&mut self.builder.payer_email),
                "payer_id" => Deserialize::begin(&mut self.builder.payer_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(country), Some(payer_email), Some(payer_id)) = (
                self.builder.country.take(),
                self.builder.payer_email.take(),
                self.builder.payer_id.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodPaypal { country, payer_email, payer_id });
            Ok(())
        }
    }
};
