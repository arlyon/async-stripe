/// Restrictions that a Connect Platform has placed on this FinancialAccount.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourcePlatformRestrictions {
    /// Restricts all inbound money movement.
    pub inbound_flows: Option<TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows>,
    /// Restricts all outbound money movement.
    pub outbound_flows: Option<TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows>,
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourcePlatformRestrictionsBuilder {
    inbound_flows:
        Option<Option<TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows>>,
    outbound_flows:
        Option<Option<TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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
            Self { inbound_flows: Deserialize::default(), outbound_flows: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(inbound_flows), Some(outbound_flows)) =
                (self.inbound_flows, self.outbound_flows)
            else {
                return None;
            };
            Some(Self::Out { inbound_flows, outbound_flows })
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
}
impl TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows::*;
        match self {
            Restricted => "restricted",
            Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows"))
    }
}
/// Restricts all outbound money movement.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    Restricted,
    Unrestricted,
}
impl TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows::*;
        match self {
            Restricted => "restricted",
            Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows"))
    }
}
