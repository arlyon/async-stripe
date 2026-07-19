#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodBoleto {
    /// Uniquely identifies the customer tax id (CNPJ or CPF)
    pub tax_id: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodBoleto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodBoleto").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodBoletoBuilder {
    tax_id: Option<String>,
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

    impl Deserialize for PaymentMethodBoleto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodBoleto>,
        builder: PaymentMethodBoletoBuilder,
    }

    impl Visitor for Place<PaymentMethodBoleto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodBoletoBuilder { tax_id: Deserialize::default() },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "tax_id" => Deserialize::begin(&mut self.builder.tax_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(tax_id),) = (self.builder.tax_id.take(),) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodBoleto { tax_id });
            Ok(())
        }
    }
};
