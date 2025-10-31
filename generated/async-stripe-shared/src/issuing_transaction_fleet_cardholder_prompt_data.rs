#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionFleetCardholderPromptData {
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
#[doc(hidden)]
pub struct IssuingTransactionFleetCardholderPromptDataBuilder {
    driver_id: Option<Option<String>>,
    odometer: Option<Option<i64>>,
    unspecified_id: Option<Option<String>>,
    user_id: Option<Option<String>>,
    vehicle_number: Option<Option<String>>,
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

    impl Deserialize for IssuingTransactionFleetCardholderPromptData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionFleetCardholderPromptData>,
        builder: IssuingTransactionFleetCardholderPromptDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionFleetCardholderPromptData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionFleetCardholderPromptDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingTransactionFleetCardholderPromptDataBuilder {
        type Out = IssuingTransactionFleetCardholderPromptData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "driver_id" => Deserialize::begin(&mut self.driver_id),
                "odometer" => Deserialize::begin(&mut self.odometer),
                "unspecified_id" => Deserialize::begin(&mut self.unspecified_id),
                "user_id" => Deserialize::begin(&mut self.user_id),
                "vehicle_number" => Deserialize::begin(&mut self.vehicle_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                driver_id: Deserialize::default(),
                odometer: Deserialize::default(),
                unspecified_id: Deserialize::default(),
                user_id: Deserialize::default(),
                vehicle_number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(driver_id),
                Some(odometer),
                Some(unspecified_id),
                Some(user_id),
                Some(vehicle_number),
            ) = (
                self.driver_id.take(),
                self.odometer,
                self.unspecified_id.take(),
                self.user_id.take(),
                self.vehicle_number.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { driver_id, odometer, unspecified_id, user_id, vehicle_number })
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

    impl ObjectDeser for IssuingTransactionFleetCardholderPromptData {
        type Builder = IssuingTransactionFleetCardholderPromptDataBuilder;
    }

    impl FromValueOpt for IssuingTransactionFleetCardholderPromptData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingTransactionFleetCardholderPromptDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "driver_id" => b.driver_id = FromValueOpt::from_value(v),
                    "odometer" => b.odometer = FromValueOpt::from_value(v),
                    "unspecified_id" => b.unspecified_id = FromValueOpt::from_value(v),
                    "user_id" => b.user_id = FromValueOpt::from_value(v),
                    "vehicle_number" => b.vehicle_number = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
