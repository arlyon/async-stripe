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
    online: Option<stripe_payment::ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline>,
type_: Option<String>,

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
            builder: ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceBuilder {
        type Out = ConfirmationTokensResourceMandateDataResourceCustomerAcceptance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "online" => stripe_types::miniserde_helpers::begin_nullable(&mut self.online),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { online: None, type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { online, type_: Some(type_) } => Some(Self::Out { online, type_ }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for ConfirmationTokensResourceMandateDataResourceCustomerAcceptance {
        type Builder = ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceBuilder;
    }

    impl FromValueOpt for ConfirmationTokensResourceMandateDataResourceCustomerAcceptance {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
