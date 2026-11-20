#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsIndustryMetadataLodgingDataResourceLodging {
pub accommodation: Option<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodation>,
pub affiliate: Option<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliate>,
    /// Booking confirmation number for the lodging.
pub booking_number: Option<String>,
    /// Check-in date.
pub checkin_at: Option<stripe_types::Timestamp>,
    /// Check-out date.
pub checkout_at: Option<stripe_types::Timestamp>,
    /// Customer service phone number for the lodging company.
pub customer_service_phone_number: Option<String>,
    /// Whether the lodging is compliant with any hotel fire safety regulations.
pub fire_safety_act_compliance_indicator: Option<bool>,
    /// List of guests for the lodging.
pub guests: Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryGuest>>,
pub host: Option<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHost>,
    /// List of insurances for the lodging.
pub insurances: Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryInsurance>>,
    /// Whether the renter is a no-show.
pub no_show_indicator: Option<bool>,
    /// Renter ID number for the lodging.
pub renter_id_number: Option<String>,
    /// Renter name for the lodging.
pub renter_name: Option<String>,
pub total: Option<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotal>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsIndustryMetadataLodgingDataResourceLodging {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsIndustryMetadataLodgingDataResourceLodging")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsIndustryMetadataLodgingDataResourceLodgingBuilder {
    accommodation: Option<Option<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodation>>,
affiliate: Option<Option<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliate>>,
booking_number: Option<Option<String>>,
checkin_at: Option<Option<stripe_types::Timestamp>>,
checkout_at: Option<Option<stripe_types::Timestamp>>,
customer_service_phone_number: Option<Option<String>>,
fire_safety_act_compliance_indicator: Option<Option<bool>>,
guests: Option<Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryGuest>>>,
host: Option<Option<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHost>>,
insurances: Option<Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryInsurance>>>,
no_show_indicator: Option<Option<bool>>,
renter_id_number: Option<Option<String>>,
renter_name: Option<Option<String>>,
total: Option<Option<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotal>>,

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

    impl Deserialize for PaymentFlowsIndustryMetadataLodgingDataResourceLodging {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsIndustryMetadataLodgingDataResourceLodging>,
        builder: PaymentFlowsIndustryMetadataLodgingDataResourceLodgingBuilder,
    }

    impl Visitor for Place<PaymentFlowsIndustryMetadataLodgingDataResourceLodging> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    PaymentFlowsIndustryMetadataLodgingDataResourceLodgingBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingBuilder {
        type Out = PaymentFlowsIndustryMetadataLodgingDataResourceLodging;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "accommodation" => Deserialize::begin(&mut self.accommodation),
                "affiliate" => Deserialize::begin(&mut self.affiliate),
                "booking_number" => Deserialize::begin(&mut self.booking_number),
                "checkin_at" => Deserialize::begin(&mut self.checkin_at),
                "checkout_at" => Deserialize::begin(&mut self.checkout_at),
                "customer_service_phone_number" => {
                    Deserialize::begin(&mut self.customer_service_phone_number)
                }
                "fire_safety_act_compliance_indicator" => {
                    Deserialize::begin(&mut self.fire_safety_act_compliance_indicator)
                }
                "guests" => Deserialize::begin(&mut self.guests),
                "host" => Deserialize::begin(&mut self.host),
                "insurances" => Deserialize::begin(&mut self.insurances),
                "no_show_indicator" => Deserialize::begin(&mut self.no_show_indicator),
                "renter_id_number" => Deserialize::begin(&mut self.renter_id_number),
                "renter_name" => Deserialize::begin(&mut self.renter_name),
                "total" => Deserialize::begin(&mut self.total),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                accommodation: Some(None),
                affiliate: Some(None),
                booking_number: Some(None),
                checkin_at: Some(None),
                checkout_at: Some(None),
                customer_service_phone_number: Some(None),
                fire_safety_act_compliance_indicator: Some(None),
                guests: Some(None),
                host: Some(None),
                insurances: Some(None),
                no_show_indicator: Some(None),
                renter_id_number: Some(None),
                renter_name: Some(None),
                total: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(accommodation),
                Some(affiliate),
                Some(booking_number),
                Some(checkin_at),
                Some(checkout_at),
                Some(customer_service_phone_number),
                Some(fire_safety_act_compliance_indicator),
                Some(guests),
                Some(host),
                Some(insurances),
                Some(no_show_indicator),
                Some(renter_id_number),
                Some(renter_name),
                Some(total),
            ) = (
                self.accommodation.take(),
                self.affiliate.take(),
                self.booking_number.take(),
                self.checkin_at,
                self.checkout_at,
                self.customer_service_phone_number.take(),
                self.fire_safety_act_compliance_indicator,
                self.guests.take(),
                self.host.take(),
                self.insurances.take(),
                self.no_show_indicator,
                self.renter_id_number.take(),
                self.renter_name.take(),
                self.total.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                accommodation,
                affiliate,
                booking_number,
                checkin_at,
                checkout_at,
                customer_service_phone_number,
                fire_safety_act_compliance_indicator,
                guests,
                host,
                insurances,
                no_show_indicator,
                renter_id_number,
                renter_name,
                total,
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

    impl ObjectDeser for PaymentFlowsIndustryMetadataLodgingDataResourceLodging {
        type Builder = PaymentFlowsIndustryMetadataLodgingDataResourceLodgingBuilder;
    }

    impl FromValueOpt for PaymentFlowsIndustryMetadataLodgingDataResourceLodging {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentFlowsIndustryMetadataLodgingDataResourceLodgingBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "accommodation" => b.accommodation = FromValueOpt::from_value(v),
                    "affiliate" => b.affiliate = FromValueOpt::from_value(v),
                    "booking_number" => b.booking_number = FromValueOpt::from_value(v),
                    "checkin_at" => b.checkin_at = FromValueOpt::from_value(v),
                    "checkout_at" => b.checkout_at = FromValueOpt::from_value(v),
                    "customer_service_phone_number" => {
                        b.customer_service_phone_number = FromValueOpt::from_value(v)
                    }
                    "fire_safety_act_compliance_indicator" => {
                        b.fire_safety_act_compliance_indicator = FromValueOpt::from_value(v)
                    }
                    "guests" => b.guests = FromValueOpt::from_value(v),
                    "host" => b.host = FromValueOpt::from_value(v),
                    "insurances" => b.insurances = FromValueOpt::from_value(v),
                    "no_show_indicator" => b.no_show_indicator = FromValueOpt::from_value(v),
                    "renter_id_number" => b.renter_id_number = FromValueOpt::from_value(v),
                    "renter_name" => b.renter_name = FromValueOpt::from_value(v),
                    "total" => b.total = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
