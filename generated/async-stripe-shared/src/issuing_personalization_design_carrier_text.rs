#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct IssuingPersonalizationDesignCarrierTextBuilder {
    footer_body: Option<Option<String>>,
    footer_title: Option<Option<String>>,
    header_body: Option<Option<String>>,
    header_title: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: IssuingPersonalizationDesignCarrierTextBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingPersonalizationDesignCarrierTextBuilder {
        type Out = IssuingPersonalizationDesignCarrierText;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "footer_body" => Deserialize::begin(&mut self.footer_body),
                "footer_title" => Deserialize::begin(&mut self.footer_title),
                "header_body" => Deserialize::begin(&mut self.header_body),
                "header_title" => Deserialize::begin(&mut self.header_title),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                footer_body: Deserialize::default(),
                footer_title: Deserialize::default(),
                header_body: Deserialize::default(),
                header_title: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                footer_body: self.footer_body.take()?,
                footer_title: self.footer_title.take()?,
                header_body: self.header_body.take()?,
                header_title: self.header_title.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for IssuingPersonalizationDesignCarrierText {
        type Builder = IssuingPersonalizationDesignCarrierTextBuilder;
    }

    impl FromValueOpt for IssuingPersonalizationDesignCarrierText {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingPersonalizationDesignCarrierTextBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "footer_body" => b.footer_body = Some(FromValueOpt::from_value(v)?),
                    "footer_title" => b.footer_title = Some(FromValueOpt::from_value(v)?),
                    "header_body" => b.header_body = Some(FromValueOpt::from_value(v)?),
                    "header_title" => b.header_title = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
