use crate::config::{Client, Response};
use crate::ids::{BankAccountId, PaymentSourceId};
use crate::params::{Identifiable, List, Metadata, RangeQuery, Timestamp};
use crate::resources::{
    Address, BankAccount, BankAccountVerifyParams, Currency, Deleted, Discount, PaymentSource,
    PaymentSourceParams, Subscription,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomerShippingDetails {
    pub address: Address,
    pub name: String,
    pub phone: String,
}

/// The set of parameters that can be used when creating or updating a customer.
///
/// For more details see https://stripe.com/docs/api#create_customer and https://stripe.com/docs/api#update_customer.
#[derive(Clone, Debug, Default, Serialize)]
pub struct CustomerParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_vat_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<PaymentSourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CustomerShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentSourceParams<'a>>,
}

/// The set of parameters that can be used when listing customers.
///
/// For more details see https://stripe.com/docs/api#list_customers
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}

/// The resource representing a Stripe customer.
///
/// For more details see https://stripe.com/docs/api#customers.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Customer {
    pub id: String,
    pub account_balance: i64,
    pub business_vat_id: Option<String>,
    pub created: u64,
    pub currency: Option<Currency>,
    pub default_source: Option<PaymentSourceId>,
    pub delinquent: bool,
    pub desc: Option<String>,
    pub discount: Option<Discount>,
    pub email: Option<String>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub shipping: Option<CustomerShippingDetails>,
    pub sources: List<PaymentSource>,
    pub subscriptions: List<Subscription>,
}

impl Customer {
    /// Creates a new customer.
    ///
    /// For more details see https://stripe.com/docs/api#create_customer.
    pub fn create(client: &Client, params: CustomerParams<'_>) -> Response<Customer> {
        client.post_form("/customers", params)
    }

    /// Retrieves the details of a customer.
    ///
    /// For more details see https://stripe.com/docs/api#retrieve_customer.
    pub fn retrieve(client: &Client, customer_id: &str) -> Response<Customer> {
        client.get(&format!("/customers/{}", customer_id))
    }

    /// Updates a customer's properties.
    ///
    /// For more details see https://stripe.com/docs/api#update_customer.
    pub fn update(
        client: &Client,
        customer_id: &str,
        params: CustomerParams<'_>,
    ) -> Response<Customer> {
        client.post_form(&format!("/customers/{}", customer_id), params)
    }

    /// Deletes a customer.
    ///
    /// For more details see https://stripe.com/docs/api#delete_customer.
    pub fn delete(client: &Client, customer_id: &str) -> Response<Deleted> {
        client.delete(&format!("/customers/{}", customer_id))
    }

    /// List customers.
    ///
    /// For more details see https://stripe.com/docs/api#list_customers.
    pub fn list(client: &Client, params: CustomerListParams<'_>) -> Response<List<Customer>> {
        client.get_query("/customers", &params)
    }

    /// Attaches a source to a customer, does not change default Source for the Customer
    ///
    /// For more details see [https://stripe.com/docs/api#attach_source](https://stripe.com/docs/api#attach_source).
    pub fn attach_source(
        client: &Client,
        customer_id: &str,
        source: PaymentSourceParams<'_>,
    ) -> Response<PaymentSource> {
        #[derive(Serialize)]
        struct AttachSource<'a> {
            source: PaymentSourceParams<'a>,
        }
        let params = AttachSource { source };
        client.post_form(&format!("/customers/{}/sources", customer_id), params)
    }

    /// Detaches a source from a customer
    ///
    /// For more details see [https://stripe.com/docs/api#detach_source](https://stripe.com/docs/api#detach_source).
    pub fn detach_source(
        client: &Client,
        customer_id: &str,
        source_id: &PaymentSourceId,
    ) -> Response<PaymentSource> {
        client.delete(&format!("/customers/{}/sources/{}", customer_id, source_id))
    }

    /// Retrieves a Card, BankAccount, or Source for a Customer
    pub fn retrieve_source(
        client: &Client,
        customer_id: &str,
        source_id: &PaymentSourceId,
    ) -> Response<PaymentSource> {
        client.get(&format!("/customers/{}/sources/{}", customer_id, source_id))
    }

    /// Verifies a Bank Account for a Customer.
    ///
    /// For more details see https://stripe.com/docs/api/customer_bank_accounts/verify.
    pub fn verify_bank_account(
        client: &Client,
        customer_id: &str,
        bank_account_id: &BankAccountId,
        params: BankAccountVerifyParams<'_>,
    ) -> Response<BankAccount> {
        client.post_form(
            &format!("/customers/{}/sources/{}/verify", customer_id, bank_account_id),
            params,
        )
    }
}

impl Identifiable for Customer {
    fn id(&self) -> &str {
        &self.id
    }
}
