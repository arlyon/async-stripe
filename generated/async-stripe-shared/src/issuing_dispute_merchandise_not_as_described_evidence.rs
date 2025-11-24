#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingDisputeMerchandiseNotAsDescribedEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Date when the product was received.
    pub received_at: Option<stripe_types::Timestamp>,
    /// Description of the cardholder's attempt to return the product.
    pub return_description: Option<String>,
    /// Result of cardholder's attempt to return the product.
    pub return_status: Option<IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus>,
    /// Date when the product was returned or attempted to be returned.
    pub returned_at: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct IssuingDisputeMerchandiseNotAsDescribedEvidenceBuilder {
    additional_documentation: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    explanation: Option<Option<String>>,
    received_at: Option<Option<stripe_types::Timestamp>>,
    return_description: Option<Option<String>>,
    return_status: Option<Option<IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus>>,
    returned_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for IssuingDisputeMerchandiseNotAsDescribedEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeMerchandiseNotAsDescribedEvidence>,
        builder: IssuingDisputeMerchandiseNotAsDescribedEvidenceBuilder,
    }

    impl Visitor for Place<IssuingDisputeMerchandiseNotAsDescribedEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingDisputeMerchandiseNotAsDescribedEvidenceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingDisputeMerchandiseNotAsDescribedEvidenceBuilder {
        type Out = IssuingDisputeMerchandiseNotAsDescribedEvidence;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "additional_documentation" => {
                    Deserialize::begin(&mut self.additional_documentation)
                }
                "explanation" => Deserialize::begin(&mut self.explanation),
                "received_at" => Deserialize::begin(&mut self.received_at),
                "return_description" => Deserialize::begin(&mut self.return_description),
                "return_status" => Deserialize::begin(&mut self.return_status),
                "returned_at" => Deserialize::begin(&mut self.returned_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                additional_documentation: Deserialize::default(),
                explanation: Deserialize::default(),
                received_at: Deserialize::default(),
                return_description: Deserialize::default(),
                return_status: Deserialize::default(),
                returned_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(additional_documentation),
                Some(explanation),
                Some(received_at),
                Some(return_description),
                Some(return_status),
                Some(returned_at),
            ) = (
                self.additional_documentation.take(),
                self.explanation.take(),
                self.received_at,
                self.return_description.take(),
                self.return_status.take(),
                self.returned_at,
            )
            else {
                return None;
            };
            Some(Self::Out {
                additional_documentation,
                explanation,
                received_at,
                return_description,
                return_status,
                returned_at,
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

    impl ObjectDeser for IssuingDisputeMerchandiseNotAsDescribedEvidence {
        type Builder = IssuingDisputeMerchandiseNotAsDescribedEvidenceBuilder;
    }

    impl FromValueOpt for IssuingDisputeMerchandiseNotAsDescribedEvidence {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingDisputeMerchandiseNotAsDescribedEvidenceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "additional_documentation" => {
                        b.additional_documentation = FromValueOpt::from_value(v)
                    }
                    "explanation" => b.explanation = FromValueOpt::from_value(v),
                    "received_at" => b.received_at = FromValueOpt::from_value(v),
                    "return_description" => b.return_description = FromValueOpt::from_value(v),
                    "return_status" => b.return_status = FromValueOpt::from_value(v),
                    "returned_at" => b.returned_at = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Result of cardholder's attempt to return the product.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    MerchantRejected,
    Successful,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    pub fn as_str(&self) -> &str {
        use IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus::*;
        match self {
            MerchantRejected => "merchant_rejected",
            Successful => "successful",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus::*;
        match s {
            "merchant_rejected" => Ok(MerchantRejected),
            "successful" => Ok(Successful),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
