/// Information about a email being collected using a reader
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceEmail {
    /// The collected email address
    pub value: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceEmail").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceEmailBuilder {
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

    impl Deserialize for TerminalReaderReaderResourceEmail {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceEmail>,
        builder: TerminalReaderReaderResourceEmailBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceEmail> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceEmailBuilder { value: Deserialize::default() },
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
            *self.out = Some(TerminalReaderReaderResourceEmail { value });
            Ok(())
        }
    }
};
