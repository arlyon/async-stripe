#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedFinancialAccountFeatures {
    /// Whether Stripe user authentication is disabled.
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    pub disable_stripe_user_authentication: bool,
    /// Whether external account collection is enabled.
    /// This feature can only be `false` for accounts where you’re responsible for collecting updated information when requirements are due or change, like Custom accounts.
    /// The default value for this feature is `true`.
    pub external_account_collection: bool,
    /// Whether to allow sending money.
    pub send_money: bool,
    /// Whether to allow transferring balance.
    pub transfer_balance: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConnectEmbeddedFinancialAccountFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConnectEmbeddedFinancialAccountFeatures").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConnectEmbeddedFinancialAccountFeaturesBuilder {
    disable_stripe_user_authentication: Option<bool>,
    external_account_collection: Option<bool>,
    send_money: Option<bool>,
    transfer_balance: Option<bool>,
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

    impl Deserialize for ConnectEmbeddedFinancialAccountFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedFinancialAccountFeatures>,
        builder: ConnectEmbeddedFinancialAccountFeaturesBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedFinancialAccountFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedFinancialAccountFeaturesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConnectEmbeddedFinancialAccountFeaturesBuilder {
        type Out = ConnectEmbeddedFinancialAccountFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "disable_stripe_user_authentication" => {
                    Deserialize::begin(&mut self.disable_stripe_user_authentication)
                }
                "external_account_collection" => {
                    Deserialize::begin(&mut self.external_account_collection)
                }
                "send_money" => Deserialize::begin(&mut self.send_money),
                "transfer_balance" => Deserialize::begin(&mut self.transfer_balance),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                disable_stripe_user_authentication: None,
                external_account_collection: None,
                send_money: None,
                transfer_balance: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    disable_stripe_user_authentication: Some(disable_stripe_user_authentication),
                    external_account_collection: Some(external_account_collection),
                    send_money: Some(send_money),
                    transfer_balance: Some(transfer_balance),
                } => Some(Self::Out {
                    disable_stripe_user_authentication,
                    external_account_collection,
                    send_money,
                    transfer_balance,
                }),
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

    impl ObjectDeser for ConnectEmbeddedFinancialAccountFeatures {
        type Builder = ConnectEmbeddedFinancialAccountFeaturesBuilder;
    }

    impl FromValueOpt for ConnectEmbeddedFinancialAccountFeatures {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
