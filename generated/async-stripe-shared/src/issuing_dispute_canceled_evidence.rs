#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingDisputeCanceledEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// Date when order was canceled.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Whether the cardholder was provided with a cancellation policy.
    pub cancellation_policy_provided: Option<bool>,
    /// Reason for canceling the order.
    pub cancellation_reason: Option<String>,
    /// Date when the cardholder expected to receive the product.
    pub expected_at: Option<stripe_types::Timestamp>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,
    /// Whether the product was a merchandise or service.
    pub product_type: Option<IssuingDisputeCanceledEvidenceProductType>,
    /// Result of cardholder's attempt to return the product.
    pub return_status: Option<IssuingDisputeCanceledEvidenceReturnStatus>,
    /// Date when the product was returned or attempted to be returned.
    pub returned_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingDisputeCanceledEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingDisputeCanceledEvidence").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingDisputeCanceledEvidenceBuilder {
    additional_documentation: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    cancellation_policy_provided: Option<Option<bool>>,
    cancellation_reason: Option<Option<String>>,
    expected_at: Option<Option<stripe_types::Timestamp>>,
    explanation: Option<Option<String>>,
    product_description: Option<Option<String>>,
    product_type: Option<Option<IssuingDisputeCanceledEvidenceProductType>>,
    return_status: Option<Option<IssuingDisputeCanceledEvidenceReturnStatus>>,
    returned_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for IssuingDisputeCanceledEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeCanceledEvidence>,
        builder: IssuingDisputeCanceledEvidenceBuilder,
    }

    impl Visitor for Place<IssuingDisputeCanceledEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingDisputeCanceledEvidenceBuilder {
                    additional_documentation: Deserialize::default(),
                    canceled_at: Deserialize::default(),
                    cancellation_policy_provided: Deserialize::default(),
                    cancellation_reason: Deserialize::default(),
                    expected_at: Deserialize::default(),
                    explanation: Deserialize::default(),
                    product_description: Deserialize::default(),
                    product_type: Deserialize::default(),
                    return_status: Deserialize::default(),
                    returned_at: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "additional_documentation" => {
                    Deserialize::begin(&mut self.builder.additional_documentation)
                }
                "canceled_at" => Deserialize::begin(&mut self.builder.canceled_at),
                "cancellation_policy_provided" => {
                    Deserialize::begin(&mut self.builder.cancellation_policy_provided)
                }
                "cancellation_reason" => Deserialize::begin(&mut self.builder.cancellation_reason),
                "expected_at" => Deserialize::begin(&mut self.builder.expected_at),
                "explanation" => Deserialize::begin(&mut self.builder.explanation),
                "product_description" => Deserialize::begin(&mut self.builder.product_description),
                "product_type" => Deserialize::begin(&mut self.builder.product_type),
                "return_status" => Deserialize::begin(&mut self.builder.return_status),
                "returned_at" => Deserialize::begin(&mut self.builder.returned_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(additional_documentation),
                Some(canceled_at),
                Some(cancellation_policy_provided),
                Some(cancellation_reason),
                Some(expected_at),
                Some(explanation),
                Some(product_description),
                Some(product_type),
                Some(return_status),
                Some(returned_at),
            ) = (
                self.builder.additional_documentation.take(),
                self.builder.canceled_at,
                self.builder.cancellation_policy_provided,
                self.builder.cancellation_reason.take(),
                self.builder.expected_at,
                self.builder.explanation.take(),
                self.builder.product_description.take(),
                self.builder.product_type.take(),
                self.builder.return_status.take(),
                self.builder.returned_at,
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingDisputeCanceledEvidence {
                additional_documentation,
                canceled_at,
                cancellation_policy_provided,
                cancellation_reason,
                expected_at,
                explanation,
                product_description,
                product_type,
                return_status,
                returned_at,
            });
            Ok(())
        }
    }
};
/// Whether the product was a merchandise or service.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingDisputeCanceledEvidenceProductType {
    Merchandise,
    Service,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingDisputeCanceledEvidenceProductType {
    pub fn as_str(&self) -> &str {
        use IssuingDisputeCanceledEvidenceProductType::*;
        match self {
            Merchandise => "merchandise",
            Service => "service",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingDisputeCanceledEvidenceProductType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingDisputeCanceledEvidenceProductType::*;
        match s {
            "merchandise" => Ok(Merchandise),
            "service" => Ok(Service),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingDisputeCanceledEvidenceProductType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingDisputeCanceledEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingDisputeCanceledEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingDisputeCanceledEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingDisputeCanceledEvidenceProductType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingDisputeCanceledEvidenceProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for IssuingDisputeCanceledEvidenceProductType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingDisputeCanceledEvidenceProductType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IssuingDisputeCanceledEvidenceProductType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingDisputeCanceledEvidenceProductType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Result of cardholder's attempt to return the product.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingDisputeCanceledEvidenceReturnStatus {
    MerchantRejected,
    Successful,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingDisputeCanceledEvidenceReturnStatus {
    pub fn as_str(&self) -> &str {
        use IssuingDisputeCanceledEvidenceReturnStatus::*;
        match self {
            MerchantRejected => "merchant_rejected",
            Successful => "successful",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingDisputeCanceledEvidenceReturnStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingDisputeCanceledEvidenceReturnStatus::*;
        match s {
            "merchant_rejected" => Ok(MerchantRejected),
            "successful" => Ok(Successful),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingDisputeCanceledEvidenceReturnStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingDisputeCanceledEvidenceReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingDisputeCanceledEvidenceReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingDisputeCanceledEvidenceReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingDisputeCanceledEvidenceReturnStatus))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingDisputeCanceledEvidenceReturnStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for IssuingDisputeCanceledEvidenceReturnStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingDisputeCanceledEvidenceReturnStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IssuingDisputeCanceledEvidenceReturnStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingDisputeCanceledEvidenceReturnStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
