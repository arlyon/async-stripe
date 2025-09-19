/// Shows last VerificationSession error
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoSessionLastError {
    /// A short machine-readable string giving the reason for the verification or user-session failure.
    pub code: Option<GelatoSessionLastErrorCode>,
    /// A message that explains the reason for verification or user-session failure.
    pub reason: Option<String>,
}
#[doc(hidden)]
pub struct GelatoSessionLastErrorBuilder {
    code: Option<Option<GelatoSessionLastErrorCode>>,
    reason: Option<Option<String>>,
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

    impl Deserialize for GelatoSessionLastError {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoSessionLastError>,
        builder: GelatoSessionLastErrorBuilder,
    }

    impl Visitor for Place<GelatoSessionLastError> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoSessionLastErrorBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoSessionLastErrorBuilder {
        type Out = GelatoSessionLastError;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.code),
                "reason" => Deserialize::begin(&mut self.reason),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { code: Deserialize::default(), reason: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(code), Some(reason)) = (self.code.take(), self.reason.take()) else {
                return None;
            };
            Some(Self::Out { code, reason })
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

    impl ObjectDeser for GelatoSessionLastError {
        type Builder = GelatoSessionLastErrorBuilder;
    }

    impl FromValueOpt for GelatoSessionLastError {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoSessionLastErrorBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "code" => b.code = FromValueOpt::from_value(v),
                    "reason" => b.reason = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// A short machine-readable string giving the reason for the verification or user-session failure.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum GelatoSessionLastErrorCode {
    Abandoned,
    ConsentDeclined,
    CountryNotSupported,
    DeviceNotSupported,
    DocumentExpired,
    DocumentTypeNotSupported,
    DocumentUnverifiedOther,
    EmailUnverifiedOther,
    EmailVerificationDeclined,
    IdNumberInsufficientDocumentData,
    IdNumberMismatch,
    IdNumberUnverifiedOther,
    PhoneUnverifiedOther,
    PhoneVerificationDeclined,
    SelfieDocumentMissingPhoto,
    SelfieFaceMismatch,
    SelfieManipulated,
    SelfieUnverifiedOther,
    UnderSupportedAge,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl GelatoSessionLastErrorCode {
    pub fn as_str(&self) -> &str {
        use GelatoSessionLastErrorCode::*;
        match self {
            Abandoned => "abandoned",
            ConsentDeclined => "consent_declined",
            CountryNotSupported => "country_not_supported",
            DeviceNotSupported => "device_not_supported",
            DocumentExpired => "document_expired",
            DocumentTypeNotSupported => "document_type_not_supported",
            DocumentUnverifiedOther => "document_unverified_other",
            EmailUnverifiedOther => "email_unverified_other",
            EmailVerificationDeclined => "email_verification_declined",
            IdNumberInsufficientDocumentData => "id_number_insufficient_document_data",
            IdNumberMismatch => "id_number_mismatch",
            IdNumberUnverifiedOther => "id_number_unverified_other",
            PhoneUnverifiedOther => "phone_unverified_other",
            PhoneVerificationDeclined => "phone_verification_declined",
            SelfieDocumentMissingPhoto => "selfie_document_missing_photo",
            SelfieFaceMismatch => "selfie_face_mismatch",
            SelfieManipulated => "selfie_manipulated",
            SelfieUnverifiedOther => "selfie_unverified_other",
            UnderSupportedAge => "under_supported_age",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for GelatoSessionLastErrorCode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoSessionLastErrorCode::*;
        match s {
            "abandoned" => Ok(Abandoned),
            "consent_declined" => Ok(ConsentDeclined),
            "country_not_supported" => Ok(CountryNotSupported),
            "device_not_supported" => Ok(DeviceNotSupported),
            "document_expired" => Ok(DocumentExpired),
            "document_type_not_supported" => Ok(DocumentTypeNotSupported),
            "document_unverified_other" => Ok(DocumentUnverifiedOther),
            "email_unverified_other" => Ok(EmailUnverifiedOther),
            "email_verification_declined" => Ok(EmailVerificationDeclined),
            "id_number_insufficient_document_data" => Ok(IdNumberInsufficientDocumentData),
            "id_number_mismatch" => Ok(IdNumberMismatch),
            "id_number_unverified_other" => Ok(IdNumberUnverifiedOther),
            "phone_unverified_other" => Ok(PhoneUnverifiedOther),
            "phone_verification_declined" => Ok(PhoneVerificationDeclined),
            "selfie_document_missing_photo" => Ok(SelfieDocumentMissingPhoto),
            "selfie_face_mismatch" => Ok(SelfieFaceMismatch),
            "selfie_manipulated" => Ok(SelfieManipulated),
            "selfie_unverified_other" => Ok(SelfieUnverifiedOther),
            "under_supported_age" => Ok(UnderSupportedAge),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for GelatoSessionLastErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoSessionLastErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoSessionLastErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for GelatoSessionLastErrorCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoSessionLastErrorCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoSessionLastErrorCode::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoSessionLastErrorCode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoSessionLastErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
