#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionFleetTaxData {
    /// Amount of state or provincial Sales Tax included in the transaction amount.
    /// Null if not reported by merchant or not subject to tax.
    pub local_amount_decimal: Option<String>,
    /// Amount of national Sales Tax or VAT included in the transaction amount.
    /// Null if not reported by merchant or not subject to tax.
    pub national_amount_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingTransactionFleetTaxData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingTransactionFleetTaxData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingTransactionFleetTaxDataBuilder {
    local_amount_decimal: Option<Option<String>>,
    national_amount_decimal: Option<Option<String>>,
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

    impl Deserialize for IssuingTransactionFleetTaxData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionFleetTaxData>,
        builder: IssuingTransactionFleetTaxDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionFleetTaxData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionFleetTaxDataBuilder {
                    local_amount_decimal: Deserialize::default(),
                    national_amount_decimal: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "local_amount_decimal" => {
                    Deserialize::begin(&mut self.builder.local_amount_decimal)
                }
                "national_amount_decimal" => {
                    Deserialize::begin(&mut self.builder.national_amount_decimal)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(local_amount_decimal), Some(national_amount_decimal)) = (
                self.builder.local_amount_decimal.take(),
                self.builder.national_amount_decimal.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(IssuingTransactionFleetTaxData {
                local_amount_decimal,
                national_amount_decimal,
            });
            Ok(())
        }
    }
};
