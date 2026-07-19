#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodConfigBizPaymentMethodConfigurationDetails {
    /// ID of the payment method configuration used.
    pub id: String,
    /// ID of the parent payment method configuration used.
    pub parent: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodConfigBizPaymentMethodConfigurationDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodConfigBizPaymentMethodConfigurationDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodConfigBizPaymentMethodConfigurationDetailsBuilder {
    id: Option<String>,
    parent: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodConfigBizPaymentMethodConfigurationDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodConfigBizPaymentMethodConfigurationDetails>,
        builder: PaymentMethodConfigBizPaymentMethodConfigurationDetailsBuilder,
    }

    impl Visitor for Place<PaymentMethodConfigBizPaymentMethodConfigurationDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodConfigBizPaymentMethodConfigurationDetailsBuilder {
                    id: Deserialize::default(),
                    parent: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.builder.id),
                "parent" => Deserialize::begin(&mut self.builder.parent),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(id), Some(parent)) = (self.builder.id.take(), self.builder.parent.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(PaymentMethodConfigBizPaymentMethodConfigurationDetails { id, parent });
            Ok(())
        }
    }
};
