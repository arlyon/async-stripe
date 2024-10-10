use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{BankAccountId, CardId, CustomerId, PaymentSourceId};
use crate::params::{Deleted, Expand, List, SearchList};
use crate::resources::{
    BankAccount, Customer, PaymentMethod, PaymentSource, PaymentSourceParams, Source,
};

#[derive(Clone, Debug, Serialize, Default, Eq, PartialEq)]
pub struct CustomerPaymentMethodRetrieval<'a> {
    /// A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list.
    ///For instance, if you make a list request and receive 100 objects, starting with `obj_bar`,
    ///your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list.
    ///For instance, if you make a list request and receive 100 objects, ending with `obj_foo`,
    ///your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    /// An optional filter on the list, based on the object type field. Without the filter, the list includes all current and future payment method types. If your integration expects only one type of payment method in the response, make sure to provide a type value in the request.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CustomerPaymentMethodRetrievalType>,
}

impl<'a> CustomerPaymentMethodRetrieval<'a> {
    pub fn new() -> Self {
        CustomerPaymentMethodRetrieval::default()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerPaymentMethodRetrievalType {
    AcssDebit,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Oxxo,
    P24,
    SepaDebit,
    Sofort,
    WechatPay,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CustomerSearchParams<'a> {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    pub expand: &'a [&'a str],
}

impl<'a> CustomerSearchParams<'a> {
    pub fn new() -> CustomerSearchParams<'a> {
        CustomerSearchParams::default()
    }
}

impl Customer {
    /// Attaches a source to a customer, does not change default Source for the Customer
    ///
    /// For more details see <https://stripe.com/docs/api#attach_source>.
    pub fn attach_source(
        client: &Client,
        customer_id: &CustomerId,
        source: PaymentSourceParams,
    ) -> Response<PaymentSource> {
        #[derive(Serialize)]
        struct AttachSource {
            source: PaymentSourceParams,
        }
        let params = AttachSource { source };
        client.post_form(&format!("/customers/{}/sources", customer_id), params)
    }

    /// Detaches a source from a customer
    ///
    /// For more details see <https://stripe.com/docs/api#detach_source>.
    pub fn detach_source(
        client: &Client,
        customer_id: &CustomerId,
        source_id: &PaymentSourceId,
    ) -> Response<DetachedSource> {
        client.delete(&format!("/customers/{}/sources/{}", customer_id, source_id))
    }

    /// Retrieves a Card, BankAccount, or Source for a Customer
    pub fn retrieve_source(
        client: &Client,
        customer_id: &CustomerId,
        source_id: &PaymentSourceId,
    ) -> Response<PaymentSource> {
        client.get(&format!("/customers/{}/sources/{}", customer_id, source_id))
    }

    /// Verifies a Bank Account for a Customer.
    ///
    /// For more details see <https://stripe.com/docs/api/customer_bank_accounts/verify>.
    pub fn verify_bank_account(
        client: &Client,
        customer_id: &CustomerId,
        bank_account_id: &BankAccountId,
        params: VerifyBankAccount<'_>,
    ) -> Response<BankAccount> {
        client.post_form(
            &format!("/customers/{}/sources/{}/verify", customer_id, bank_account_id),
            params,
        )
    }

    ///Returns a list of PaymentMethods for a given Customer
    ///
    ///For more details see <https://stripe.com/docs/api/payment_methods/customer_list>
    pub fn retrieve_payment_methods(
        client: &Client,
        customer_id: &CustomerId,
        params: CustomerPaymentMethodRetrieval<'_>,
    ) -> Response<List<PaymentMethod>> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.get_query(&format!("/customers/{}/payment_methods", customer_id), &params)
    }

    /// Searches for a customer.
    ///
    /// For more details see <https://stripe.com/docs/api/customers/search>.
    pub fn search(client: &Client, params: CustomerSearchParams) -> Response<SearchList<Customer>> {
        client.get_query("/customers/search", params)
    }
}

/// The set of parameters that can be used when verifying a Bank Account.
///
/// For more details see <https://stripe.com/docs/api/customer_bank_accounts/verify>.
#[derive(Clone, Debug, Default, Serialize)]
pub struct VerifyBankAccount<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<&'a str>,
}

impl VerifyBankAccount<'_> {
    pub fn new() -> Self {
        VerifyBankAccount { amounts: None, verification_method: None }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum DetachedSource {
    BankAccount(Deleted<BankAccountId>),
    Card(Deleted<CardId>),
    Source(Source),
}
