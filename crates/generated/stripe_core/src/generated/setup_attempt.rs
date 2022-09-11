// ======================================
// This file was automatically generated.
// ======================================

use async_stripe_core::{
    ids::{SetupAttemptId, SetupIntentId},
    params::{Expand, Expandable, List, Object, Paginable, RangeQuery, Timestamp},
    BaseClient, Client,
};
use serde::{Deserialize, Serialize};

use crate::resources::{
    Account, ApiErrors, Application, Customer, PaymentMethod, SetupAttemptPaymentMethodDetails,
    SetupIntent,
};

/// The resource representing a Stripe "PaymentFlowsSetupIntentSetupAttempt".
///
/// For more details see <https://stripe.com/docs/api/setup_attempts/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttempt {
    /// Unique identifier for the object.
    pub id: SetupAttemptId,

    /// The value of [application](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-application) on the SetupIntent at the time of this confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Expandable<Application>>,

    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    ///
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_self: Option<bool>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The value of [customer](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-customer) on the SetupIntent at the time of this confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    ///
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_directions: Option<Vec<SetupAttemptFlowDirections>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The value of [on_behalf_of](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-on_behalf_of) on the SetupIntent at the time of this confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<Expandable<Account>>,

    /// ID of the payment method used with this SetupAttempt.
    pub payment_method: Expandable<PaymentMethod>,

    pub payment_method_details: SetupAttemptPaymentMethodDetails,

    /// The error encountered during this attempt to confirm the SetupIntent, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_error: Option<Box<ApiErrors>>,

    /// ID of the SetupIntent that this attempt belongs to.
    pub setup_intent: Expandable<SetupIntent>,

    /// Status of this SetupAttempt, one of `requires_confirmation`, `requires_action`, `processing`, `succeeded`, `failed`, or `abandoned`.
    pub status: String,

    /// The value of [usage](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-usage) on the SetupIntent at the time of this confirmation, one of `off_session` or `on_session`.
    pub usage: String,
}

impl SetupAttempt {
    /// Returns a list of SetupAttempts associated with a provided SetupIntent.
    pub fn list<C: BaseClient>(
        client: &Client<C>,
        params: &ListSetupAttempts<'_>,
    ) -> <C as BaseClient>::Response<List<SetupAttempt>> {
        client.get_query("/setup_attempts", &params)
    }
}

impl Object for SetupAttempt {
    type Id = SetupAttemptId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "setup_attempt"
    }
}

/// The parameters for `SetupAttempt::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListSetupAttempts<'a> {
    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<SetupAttemptId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Only return SetupAttempts created by the SetupIntent specified by
    /// this ID.
    pub setup_intent: SetupIntentId,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<SetupAttemptId>,
}

impl<'a> ListSetupAttempts<'a> {
    pub fn new(setup_intent: SetupIntentId) -> Self {
        ListSetupAttempts {
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            setup_intent,
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListSetupAttempts<'_> {
    type O = SetupAttempt;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// An enum representing the possible values of an `SetupAttempt`'s `flow_directions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupAttemptFlowDirections {
    Inbound,
    Outbound,
}

impl SetupAttemptFlowDirections {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupAttemptFlowDirections::Inbound => "inbound",
            SetupAttemptFlowDirections::Outbound => "outbound",
        }
    }
}

impl AsRef<str> for SetupAttemptFlowDirections {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupAttemptFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupAttemptFlowDirections {
    fn default() -> Self {
        Self::Inbound
    }
}
