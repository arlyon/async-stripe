#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardholderIdDocument {
    /// The back of a document returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `identity_document`.
    pub back: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// The front of a document returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `identity_document`.
    pub front: Option<stripe_types::Expandable<stripe_shared::File>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardholderIdDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingCardholderIdDocument").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingCardholderIdDocumentBuilder {
    back: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
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

    impl Deserialize for IssuingCardholderIdDocument {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardholderIdDocument>,
        builder: IssuingCardholderIdDocumentBuilder,
    }

    impl Visitor for Place<IssuingCardholderIdDocument> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardholderIdDocumentBuilder {
                    back: Deserialize::default(),
                    front: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "back" => Deserialize::begin(&mut self.builder.back),
                "front" => Deserialize::begin(&mut self.builder.front),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(back), Some(front)) = (self.builder.back.take(), self.builder.front.take())
            else {
                return Ok(());
            };
            *self.out = Some(IssuingCardholderIdDocument { back, front });
            Ok(())
        }
    }
};
