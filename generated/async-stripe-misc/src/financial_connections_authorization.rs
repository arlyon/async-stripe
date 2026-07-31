/// An Authorization represents the set of credentials used to connect a group of Financial Connections Accounts.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FinancialConnectionsAuthorization {
    /// Unique identifier for the object.
    pub id: stripe_misc::FinancialConnectionsAuthorizationId,
    /// The name of the institution that this authorization belongs to.
    pub institution_name: String,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The status of the connection to the Authorization.
    pub status: FinancialConnectionsAuthorizationStatus,
    pub status_details: stripe_misc::BankConnectionsResourceAuthorizationStatusDetails,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FinancialConnectionsAuthorization").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct FinancialConnectionsAuthorizationBuilder {
    id: Option<stripe_misc::FinancialConnectionsAuthorizationId>,
    institution_name: Option<String>,
    livemode: Option<bool>,
    status: Option<FinancialConnectionsAuthorizationStatus>,
    status_details: Option<stripe_misc::BankConnectionsResourceAuthorizationStatusDetails>,
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

    impl Deserialize for FinancialConnectionsAuthorization {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FinancialConnectionsAuthorization>,
        builder: FinancialConnectionsAuthorizationBuilder,
    }

    impl Visitor for Place<FinancialConnectionsAuthorization> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FinancialConnectionsAuthorizationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for FinancialConnectionsAuthorizationBuilder {
        type Out = FinancialConnectionsAuthorization;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.id),
                "institution_name" => Deserialize::begin(&mut self.institution_name),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "status" => Deserialize::begin(&mut self.status),
                "status_details" => Deserialize::begin(&mut self.status_details),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                id: None,
                institution_name: None,
                livemode: None,
                status: None,
                status_details: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(id),
                Some(institution_name),
                Some(livemode),
                Some(status),
                Some(status_details),
            ) = (
                self.id.take(),
                self.institution_name.take(),
                self.livemode,
                self.status.take(),
                self.status_details.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { id, institution_name, livemode, status, status_details })
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

    impl ObjectDeser for FinancialConnectionsAuthorization {
        type Builder = FinancialConnectionsAuthorizationBuilder;
    }

    impl FromValueOpt for FinancialConnectionsAuthorization {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FinancialConnectionsAuthorizationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "id" => b.id = FromValueOpt::from_value(v),
                    "institution_name" => b.institution_name = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "status_details" => b.status_details = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAuthorization {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("FinancialConnectionsAuthorization", 6)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("institution_name", &self.institution_name)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("status_details", &self.status_details)?;

        s.serialize_field("object", "financial_connections.authorization")?;
        s.end()
    }
}
/// The status of the connection to the Authorization.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FinancialConnectionsAuthorizationStatus {
    Active,
    Inactive,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FinancialConnectionsAuthorizationStatus {
    pub fn as_str(&self) -> &str {
        use FinancialConnectionsAuthorizationStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FinancialConnectionsAuthorizationStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsAuthorizationStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "FinancialConnectionsAuthorizationStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FinancialConnectionsAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FinancialConnectionsAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FinancialConnectionsAuthorizationStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAuthorizationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for FinancialConnectionsAuthorizationStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<FinancialConnectionsAuthorizationStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FinancialConnectionsAuthorizationStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FinancialConnectionsAuthorizationStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FinancialConnectionsAuthorizationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for FinancialConnectionsAuthorization {
    type Id = stripe_misc::FinancialConnectionsAuthorizationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(FinancialConnectionsAuthorizationId);
