#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoVerificationSessionOptions {
    pub document: Option<stripe_misc::GelatoSessionDocumentOptions>,
    pub email: Option<stripe_misc::GelatoSessionEmailOptions>,
    pub id_number: Option<stripe_misc::GelatoSessionIdNumberOptions>,
    pub matching: Option<stripe_misc::GelatoSessionMatchingOptions>,
    pub phone: Option<stripe_misc::GelatoSessionPhoneOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoVerificationSessionOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoVerificationSessionOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoVerificationSessionOptionsBuilder {
    document: Option<Option<stripe_misc::GelatoSessionDocumentOptions>>,
    email: Option<Option<stripe_misc::GelatoSessionEmailOptions>>,
    id_number: Option<Option<stripe_misc::GelatoSessionIdNumberOptions>>,
    matching: Option<Option<stripe_misc::GelatoSessionMatchingOptions>>,
    phone: Option<Option<stripe_misc::GelatoSessionPhoneOptions>>,
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

    impl Deserialize for GelatoVerificationSessionOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoVerificationSessionOptions>,
        builder: GelatoVerificationSessionOptionsBuilder,
    }

    impl Visitor for Place<GelatoVerificationSessionOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoVerificationSessionOptionsBuilder {
                    document: Deserialize::default(),
                    email: Deserialize::default(),
                    id_number: Deserialize::default(),
                    matching: Deserialize::default(),
                    phone: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "document" => Deserialize::begin(&mut self.builder.document),
                "email" => Deserialize::begin(&mut self.builder.email),
                "id_number" => Deserialize::begin(&mut self.builder.id_number),
                "matching" => Deserialize::begin(&mut self.builder.matching),
                "phone" => Deserialize::begin(&mut self.builder.phone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(document), Some(email), Some(id_number), Some(matching), Some(phone)) = (
                self.builder.document.take(),
                self.builder.email,
                self.builder.id_number,
                self.builder.matching.take(),
                self.builder.phone,
            ) else {
                return Ok(());
            };
            *self.out = Some(GelatoVerificationSessionOptions {
                document,
                email,
                id_number,
                matching,
                phone,
            });
            Ok(())
        }
    }
};
