#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct LegalEntityPersonVerification {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    pub additional_document: Option<stripe_shared::LegalEntityPersonVerificationDocument>,
    /// A user-displayable string describing the verification state for the person.
    /// For example, this may say "Provided identity information could not be verified".
    pub details: Option<String>,
    /// One of `document_address_mismatch`, `document_dob_mismatch`, `document_duplicate_type`, `document_id_number_mismatch`, `document_name_mismatch`, `document_nationality_mismatch`, `failed_keyed_identity`, or `failed_other`.
    /// A machine-readable code specifying the verification state for the person.
    pub details_code: Option<String>,
    pub document: Option<stripe_shared::LegalEntityPersonVerificationDocument>,
    /// The state of verification for the person.
    /// Possible values are `unverified`, `pending`, or `verified`.
    /// Please refer [guide](https://stripe.com/docs/connect/handling-api-verification) to handle verification updates.
    pub status: String,
}
#[doc(hidden)]
pub struct LegalEntityPersonVerificationBuilder {
    additional_document: Option<Option<stripe_shared::LegalEntityPersonVerificationDocument>>,
    details: Option<Option<String>>,
    details_code: Option<Option<String>>,
    document: Option<Option<stripe_shared::LegalEntityPersonVerificationDocument>>,
    status: Option<String>,
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

    impl Deserialize for LegalEntityPersonVerification {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LegalEntityPersonVerification>,
        builder: LegalEntityPersonVerificationBuilder,
    }

    impl Visitor for Place<LegalEntityPersonVerification> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: LegalEntityPersonVerificationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for LegalEntityPersonVerificationBuilder {
        type Out = LegalEntityPersonVerification;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "additional_document" => Deserialize::begin(&mut self.additional_document),
                "details" => Deserialize::begin(&mut self.details),
                "details_code" => Deserialize::begin(&mut self.details_code),
                "document" => Deserialize::begin(&mut self.document),
                "status" => Deserialize::begin(&mut self.status),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                additional_document: Deserialize::default(),
                details: Deserialize::default(),
                details_code: Deserialize::default(),
                document: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(additional_document),
                Some(details),
                Some(details_code),
                Some(document),
                Some(status),
            ) = (
                self.additional_document.take(),
                self.details.take(),
                self.details_code.take(),
                self.document.take(),
                self.status.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { additional_document, details, details_code, document, status })
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

    impl ObjectDeser for LegalEntityPersonVerification {
        type Builder = LegalEntityPersonVerificationBuilder;
    }

    impl FromValueOpt for LegalEntityPersonVerification {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = LegalEntityPersonVerificationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "additional_document" => b.additional_document = FromValueOpt::from_value(v),
                    "details" => b.details = FromValueOpt::from_value(v),
                    "details_code" => b.details_code = FromValueOpt::from_value(v),
                    "document" => b.document = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
