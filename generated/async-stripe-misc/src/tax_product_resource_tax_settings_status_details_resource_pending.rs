#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxSettingsStatusDetailsResourcePending {
    /// The list of missing fields that are required to perform calculations.
    /// It includes the entry `head_office` when the status is `pending`.
    /// It is recommended to set the optional values even if they aren't listed as required for calculating taxes.
    /// Calculations can fail if missing fields aren't explicitly provided on every call.
    pub missing_fields: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceTaxSettingsStatusDetailsResourcePending {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourceTaxSettingsStatusDetailsResourcePending")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductResourceTaxSettingsStatusDetailsResourcePendingBuilder {
    missing_fields: Option<Option<Vec<String>>>,
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

    impl Deserialize for TaxProductResourceTaxSettingsStatusDetailsResourcePending {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxSettingsStatusDetailsResourcePending>,
        builder: TaxProductResourceTaxSettingsStatusDetailsResourcePendingBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxSettingsStatusDetailsResourcePending> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceTaxSettingsStatusDetailsResourcePendingBuilder {
                    missing_fields: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "missing_fields" => Deserialize::begin(&mut self.builder.missing_fields),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(missing_fields),) = (self.builder.missing_fields.take(),) else {
                return Ok(());
            };
            *self.out =
                Some(TaxProductResourceTaxSettingsStatusDetailsResourcePending { missing_fields });
            Ok(())
        }
    }
};
