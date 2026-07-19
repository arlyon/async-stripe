#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeWechat {
    pub prepay_id: Option<String>,
    pub qr_code_url: Option<String>,
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTypeWechat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTypeWechat").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceTypeWechatBuilder {
    prepay_id: Option<Option<String>>,
    qr_code_url: Option<Option<String>>,
    statement_descriptor: Option<Option<String>>,
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
                builder: SourceTypeWechatBuilder {
                    prepay_id: Deserialize::default(),
                    qr_code_url: Deserialize::default(),
                    statement_descriptor: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "prepay_id" => Deserialize::begin(&mut self.builder.prepay_id),
                "qr_code_url" => Deserialize::begin(&mut self.builder.qr_code_url),
                "statement_descriptor" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(prepay_id), Some(qr_code_url), Some(statement_descriptor)) = (
                self.builder.prepay_id.take(),
                self.builder.qr_code_url.take(),
                self.builder.statement_descriptor.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(SourceTypeWechat { prepay_id, qr_code_url, statement_descriptor });
            Ok(())
        }
    }
};
