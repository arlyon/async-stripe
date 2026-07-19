/// This hash contains details about the online acceptance.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline {
    /// The IP address from which the Mandate was accepted by the customer.
    pub ip_address: Option<String>,
    /// The user agent of the browser from which the Mandate was accepted by the customer.
    pub user_agent: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnlineBuilder {
    ip_address: Option<Option<String>>,
    user_agent: Option<Option<String>>,
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
            builder: ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnlineBuilder { ip_address: Deserialize::default(),
user_agent: Deserialize::default(),
 },
        }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ip_address" => Deserialize::begin(&mut self.builder.ip_address),
                "user_agent" => Deserialize::begin(&mut self.builder.user_agent),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(ip_address), Some(user_agent)) =
                (self.builder.ip_address.take(), self.builder.user_agent.take())
            else {
                return Ok(());
            };
            *self.out = Some(
                ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline {
                    ip_address,
                    user_agent,
                },
            );
            Ok(())
        }
    }
};
