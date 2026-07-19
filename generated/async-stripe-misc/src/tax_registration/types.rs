/// A Tax `Registration` lets us know that your business is registered to collect tax on payments within a region, enabling you to [automatically collect tax](https://docs.stripe.com/tax).
///
/// Stripe doesn't register on your behalf with the relevant authorities when you create a Tax `Registration` object.
/// For more information on how to register to collect tax, see [our guide](https://docs.stripe.com/tax/registering).
///
/// Related guide: [Using the Registrations API](https://docs.stripe.com/tax/registrations-api)
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The status of the registration.
    /// This field is present for convenience and can be deduced from `active_from` and `expires_at`.
    pub status: TaxRegistrationStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxRegistration").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: TaxRegistrationBuilder {
                    active_from: Deserialize::default(),
                    country: Deserialize::default(),
                    country_options: Deserialize::default(),
                    created: Deserialize::default(),
                    expires_at: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    status: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active_from" => Deserialize::begin(&mut self.builder.active_from),
                "country" => Deserialize::begin(&mut self.builder.country),
                "country_options" => Deserialize::begin(&mut self.builder.country_options),
                "created" => Deserialize::begin(&mut self.builder.created),
                "expires_at" => Deserialize::begin(&mut self.builder.expires_at),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "status" => Deserialize::begin(&mut self.builder.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.active_from,
                self.builder.country.take(),
                self.builder.country_options.take(),
                self.builder.created,
                self.builder.expires_at,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.status.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TaxRegistration {
                active_from,
                country,
                country_options,
                created,
                expires_at,
                id,
                livemode,
                status,
            });
            Ok(())
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxRegistrationStatus {
    Active,
    Expired,
    Scheduled,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxRegistrationStatus {
    pub fn as_str(&self) -> &str {
        use TaxRegistrationStatus::*;
        match self {
            Active => "active",
            Expired => "expired",
            Scheduled => "scheduled",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxRegistrationStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxRegistrationStatus::*;
        match s {
            "active" => Ok(Active),
            "expired" => Ok(Expired),
            "scheduled" => Ok(Scheduled),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "TaxRegistrationStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TaxRegistrationStatus)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for TaxRegistrationStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TaxRegistrationStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxRegistrationStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxRegistrationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
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
