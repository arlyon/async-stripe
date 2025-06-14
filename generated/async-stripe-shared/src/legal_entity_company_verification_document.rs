#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct LegalEntityCompanyVerificationDocument {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    pub back: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// A user-displayable string describing the verification state of this document.
    pub details: Option<String>,
    /// One of `document_corrupt`, `document_expired`, `document_failed_copy`, `document_failed_greyscale`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_not_readable`, `document_not_uploaded`, `document_type_not_supported`, or `document_too_large`.
    /// A machine-readable code specifying the verification state for this document.
    pub details_code: Option<String>,
    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    pub front: Option<stripe_types::Expandable<stripe_shared::File>>,
}
#[doc(hidden)]
pub struct LegalEntityCompanyVerificationDocumentBuilder {
    back: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    details: Option<Option<String>>,
    details_code: Option<Option<String>>,
    front: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for LegalEntityCompanyVerificationDocument {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LegalEntityCompanyVerificationDocument>,
        builder: LegalEntityCompanyVerificationDocumentBuilder,
    }

    impl Visitor for Place<LegalEntityCompanyVerificationDocument> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: LegalEntityCompanyVerificationDocumentBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for LegalEntityCompanyVerificationDocumentBuilder {
        type Out = LegalEntityCompanyVerificationDocument;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "back" => Deserialize::begin(&mut self.back),
                "details" => Deserialize::begin(&mut self.details),
                "details_code" => Deserialize::begin(&mut self.details_code),
                "front" => Deserialize::begin(&mut self.front),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                back: Deserialize::default(),
                details: Deserialize::default(),
                details_code: Deserialize::default(),
                front: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(back), Some(details), Some(details_code), Some(front)) = (
                self.back.take(),
                self.details.take(),
                self.details_code.take(),
                self.front.take(),
            ) else {
                return None;
            };
            Some(Self::Out { back, details, details_code, front })
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

    impl ObjectDeser for LegalEntityCompanyVerificationDocument {
        type Builder = LegalEntityCompanyVerificationDocumentBuilder;
    }

    impl FromValueOpt for LegalEntityCompanyVerificationDocument {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = LegalEntityCompanyVerificationDocumentBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "back" => b.back = FromValueOpt::from_value(v),
                    "details" => b.details = FromValueOpt::from_value(v),
                    "details_code" => b.details_code = FromValueOpt::from_value(v),
                    "front" => b.front = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
