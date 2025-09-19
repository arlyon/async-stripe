#[derive(Clone, Debug)]
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
            Self {
                local_amount_decimal: Deserialize::default(),
                national_amount_decimal: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(local_amount_decimal), Some(national_amount_decimal)) =
                (self.local_amount_decimal.take(), self.national_amount_decimal.take())
            else {
                return None;
            };
            Some(Self::Out { local_amount_decimal, national_amount_decimal })
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

    impl ObjectDeser for IssuingAuthorizationFleetTaxData {
        type Builder = IssuingAuthorizationFleetTaxDataBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationFleetTaxData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingAuthorizationFleetTaxDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "local_amount_decimal" => b.local_amount_decimal = FromValueOpt::from_value(v),
                    "national_amount_decimal" => {
                        b.national_amount_decimal = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
