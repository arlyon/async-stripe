#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoVerifiedOutputs {
    /// The user's verified address.
    pub address: Option<stripe_shared::Address>,
    /// The user’s verified date of birth.
    pub dob: Option<stripe_misc::GelatoDataVerifiedOutputsDate>,
    /// The user's verified email address
    pub email: Option<String>,
    /// The user's verified first name.
    pub first_name: Option<String>,
    /// The user's verified id number.
    pub id_number: Option<String>,
    /// The user's verified id number type.
    pub id_number_type: Option<GelatoVerifiedOutputsIdNumberType>,
    /// The user's verified last name.
    pub last_name: Option<String>,
    /// The user's verified phone number
    pub phone: Option<String>,
    /// The user's verified sex.
    pub sex: Option<GelatoVerifiedOutputsSex>,
    /// The user's verified place of birth as it appears in the document.
    pub unparsed_place_of_birth: Option<String>,
    /// The user's verified sex as it appears in the document.
    pub unparsed_sex: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoVerifiedOutputs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoVerifiedOutputs").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoVerifiedOutputsBuilder {
    address: Option<Option<stripe_shared::Address>>,
    dob: Option<Option<stripe_misc::GelatoDataVerifiedOutputsDate>>,
    email: Option<Option<String>>,
    first_name: Option<Option<String>>,
    id_number: Option<Option<String>>,
    id_number_type: Option<Option<GelatoVerifiedOutputsIdNumberType>>,
    last_name: Option<Option<String>>,
    phone: Option<Option<String>>,
    sex: Option<Option<GelatoVerifiedOutputsSex>>,
    unparsed_place_of_birth: Option<Option<String>>,
    unparsed_sex: Option<Option<String>>,
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

    impl Deserialize for GelatoVerifiedOutputs {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoVerifiedOutputs>,
        builder: GelatoVerifiedOutputsBuilder,
    }

    impl Visitor for Place<GelatoVerifiedOutputs> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoVerifiedOutputsBuilder {
                    address: Deserialize::default(),
                    dob: Deserialize::default(),
                    email: Deserialize::default(),
                    first_name: Deserialize::default(),
                    id_number: Deserialize::default(),
                    id_number_type: Deserialize::default(),
                    last_name: Deserialize::default(),
                    phone: Deserialize::default(),
                    sex: Deserialize::default(),
                    unparsed_place_of_birth: Deserialize::default(),
                    unparsed_sex: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.builder.address),
                "dob" => Deserialize::begin(&mut self.builder.dob),
                "email" => Deserialize::begin(&mut self.builder.email),
                "first_name" => Deserialize::begin(&mut self.builder.first_name),
                "id_number" => Deserialize::begin(&mut self.builder.id_number),
                "id_number_type" => Deserialize::begin(&mut self.builder.id_number_type),
                "last_name" => Deserialize::begin(&mut self.builder.last_name),
                "phone" => Deserialize::begin(&mut self.builder.phone),
                "sex" => Deserialize::begin(&mut self.builder.sex),
                "unparsed_place_of_birth" => {
                    Deserialize::begin(&mut self.builder.unparsed_place_of_birth)
                }
                "unparsed_sex" => Deserialize::begin(&mut self.builder.unparsed_sex),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(address),
                Some(dob),
                Some(email),
                Some(first_name),
                Some(id_number),
                Some(id_number_type),
                Some(last_name),
                Some(phone),
                Some(sex),
                Some(unparsed_place_of_birth),
                Some(unparsed_sex),
            ) = (
                self.builder.address.take(),
                self.builder.dob,
                self.builder.email.take(),
                self.builder.first_name.take(),
                self.builder.id_number.take(),
                self.builder.id_number_type.take(),
                self.builder.last_name.take(),
                self.builder.phone.take(),
                self.builder.sex.take(),
                self.builder.unparsed_place_of_birth.take(),
                self.builder.unparsed_sex.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(GelatoVerifiedOutputs {
                address,
                dob,
                email,
                first_name,
                id_number,
                id_number_type,
                last_name,
                phone,
                sex,
                unparsed_place_of_birth,
                unparsed_sex,
            });
            Ok(())
        }
    }
};
/// The user's verified id number type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum GelatoVerifiedOutputsIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl GelatoVerifiedOutputsIdNumberType {
    pub fn as_str(&self) -> &str {
        use GelatoVerifiedOutputsIdNumberType::*;
        match self {
            BrCpf => "br_cpf",
            SgNric => "sg_nric",
            UsSsn => "us_ssn",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for GelatoVerifiedOutputsIdNumberType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoVerifiedOutputsIdNumberType::*;
        match s {
            "br_cpf" => Ok(BrCpf),
            "sg_nric" => Ok(SgNric),
            "us_ssn" => Ok(UsSsn),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "GelatoVerifiedOutputsIdNumberType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for GelatoVerifiedOutputsIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for GelatoVerifiedOutputsIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoVerifiedOutputsIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(GelatoVerifiedOutputsIdNumberType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoVerifiedOutputsIdNumberType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for GelatoVerifiedOutputsIdNumberType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<GelatoVerifiedOutputsIdNumberType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoVerifiedOutputsIdNumberType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoVerifiedOutputsIdNumberType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The user's verified sex.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum GelatoVerifiedOutputsSex {
    Redacted,
    Female,
    Male,
    Unknown,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    /// This variant is prefixed with an underscore to avoid conflicts with Stripe's 'Unknown' variant.
    _Unknown(String),
}
impl GelatoVerifiedOutputsSex {
    pub fn as_str(&self) -> &str {
        use GelatoVerifiedOutputsSex::*;
        match self {
            Redacted => "[redacted]",
            Female => "female",
            Male => "male",
            Unknown => "unknown",
            _Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for GelatoVerifiedOutputsSex {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoVerifiedOutputsSex::*;
        match s {
            "[redacted]" => Ok(Redacted),
            "female" => Ok(Female),
            "male" => Ok(Male),
            "unknown" => Ok(Unknown),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "GelatoVerifiedOutputsSex");
                Ok(_Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for GelatoVerifiedOutputsSex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for GelatoVerifiedOutputsSex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoVerifiedOutputsSex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(GelatoVerifiedOutputsSex)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoVerifiedOutputsSex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for GelatoVerifiedOutputsSex {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<GelatoVerifiedOutputsSex> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoVerifiedOutputsSex::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoVerifiedOutputsSex {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
