/// Shows last VerificationSession error
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoSessionLastError {
    /// A short machine-readable string giving the reason for the verification or user-session failure.
    pub code: Option<GelatoSessionLastErrorCode>,
    /// A message that explains the reason for verification or user-session failure.
    pub reason: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoSessionLastError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoSessionLastError").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoSessionLastErrorBuilder {
    code: Option<Option<GelatoSessionLastErrorCode>>,
    reason: Option<Option<String>>,
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
                builder: GelatoSessionLastErrorBuilder {
                    code: Deserialize::default(),
                    reason: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.builder.code),
                "reason" => Deserialize::begin(&mut self.builder.reason),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(code), Some(reason)) = (self.builder.code.take(), self.builder.reason.take())
            else {
                return Ok(());
            };
            *self.out = Some(GelatoSessionLastError { code, reason });
            Ok(())
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
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "GelatoSessionLastErrorCode");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for GelatoSessionLastErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for GelatoSessionLastErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoSessionLastErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(GelatoSessionLastErrorCode)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for GelatoSessionLastErrorCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<GelatoSessionLastErrorCode> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoSessionLastErrorCode::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoSessionLastErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
