use serde_derive::{Deserialize, Serialize};

use crate::ids::BalanceTransactionSourceId;
use crate::params::Object;
use crate::resources::BalanceTransactionSourceUnion;

impl Object for BalanceTransactionSourceUnion {
    type Id = BalanceTransactionSourceId;
    fn id(&self) -> Self::Id {
        use BalanceTransactionSourceId as Id;
        use BalanceTransactionSourceUnion as Source;

        match self {
            Source::ApplicationFee(x) => Id::ApplicationFee(x.id()),
            Source::ApplicationFeeRefund(x) => Id::ApplicationFeeRefund(x.id()),
            Source::Charge(x) => Id::Charge(x.id()),
            Source::ConnectCollectionTransfer(_) => Id::None,
            Source::Dispute(x) => Id::Dispute(x.id()),
            Source::IssuingAuthorization(x) => Id::IssuingAuthorization(x.id()),
            Source::IssuingDispute(x) => Id::IssuingDispute(x.id()),
            Source::IssuingTransaction(x) => Id::IssuingTransaction(x.id()),
            Source::PlatformTaxFee(_) => Id::None,
            Source::Payout(x) => Id::Payout(x.id()),
            Source::Refund(x) => Id::Refund(x.id()),
            Source::ReserveTransaction(_) => Id::None,
            Source::TaxDeductedAtSource(_) => Id::None,
            Source::Topup(x) => Id::Topup(x.id()),
            Source::Transfer(x) => Id::Transfer(x.id()),
            Source::TransferReversal(x) => Id::TransferReversal(x.id()),
        }
    }
    fn object(&self) -> &'static str {
        use BalanceTransactionSourceUnion as Source;

        match self {
            Source::ApplicationFee(x) => x.object(),
            Source::ApplicationFeeRefund(x) => x.object(),
            Source::Charge(x) => x.object(),
            Source::ConnectCollectionTransfer(x) => x.object(),
            Source::Dispute(x) => x.object(),
            Source::IssuingAuthorization(x) => x.object(),
            Source::IssuingDispute(x) => x.object(),
            Source::IssuingTransaction(x) => x.object(),
            Source::PlatformTaxFee(x) => x.object(),
            Source::Payout(x) => x.object(),
            Source::Refund(x) => x.object(),
            Source::ReserveTransaction(x) => x.object(),
            Source::TaxDeductedAtSource(x) => x.object(),
            Source::Topup(x) => x.object(),
            Source::Transfer(x) => x.object(),
            Source::TransferReversal(x) => x.object(),
        }
    }
}

/// An enum representing the possible values of an `BalanceTransaction`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BalanceTransactionStatus {
    Available,
    Pending,
}

impl BalanceTransactionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            BalanceTransactionStatus::Available => "available",
            BalanceTransactionStatus::Pending => "pending",
        }
    }
}

impl AsRef<str> for BalanceTransactionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BalanceTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl std::default::Default for BalanceTransactionStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// An enum representing the possible values of an `Fee`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FeeType {
    ApplicationFee,
    StripeFee,
    Tax,
}

impl FeeType {
    pub fn as_str(self) -> &'static str {
        match self {
            FeeType::ApplicationFee => "application_fee",
            FeeType::StripeFee => "stripe_fee",
            FeeType::Tax => "tax",
        }
    }
}

impl AsRef<str> for FeeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FeeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl std::default::Default for FeeType {
    fn default() -> Self {
        FeeType::ApplicationFee
    }
}
