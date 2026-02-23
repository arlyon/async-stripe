#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionFleetData {
    /// Answers to prompts presented to cardholder at point of sale.
    pub cardholder_prompt_data: Option<stripe_shared::IssuingTransactionFleetCardholderPromptData>,
    /// The type of purchase. One of `fuel_purchase`, `non_fuel_purchase`, or `fuel_and_non_fuel_purchase`.
    pub purchase_type: Option<String>,
    /// More information about the total amount.
    /// This information is not guaranteed to be accurate as some merchants may provide unreliable data.
    pub reported_breakdown: Option<stripe_shared::IssuingTransactionFleetReportedBreakdown>,
    /// The type of fuel service. One of `non_fuel_transaction`, `full_service`, or `self_service`.
    pub service_type: Option<String>,
}
#[doc(hidden)]
pub struct IssuingTransactionFleetDataBuilder {
    cardholder_prompt_data:
        Option<Option<stripe_shared::IssuingTransactionFleetCardholderPromptData>>,
    purchase_type: Option<Option<String>>,
    reported_breakdown: Option<Option<stripe_shared::IssuingTransactionFleetReportedBreakdown>>,
    service_type: Option<Option<String>>,
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

    impl Deserialize for IssuingTransactionFleetData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionFleetData>,
        builder: IssuingTransactionFleetDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionFleetData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionFleetDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingTransactionFleetDataBuilder {
        type Out = IssuingTransactionFleetData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cardholder_prompt_data" => Deserialize::begin(&mut self.cardholder_prompt_data),
                "purchase_type" => Deserialize::begin(&mut self.purchase_type),
                "reported_breakdown" => Deserialize::begin(&mut self.reported_breakdown),
                "service_type" => Deserialize::begin(&mut self.service_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                cardholder_prompt_data: Deserialize::default(),
                purchase_type: Deserialize::default(),
                reported_breakdown: Deserialize::default(),
                service_type: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(cardholder_prompt_data),
                Some(purchase_type),
                Some(reported_breakdown),
                Some(service_type),
            ) = (
                self.cardholder_prompt_data.take(),
                self.purchase_type.take(),
                self.reported_breakdown.take(),
                self.service_type.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                cardholder_prompt_data,
                purchase_type,
                reported_breakdown,
                service_type,
            })
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

    impl ObjectDeser for IssuingTransactionFleetData {
        type Builder = IssuingTransactionFleetDataBuilder;
    }

    impl FromValueOpt for IssuingTransactionFleetData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingTransactionFleetDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "cardholder_prompt_data" => {
                        b.cardholder_prompt_data = FromValueOpt::from_value(v)
                    }
                    "purchase_type" => b.purchase_type = FromValueOpt::from_value(v),
                    "reported_breakdown" => b.reported_breakdown = FromValueOpt::from_value(v),
                    "service_type" => b.service_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
