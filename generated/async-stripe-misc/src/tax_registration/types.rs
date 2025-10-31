/// A Tax `Registration` lets us know that your business is registered to collect tax on payments within a region, enabling you to [automatically collect tax](https://stripe.com/docs/tax).
///
/// Stripe doesn't register on your behalf with the relevant authorities when you create a Tax `Registration` object.
/// For more information on how to register to collect tax, see [our guide](https://stripe.com/docs/tax/registering).
///
/// Related guide: [Using the Registrations API](https://stripe.com/docs/tax/registrations-api)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxRegistration {
    /// Time at which the registration becomes active. Measured in seconds since the Unix epoch.
    pub active_from: stripe_types::Timestamp,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    pub country_options: stripe_misc::TaxProductRegistrationsResourceCountryOptions,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// If set, the registration stops being active at this time.
    /// If not set, the registration will be active indefinitely.
    /// Measured in seconds since the Unix epoch.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_misc::TaxRegistrationId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The status of the registration.
    /// This field is present for convenience and can be deduced from `active_from` and `expires_at`.
    pub status: TaxRegistrationStatus,
}
#[doc(hidden)]
pub struct TaxRegistrationBuilder {
    active_from: Option<stripe_types::Timestamp>,
    country: Option<String>,
    country_options: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptions>,
    created: Option<stripe_types::Timestamp>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    id: Option<stripe_misc::TaxRegistrationId>,
    livemode: Option<bool>,
    status: Option<TaxRegistrationStatus>,
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

    impl Deserialize for TaxRegistration {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxRegistration>,
        builder: TaxRegistrationBuilder,
    }

    impl Visitor for Place<TaxRegistration> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxRegistrationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxRegistrationBuilder {
        type Out = TaxRegistration;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active_from" => Deserialize::begin(&mut self.active_from),
                "country" => Deserialize::begin(&mut self.country),
                "country_options" => Deserialize::begin(&mut self.country_options),
                "created" => Deserialize::begin(&mut self.created),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                active_from: Deserialize::default(),
                country: Deserialize::default(),
                country_options: Deserialize::default(),
                created: Deserialize::default(),
                expires_at: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(active_from),
                Some(country),
                Some(country_options),
                Some(created),
                Some(expires_at),
                Some(id),
                Some(livemode),
                Some(status),
            ) = (
                self.active_from,
                self.country.take(),
                self.country_options.take(),
                self.created,
                self.expires_at,
                self.id.take(),
                self.livemode,
                self.status,
            )
            else {
                return None;
            };
            Some(Self::Out {
                active_from,
                country,
                country_options,
                created,
                expires_at,
                id,
                livemode,
                status,
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

    impl ObjectDeser for TaxRegistration {
        type Builder = TaxRegistrationBuilder;
    }

    impl FromValueOpt for TaxRegistration {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxRegistrationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "active_from" => b.active_from = FromValueOpt::from_value(v),
                    "country" => b.country = FromValueOpt::from_value(v),
                    "country_options" => b.country_options = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxRegistration {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TaxRegistration", 9)?;
        s.serialize_field("active_from", &self.active_from)?;
        s.serialize_field("country", &self.country)?;
        s.serialize_field("country_options", &self.country_options)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("expires_at", &self.expires_at)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("status", &self.status)?;

        s.serialize_field("object", "tax.registration")?;
        s.end()
    }
}
/// The status of the registration.
/// This field is present for convenience and can be deduced from `active_from` and `expires_at`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxRegistrationStatus {
    Active,
    Expired,
    Scheduled,
}
impl TaxRegistrationStatus {
    pub fn as_str(self) -> &'static str {
        use TaxRegistrationStatus::*;
        match self {
            Active => "active",
            Expired => "expired",
            Scheduled => "scheduled",
        }
    }
}

impl std::str::FromStr for TaxRegistrationStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxRegistrationStatus::*;
        match s {
            "active" => Ok(Active),
            "expired" => Ok(Expired),
            "scheduled" => Ok(Scheduled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxRegistrationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxRegistrationStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxRegistrationStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxRegistrationStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxRegistrationStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxRegistrationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TaxRegistrationStatus"))
    }
}
impl stripe_types::Object for TaxRegistration {
    type Id = stripe_misc::TaxRegistrationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TaxRegistrationId);
