/// This hash contains details about the customer acceptance of the Mandate.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConfirmationTokensResourceMandateDataResourceCustomerAcceptance {
    /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
pub online: Option<stripe_payment::ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline>,
    /// The type of customer acceptance information included with the Mandate.
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
pub type_: String,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmationTokensResourceMandateDataResourceCustomerAcceptance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmationTokensResourceMandateDataResourceCustomerAcceptance")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceBuilder {
    online: Option<Option<stripe_payment::ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline>>,
type_: Option<String>,

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

    impl Deserialize for ConfirmationTokensResourceMandateDataResourceCustomerAcceptance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConfirmationTokensResourceMandateDataResourceCustomerAcceptance>,
        builder: ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceBuilder,
    }

    impl Visitor for Place<ConfirmationTokensResourceMandateDataResourceCustomerAcceptance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceBuilder {
                    online: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "online" => Deserialize::begin(&mut self.builder.online),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(online), Some(type_)) =
                (self.builder.online.take(), self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out = Some(ConfirmationTokensResourceMandateDataResourceCustomerAcceptance {
                online,
                type_,
            });
            Ok(())
        }
    }
};
