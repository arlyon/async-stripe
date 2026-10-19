#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationFleetData {
    /// Answers to prompts presented to the cardholder at the point of sale.
    /// Prompted fields vary depending on the configuration of your physical fleet cards.
    /// Typical points of sale support only numeric entry.
    pub cardholder_prompt_data:
        Option<stripe_shared::IssuingAuthorizationFleetCardholderPromptData>,
    /// The type of purchase.
    pub purchase_type: Option<stripe_shared::PurchaseType>,
    /// More information about the total amount.
    /// Typically this information is received from the merchant after the authorization has been approved and the fuel dispensed.
    /// This information is not guaranteed to be accurate as some merchants may provide unreliable data.
    pub reported_breakdown: Option<stripe_shared::IssuingAuthorizationFleetReportedBreakdown>,
    /// The type of fuel service.
    pub service_type: Option<stripe_shared::ServiceType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationFleetData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationFleetData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingAuthorizationFleetDataBuilder {
    cardholder_prompt_data:
        Option<Option<stripe_shared::IssuingAuthorizationFleetCardholderPromptData>>,
    purchase_type: Option<Option<stripe_shared::PurchaseType>>,
    reported_breakdown: Option<Option<stripe_shared::IssuingAuthorizationFleetReportedBreakdown>>,
    service_type: Option<Option<stripe_shared::ServiceType>>,
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

    impl Deserialize for IssuingAuthorizationFleetData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationFleetData>,
        builder: IssuingAuthorizationFleetDataBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationFleetData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationFleetDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationFleetDataBuilder {
        type Out = IssuingAuthorizationFleetData;
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
                cardholder_prompt_data: Some(None),
                purchase_type: Some(None),
                reported_breakdown: Some(None),
                service_type: Some(None),
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
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for IssuingAuthorizationFleetData {
        type Builder = IssuingAuthorizationFleetDataBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationFleetData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingAuthorizationFleetDataBuilder::deser_default();
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
/// The type of purchase.
pub type IssuingAuthorizationFleetDataPurchaseType = stripe_shared::PurchaseType;
/// The type of fuel service.
pub type IssuingAuthorizationFleetDataServiceType = stripe_shared::ServiceType;
