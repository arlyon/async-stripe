#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationFleetCardholderPromptData {
    /// \[Deprecated\] An alphanumeric ID, though typical point of sales only support numeric entry.
    /// The card program can be configured to prompt for a vehicle ID, driver ID, or generic ID.
    pub alphanumeric_id: Option<String>,
    /// Driver ID.
    pub driver_id: Option<String>,
    /// Odometer reading.
    pub odometer: Option<i64>,
    /// An alphanumeric ID.
    /// This field is used when a vehicle ID, driver ID, or generic ID is entered by the cardholder, but the merchant or card network did not specify the prompt type.
    pub unspecified_id: Option<String>,
    /// User ID.
    pub user_id: Option<String>,
    /// Vehicle number.
    pub vehicle_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationFleetCardholderPromptData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationFleetCardholderPromptData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingAuthorizationFleetCardholderPromptDataBuilder {
    alphanumeric_id: Option<Option<String>>,
    driver_id: Option<Option<String>>,
    odometer: Option<Option<i64>>,
    unspecified_id: Option<Option<String>>,
    user_id: Option<Option<String>>,
    vehicle_number: Option<Option<String>>,
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

    impl Deserialize for IssuingAuthorizationFleetCardholderPromptData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationFleetCardholderPromptData>,
        builder: IssuingAuthorizationFleetCardholderPromptDataBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationFleetCardholderPromptData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationFleetCardholderPromptDataBuilder {
                    alphanumeric_id: Deserialize::default(),
                    driver_id: Deserialize::default(),
                    odometer: Deserialize::default(),
                    unspecified_id: Deserialize::default(),
                    user_id: Deserialize::default(),
                    vehicle_number: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "alphanumeric_id" => Deserialize::begin(&mut self.builder.alphanumeric_id),
                "driver_id" => Deserialize::begin(&mut self.builder.driver_id),
                "odometer" => Deserialize::begin(&mut self.builder.odometer),
                "unspecified_id" => Deserialize::begin(&mut self.builder.unspecified_id),
                "user_id" => Deserialize::begin(&mut self.builder.user_id),
                "vehicle_number" => Deserialize::begin(&mut self.builder.vehicle_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(alphanumeric_id),
                Some(driver_id),
                Some(odometer),
                Some(unspecified_id),
                Some(user_id),
                Some(vehicle_number),
            ) = (
                self.builder.alphanumeric_id.take(),
                self.builder.driver_id.take(),
                self.builder.odometer,
                self.builder.unspecified_id.take(),
                self.builder.user_id.take(),
                self.builder.vehicle_number.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingAuthorizationFleetCardholderPromptData {
                alphanumeric_id,
                driver_id,
                odometer,
                unspecified_id,
                user_id,
                vehicle_number,
            });
            Ok(())
        }
    }
};
