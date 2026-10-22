#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicePaymentMethodOptionsBillie {
    pub company_details: Option<stripe_shared::InvoicePaymentMethodOptionsBillieCompanyDetails>,
    /// An identifier or reference that this payment corresponds to.
    pub reference: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicePaymentMethodOptionsBillie {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicePaymentMethodOptionsBillie").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoicePaymentMethodOptionsBillieBuilder {
    company_details: Option<Option<stripe_shared::InvoicePaymentMethodOptionsBillieCompanyDetails>>,
    reference: Option<Option<String>>,
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

    impl Deserialize for InvoicePaymentMethodOptionsBillie {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePaymentMethodOptionsBillie>,
        builder: InvoicePaymentMethodOptionsBillieBuilder,
    }

    impl Visitor for Place<InvoicePaymentMethodOptionsBillie> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicePaymentMethodOptionsBillieBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicePaymentMethodOptionsBillieBuilder {
        type Out = InvoicePaymentMethodOptionsBillie;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "company_details" => Deserialize::begin(&mut self.company_details),
                "reference" => Deserialize::begin(&mut self.reference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { company_details: Some(None), reference: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(company_details), Some(reference)) =
                (self.company_details.take(), self.reference.take())
            else {
                return None;
            };
            Some(Self::Out { company_details, reference })
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

    impl ObjectDeser for InvoicePaymentMethodOptionsBillie {
        type Builder = InvoicePaymentMethodOptionsBillieBuilder;
    }

    impl FromValueOpt for InvoicePaymentMethodOptionsBillie {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicePaymentMethodOptionsBillieBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "company_details" => b.company_details = FromValueOpt::from_value(v),
                    "reference" => b.reference = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
