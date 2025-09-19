#[derive(Clone, Debug)]
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
                builder: PaymentMethodCardWalletMasterpassBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodCardWalletMasterpassBuilder {
        type Out = PaymentMethodCardWalletMasterpass;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_address" => Deserialize::begin(&mut self.billing_address),
                "email" => Deserialize::begin(&mut self.email),
                "name" => Deserialize::begin(&mut self.name),
                "shipping_address" => Deserialize::begin(&mut self.shipping_address),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                billing_address: Deserialize::default(),
                email: Deserialize::default(),
                name: Deserialize::default(),
                shipping_address: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(billing_address), Some(email), Some(name), Some(shipping_address)) = (
                self.billing_address.take(),
                self.email.take(),
                self.name.take(),
                self.shipping_address.take(),
            ) else {
                return None;
            };
            Some(Self::Out { billing_address, email, name, shipping_address })
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

    impl ObjectDeser for PaymentMethodCardWalletMasterpass {
        type Builder = PaymentMethodCardWalletMasterpassBuilder;
    }

    impl FromValueOpt for PaymentMethodCardWalletMasterpass {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodCardWalletMasterpassBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "billing_address" => b.billing_address = FromValueOpt::from_value(v),
                    "email" => b.email = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "shipping_address" => b.shipping_address = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
