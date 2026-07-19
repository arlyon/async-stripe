#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct LegalEntityCompanyVerification {
    pub document: stripe_shared::LegalEntityCompanyVerificationDocument,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for LegalEntityCompanyVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LegalEntityCompanyVerification").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct LegalEntityCompanyVerificationBuilder {
    document: Option<stripe_shared::LegalEntityCompanyVerificationDocument>,
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

    impl Deserialize for LegalEntityCompanyVerification {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LegalEntityCompanyVerification>,
        builder: LegalEntityCompanyVerificationBuilder,
    }

    impl Visitor for Place<LegalEntityCompanyVerification> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: LegalEntityCompanyVerificationBuilder { document: Deserialize::default() },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "document" => Deserialize::begin(&mut self.builder.document),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(document),) = (self.builder.document.take(),) else {
                return Ok(());
            };
            *self.out = Some(LegalEntityCompanyVerification { document });
            Ok(())
        }
    }
};
