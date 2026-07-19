#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingDisputeNotReceivedEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// Date when the cardholder expected to receive the product.
    pub expected_at: Option<stripe_types::Timestamp>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,
    /// Whether the product was a merchandise or service.
    pub product_type: Option<IssuingDisputeNotReceivedEvidenceProductType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingDisputeNotReceivedEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingDisputeNotReceivedEvidence").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingDisputeNotReceivedEvidenceBuilder {
    additional_documentation: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    expected_at: Option<Option<stripe_types::Timestamp>>,
    explanation: Option<Option<String>>,
    product_description: Option<Option<String>>,
    product_type: Option<Option<IssuingDisputeNotReceivedEvidenceProductType>>,
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

    impl Deserialize for IssuingDisputeNotReceivedEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeNotReceivedEvidence>,
        builder: IssuingDisputeNotReceivedEvidenceBuilder,
    }

    impl Visitor for Place<IssuingDisputeNotReceivedEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingDisputeNotReceivedEvidenceBuilder {
                    additional_documentation: Deserialize::default(),
                    expected_at: Deserialize::default(),
                    explanation: Deserialize::default(),
                    product_description: Deserialize::default(),
                    product_type: Deserialize::default(),
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
                "expected_at" => Deserialize::begin(&mut self.builder.expected_at),
                "explanation" => Deserialize::begin(&mut self.builder.explanation),
                "product_description" => Deserialize::begin(&mut self.builder.product_description),
                "product_type" => Deserialize::begin(&mut self.builder.product_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(additional_documentation),
                Some(expected_at),
                Some(explanation),
                Some(product_description),
                Some(product_type),
            ) = (
                self.builder.additional_documentation.take(),
                self.builder.expected_at,
                self.builder.explanation.take(),
                self.builder.product_description.take(),
                self.builder.product_type.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingDisputeNotReceivedEvidence {
                additional_documentation,
                expected_at,
                explanation,
                product_description,
                product_type,
            });
            Ok(())
        }
    }
};
/// Whether the product was a merchandise or service.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingDisputeNotReceivedEvidenceProductType {
    Merchandise,
    Service,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingDisputeNotReceivedEvidenceProductType {
    pub fn as_str(&self) -> &str {
        use IssuingDisputeNotReceivedEvidenceProductType::*;
        match self {
            Merchandise => "merchandise",
            Service => "service",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingDisputeNotReceivedEvidenceProductType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingDisputeNotReceivedEvidenceProductType::*;
        match s {
            "merchandise" => Ok(Merchandise),
            "service" => Ok(Service),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingDisputeNotReceivedEvidenceProductType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingDisputeNotReceivedEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingDisputeNotReceivedEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingDisputeNotReceivedEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingDisputeNotReceivedEvidenceProductType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingDisputeNotReceivedEvidenceProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for IssuingDisputeNotReceivedEvidenceProductType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingDisputeNotReceivedEvidenceProductType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IssuingDisputeNotReceivedEvidenceProductType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingDisputeNotReceivedEvidenceProductType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
