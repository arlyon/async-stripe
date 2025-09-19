#[derive(Clone, Debug)]
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
                builder: GelatoVerifiedOutputsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoVerifiedOutputsBuilder {
        type Out = GelatoVerifiedOutputs;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "dob" => Deserialize::begin(&mut self.dob),
                "email" => Deserialize::begin(&mut self.email),
                "first_name" => Deserialize::begin(&mut self.first_name),
                "id_number" => Deserialize::begin(&mut self.id_number),
                "id_number_type" => Deserialize::begin(&mut self.id_number_type),
                "last_name" => Deserialize::begin(&mut self.last_name),
                "phone" => Deserialize::begin(&mut self.phone),
                "sex" => Deserialize::begin(&mut self.sex),
                "unparsed_place_of_birth" => Deserialize::begin(&mut self.unparsed_place_of_birth),
                "unparsed_sex" => Deserialize::begin(&mut self.unparsed_sex),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.address.take(),
                self.dob,
                self.email.take(),
                self.first_name.take(),
                self.id_number.take(),
                self.id_number_type,
                self.last_name.take(),
                self.phone.take(),
                self.sex,
                self.unparsed_place_of_birth.take(),
                self.unparsed_sex.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
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

    impl ObjectDeser for GelatoVerifiedOutputs {
        type Builder = GelatoVerifiedOutputsBuilder;
    }

    impl FromValueOpt for GelatoVerifiedOutputs {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoVerifiedOutputsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = FromValueOpt::from_value(v),
                    "dob" => b.dob = FromValueOpt::from_value(v),
                    "email" => b.email = FromValueOpt::from_value(v),
                    "first_name" => b.first_name = FromValueOpt::from_value(v),
                    "id_number" => b.id_number = FromValueOpt::from_value(v),
                    "id_number_type" => b.id_number_type = FromValueOpt::from_value(v),
                    "last_name" => b.last_name = FromValueOpt::from_value(v),
                    "phone" => b.phone = FromValueOpt::from_value(v),
                    "sex" => b.sex = FromValueOpt::from_value(v),
                    "unparsed_place_of_birth" => {
                        b.unparsed_place_of_birth = FromValueOpt::from_value(v)
                    }
                    "unparsed_sex" => b.unparsed_sex = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The user's verified id number type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoVerifiedOutputsIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
}
impl GelatoVerifiedOutputsIdNumberType {
    pub fn as_str(self) -> &'static str {
        use GelatoVerifiedOutputsIdNumberType::*;
        match self {
            BrCpf => "br_cpf",
            SgNric => "sg_nric",
            UsSsn => "us_ssn",
        }
    }
}

impl std::str::FromStr for GelatoVerifiedOutputsIdNumberType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoVerifiedOutputsIdNumberType::*;
        match s {
            "br_cpf" => Ok(BrCpf),
            "sg_nric" => Ok(SgNric),
            "us_ssn" => Ok(UsSsn),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for GelatoVerifiedOutputsIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoVerifiedOutputsIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for GelatoVerifiedOutputsIdNumberType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoVerifiedOutputsIdNumberType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(GelatoVerifiedOutputsIdNumberType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoVerifiedOutputsIdNumberType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoVerifiedOutputsIdNumberType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for GelatoVerifiedOutputsIdNumberType")
        })
    }
}
/// The user's verified sex.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoVerifiedOutputsSex {
    Redacted,
    Female,
    Male,
    Unknown,
}
impl GelatoVerifiedOutputsSex {
    pub fn as_str(self) -> &'static str {
        use GelatoVerifiedOutputsSex::*;
        match self {
            Redacted => "[redacted]",
            Female => "female",
            Male => "male",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for GelatoVerifiedOutputsSex {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoVerifiedOutputsSex::*;
        match s {
            "[redacted]" => Ok(Redacted),
            "female" => Ok(Female),
            "male" => Ok(Male),
            "unknown" => Ok(Unknown),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for GelatoVerifiedOutputsSex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoVerifiedOutputsSex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for GelatoVerifiedOutputsSex {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoVerifiedOutputsSex> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoVerifiedOutputsSex::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoVerifiedOutputsSex);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoVerifiedOutputsSex {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for GelatoVerifiedOutputsSex"))
    }
}
