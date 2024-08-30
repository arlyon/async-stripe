// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{BillingMeterErrorReportId};
use crate::params::{Object, Timestamp};
use crate::resources::{BillingMeterErrorRelatedObject};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BillingMeterResourceMeterErrorReportNotification".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeterErrorReport {
    /// Unique identifier for the object.
    pub id: BillingMeterErrorReportId,

    pub reason: BillingMeterResourceMeterErrorReason,

    /// The related objects about the error.
    pub related_object: Option<BillingMeterErrorRelatedObject>,

    /// Summary of invalid events.
    pub summary: String,

    /// Time when validation ended.
    ///
    /// Measured in seconds since the Unix epoch.
    pub validation_end: Timestamp,

    /// Time when validation started.
    ///
    /// Measured in seconds since the Unix epoch.
    pub validation_start: Timestamp,
}

impl Object for BillingMeterErrorReport {
    type Id = BillingMeterErrorReportId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "billing.meter_error_report"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeterResourceMeterErrorReason {

    /// The number of errors generated.
    pub error_count: u64,

    /// More information about errors.
    pub error_types: Vec<BillingMeterResourceMeterErrorReasonErrorType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeterResourceMeterErrorReasonErrorType {

    pub sample_errors: Vec<BillingMeterResourceMeterErrorReasonSampleError>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeterResourceMeterErrorReasonSampleError {

    pub api_request: Option<BillingMeterResourceMeterErrorReasonSampleErrorRequest>,

    /// message of the error.
    pub error_message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingMeterResourceMeterErrorReasonSampleErrorRequest {

    /// Unique identifier for the object.
    pub id: String,

    /// idempotency_key of the request.
    pub idempotency_key: String,
}
