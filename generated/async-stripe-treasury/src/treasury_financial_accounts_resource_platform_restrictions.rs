/// Restrictions that a Connect Platform has placed on this FinancialAccount.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourcePlatformRestrictions {
    /// Restricts all inbound money movement.
    pub inbound_flows:
        Option<stripe_shared::CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows>,
    /// Restricts all outbound money movement.
    pub outbound_flows:
        Option<stripe_shared::CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryFinancialAccountsResourcePlatformRestrictions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryFinancialAccountsResourcePlatformRestrictions")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourcePlatformRestrictionsBuilder {
    inbound_flows: Option<
        Option<stripe_shared::CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows>,
    >,
    outbound_flows: Option<
        Option<stripe_shared::CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows>,
    >,
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

    impl Deserialize for TreasuryFinancialAccountsResourcePlatformRestrictions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourcePlatformRestrictions>,
        builder: TreasuryFinancialAccountsResourcePlatformRestrictionsBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourcePlatformRestrictions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    TreasuryFinancialAccountsResourcePlatformRestrictionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourcePlatformRestrictionsBuilder {
        type Out = TreasuryFinancialAccountsResourcePlatformRestrictions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "inbound_flows" => Deserialize::begin(&mut self.inbound_flows),
                "outbound_flows" => Deserialize::begin(&mut self.outbound_flows),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { inbound_flows: Some(None), outbound_flows: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(inbound_flows), Some(outbound_flows)) =
                (self.inbound_flows.take(), self.outbound_flows.take())
            else {
                return None;
            };
            Some(Self::Out { inbound_flows, outbound_flows })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourcePlatformRestrictions {
        type Builder = TreasuryFinancialAccountsResourcePlatformRestrictionsBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountsResourcePlatformRestrictions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TreasuryFinancialAccountsResourcePlatformRestrictionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "inbound_flows" => b.inbound_flows = FromValueOpt::from_value(v),
                    "outbound_flows" => b.outbound_flows = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Restricts all inbound money movement.
pub type TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows =
    stripe_shared::CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows;
/// Restricts all outbound money movement.
pub type TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows =
    stripe_shared::CreateTreasuryFinancialAccountPlatformRestrictionsInboundFlows;
