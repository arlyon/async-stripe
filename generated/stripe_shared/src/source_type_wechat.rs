#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeWechat {
    pub prepay_id: Option<String>,
    pub qr_code_url: Option<String>,
    pub statement_descriptor: Option<String>,
}
#[doc(hidden)]
pub struct SourceTypeWechatBuilder {
    prepay_id: Option<Option<String>>,
    qr_code_url: Option<Option<String>>,
    statement_descriptor: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeWechat {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeWechat>,
        builder: SourceTypeWechatBuilder,
    }

    impl Visitor for Place<SourceTypeWechat> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeWechatBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeWechatBuilder {
        type Out = SourceTypeWechat;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "prepay_id" => Deserialize::begin(&mut self.prepay_id),
                "qr_code_url" => Deserialize::begin(&mut self.qr_code_url),
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                prepay_id: Deserialize::default(),
                qr_code_url: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                prepay_id: self.prepay_id.take()?,
                qr_code_url: self.qr_code_url.take()?,
                statement_descriptor: self.statement_descriptor.take()?,
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

    impl ObjectDeser for SourceTypeWechat {
        type Builder = SourceTypeWechatBuilder;
    }

    impl FromValueOpt for SourceTypeWechat {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTypeWechatBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "prepay_id" => b.prepay_id = Some(FromValueOpt::from_value(v)?),
                    "qr_code_url" => b.qr_code_url = Some(FromValueOpt::from_value(v)?),
                    "statement_descriptor" => {
                        b.statement_descriptor = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
