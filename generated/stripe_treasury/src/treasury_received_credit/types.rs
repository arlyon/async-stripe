/// ReceivedCredits represent funds sent to a [FinancialAccount](https://stripe.com/docs/api#financial_accounts) (for example, via ACH or wire).
/// These money movements are not initiated from the FinancialAccount.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryReceivedCredit {
    /// Amount (in cents) transferred.
pub amount: i64,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
pub created: stripe_types::Timestamp,
        /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
pub description: String,
        /// Reason for the failure.
    /// A ReceivedCredit might fail because the receiving FinancialAccount is closed or frozen.
pub failure_code: Option<TreasuryReceivedCreditFailureCode>,
    /// The FinancialAccount that received the funds.
pub financial_account: Option<String>,
        /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
pub id: stripe_treasury::TreasuryReceivedCreditId,
pub initiating_payment_method_details: stripe_treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
pub linked_flows: stripe_treasury::TreasuryReceivedCreditsResourceLinkedFlows,
        /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,
    /// The rails used to send the funds.
pub network: TreasuryReceivedCreditNetwork,
    /// Details describing when a ReceivedCredit may be reversed.
pub reversal_details: Option<stripe_treasury::TreasuryReceivedCreditsResourceReversalDetails>,
        /// Status of the ReceivedCredit.
    /// ReceivedCredits are created either `succeeded` (approved) or `failed` (declined).
    /// If a ReceivedCredit is declined, the failure reason can be found in the `failure_code` field.
pub status: stripe_treasury::TreasuryReceivedCreditStatus,
    /// The Transaction associated with this object.
pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,

}
#[doc(hidden)]
pub struct TreasuryReceivedCreditBuilder {
    amount: Option<i64>,
created: Option<stripe_types::Timestamp>,
currency: Option<stripe_types::Currency>,
description: Option<String>,
failure_code: Option<Option<TreasuryReceivedCreditFailureCode>>,
financial_account: Option<Option<String>>,
hosted_regulatory_receipt_url: Option<Option<String>>,
id: Option<stripe_treasury::TreasuryReceivedCreditId>,
initiating_payment_method_details: Option<stripe_treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails>,
linked_flows: Option<stripe_treasury::TreasuryReceivedCreditsResourceLinkedFlows>,
livemode: Option<bool>,
network: Option<TreasuryReceivedCreditNetwork>,
reversal_details: Option<Option<stripe_treasury::TreasuryReceivedCreditsResourceReversalDetails>>,
status: Option<stripe_treasury::TreasuryReceivedCreditStatus>,
transaction: Option<Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>>,

}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryReceivedCredit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedCredit>,
        builder: TreasuryReceivedCreditBuilder,
    }

    impl Visitor for Place<TreasuryReceivedCredit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryReceivedCreditBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryReceivedCreditBuilder {
        type Out = TreasuryReceivedCredit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "description" => Deserialize::begin(&mut self.description),
                "failure_code" => Deserialize::begin(&mut self.failure_code),
                "financial_account" => Deserialize::begin(&mut self.financial_account),
                "hosted_regulatory_receipt_url" => {
                    Deserialize::begin(&mut self.hosted_regulatory_receipt_url)
                }
                "id" => Deserialize::begin(&mut self.id),
                "initiating_payment_method_details" => {
                    Deserialize::begin(&mut self.initiating_payment_method_details)
                }
                "linked_flows" => Deserialize::begin(&mut self.linked_flows),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "network" => Deserialize::begin(&mut self.network),
                "reversal_details" => Deserialize::begin(&mut self.reversal_details),
                "status" => Deserialize::begin(&mut self.status),
                "transaction" => Deserialize::begin(&mut self.transaction),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                failure_code: Deserialize::default(),
                financial_account: Deserialize::default(),
                hosted_regulatory_receipt_url: Deserialize::default(),
                id: Deserialize::default(),
                initiating_payment_method_details: Deserialize::default(),
                linked_flows: Deserialize::default(),
                livemode: Deserialize::default(),
                network: Deserialize::default(),
                reversal_details: Deserialize::default(),
                status: Deserialize::default(),
                transaction: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                amount: self.amount?,
                created: self.created?,
                currency: self.currency?,
                description: self.description.take()?,
                failure_code: self.failure_code?,
                financial_account: self.financial_account.take()?,
                hosted_regulatory_receipt_url: self.hosted_regulatory_receipt_url.take()?,
                id: self.id.take()?,
                initiating_payment_method_details: self.initiating_payment_method_details.take()?,
                linked_flows: self.linked_flows.take()?,
                livemode: self.livemode?,
                network: self.network?,
                reversal_details: self.reversal_details?,
                status: self.status?,
                transaction: self.transaction.take()?,
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

    impl ObjectDeser for TreasuryReceivedCredit {
        type Builder = TreasuryReceivedCreditBuilder;
    }

    impl FromValueOpt for TreasuryReceivedCredit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryReceivedCreditBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "failure_code" => b.failure_code = Some(FromValueOpt::from_value(v)?),
                    "financial_account" => b.financial_account = Some(FromValueOpt::from_value(v)?),
                    "hosted_regulatory_receipt_url" => {
                        b.hosted_regulatory_receipt_url = Some(FromValueOpt::from_value(v)?)
                    }
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "initiating_payment_method_details" => {
                        b.initiating_payment_method_details = Some(FromValueOpt::from_value(v)?)
                    }
                    "linked_flows" => b.linked_flows = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "network" => b.network = Some(FromValueOpt::from_value(v)?),
                    "reversal_details" => b.reversal_details = Some(FromValueOpt::from_value(v)?),
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "transaction" => b.transaction = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryReceivedCredit {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TreasuryReceivedCredit", 16)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("failure_code", &self.failure_code)?;
        s.serialize_field("financial_account", &self.financial_account)?;
        s.serialize_field("hosted_regulatory_receipt_url", &self.hosted_regulatory_receipt_url)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field(
            "initiating_payment_method_details",
            &self.initiating_payment_method_details,
        )?;
        s.serialize_field("linked_flows", &self.linked_flows)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("network", &self.network)?;
        s.serialize_field("reversal_details", &self.reversal_details)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("transaction", &self.transaction)?;

        s.serialize_field("object", "treasury.received_credit")?;
        s.end()
    }
}
/// Reason for the failure.
/// A ReceivedCredit might fail because the receiving FinancialAccount is closed or frozen.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditFailureCode {
    AccountClosed,
    AccountFrozen,
    Other,
}
impl TreasuryReceivedCreditFailureCode {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditFailureCode::*;
        match self {
            AccountClosed => "account_closed",
            AccountFrozen => "account_frozen",
            Other => "other",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditFailureCode {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditFailureCode::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "account_frozen" => Ok(AccountFrozen),
            "other" => Ok(Other),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TreasuryReceivedCreditFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryReceivedCreditFailureCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryReceivedCreditFailureCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TreasuryReceivedCreditFailureCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(TreasuryReceivedCreditFailureCode::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TreasuryReceivedCreditFailureCode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditFailureCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TreasuryReceivedCreditFailureCode")
        })
    }
}
/// The rails used to send the funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditNetwork {
    Ach,
    Card,
    Stripe,
    UsDomesticWire,
}
impl TreasuryReceivedCreditNetwork {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditNetwork::*;
        match self {
            Ach => "ach",
            Card => "card",
            Stripe => "stripe",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditNetwork {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "card" => Ok(Card),
            "stripe" => Ok(Stripe),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TreasuryReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryReceivedCreditNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryReceivedCreditNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TreasuryReceivedCreditNetwork> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryReceivedCreditNetwork::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TreasuryReceivedCreditNetwork);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TreasuryReceivedCreditNetwork")
        })
    }
}
impl stripe_types::Object for TreasuryReceivedCredit {
    type Id = stripe_treasury::TreasuryReceivedCreditId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryReceivedCreditId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditStatus {
    Failed,
    Succeeded,
}
impl TreasuryReceivedCreditStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditStatus::*;
        match self {
            Failed => "failed",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditStatus::*;
        match s {
            "failed" => Ok(Failed),
            "succeeded" => Ok(Succeeded),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TreasuryReceivedCreditStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryReceivedCreditStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TreasuryReceivedCreditStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TreasuryReceivedCreditStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TreasuryReceivedCreditStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedCreditStatus"))
    }
}
