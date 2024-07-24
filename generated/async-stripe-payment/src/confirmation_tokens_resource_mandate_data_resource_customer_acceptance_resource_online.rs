/// This hash contains details about the online acceptance.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline {
    /// The IP address from which the Mandate was accepted by the customer.
    pub ip_address: Option<String>,
    /// The user agent of the browser from which the Mandate was accepted by the customer.
    pub user_agent: Option<String>,
}
#[doc(hidden)]
pub struct ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnlineBuilder {
    ip_address: Option<Option<String>>,
    user_agent: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline,
        >,
        builder:
            ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnlineBuilder,
    }

    impl Visitor
        for Place<ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnlineBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnlineBuilder
    {
        type Out = ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ip_address" => Deserialize::begin(&mut self.ip_address),
                "user_agent" => Deserialize::begin(&mut self.user_agent),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { ip_address: Deserialize::default(), user_agent: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                ip_address: self.ip_address.take()?,
                user_agent: self.user_agent.take()?,
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

    impl ObjectDeser for ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline {
        type Builder =
            ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnlineBuilder;
    }

    impl FromValueOpt
        for ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnlineBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "ip_address" => b.ip_address = Some(FromValueOpt::from_value(v)?),
                    "user_agent" => b.user_agent = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
