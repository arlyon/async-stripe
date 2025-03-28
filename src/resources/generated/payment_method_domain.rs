// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{PaymentMethodDomainId};
use crate::params::{Expand, List, Object, Paginable, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "PaymentMethodDomainResourcePaymentMethodDomain".
///
/// For more details see <https://stripe.com/docs/api/payment_method_domains/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDomain {
    /// Unique identifier for the object.
    pub id: PaymentMethodDomainId,

    pub amazon_pay: PaymentMethodDomainResourcePaymentMethodStatus,

    pub apple_pay: PaymentMethodDomainResourcePaymentMethodStatus,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The domain name that this payment method domain object represents.
    pub domain_name: String,

    /// Whether this payment method domain is enabled.
    ///
    /// If the domain is not enabled, payment methods that require a payment method domain will not appear in Elements.
    pub enabled: bool,

    pub google_pay: PaymentMethodDomainResourcePaymentMethodStatus,

    pub link: PaymentMethodDomainResourcePaymentMethodStatus,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    pub paypal: PaymentMethodDomainResourcePaymentMethodStatus,
}

impl PaymentMethodDomain {

    /// Lists the details of existing payment method domains.
pub fn list(client: &Client, params: &ListPaymentMethodDomains<'_>) -> Response<List<PaymentMethodDomain>> {
   client.get_query("/payment_method_domains", params)
}


    /// Creates a payment method domain.
    pub fn create(client: &Client, params: CreatePaymentMethodDomain<'_>) -> Response<PaymentMethodDomain> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/payment_method_domains", &params)
    }

    /// Retrieves the details of an existing payment method domain.
    pub fn retrieve(client: &Client, id: &PaymentMethodDomainId, expand: &[&str]) -> Response<PaymentMethodDomain> {
        client.get_query(&format!("/payment_method_domains/{}", id), Expand { expand })
    }

    /// Updates an existing payment method domain.
    pub fn update(client: &Client, id: &PaymentMethodDomainId, params: UpdatePaymentMethodDomain<'_>) -> Response<PaymentMethodDomain> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/payment_method_domains/{}", id), &params)
    }
}

impl Object for PaymentMethodDomain {
    type Id = PaymentMethodDomainId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "payment_method_domain"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDomainResourcePaymentMethodStatus {

    /// The status of the payment method on the domain.
    pub status: PaymentMethodDomainResourcePaymentMethodStatusStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<PaymentMethodDomainResourcePaymentMethodStatusDetails>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDomainResourcePaymentMethodStatusDetails {

    /// The error message associated with the status of the payment method on the domain.
    pub error_message: String,
}

/// The parameters for `PaymentMethodDomain::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreatePaymentMethodDomain<'a> {

    /// The domain name that this payment method domain object represents.
    pub domain_name: &'a str,

    /// Whether this payment method domain is enabled.
    ///
    /// If the domain is not enabled, payment methods that require a payment method domain will not appear in Elements or Embedded Checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
}

impl<'a> CreatePaymentMethodDomain<'a> {
    pub fn new(domain_name: &'a str) -> Self {
        CreatePaymentMethodDomain {
            domain_name,
            enabled: Default::default(),
            expand: Default::default(),
        }
    }
}

/// The parameters for `PaymentMethodDomain::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListPaymentMethodDomains<'a> {

    /// The domain name that this payment method domain object represents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<&'a str>,

    /// Whether this payment method domain is enabled.
    ///
    /// If the domain is not enabled, payment methods will not appear in Elements or Embedded Checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<PaymentMethodDomainId>,

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
    pub starting_after: Option<PaymentMethodDomainId>,
}

impl<'a> ListPaymentMethodDomains<'a> {
    pub fn new() -> Self {
        ListPaymentMethodDomains {
            domain_name: Default::default(),
            enabled: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListPaymentMethodDomains<'_> {
    type O = PaymentMethodDomain;
    fn set_last(&mut self, item: Self::O) {
                self.starting_after = Some(item.id());
            }}
/// The parameters for `PaymentMethodDomain::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdatePaymentMethodDomain<'a> {

    /// Whether this payment method domain is enabled.
    ///
    /// If the domain is not enabled, payment methods that require a payment method domain will not appear in Elements or Embedded Checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
}

impl<'a> UpdatePaymentMethodDomain<'a> {
    pub fn new() -> Self {
        UpdatePaymentMethodDomain {
            enabled: Default::default(),
            expand: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `PaymentMethodDomainResourcePaymentMethodStatus`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDomainResourcePaymentMethodStatusStatus {
    Active,
    Inactive,
}

impl PaymentMethodDomainResourcePaymentMethodStatusStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDomainResourcePaymentMethodStatusStatus::Active => "active",
            PaymentMethodDomainResourcePaymentMethodStatusStatus::Inactive => "inactive",
        }
    }
}

impl AsRef<str> for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn default() -> Self {
        Self::Active
    }
}
