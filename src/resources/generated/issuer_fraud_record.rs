// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{ChargeId, IssuerFraudRecordId};
use crate::params::{Expand, Expandable, List, Object, Timestamp};
use crate::resources::Charge;

/// The resource representing a Stripe "IssuerFraudRecord".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuerFraudRecord {
    /// Unique identifier for the object.
    pub id: IssuerFraudRecordId,

    /// An IFR is actionable if it has not received a dispute and has not been fully refunded.
    ///
    /// You may wish to proactively refund a charge that receives an IFR, in order to avoid receiving a dispute later.
    pub actionable: bool,

    /// ID of the charge this issuer fraud record is for, optionally expanded.
    pub charge: Expandable<Charge>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The type of fraud labelled by the issuer.
    ///
    /// One of `card_never_received`, `fraudulent_card_application`, `made_with_counterfeit_card`, `made_with_lost_card`, `made_with_stolen_card`, `misc`, `unauthorized_use_of_card`.
    pub fraud_type: String,

    /// If true, the associated charge is subject to [liability shift](https://stripe.com/docs/payments/3d-secure#disputed-payments).
    pub has_liability_shift: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The timestamp at which the card issuer posted the issuer fraud record.
    pub post_date: Timestamp,
}

impl IssuerFraudRecord {
    /// Returns a list of issuer fraud records.
    pub fn list(
        client: &Client,
        params: ListIssuerFraudRecords<'_>,
    ) -> Response<List<IssuerFraudRecord>> {
        client.get_query("/issuer_fraud_records", &params)
    }

    /// Retrieves the details of an issuer fraud record that has previously been created.
    ///
    /// Please refer to the [issuer fraud record](https://stripe.com/docs/api#issuer_fraud_record_object) object reference for more details.
    pub fn retrieve(
        client: &Client,
        id: &IssuerFraudRecordId,
        expand: &[&str],
    ) -> Response<IssuerFraudRecord> {
        client.get_query(&format!("/issuer_fraud_records/{}", id), &Expand { expand })
    }
}

impl Object for IssuerFraudRecord {
    type Id = IssuerFraudRecordId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuer_fraud_record"
    }
}

/// The parameters for `IssuerFraudRecord::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListIssuerFraudRecords<'a> {
    /// Only return issuer fraud records for the charge specified by this charge ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<ChargeId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<IssuerFraudRecordId>,

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
    pub starting_after: Option<IssuerFraudRecordId>,
}

impl<'a> ListIssuerFraudRecords<'a> {
    pub fn new() -> Self {
        ListIssuerFraudRecords {
            charge: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
