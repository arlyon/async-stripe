#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxSettingsStatusDetails {
    pub active: Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetailsResourceActive>,
    pub pending: Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetailsResourcePending>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceTaxSettingsStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourceTaxSettingsStatusDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductResourceTaxSettingsStatusDetailsBuilder {
    active: Option<Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetailsResourceActive>>,
    pending: Option<Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetailsResourcePending>>,
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

    impl Deserialize for TaxProductResourceTaxSettingsStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxSettingsStatusDetails>,
        builder: TaxProductResourceTaxSettingsStatusDetailsBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxSettingsStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceTaxSettingsStatusDetailsBuilder {
                    active: Deserialize::default(),
                    pending: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.builder.active),
                "pending" => Deserialize::begin(&mut self.builder.pending),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(active), Some(pending)) = (self.builder.active, self.builder.pending.take())
            else {
                return Ok(());
            };
            *self.out = Some(TaxProductResourceTaxSettingsStatusDetails { active, pending });
            Ok(())
        }
    }
};
