#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingTransactionFleetData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingTransactionFleetData").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: IssuingTransactionFleetDataBuilder {
                    cardholder_prompt_data: Deserialize::default(),
                    purchase_type: Deserialize::default(),
                    reported_breakdown: Deserialize::default(),
                    service_type: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cardholder_prompt_data" => {
                    Deserialize::begin(&mut self.builder.cardholder_prompt_data)
                }
                "purchase_type" => Deserialize::begin(&mut self.builder.purchase_type),
                "reported_breakdown" => Deserialize::begin(&mut self.builder.reported_breakdown),
                "service_type" => Deserialize::begin(&mut self.builder.service_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(cardholder_prompt_data),
                Some(purchase_type),
                Some(reported_breakdown),
                Some(service_type),
            ) = (
                self.builder.cardholder_prompt_data.take(),
                self.builder.purchase_type.take(),
                self.builder.reported_breakdown.take(),
                self.builder.service_type.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingTransactionFleetData {
                cardholder_prompt_data,
                purchase_type,
                reported_breakdown,
                service_type,
            });
            Ok(())
        }
    }
};
