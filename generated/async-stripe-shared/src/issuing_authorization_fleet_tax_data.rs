#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationFleetTaxData {
    /// Amount of state or provincial Sales Tax included in the transaction amount.
    /// `null` if not reported by merchant or not subject to tax.
    pub local_amount_decimal: Option<String>,
    /// Amount of national Sales Tax or VAT included in the transaction amount.
    /// `null` if not reported by merchant or not subject to tax.
    pub national_amount_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationFleetTaxData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationFleetTaxData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingAuthorizationFleetTaxDataBuilder {
    local_amount_decimal: Option<Option<String>>,
    national_amount_decimal: Option<Option<String>>,
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

    impl Deserialize for IssuingAuthorizationFleetTaxData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationFleetTaxData>,
        builder: IssuingAuthorizationFleetTaxDataBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationFleetTaxData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationFleetTaxDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationFleetTaxDataBuilder {
        type Out = IssuingAuthorizationFleetTaxData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "local_amount_decimal" => Deserialize::begin(&mut self.local_amount_decimal),
                "national_amount_decimal" => Deserialize::begin(&mut self.national_amount_decimal),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { local_amount_decimal: Some(None), national_amount_decimal: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    local_amount_decimal: Some(local_amount_decimal),
                    national_amount_decimal: Some(national_amount_decimal),
                } => Some(Self::Out { local_amount_decimal, national_amount_decimal }),
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for IssuingAuthorizationFleetTaxData {
        type Builder = IssuingAuthorizationFleetTaxDataBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationFleetTaxData {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
