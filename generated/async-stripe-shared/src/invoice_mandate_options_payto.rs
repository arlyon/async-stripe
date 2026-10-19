#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceMandateOptionsPayto {
    /// The maximum amount that can be collected in a single invoice.
    /// If you don't specify a maximum, then there is no limit.
    pub amount: Option<i64>,
    /// Only `maximum` is supported.
    pub amount_type: Option<stripe_shared::AmountType>,
    /// The purpose for which payments are made. Has a default value based on your merchant category code.
    pub purpose: Option<stripe_shared::Purpose>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceMandateOptionsPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceMandateOptionsPayto").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoiceMandateOptionsPaytoBuilder {
    amount: Option<Option<i64>>,
    amount_type: Option<Option<stripe_shared::AmountType>>,
    purpose: Option<Option<stripe_shared::Purpose>>,
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

    impl Deserialize for InvoiceMandateOptionsPayto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceMandateOptionsPayto>,
        builder: InvoiceMandateOptionsPaytoBuilder,
    }

    impl Visitor for Place<InvoiceMandateOptionsPayto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoiceMandateOptionsPaytoBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoiceMandateOptionsPaytoBuilder {
        type Out = InvoiceMandateOptionsPayto;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_type" => Deserialize::begin(&mut self.amount_type),
                "purpose" => Deserialize::begin(&mut self.purpose),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { amount: Some(None), amount_type: Some(None), purpose: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(amount_type), Some(purpose)) =
                (self.amount, self.amount_type.take(), self.purpose.take())
            else {
                return None;
            };
            Some(Self::Out { amount, amount_type, purpose })
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

    impl ObjectDeser for InvoiceMandateOptionsPayto {
        type Builder = InvoiceMandateOptionsPaytoBuilder;
    }

    impl FromValueOpt for InvoiceMandateOptionsPayto {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoiceMandateOptionsPaytoBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_type" => b.amount_type = FromValueOpt::from_value(v),
                    "purpose" => b.purpose = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Only `maximum` is supported.
pub type InvoiceMandateOptionsPaytoAmountType = stripe_shared::AmountType;
/// The purpose for which payments are made. Has a default value based on your merchant category code.
pub type InvoiceMandateOptionsPaytoPurpose = stripe_shared::Purpose;
