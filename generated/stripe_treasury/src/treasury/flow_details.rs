#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FlowDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<stripe_treasury::treasury::credit_reversal::CreditReversal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_reversal: Option<stripe_treasury::treasury::debit_reversal::DebitReversal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfer: Option<stripe_treasury::treasury::inbound_transfer::InboundTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_authorization: Option<stripe_types::issuing::authorization::Authorization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payment: Option<stripe_treasury::treasury::outbound_payment::OutboundPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfer: Option<stripe_treasury::treasury::outbound_transfer::OutboundTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_credit: Option<stripe_treasury::treasury::received_credit::ReceivedCredit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_debit: Option<stripe_treasury::treasury::received_debit::ReceivedDebit>,
    /// Type of the flow that created the Transaction.
    ///
    /// Set to the same value as `flow_type`.
    #[serde(rename = "type")]
    pub type_: FlowDetailsType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FlowDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Type of the flow that created the Transaction.
///
/// Set to the same value as `flow_type`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FlowDetailsType {
    CreditReversal,
    DebitReversal,
    InboundTransfer,
    IssuingAuthorization,
    Other,
    OutboundPayment,
    OutboundTransfer,
    ReceivedCredit,
    ReceivedDebit,
}

impl FlowDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreditReversal => "credit_reversal",
            Self::DebitReversal => "debit_reversal",
            Self::InboundTransfer => "inbound_transfer",
            Self::IssuingAuthorization => "issuing_authorization",
            Self::Other => "other",
            Self::OutboundPayment => "outbound_payment",
            Self::OutboundTransfer => "outbound_transfer",
            Self::ReceivedCredit => "received_credit",
            Self::ReceivedDebit => "received_debit",
        }
    }
}

impl std::str::FromStr for FlowDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "credit_reversal" => Ok(Self::CreditReversal),
            "debit_reversal" => Ok(Self::DebitReversal),
            "inbound_transfer" => Ok(Self::InboundTransfer),
            "issuing_authorization" => Ok(Self::IssuingAuthorization),
            "other" => Ok(Self::Other),
            "outbound_payment" => Ok(Self::OutboundPayment),
            "outbound_transfer" => Ok(Self::OutboundTransfer),
            "received_credit" => Ok(Self::ReceivedCredit),
            "received_debit" => Ok(Self::ReceivedDebit),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for FlowDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FlowDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for FlowDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FlowDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for FlowDetailsType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FlowDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<FlowDetailsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FlowDetailsType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
