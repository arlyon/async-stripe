// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::IssuingSettlementId;
use crate::params::{Metadata, Object, Timestamp};
use crate::resources::Currency;

/// The resource representing a Stripe "IssuingSettlement".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingSettlement {
    /// Unique identifier for the object.
    pub id: IssuingSettlementId,

    /// The Bank Identification Number reflecting this settlement record.
    pub bin: String,

    /// The date that the transactions are cleared and posted to user's accounts.
    pub clearing_date: Timestamp,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The total interchange received as reimbursement for the transactions.
    pub interchange_fees: i64,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The total net amount required to settle with the network.
    pub net_total: i64,

    /// The card network for this settlement report.
    ///
    /// One of ["visa"].
    pub network: IssuingSettlementNetwork,

    /// The total amount of fees owed to the network.
    pub network_fees: i64,

    /// The Settlement Identification Number assigned by the network.
    pub network_settlement_identifier: String,

    /// One of `international` or `uk_national_net`.
    pub settlement_service: String,

    /// The total number of transactions reflected in this settlement.
    pub transaction_count: u64,

    /// The total transaction amount reflected in this settlement.
    pub transaction_volume: i64,
}

impl Object for IssuingSettlement {
    type Id = IssuingSettlementId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.settlement"
    }
}

/// An enum representing the possible values of an `IssuingSettlement`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingSettlementNetwork {
    Visa,
}

impl IssuingSettlementNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingSettlementNetwork::Visa => "visa",
        }
    }
}

impl AsRef<str> for IssuingSettlementNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingSettlementNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingSettlementNetwork {
    fn default() -> Self {
        Self::Visa
    }
}
