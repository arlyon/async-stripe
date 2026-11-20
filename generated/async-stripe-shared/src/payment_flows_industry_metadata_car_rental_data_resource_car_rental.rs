#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsIndustryMetadataCarRentalDataResourceCarRental {
pub affiliate: Option<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryAffiliate>,
    /// The booking number associated with the car rental.
pub booking_number: Option<String>,
    /// The name of the car rental company.
pub carrier_name: Option<String>,
    /// The customer service phone number of the car rental company.
pub customer_service_phone_number: Option<String>,
    /// Number of days the car is being rented.
pub days_rented: Option<u32>,
pub distance: Option<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDistance>,
    /// The details of the drivers associated with the rental.
pub drivers: Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriver>>,
pub drop_off: Option<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocation>,
    /// Insurance details for the car rental.
pub insurances: Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsurance>>,
    /// Indicates if the customer did not keep nor cancel their booking.
pub no_show_indicator: Option<bool>,
pub pickup: Option<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocation>,
    /// Name of the person renting the vehicle.
pub renter_name: Option<String>,
pub total: Option<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryTotal>,
pub vehicle: Option<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicle>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRental {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsIndustryMetadataCarRentalDataResourceCarRental")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalBuilder {
    affiliate: Option<Option<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryAffiliate>>,
booking_number: Option<Option<String>>,
carrier_name: Option<Option<String>>,
customer_service_phone_number: Option<Option<String>>,
days_rented: Option<Option<u32>>,
distance: Option<Option<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDistance>>,
drivers: Option<Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriver>>>,
drop_off: Option<Option<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocation>>,
insurances: Option<Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsurance>>>,
no_show_indicator: Option<Option<bool>>,
pickup: Option<Option<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocation>>,
renter_name: Option<Option<String>>,
total: Option<Option<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryTotal>>,
vehicle: Option<Option<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicle>>,

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

    impl Deserialize for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRental {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsIndustryMetadataCarRentalDataResourceCarRental>,
        builder: PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalBuilder,
    }

    impl Visitor for Place<PaymentFlowsIndustryMetadataCarRentalDataResourceCarRental> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalBuilder::deser_default(
                    ),
            }))
        }
    }

    impl MapBuilder for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalBuilder {
        type Out = PaymentFlowsIndustryMetadataCarRentalDataResourceCarRental;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "affiliate" => Deserialize::begin(&mut self.affiliate),
                "booking_number" => Deserialize::begin(&mut self.booking_number),
                "carrier_name" => Deserialize::begin(&mut self.carrier_name),
                "customer_service_phone_number" => {
                    Deserialize::begin(&mut self.customer_service_phone_number)
                }
                "days_rented" => Deserialize::begin(&mut self.days_rented),
                "distance" => Deserialize::begin(&mut self.distance),
                "drivers" => Deserialize::begin(&mut self.drivers),
                "drop_off" => Deserialize::begin(&mut self.drop_off),
                "insurances" => Deserialize::begin(&mut self.insurances),
                "no_show_indicator" => Deserialize::begin(&mut self.no_show_indicator),
                "pickup" => Deserialize::begin(&mut self.pickup),
                "renter_name" => Deserialize::begin(&mut self.renter_name),
                "total" => Deserialize::begin(&mut self.total),
                "vehicle" => Deserialize::begin(&mut self.vehicle),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                affiliate: Some(None),
                booking_number: Some(None),
                carrier_name: Some(None),
                customer_service_phone_number: Some(None),
                days_rented: Some(None),
                distance: Some(None),
                drivers: Some(None),
                drop_off: Some(None),
                insurances: Some(None),
                no_show_indicator: Some(None),
                pickup: Some(None),
                renter_name: Some(None),
                total: Some(None),
                vehicle: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(affiliate),
                Some(booking_number),
                Some(carrier_name),
                Some(customer_service_phone_number),
                Some(days_rented),
                Some(distance),
                Some(drivers),
                Some(drop_off),
                Some(insurances),
                Some(no_show_indicator),
                Some(pickup),
                Some(renter_name),
                Some(total),
                Some(vehicle),
            ) = (
                self.affiliate.take(),
                self.booking_number.take(),
                self.carrier_name.take(),
                self.customer_service_phone_number.take(),
                self.days_rented,
                self.distance.take(),
                self.drivers.take(),
                self.drop_off.take(),
                self.insurances.take(),
                self.no_show_indicator,
                self.pickup.take(),
                self.renter_name.take(),
                self.total.take(),
                self.vehicle.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                affiliate,
                booking_number,
                carrier_name,
                customer_service_phone_number,
                days_rented,
                distance,
                drivers,
                drop_off,
                insurances,
                no_show_indicator,
                pickup,
                renter_name,
                total,
                vehicle,
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

    impl ObjectDeser for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRental {
        type Builder = PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalBuilder;
    }

    impl FromValueOpt for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRental {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "affiliate" => b.affiliate = FromValueOpt::from_value(v),
                    "booking_number" => b.booking_number = FromValueOpt::from_value(v),
                    "carrier_name" => b.carrier_name = FromValueOpt::from_value(v),
                    "customer_service_phone_number" => {
                        b.customer_service_phone_number = FromValueOpt::from_value(v)
                    }
                    "days_rented" => b.days_rented = FromValueOpt::from_value(v),
                    "distance" => b.distance = FromValueOpt::from_value(v),
                    "drivers" => b.drivers = FromValueOpt::from_value(v),
                    "drop_off" => b.drop_off = FromValueOpt::from_value(v),
                    "insurances" => b.insurances = FromValueOpt::from_value(v),
                    "no_show_indicator" => b.no_show_indicator = FromValueOpt::from_value(v),
                    "pickup" => b.pickup = FromValueOpt::from_value(v),
                    "renter_name" => b.renter_name = FromValueOpt::from_value(v),
                    "total" => b.total = FromValueOpt::from_value(v),
                    "vehicle" => b.vehicle = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
