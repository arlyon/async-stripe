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
    pub product_type: Option<stripe_shared::IssuingDisputeOtherEvidenceProductType>,
    /// Result of cardholder's attempt to return the product.
    pub return_status: Option<stripe_shared::ReturnStatus>,
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
    product_type: Option<Option<stripe_shared::IssuingDisputeOtherEvidenceProductType>>,
    return_status: Option<Option<stripe_shared::ReturnStatus>>,
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
                builder: IssuingDisputeCanceledEvidenceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingDisputeCanceledEvidenceBuilder {
        type Out = IssuingDisputeCanceledEvidence;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "additional_documentation" => {
                    Deserialize::begin(&mut self.additional_documentation)
                }
                "canceled_at" => Deserialize::begin(&mut self.canceled_at),
                "cancellation_policy_provided" => {
                    Deserialize::begin(&mut self.cancellation_policy_provided)
                }
                "cancellation_reason" => Deserialize::begin(&mut self.cancellation_reason),
                "expected_at" => Deserialize::begin(&mut self.expected_at),
                "explanation" => Deserialize::begin(&mut self.explanation),
                "product_description" => Deserialize::begin(&mut self.product_description),
                "product_type" => Deserialize::begin(&mut self.product_type),
                "return_status" => Deserialize::begin(&mut self.return_status),
                "returned_at" => Deserialize::begin(&mut self.returned_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                additional_documentation: Some(None),
                canceled_at: Some(None),
                cancellation_policy_provided: Some(None),
                cancellation_reason: Some(None),
                expected_at: Some(None),
                explanation: Some(None),
                product_description: Some(None),
                product_type: Some(None),
                return_status: Some(None),
                returned_at: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.additional_documentation.take(),
                self.canceled_at,
                self.cancellation_policy_provided,
                self.cancellation_reason.take(),
                self.expected_at,
                self.explanation.take(),
                self.product_description.take(),
                self.product_type.take(),
                self.return_status.take(),
                self.returned_at,
            )
            else {
                return None;
            };
            Some(Self::Out {
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

    impl ObjectDeser for IssuingDisputeCanceledEvidence {
        type Builder = IssuingDisputeCanceledEvidenceBuilder;
    }

    impl FromValueOpt for IssuingDisputeCanceledEvidence {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingDisputeCanceledEvidenceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "additional_documentation" => {
                        b.additional_documentation = FromValueOpt::from_value(v)
                    }
                    "canceled_at" => b.canceled_at = FromValueOpt::from_value(v),
                    "cancellation_policy_provided" => {
                        b.cancellation_policy_provided = FromValueOpt::from_value(v)
                    }
                    "cancellation_reason" => b.cancellation_reason = FromValueOpt::from_value(v),
                    "expected_at" => b.expected_at = FromValueOpt::from_value(v),
                    "explanation" => b.explanation = FromValueOpt::from_value(v),
                    "product_description" => b.product_description = FromValueOpt::from_value(v),
                    "product_type" => b.product_type = FromValueOpt::from_value(v),
                    "return_status" => b.return_status = FromValueOpt::from_value(v),
                    "returned_at" => b.returned_at = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Whether the product was a merchandise or service.
pub type IssuingDisputeCanceledEvidenceProductType =
    stripe_shared::IssuingDisputeOtherEvidenceProductType;
/// Result of cardholder's attempt to return the product.
pub type IssuingDisputeCanceledEvidenceReturnStatus = stripe_shared::ReturnStatus;
