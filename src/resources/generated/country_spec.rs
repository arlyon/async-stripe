// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{CountrySpecId};
use crate::params::{Expand, List, Object, Paginable};
use crate::resources::{Currency};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "CountrySpec".
///
/// For more details see <https://stripe.com/docs/api/country_specs/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CountrySpec {
    /// Unique identifier for the object.
    ///
    /// Represented as the ISO country code for this country.
    pub id: CountrySpecId,

    /// The default currency for this country.
    ///
    /// This applies to both payment methods and bank accounts.
    pub default_currency: Currency,

    /// Currencies that can be accepted in the specific country (for transfers).
    pub supported_bank_account_currencies: Vec<String>,

    /// Currencies that can be accepted in the specified country (for payments).
    pub supported_payment_currencies: Vec<String>,

    /// Payment methods available in the specified country.
    ///
    /// You may need to enable some payment methods (e.g., [ACH](https://stripe.com/docs/ach)) on your account before they appear in this list.
    /// The `stripe` payment method refers to [charging through your platform](https://stripe.com/docs/connect/destination-charges).
    pub supported_payment_methods: Vec<String>,

    /// Countries that can accept transfers from the specified country.
    pub supported_transfer_countries: Vec<String>,

    pub verification_fields: CountrySpecVerificationFields,
}

impl CountrySpec {

    /// Lists all Country Spec objects available in the API.
pub fn list(client: &Client, params: &ListCountrySpecs<'_>) -> Response<List<CountrySpec>> {
   client.get_query("/country_specs", params)
}


    /// Returns a Country Spec for a given Country code.
    pub fn retrieve(client: &Client, id: &CountrySpecId, expand: &[&str]) -> Response<CountrySpec> {
        client.get_query(&format!("/country_specs/{}", id), Expand { expand })
    }
}

impl Object for CountrySpec {
    type Id = CountrySpecId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "country_spec"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CountrySpecVerificationFields {

    pub company: CountrySpecVerificationFieldDetails,

    pub individual: CountrySpecVerificationFieldDetails,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CountrySpecVerificationFieldDetails {

    /// Additional fields which are only required for some users.
    pub additional: Vec<String>,

    /// Fields which every account must eventually provide.
    pub minimum: Vec<String>,
}

/// The parameters for `CountrySpec::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListCountrySpecs<'a> {

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<CountrySpecId>,

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
    pub starting_after: Option<CountrySpecId>,
}

impl<'a> ListCountrySpecs<'a> {
    pub fn new() -> Self {
        ListCountrySpecs {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListCountrySpecs<'_> {
    type O = CountrySpec;
    fn set_last(&mut self, item: Self::O) {
                self.starting_after = Some(item.id());
            }}