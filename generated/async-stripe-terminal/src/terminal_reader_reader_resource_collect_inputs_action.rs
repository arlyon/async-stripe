/// Represents a reader action to collect customer inputs
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceCollectInputsAction {
    /// List of inputs to be collected.
    pub inputs: Vec<stripe_terminal::TerminalReaderReaderResourceInput>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceCollectInputsActionBuilder {
    inputs: Option<Vec<stripe_terminal::TerminalReaderReaderResourceInput>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
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
                builder: TerminalReaderReaderResourceCollectInputsActionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceCollectInputsActionBuilder {
        type Out = TerminalReaderReaderResourceCollectInputsAction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "inputs" => Deserialize::begin(&mut self.inputs),
                "metadata" => Deserialize::begin(&mut self.metadata),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { inputs: Deserialize::default(), metadata: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(inputs), Some(metadata)) = (self.inputs.take(), self.metadata.take()) else {
                return None;
            };
            Some(Self::Out { inputs, metadata })
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

    impl ObjectDeser for TerminalReaderReaderResourceCollectInputsAction {
        type Builder = TerminalReaderReaderResourceCollectInputsActionBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceCollectInputsAction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalReaderReaderResourceCollectInputsActionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "inputs" => b.inputs = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
