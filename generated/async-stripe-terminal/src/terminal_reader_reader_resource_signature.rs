/// Information about a signature being collected using a reader
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceSignature {
    /// The File ID of a collected signature image
    pub value: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceSignature").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceSignatureBuilder {
    value: Option<Option<String>>,
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

    impl Deserialize for TerminalReaderReaderResourceSignature {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceSignature>,
        builder: TerminalReaderReaderResourceSignatureBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceSignature> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceSignatureBuilder {
                    value: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "value" => Deserialize::begin(&mut self.builder.value),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(value),) = (self.builder.value.take(),) else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceSignature { value });
            Ok(())
        }
    }
};
