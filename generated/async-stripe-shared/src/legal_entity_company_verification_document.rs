#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct LegalEntityCompanyVerificationDocument {
    /// The back of a document returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `additional_verification`.
    /// Note that `additional_verification` files are [not downloadable](/file-upload#uploading-a-file).
    pub back: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// A user-displayable string describing the verification state of this document.
    pub details: Option<String>,
    /// One of `document_corrupt`, `document_expired`, `document_failed_copy`, `document_failed_greyscale`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_not_readable`, `document_not_uploaded`, `document_type_not_supported`, or `document_too_large`.
    /// A machine-readable code specifying the verification state for this document.
    pub details_code: Option<String>,
    /// The front of a document returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `additional_verification`.
    /// Note that `additional_verification` files are [not downloadable](/file-upload#uploading-a-file).
    pub front: Option<stripe_types::Expandable<stripe_shared::File>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for LegalEntityCompanyVerificationDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LegalEntityCompanyVerificationDocument").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: LegalEntityCompanyVerificationDocumentBuilder {
                    back: Deserialize::default(),
                    details: Deserialize::default(),
                    details_code: Deserialize::default(),
                    front: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "back" => Deserialize::begin(&mut self.builder.back),
                "details" => Deserialize::begin(&mut self.builder.details),
                "details_code" => Deserialize::begin(&mut self.builder.details_code),
                "front" => Deserialize::begin(&mut self.builder.front),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(back), Some(details), Some(details_code), Some(front)) = (
                self.builder.back.take(),
                self.builder.details.take(),
                self.builder.details_code.take(),
                self.builder.front.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(LegalEntityCompanyVerificationDocument { back, details, details_code, front });
            Ok(())
        }
    }
};
