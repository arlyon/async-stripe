#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodConfigBizPaymentMethodConfigurationDetails {
    /// ID of the payment method configuration used.
    pub id: String,
    /// ID of the parent payment method configuration used.
    pub parent: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodConfigBizPaymentMethodConfigurationDetailsBuilder {
    id: Option<String>,
    parent: Option<Option<String>>,
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
                builder:
                    PaymentMethodConfigBizPaymentMethodConfigurationDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodConfigBizPaymentMethodConfigurationDetailsBuilder {
        type Out = PaymentMethodConfigBizPaymentMethodConfigurationDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.id),
                "parent" => Deserialize::begin(&mut self.parent),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { id: Deserialize::default(), parent: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(id), Some(parent)) = (self.id.take(), self.parent.take()) else {
                return None;
            };
            Some(Self::Out { id, parent })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PaymentMethodConfigBizPaymentMethodConfigurationDetails {
        type Builder = PaymentMethodConfigBizPaymentMethodConfigurationDetailsBuilder;
    }

    impl FromValueOpt for PaymentMethodConfigBizPaymentMethodConfigurationDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentMethodConfigBizPaymentMethodConfigurationDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "id" => b.id = FromValueOpt::from_value(v),
                    "parent" => b.parent = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
