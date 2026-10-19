#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationVerificationData {
    /// Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`.
    pub address_line1_check: stripe_shared::IssuingAuthorizationVerificationDataCvcCheck,
    /// Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`.
    pub address_postal_code_check: stripe_shared::IssuingAuthorizationVerificationDataCvcCheck,
    /// The exemption applied to this authorization.
    pub authentication_exemption:
        Option<stripe_shared::IssuingAuthorizationAuthenticationExemption>,
    /// Whether the cardholder provided a CVC and if it matched Stripe’s record.
    pub cvc_check: stripe_shared::IssuingAuthorizationVerificationDataCvcCheck,
    /// Whether the cardholder provided an expiry date and if it matched Stripe’s record.
    pub expiry_check: stripe_shared::IssuingAuthorizationVerificationDataCvcCheck,
    /// The postal code submitted as part of the authorization used for postal code verification.
    pub postal_code: Option<String>,
    /// 3D Secure details.
    pub three_d_secure: Option<stripe_shared::IssuingAuthorizationThreeDSecure>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationVerificationData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationVerificationData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingAuthorizationVerificationDataBuilder {
    address_line1_check: Option<stripe_shared::IssuingAuthorizationVerificationDataCvcCheck>,
    address_postal_code_check: Option<stripe_shared::IssuingAuthorizationVerificationDataCvcCheck>,
    authentication_exemption:
        Option<Option<stripe_shared::IssuingAuthorizationAuthenticationExemption>>,
    cvc_check: Option<stripe_shared::IssuingAuthorizationVerificationDataCvcCheck>,
    expiry_check: Option<stripe_shared::IssuingAuthorizationVerificationDataCvcCheck>,
    postal_code: Option<Option<String>>,
    three_d_secure: Option<Option<stripe_shared::IssuingAuthorizationThreeDSecure>>,
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

    impl Deserialize for IssuingAuthorizationVerificationData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationVerificationData>,
        builder: IssuingAuthorizationVerificationDataBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationVerificationData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationVerificationDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationVerificationDataBuilder {
        type Out = IssuingAuthorizationVerificationData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address_line1_check" => Deserialize::begin(&mut self.address_line1_check),
                "address_postal_code_check" => {
                    Deserialize::begin(&mut self.address_postal_code_check)
                }
                "authentication_exemption" => {
                    Deserialize::begin(&mut self.authentication_exemption)
                }
                "cvc_check" => Deserialize::begin(&mut self.cvc_check),
                "expiry_check" => Deserialize::begin(&mut self.expiry_check),
                "postal_code" => Deserialize::begin(&mut self.postal_code),
                "three_d_secure" => Deserialize::begin(&mut self.three_d_secure),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address_line1_check: None,
                address_postal_code_check: None,
                authentication_exemption: Some(None),
                cvc_check: None,
                expiry_check: None,
                postal_code: Some(None),
                three_d_secure: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(address_line1_check),
                Some(address_postal_code_check),
                Some(authentication_exemption),
                Some(cvc_check),
                Some(expiry_check),
                Some(postal_code),
                Some(three_d_secure),
            ) = (
                self.address_line1_check.take(),
                self.address_postal_code_check.take(),
                self.authentication_exemption.take(),
                self.cvc_check.take(),
                self.expiry_check.take(),
                self.postal_code.take(),
                self.three_d_secure.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                address_line1_check,
                address_postal_code_check,
                authentication_exemption,
                cvc_check,
                expiry_check,
                postal_code,
                three_d_secure,
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

    impl ObjectDeser for IssuingAuthorizationVerificationData {
        type Builder = IssuingAuthorizationVerificationDataBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationVerificationData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingAuthorizationVerificationDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address_line1_check" => b.address_line1_check = FromValueOpt::from_value(v),
                    "address_postal_code_check" => {
                        b.address_postal_code_check = FromValueOpt::from_value(v)
                    }
                    "authentication_exemption" => {
                        b.authentication_exemption = FromValueOpt::from_value(v)
                    }
                    "cvc_check" => b.cvc_check = FromValueOpt::from_value(v),
                    "expiry_check" => b.expiry_check = FromValueOpt::from_value(v),
                    "postal_code" => b.postal_code = FromValueOpt::from_value(v),
                    "three_d_secure" => b.three_d_secure = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`.
pub type IssuingAuthorizationVerificationDataAddressLine1Check =
    stripe_shared::IssuingAuthorizationVerificationDataCvcCheck;
/// Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`.
pub type IssuingAuthorizationVerificationDataAddressPostalCodeCheck =
    stripe_shared::IssuingAuthorizationVerificationDataCvcCheck;
/// Whether the cardholder provided a CVC and if it matched Stripe’s record.
pub type IssuingAuthorizationVerificationDataCvcCheck =
    stripe_shared::IssuingAuthorizationVerificationDataCvcCheck;
/// Whether the cardholder provided an expiry date and if it matched Stripe’s record.
pub type IssuingAuthorizationVerificationDataExpiryCheck =
    stripe_shared::IssuingAuthorizationVerificationDataCvcCheck;
