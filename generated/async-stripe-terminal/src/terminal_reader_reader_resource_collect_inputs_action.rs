/// Represents a reader action to collect customer inputs
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceCollectInputsAction {
    /// List of inputs to be collected.
    pub inputs: Vec<stripe_terminal::TerminalReaderReaderResourceInput>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceCollectInputsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceCollectInputsAction").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceCollectInputsActionBuilder {
    inputs: Option<Vec<stripe_terminal::TerminalReaderReaderResourceInput>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
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

    impl Deserialize for TerminalReaderReaderResourceCollectInputsAction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceCollectInputsAction>,
        builder: TerminalReaderReaderResourceCollectInputsActionBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceCollectInputsAction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceCollectInputsActionBuilder {
                    inputs: Deserialize::default(),
                    metadata: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "inputs" => Deserialize::begin(&mut self.builder.inputs),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(inputs), Some(metadata)) =
                (self.builder.inputs.take(), self.builder.metadata.take())
            else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceCollectInputsAction { inputs, metadata });
            Ok(())
        }
    }
};
