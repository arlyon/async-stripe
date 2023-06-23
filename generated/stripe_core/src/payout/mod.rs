/// A `Payout` object is created when you receive funds from Stripe, or when you
/// initiate a payout to either a bank account or debit card of a [connected
/// Stripe account](/docs/connect/bank-debit-card-payouts).
///
/// You can retrieve individual payouts, as well as list all payouts.
/// Payouts are made on [varying schedules](/docs/connect/manage-payout-schedule), depending on your country and industry.  Related guide: [Receiving Payouts](https://stripe.com/docs/payouts).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Payout {
    /// Amount (in %s) to be transferred to your bank account or debit card.
    pub amount: i64,
    /// Date the payout is expected to arrive in the bank.
    ///
    /// This factors in delays like weekends or bank holidays.
    pub arrival_date: stripe_types::Timestamp,
    /// Returns `true` if the payout was created by an [automated payout schedule](https://stripe.com/docs/payouts#payout-schedule), and `false` if it was [requested manually](https://stripe.com/docs/payouts#manual-payouts).
    pub automatic: bool,
    /// ID of the balance transaction that describes the impact of this payout on your account balance.
    pub balance_transaction:
        Option<stripe_types::Expandable<stripe_core::balance_transaction::BalanceTransaction>>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// ID of the bank account or card the payout was sent to.
    pub destination:
        Option<stripe_types::Expandable<stripe_core::external_account::ExternalAccount>>,
    /// If the payout failed or was canceled, this will be the ID of the balance transaction that reversed the initial balance transaction, and puts the funds from the failed payout back in your balance.
    pub failure_balance_transaction:
        Option<stripe_types::Expandable<stripe_core::balance_transaction::BalanceTransaction>>,
    /// Error code explaining reason for payout failure if available.
    ///
    /// See [Types of payout failures](https://stripe.com/docs/api#payout_failures) for a list of failure codes.
    pub failure_code: Option<String>,
    /// Message to user further explaining reason for payout failure if available.
    pub failure_message: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_core::payout::PayoutId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<stripe_types::Metadata>,
    /// The method used to send this payout, which can be `standard` or `instant`.
    ///
    /// `instant` is only supported for payouts to debit cards.
    /// (See [Instant payouts for marketplaces](https://stripe.com/blog/instant-payouts-for-marketplaces) for more information.).
    pub method: String,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: PayoutObject,
    /// If the payout reverses another, this is the ID of the original payout.
    pub original_payout: Option<stripe_types::Expandable<stripe_core::payout::Payout>>,
    /// If the payout was reversed, this is the ID of the payout that reverses this payout.
    pub reversed_by: Option<stripe_types::Expandable<stripe_core::payout::Payout>>,
    /// The source balance this payout came from.
    ///
    /// One of `card`, `fpx`, or `bank_account`.
    pub source_type: String,
    /// Extra information about a payout to be displayed on the user's bank statement.
    pub statement_descriptor: Option<String>,
    /// Current status of the payout: `paid`, `pending`, `in_transit`, `canceled` or `failed`.
    ///
    /// A payout is `pending` until it is submitted to the bank, when it becomes `in_transit`.
    /// The status then changes to `paid` if the transaction goes through, or to `failed` or `canceled` (within 5 business days).
    /// Some failed payouts may initially show as `paid` but then change to `failed`.
    pub status: String,
    /// Can be `bank_account` or `card`.
    #[serde(rename = "type")]
    pub type_: PayoutType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Payout {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum PayoutObject {
    Payout,
}

impl PayoutObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Payout => "payout",
        }
    }
}

impl AsRef<str> for PayoutObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PayoutObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Can be `bank_account` or `card`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum PayoutType {
    BankAccount,
    Card,
}

impl PayoutType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankAccount => "bank_account",
            Self::Card => "card",
        }
    }
}

impl AsRef<str> for PayoutType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PayoutType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for Payout {
    type Id = stripe_core::payout::PayoutId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(PayoutId, "po_");
pub mod requests;
