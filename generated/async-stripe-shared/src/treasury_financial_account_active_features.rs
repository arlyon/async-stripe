#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryFinancialAccountActiveFeatures {
    CardIssuing,
    DepositInsurance,
    FinancialAddressesAba,
    FinancialAddressesAbaForwarding,
    InboundTransfersAch,
    IntraStripeFlows,
    OutboundPaymentsAch,
    OutboundPaymentsUsDomesticWire,
    OutboundTransfersAch,
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryFinancialAccountActiveFeatures {
    pub fn as_str(&self) -> &str {
        use TreasuryFinancialAccountActiveFeatures::*;
        match self {
            CardIssuing => "card_issuing",
            DepositInsurance => "deposit_insurance",
            FinancialAddressesAba => "financial_addresses.aba",
            FinancialAddressesAbaForwarding => "financial_addresses.aba.forwarding",
            InboundTransfersAch => "inbound_transfers.ach",
            IntraStripeFlows => "intra_stripe_flows",
            OutboundPaymentsAch => "outbound_payments.ach",
            OutboundPaymentsUsDomesticWire => "outbound_payments.us_domestic_wire",
            OutboundTransfersAch => "outbound_transfers.ach",
            OutboundTransfersUsDomesticWire => "outbound_transfers.us_domestic_wire",
            RemoteDepositCapture => "remote_deposit_capture",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountActiveFeatures {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountActiveFeatures::*;
        match s {
            "card_issuing" => Ok(CardIssuing),
            "deposit_insurance" => Ok(DepositInsurance),
            "financial_addresses.aba" => Ok(FinancialAddressesAba),
            "financial_addresses.aba.forwarding" => Ok(FinancialAddressesAbaForwarding),
            "inbound_transfers.ach" => Ok(InboundTransfersAch),
            "intra_stripe_flows" => Ok(IntraStripeFlows),
            "outbound_payments.ach" => Ok(OutboundPaymentsAch),
            "outbound_payments.us_domestic_wire" => Ok(OutboundPaymentsUsDomesticWire),
            "outbound_transfers.ach" => Ok(OutboundTransfersAch),
            "outbound_transfers.us_domestic_wire" => Ok(OutboundTransfersUsDomesticWire),
            "remote_deposit_capture" => Ok(RemoteDepositCapture),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryFinancialAccountActiveFeatures"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountActiveFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryFinancialAccountActiveFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryFinancialAccountActiveFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryFinancialAccountActiveFeatures)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountActiveFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryFinancialAccountActiveFeatures {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TreasuryFinancialAccountActiveFeatures> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryFinancialAccountActiveFeatures::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TreasuryFinancialAccountActiveFeatures);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountActiveFeatures {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
