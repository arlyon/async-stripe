#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodPayto {
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// The PayID alias for the bank account.
    pub pay_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodPayto").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodPaytoBuilder {
    bsb_number: Option<Option<String>>,
    last4: Option<Option<String>>,
    pay_id: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodPayto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodPayto>,
        builder: PaymentMethodPaytoBuilder,
    }

    impl Visitor for Place<PaymentMethodPayto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodPaytoBuilder {
                    bsb_number: Deserialize::default(),
                    last4: Deserialize::default(),
                    pay_id: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bsb_number" => Deserialize::begin(&mut self.builder.bsb_number),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "pay_id" => Deserialize::begin(&mut self.builder.pay_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(bsb_number), Some(last4), Some(pay_id)) = (
                self.builder.bsb_number.take(),
                self.builder.last4.take(),
                self.builder.pay_id.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodPayto { bsb_number, last4, pay_id });
            Ok(())
        }
    }
};
