/// This hash contains details about the customer acceptance of the Mandate.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConfirmationTokensResourceMandateDataResourceCustomerAcceptance {
    /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
pub online: Option<stripe_payment::ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline>,
    /// The type of customer acceptance information included with the Mandate.
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
pub type_: String,

}
#[doc(hidden)]
pub struct ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceBuilder {
    online: Option<Option<stripe_payment::ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline>>,
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
    use miniserde::{make_place, Deserialize, Result};
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
                "online" => Deserialize::begin(&mut self.online),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { online: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(online), Some(type_)) = (self.online.take(), self.type_.take()) else {
                return None;
            };
            Some(Self::Out { online, type_ })
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

    impl ObjectDeser for ConfirmationTokensResourceMandateDataResourceCustomerAcceptance {
        type Builder = ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceBuilder;
    }

    impl FromValueOpt for ConfirmationTokensResourceMandateDataResourceCustomerAcceptance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "online" => b.online = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
