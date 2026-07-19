#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutSessionWalletOptions {
    pub link: Option<stripe_shared::CheckoutLinkWalletOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutSessionWalletOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CheckoutSessionWalletOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CheckoutSessionWalletOptionsBuilder {
    link: Option<Option<stripe_shared::CheckoutLinkWalletOptions>>,
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

    impl Deserialize for CheckoutSessionWalletOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutSessionWalletOptions>,
        builder: CheckoutSessionWalletOptionsBuilder,
    }

    impl Visitor for Place<CheckoutSessionWalletOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CheckoutSessionWalletOptionsBuilder { link: Deserialize::default() },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "link" => Deserialize::begin(&mut self.builder.link),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(link),) = (self.builder.link.take(),) else {
                return Ok(());
            };
            *self.out = Some(CheckoutSessionWalletOptions { link });
            Ok(())
        }
    }
};
