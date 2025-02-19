// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::TaxCodeId;
use crate::params::{Expand, List, Object, Paginable};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TaxProductResourceTaxCode".
///
/// For more details see <https://stripe.com/docs/api/tax_codes/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxCode {
    /// Unique identifier for the object.
    pub id: TaxCodeId,

    /// A detailed description of which types of products the tax code represents.
    pub description: String,

    /// A short name for the tax code.
    pub name: String,
}

impl TaxCode {
    /// A list of [all tax codes available](https://stripe.com/docs/tax/tax-categories) to add to Products in order to allow specific tax calculations.
    pub fn list(client: &Client, params: &ListTaxCodes<'_>) -> Response<List<TaxCode>> {
        client.get_query("/tax_codes", params)
    }

    /// Retrieves the details of an existing tax code.
    ///
    /// Supply the unique tax code ID and Stripe will return the corresponding tax code information.
    pub fn retrieve(client: &Client, id: &TaxCodeId, expand: &[&str]) -> Response<TaxCode> {
        client.get_query(&format!("/tax_codes/{}", id), Expand { expand })
    }
}

impl Object for TaxCode {
    type Id = TaxCodeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax_code"
    }
}

/// The parameters for `TaxCode::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListTaxCodes<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<TaxCodeId>,

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
    pub starting_after: Option<TaxCodeId>,
}

impl<'a> ListTaxCodes<'a> {
    pub fn new() -> Self {
        ListTaxCodes {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListTaxCodes<'_> {
    type O = TaxCode;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
