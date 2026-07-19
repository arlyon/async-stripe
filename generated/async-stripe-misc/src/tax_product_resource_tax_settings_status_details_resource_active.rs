#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxSettingsStatusDetailsResourceActive {}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceTaxSettingsStatusDetailsResourceActive {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourceTaxSettingsStatusDetailsResourceActive")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductResourceTaxSettingsStatusDetailsResourceActiveBuilder {}

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

    impl Deserialize for TaxProductResourceTaxSettingsStatusDetailsResourceActive {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxSettingsStatusDetailsResourceActive>,
        builder: TaxProductResourceTaxSettingsStatusDetailsResourceActiveBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxSettingsStatusDetailsResourceActive> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceTaxSettingsStatusDetailsResourceActiveBuilder {},
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let () = () else {
                return Ok(());
            };
            *self.out = Some(TaxProductResourceTaxSettingsStatusDetailsResourceActive {});
            Ok(())
        }
    }
};
