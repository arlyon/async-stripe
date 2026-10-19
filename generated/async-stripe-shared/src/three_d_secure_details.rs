#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThreeDSecureDetails {
    /// For authenticated transactions: how the customer was authenticated by
    /// the issuing bank.
    pub authentication_flow: Option<stripe_shared::AuthenticationFlow>,
    /// The Electronic Commerce Indicator (ECI). A protocol-level field
    /// indicating what degree of authentication was performed.
    pub electronic_commerce_indicator: Option<stripe_shared::ElectronicCommerceIndicator>,
    /// Indicates the outcome of 3D Secure authentication.
    pub result: Option<stripe_shared::ThreeDSecureDetailsResult>,
    /// Additional information about why 3D Secure succeeded or failed based
    /// on the `result`.
    pub result_reason: Option<stripe_shared::ResultReason>,
    /// The 3D Secure 1 XID or 3D Secure 2 Directory Server Transaction ID
    /// (dsTransId) for this payment.
    pub transaction_id: Option<String>,
    /// The version of 3D Secure that was used.
    pub version: Option<stripe_shared::ThreeDSecureDetailsVersion>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ThreeDSecureDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ThreeDSecureDetailsBuilder {
    authentication_flow: Option<Option<stripe_shared::AuthenticationFlow>>,
    electronic_commerce_indicator: Option<Option<stripe_shared::ElectronicCommerceIndicator>>,
    result: Option<Option<stripe_shared::ThreeDSecureDetailsResult>>,
    result_reason: Option<Option<stripe_shared::ResultReason>>,
    transaction_id: Option<Option<String>>,
    version: Option<Option<stripe_shared::ThreeDSecureDetailsVersion>>,
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

    impl Deserialize for ThreeDSecureDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThreeDSecureDetails>,
        builder: ThreeDSecureDetailsBuilder,
    }

    impl Visitor for Place<ThreeDSecureDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThreeDSecureDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThreeDSecureDetailsBuilder {
        type Out = ThreeDSecureDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "authentication_flow" => Deserialize::begin(&mut self.authentication_flow),
                "electronic_commerce_indicator" => {
                    Deserialize::begin(&mut self.electronic_commerce_indicator)
                }
                "result" => Deserialize::begin(&mut self.result),
                "result_reason" => Deserialize::begin(&mut self.result_reason),
                "transaction_id" => Deserialize::begin(&mut self.transaction_id),
                "version" => Deserialize::begin(&mut self.version),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                authentication_flow: Some(None),
                electronic_commerce_indicator: Some(None),
                result: Some(None),
                result_reason: Some(None),
                transaction_id: Some(None),
                version: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(authentication_flow),
                Some(electronic_commerce_indicator),
                Some(result),
                Some(result_reason),
                Some(transaction_id),
                Some(version),
            ) = (
                self.authentication_flow.take(),
                self.electronic_commerce_indicator.take(),
                self.result.take(),
                self.result_reason.take(),
                self.transaction_id.take(),
                self.version.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                authentication_flow,
                electronic_commerce_indicator,
                result,
                result_reason,
                transaction_id,
                version,
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

    impl ObjectDeser for ThreeDSecureDetails {
        type Builder = ThreeDSecureDetailsBuilder;
    }

    impl FromValueOpt for ThreeDSecureDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThreeDSecureDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "authentication_flow" => b.authentication_flow = FromValueOpt::from_value(v),
                    "electronic_commerce_indicator" => {
                        b.electronic_commerce_indicator = FromValueOpt::from_value(v)
                    }
                    "result" => b.result = FromValueOpt::from_value(v),
                    "result_reason" => b.result_reason = FromValueOpt::from_value(v),
                    "transaction_id" => b.transaction_id = FromValueOpt::from_value(v),
                    "version" => b.version = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// For authenticated transactions: how the customer was authenticated by
/// the issuing bank.
pub type ThreeDSecureDetailsAuthenticationFlow = stripe_shared::AuthenticationFlow;
/// The Electronic Commerce Indicator (ECI). A protocol-level field
/// indicating what degree of authentication was performed.
pub type ThreeDSecureDetailsElectronicCommerceIndicator =
    stripe_shared::ElectronicCommerceIndicator;
/// Indicates the outcome of 3D Secure authentication.
pub type ThreeDSecureDetailsResult = stripe_shared::ThreeDSecureDetailsResult;
/// Additional information about why 3D Secure succeeded or failed based
/// on the `result`.
pub type ThreeDSecureDetailsResultReason = stripe_shared::ResultReason;
/// The version of 3D Secure that was used.
pub type ThreeDSecureDetailsVersion = stripe_shared::ThreeDSecureDetailsVersion;
