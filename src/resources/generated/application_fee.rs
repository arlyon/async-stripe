// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{ApplicationFeeId, ChargeId};
use crate::params::{Expand, Expandable, List, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{Account, Application, ApplicationFeeRefund, BalanceTransaction, Charge, Currency};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "PlatformFee".
///
/// For more details see <https://stripe.com/docs/api/application_fees/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ApplicationFee {
    /// Unique identifier for the object.
    pub id: ApplicationFeeId,

    /// ID of the Stripe account this fee was taken from.
    pub account: Expandable<Account>,

    /// Amount earned, in cents (or local equivalent).
    pub amount: i64,

    /// Amount in cents (or local equivalent) refunded (can be less than the amount attribute on the fee if a partial refund was issued).
    pub amount_refunded: i64,

    /// ID of the Connect application that earned the fee.
    pub application: Expandable<Application>,

    /// Balance transaction that describes the impact of this collected application fee on your account balance (not including refunds).
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// ID of the charge that the application fee was taken from.
    pub charge: Expandable<Charge>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// Polymorphic source of the application fee.
    ///
    /// Includes the ID of the object the application fee was created from.
    pub fee_source: Option<PlatformEarningFeeSource>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// ID of the corresponding charge on the platform account, if this fee was the result of a charge using the `destination` parameter.
    pub originating_transaction: Option<Expandable<Charge>>,

    /// Whether the fee has been fully refunded.
    ///
    /// If the fee is only partially refunded, this attribute will still be false.
    pub refunded: bool,

    /// A list of refunds that have been applied to the fee.
    pub refunds: List<ApplicationFeeRefund>,
}

impl ApplicationFee {

    /// Returns a list of application fees you’ve previously collected.
    ///
    /// The application fees are returned in sorted order, with the most recent fees appearing first.
pub fn list(client: &Client, params: &ListApplicationFees<'_>) -> Response<List<ApplicationFee>> {
   client.get_query("/application_fees", params)
}


    /// Retrieves the details of an application fee that your account has collected.
    ///
    /// The same information is returned when refunding the application fee.
    pub fn retrieve(client: &Client, id: &ApplicationFeeId, expand: &[&str]) -> Response<ApplicationFee> {
        client.get_query(&format!("/application_fees/{}", id), Expand { expand })
    }
}

impl Object for ApplicationFee {
    type Id = ApplicationFeeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "application_fee"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlatformEarningFeeSource {

    /// Charge ID that created this application fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<String>,

    /// Payout ID that created this application fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<String>,

    /// Type of object that created the application fee, either `charge` or `payout`.
    #[serde(rename = "type")]
    pub type_: PlatformEarningFeeSourceType,
}

/// The parameters for `ApplicationFee::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListApplicationFees<'a> {

    /// Only return application fees for the charge specified by this charge ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<ChargeId>,

    /// Only return applications fees that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<ApplicationFeeId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<ApplicationFeeId>,
}

impl<'a> ListApplicationFees<'a> {
    pub fn new() -> Self {
        ListApplicationFees {
            charge: Default::default(),
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListApplicationFees<'_> {
    type O = ApplicationFee;
    fn set_last(&mut self, item: Self::O) {
                self.starting_after = Some(item.id());
            }}
/// An enum representing the possible values of an `PlatformEarningFeeSource`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlatformEarningFeeSourceType {
    Charge,
    Payout,
}

impl PlatformEarningFeeSourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            PlatformEarningFeeSourceType::Charge => "charge",
            PlatformEarningFeeSourceType::Payout => "payout",
        }
    }
}

impl AsRef<str> for PlatformEarningFeeSourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlatformEarningFeeSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PlatformEarningFeeSourceType {
    fn default() -> Self {
        Self::Charge
    }
}
