#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsAfterCompletionHostedConfirmation {
    /// A custom message to display to the customer after the flow is completed.
    pub custom_message: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalFlowsAfterCompletionHostedConfirmation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalFlowsAfterCompletionHostedConfirmation").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalFlowsAfterCompletionHostedConfirmationBuilder {
    custom_message: Option<Option<String>>,
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

    impl Deserialize for PortalFlowsAfterCompletionHostedConfirmation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsAfterCompletionHostedConfirmation>,
        builder: PortalFlowsAfterCompletionHostedConfirmationBuilder,
    }

    impl Visitor for Place<PortalFlowsAfterCompletionHostedConfirmation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalFlowsAfterCompletionHostedConfirmationBuilder {
                    custom_message: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "custom_message" => Deserialize::begin(&mut self.builder.custom_message),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(custom_message),) = (self.builder.custom_message.take(),) else {
                return Ok(());
            };
            *self.out = Some(PortalFlowsAfterCompletionHostedConfirmation { custom_message });
            Ok(())
        }
    }
};
