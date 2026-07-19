#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingPersonalizationDesignCarrierText {
    /// The footer body text of the carrier letter.
    pub footer_body: Option<String>,
    /// The footer title text of the carrier letter.
    pub footer_title: Option<String>,
    /// The header body text of the carrier letter.
    pub header_body: Option<String>,
    /// The header title text of the carrier letter.
    pub header_title: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingPersonalizationDesignCarrierText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingPersonalizationDesignCarrierText").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingPersonalizationDesignCarrierTextBuilder {
    footer_body: Option<Option<String>>,
    footer_title: Option<Option<String>>,
    header_body: Option<Option<String>>,
    header_title: Option<Option<String>>,
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

    impl Deserialize for IssuingPersonalizationDesignCarrierText {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingPersonalizationDesignCarrierText>,
        builder: IssuingPersonalizationDesignCarrierTextBuilder,
    }

    impl Visitor for Place<IssuingPersonalizationDesignCarrierText> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingPersonalizationDesignCarrierTextBuilder {
                    footer_body: Deserialize::default(),
                    footer_title: Deserialize::default(),
                    header_body: Deserialize::default(),
                    header_title: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "footer_body" => Deserialize::begin(&mut self.builder.footer_body),
                "footer_title" => Deserialize::begin(&mut self.builder.footer_title),
                "header_body" => Deserialize::begin(&mut self.builder.header_body),
                "header_title" => Deserialize::begin(&mut self.builder.header_title),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(footer_body), Some(footer_title), Some(header_body), Some(header_title)) = (
                self.builder.footer_body.take(),
                self.builder.footer_title.take(),
                self.builder.header_body.take(),
                self.builder.header_title.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(IssuingPersonalizationDesignCarrierText {
                footer_body,
                footer_title,
                header_body,
                header_title,
            });
            Ok(())
        }
    }
};
