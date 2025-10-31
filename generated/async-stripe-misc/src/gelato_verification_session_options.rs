#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoVerificationSessionOptions {
    pub document: Option<stripe_misc::GelatoSessionDocumentOptions>,
    pub email: Option<stripe_misc::GelatoSessionEmailOptions>,
    pub id_number: Option<stripe_misc::GelatoSessionIdNumberOptions>,
    pub matching: Option<stripe_misc::GelatoSessionMatchingOptions>,
    pub phone: Option<stripe_misc::GelatoSessionPhoneOptions>,
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
                builder: GelatoVerificationSessionOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoVerificationSessionOptionsBuilder {
        type Out = GelatoVerificationSessionOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "document" => Deserialize::begin(&mut self.document),
                "email" => Deserialize::begin(&mut self.email),
                "id_number" => Deserialize::begin(&mut self.id_number),
                "matching" => Deserialize::begin(&mut self.matching),
                "phone" => Deserialize::begin(&mut self.phone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                document: Deserialize::default(),
                email: Deserialize::default(),
                id_number: Deserialize::default(),
                matching: Deserialize::default(),
                phone: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(document), Some(email), Some(id_number), Some(matching), Some(phone)) =
                (self.document.take(), self.email, self.id_number, self.matching, self.phone)
            else {
                return None;
            };
            Some(Self::Out { document, email, id_number, matching, phone })
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

    impl ObjectDeser for GelatoVerificationSessionOptions {
        type Builder = GelatoVerificationSessionOptionsBuilder;
    }

    impl FromValueOpt for GelatoVerificationSessionOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoVerificationSessionOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "document" => b.document = FromValueOpt::from_value(v),
                    "email" => b.email = FromValueOpt::from_value(v),
                    "id_number" => b.id_number = FromValueOpt::from_value(v),
                    "matching" => b.matching = FromValueOpt::from_value(v),
                    "phone" => b.phone = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
