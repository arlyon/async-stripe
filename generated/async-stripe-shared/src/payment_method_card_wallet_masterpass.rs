#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodCardWalletMasterpass {
    /// Owner's verified billing address.
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub billing_address: Option<stripe_shared::Address>,
    /// Owner's verified email.
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub email: Option<String>,
    /// Owner's verified full name.
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub name: Option<String>,
    /// Owner's verified shipping address.
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub shipping_address: Option<stripe_shared::Address>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodCardWalletMasterpass {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodCardWalletMasterpass").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodCardWalletMasterpassBuilder {
    billing_address: Option<Option<stripe_shared::Address>>,
    email: Option<Option<String>>,
    name: Option<Option<String>>,
    shipping_address: Option<Option<stripe_shared::Address>>,
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

    impl Deserialize for PaymentMethodCardWalletMasterpass {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodCardWalletMasterpass>,
        builder: PaymentMethodCardWalletMasterpassBuilder,
    }

    impl Visitor for Place<PaymentMethodCardWalletMasterpass> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodCardWalletMasterpassBuilder {
                    billing_address: Deserialize::default(),
                    email: Deserialize::default(),
                    name: Deserialize::default(),
                    shipping_address: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_address" => Deserialize::begin(&mut self.builder.billing_address),
                "email" => Deserialize::begin(&mut self.builder.email),
                "name" => Deserialize::begin(&mut self.builder.name),
                "shipping_address" => Deserialize::begin(&mut self.builder.shipping_address),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(billing_address), Some(email), Some(name), Some(shipping_address)) = (
                self.builder.billing_address.take(),
                self.builder.email.take(),
                self.builder.name.take(),
                self.builder.shipping_address.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodCardWalletMasterpass {
                billing_address,
                email,
                name,
                shipping_address,
            });
            Ok(())
        }
    }
};
