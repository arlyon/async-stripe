#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    /// Please refer [guide](https://docs.stripe.com/connect/handling-api-verification) to handle verification updates.
    pub status: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for LegalEntityPersonVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LegalEntityPersonVerification").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: LegalEntityPersonVerificationBuilder {
                    additional_document: Deserialize::default(),
                    details: Deserialize::default(),
                    details_code: Deserialize::default(),
                    document: Deserialize::default(),
                    status: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "additional_document" => Deserialize::begin(&mut self.builder.additional_document),
                "details" => Deserialize::begin(&mut self.builder.details),
                "details_code" => Deserialize::begin(&mut self.builder.details_code),
                "document" => Deserialize::begin(&mut self.builder.document),
                "status" => Deserialize::begin(&mut self.builder.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(additional_document),
                Some(details),
                Some(details_code),
                Some(document),
                Some(status),
            ) = (
                self.builder.additional_document.take(),
                self.builder.details.take(),
                self.builder.details_code.take(),
                self.builder.document.take(),
                self.builder.status.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(LegalEntityPersonVerification {
                additional_document,
                details,
                details_code,
                document,
                status,
            });
            Ok(())
        }
    }
};
