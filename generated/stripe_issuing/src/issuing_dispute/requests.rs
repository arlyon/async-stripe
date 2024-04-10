#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListIssuingDispute<'a> {
    /// Select Issuing disputes that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Select Issuing disputes with the given status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_shared::IssuingDisputeStatus>,
    /// Select the Issuing dispute for the given transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<&'a str>,
}
impl<'a> ListIssuingDispute<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListIssuingDispute<'a> {
    /// Returns a list of Issuing `Dispute` objects.
    /// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_shared::IssuingDispute>> {
        client.get_query("/issuing/disputes", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_shared::IssuingDispute>> {
        stripe::ListPaginator::from_list_params("/issuing/disputes", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveIssuingDispute<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveIssuingDispute<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveIssuingDispute<'a> {
    /// Retrieves an Issuing `Dispute` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        dispute: &stripe_shared::IssuingDisputeId,
    ) -> stripe::Response<stripe_shared::IssuingDispute> {
        client.get_query(&format!("/issuing/disputes/{dispute}"), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateIssuingDispute<'a> {
    /// The dispute amount in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// If not set, defaults to the full transaction amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Evidence provided for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<CreateIssuingDisputeEvidence<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The ID of the issuing transaction to create a dispute for.
    /// For transaction on Treasury FinancialAccounts, use `treasury.received_debit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<&'a str>,
    /// Params for disputes related to Treasury FinancialAccounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<CreateIssuingDisputeTreasury<'a>>,
}
impl<'a> CreateIssuingDispute<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Evidence provided for the dispute.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateIssuingDisputeEvidence<'a> {
    /// Evidence provided when `reason` is 'canceled'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<CreateIssuingDisputeEvidenceCanceled<'a>>,
    /// Evidence provided when `reason` is 'duplicate'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<Duplicate<'a>>,
    /// Evidence provided when `reason` is 'fraudulent'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<Fraudulent<'a>>,
    /// Evidence provided when `reason` is 'merchandise_not_as_described'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_not_as_described:
        Option<CreateIssuingDisputeEvidenceMerchandiseNotAsDescribed<'a>>,
    /// Evidence provided when `reason` is 'not_received'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_received: Option<CreateIssuingDisputeEvidenceNotReceived<'a>>,
    /// Evidence provided when `reason` is 'other'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<CreateIssuingDisputeEvidenceOther<'a>>,
    /// The reason for filing the dispute. The evidence should be submitted in the field of the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<CreateIssuingDisputeEvidenceReason>,
    /// Evidence provided when `reason` is 'service_not_as_described'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_not_as_described: Option<ServiceNotAsDescribed<'a>>,
}
impl<'a> CreateIssuingDisputeEvidence<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Evidence provided when `reason` is 'canceled'.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateIssuingDisputeEvidenceCanceled<'a> {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<&'a str>,
    /// Date when order was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Whether the cardholder was provided with a cancellation policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_provided: Option<bool>,
    /// Reason for canceling the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<&'a str>,
    /// Date when the cardholder expected to receive the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<stripe_types::Timestamp>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<&'a str>,
    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<&'a str>,
    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<CreateIssuingDisputeEvidenceCanceledProductType>,
    /// Result of cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_status: Option<CreateIssuingDisputeEvidenceCanceledReturnStatus>,
    /// Date when the product was returned or attempted to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<stripe_types::Timestamp>,
}
impl<'a> CreateIssuingDisputeEvidenceCanceled<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether the product was a merchandise or service.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIssuingDisputeEvidenceCanceledProductType {
    Merchandise,
    Service,
}
impl CreateIssuingDisputeEvidenceCanceledProductType {
    pub fn as_str(self) -> &'static str {
        use CreateIssuingDisputeEvidenceCanceledProductType::*;
        match self {
            Merchandise => "merchandise",
            Service => "service",
        }
    }
}

impl std::str::FromStr for CreateIssuingDisputeEvidenceCanceledProductType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingDisputeEvidenceCanceledProductType::*;
        match s {
            "merchandise" => Ok(Merchandise),
            "service" => Ok(Service),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateIssuingDisputeEvidenceCanceledProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIssuingDisputeEvidenceCanceledProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIssuingDisputeEvidenceCanceledProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateIssuingDisputeEvidenceCanceledProductType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateIssuingDisputeEvidenceCanceledProductType",
            )
        })
    }
}
/// Result of cardholder's attempt to return the product.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIssuingDisputeEvidenceCanceledReturnStatus {
    MerchantRejected,
    Successful,
}
impl CreateIssuingDisputeEvidenceCanceledReturnStatus {
    pub fn as_str(self) -> &'static str {
        use CreateIssuingDisputeEvidenceCanceledReturnStatus::*;
        match self {
            MerchantRejected => "merchant_rejected",
            Successful => "successful",
        }
    }
}

impl std::str::FromStr for CreateIssuingDisputeEvidenceCanceledReturnStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingDisputeEvidenceCanceledReturnStatus::*;
        match s {
            "merchant_rejected" => Ok(MerchantRejected),
            "successful" => Ok(Successful),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateIssuingDisputeEvidenceCanceledReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIssuingDisputeEvidenceCanceledReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIssuingDisputeEvidenceCanceledReturnStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateIssuingDisputeEvidenceCanceledReturnStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateIssuingDisputeEvidenceCanceledReturnStatus",
            )
        })
    }
}
/// Evidence provided when `reason` is 'merchandise_not_as_described'.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateIssuingDisputeEvidenceMerchandiseNotAsDescribed<'a> {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<&'a str>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<&'a str>,
    /// Date when the product was received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<stripe_types::Timestamp>,
    /// Description of the cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_description: Option<&'a str>,
    /// Result of cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_status: Option<CreateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus>,
    /// Date when the product was returned or attempted to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<stripe_types::Timestamp>,
}
impl<'a> CreateIssuingDisputeEvidenceMerchandiseNotAsDescribed<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Result of cardholder's attempt to return the product.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus {
    MerchantRejected,
    Successful,
}
impl CreateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus {
    pub fn as_str(self) -> &'static str {
        use CreateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus::*;
        match self {
            MerchantRejected => "merchant_rejected",
            Successful => "successful",
        }
    }
}

impl std::str::FromStr for CreateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus::*;
        match s {
            "merchant_rejected" => Ok(MerchantRejected),
            "successful" => Ok(Successful),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus"))
    }
}
/// Evidence provided when `reason` is 'not_received'.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateIssuingDisputeEvidenceNotReceived<'a> {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<&'a str>,
    /// Date when the cardholder expected to receive the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<stripe_types::Timestamp>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<&'a str>,
    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<&'a str>,
    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<CreateIssuingDisputeEvidenceNotReceivedProductType>,
}
impl<'a> CreateIssuingDisputeEvidenceNotReceived<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether the product was a merchandise or service.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIssuingDisputeEvidenceNotReceivedProductType {
    Merchandise,
    Service,
}
impl CreateIssuingDisputeEvidenceNotReceivedProductType {
    pub fn as_str(self) -> &'static str {
        use CreateIssuingDisputeEvidenceNotReceivedProductType::*;
        match self {
            Merchandise => "merchandise",
            Service => "service",
        }
    }
}

impl std::str::FromStr for CreateIssuingDisputeEvidenceNotReceivedProductType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingDisputeEvidenceNotReceivedProductType::*;
        match s {
            "merchandise" => Ok(Merchandise),
            "service" => Ok(Service),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateIssuingDisputeEvidenceNotReceivedProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIssuingDisputeEvidenceNotReceivedProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIssuingDisputeEvidenceNotReceivedProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateIssuingDisputeEvidenceNotReceivedProductType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateIssuingDisputeEvidenceNotReceivedProductType",
            )
        })
    }
}
/// Evidence provided when `reason` is 'other'.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateIssuingDisputeEvidenceOther<'a> {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<&'a str>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<&'a str>,
    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<&'a str>,
    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<CreateIssuingDisputeEvidenceOtherProductType>,
}
impl<'a> CreateIssuingDisputeEvidenceOther<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether the product was a merchandise or service.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIssuingDisputeEvidenceOtherProductType {
    Merchandise,
    Service,
}
impl CreateIssuingDisputeEvidenceOtherProductType {
    pub fn as_str(self) -> &'static str {
        use CreateIssuingDisputeEvidenceOtherProductType::*;
        match self {
            Merchandise => "merchandise",
            Service => "service",
        }
    }
}

impl std::str::FromStr for CreateIssuingDisputeEvidenceOtherProductType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingDisputeEvidenceOtherProductType::*;
        match s {
            "merchandise" => Ok(Merchandise),
            "service" => Ok(Service),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateIssuingDisputeEvidenceOtherProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIssuingDisputeEvidenceOtherProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIssuingDisputeEvidenceOtherProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateIssuingDisputeEvidenceOtherProductType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateIssuingDisputeEvidenceOtherProductType",
            )
        })
    }
}
/// The reason for filing the dispute. The evidence should be submitted in the field of the same name.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIssuingDisputeEvidenceReason {
    Canceled,
    Duplicate,
    Fraudulent,
    MerchandiseNotAsDescribed,
    NotReceived,
    Other,
    ServiceNotAsDescribed,
}
impl CreateIssuingDisputeEvidenceReason {
    pub fn as_str(self) -> &'static str {
        use CreateIssuingDisputeEvidenceReason::*;
        match self {
            Canceled => "canceled",
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            MerchandiseNotAsDescribed => "merchandise_not_as_described",
            NotReceived => "not_received",
            Other => "other",
            ServiceNotAsDescribed => "service_not_as_described",
        }
    }
}

impl std::str::FromStr for CreateIssuingDisputeEvidenceReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingDisputeEvidenceReason::*;
        match s {
            "canceled" => Ok(Canceled),
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "merchandise_not_as_described" => Ok(MerchandiseNotAsDescribed),
            "not_received" => Ok(NotReceived),
            "other" => Ok(Other),
            "service_not_as_described" => Ok(ServiceNotAsDescribed),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateIssuingDisputeEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIssuingDisputeEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIssuingDisputeEvidenceReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateIssuingDisputeEvidenceReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateIssuingDisputeEvidenceReason")
        })
    }
}
/// Params for disputes related to Treasury FinancialAccounts
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateIssuingDisputeTreasury<'a> {
    /// The ID of the ReceivedDebit to initiate an Issuings dispute for.
    pub received_debit: &'a str,
}
impl<'a> CreateIssuingDisputeTreasury<'a> {
    pub fn new(received_debit: &'a str) -> Self {
        Self { received_debit }
    }
}
impl<'a> CreateIssuingDispute<'a> {
    /// Creates an Issuing `Dispute` object.
    /// Individual pieces of evidence within the `evidence` object are optional at this point.
    /// Stripe only validates that required evidence is present during submission.
    /// Refer to [Dispute reasons and evidence](https://stripe.com/docs/issuing/purchases/disputes#dispute-reasons-and-evidence) for more details about evidence requirements.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_shared::IssuingDispute> {
        client.send_form("/issuing/disputes", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateIssuingDispute<'a> {
    /// The dispute amount in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Evidence provided for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<UpdateIssuingDisputeEvidence<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateIssuingDispute<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Evidence provided for the dispute.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateIssuingDisputeEvidence<'a> {
    /// Evidence provided when `reason` is 'canceled'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<UpdateIssuingDisputeEvidenceCanceled<'a>>,
    /// Evidence provided when `reason` is 'duplicate'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<Duplicate<'a>>,
    /// Evidence provided when `reason` is 'fraudulent'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<Fraudulent<'a>>,
    /// Evidence provided when `reason` is 'merchandise_not_as_described'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_not_as_described:
        Option<UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribed<'a>>,
    /// Evidence provided when `reason` is 'not_received'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_received: Option<UpdateIssuingDisputeEvidenceNotReceived<'a>>,
    /// Evidence provided when `reason` is 'other'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<UpdateIssuingDisputeEvidenceOther<'a>>,
    /// The reason for filing the dispute. The evidence should be submitted in the field of the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<UpdateIssuingDisputeEvidenceReason>,
    /// Evidence provided when `reason` is 'service_not_as_described'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_not_as_described: Option<ServiceNotAsDescribed<'a>>,
}
impl<'a> UpdateIssuingDisputeEvidence<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Evidence provided when `reason` is 'canceled'.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateIssuingDisputeEvidenceCanceled<'a> {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<&'a str>,
    /// Date when order was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Whether the cardholder was provided with a cancellation policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_provided: Option<bool>,
    /// Reason for canceling the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<&'a str>,
    /// Date when the cardholder expected to receive the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<stripe_types::Timestamp>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<&'a str>,
    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<&'a str>,
    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<UpdateIssuingDisputeEvidenceCanceledProductType>,
    /// Result of cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_status: Option<UpdateIssuingDisputeEvidenceCanceledReturnStatus>,
    /// Date when the product was returned or attempted to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<stripe_types::Timestamp>,
}
impl<'a> UpdateIssuingDisputeEvidenceCanceled<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether the product was a merchandise or service.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateIssuingDisputeEvidenceCanceledProductType {
    Merchandise,
    Service,
}
impl UpdateIssuingDisputeEvidenceCanceledProductType {
    pub fn as_str(self) -> &'static str {
        use UpdateIssuingDisputeEvidenceCanceledProductType::*;
        match self {
            Merchandise => "merchandise",
            Service => "service",
        }
    }
}

impl std::str::FromStr for UpdateIssuingDisputeEvidenceCanceledProductType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateIssuingDisputeEvidenceCanceledProductType::*;
        match s {
            "merchandise" => Ok(Merchandise),
            "service" => Ok(Service),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UpdateIssuingDisputeEvidenceCanceledProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateIssuingDisputeEvidenceCanceledProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateIssuingDisputeEvidenceCanceledProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateIssuingDisputeEvidenceCanceledProductType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateIssuingDisputeEvidenceCanceledProductType",
            )
        })
    }
}
/// Result of cardholder's attempt to return the product.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateIssuingDisputeEvidenceCanceledReturnStatus {
    MerchantRejected,
    Successful,
}
impl UpdateIssuingDisputeEvidenceCanceledReturnStatus {
    pub fn as_str(self) -> &'static str {
        use UpdateIssuingDisputeEvidenceCanceledReturnStatus::*;
        match self {
            MerchantRejected => "merchant_rejected",
            Successful => "successful",
        }
    }
}

impl std::str::FromStr for UpdateIssuingDisputeEvidenceCanceledReturnStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateIssuingDisputeEvidenceCanceledReturnStatus::*;
        match s {
            "merchant_rejected" => Ok(MerchantRejected),
            "successful" => Ok(Successful),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UpdateIssuingDisputeEvidenceCanceledReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateIssuingDisputeEvidenceCanceledReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateIssuingDisputeEvidenceCanceledReturnStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateIssuingDisputeEvidenceCanceledReturnStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateIssuingDisputeEvidenceCanceledReturnStatus",
            )
        })
    }
}
/// Evidence provided when `reason` is 'merchandise_not_as_described'.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribed<'a> {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<&'a str>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<&'a str>,
    /// Date when the product was received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<stripe_types::Timestamp>,
    /// Description of the cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_description: Option<&'a str>,
    /// Result of cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_status: Option<UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus>,
    /// Date when the product was returned or attempted to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<stripe_types::Timestamp>,
}
impl<'a> UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribed<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Result of cardholder's attempt to return the product.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus {
    MerchantRejected,
    Successful,
}
impl UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus {
    pub fn as_str(self) -> &'static str {
        use UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus::*;
        match self {
            MerchantRejected => "merchant_rejected",
            Successful => "successful",
        }
    }
}

impl std::str::FromStr for UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus::*;
        match s {
            "merchant_rejected" => Ok(MerchantRejected),
            "successful" => Ok(Successful),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus"))
    }
}
/// Evidence provided when `reason` is 'not_received'.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateIssuingDisputeEvidenceNotReceived<'a> {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<&'a str>,
    /// Date when the cardholder expected to receive the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<stripe_types::Timestamp>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<&'a str>,
    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<&'a str>,
    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<UpdateIssuingDisputeEvidenceNotReceivedProductType>,
}
impl<'a> UpdateIssuingDisputeEvidenceNotReceived<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether the product was a merchandise or service.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateIssuingDisputeEvidenceNotReceivedProductType {
    Merchandise,
    Service,
}
impl UpdateIssuingDisputeEvidenceNotReceivedProductType {
    pub fn as_str(self) -> &'static str {
        use UpdateIssuingDisputeEvidenceNotReceivedProductType::*;
        match self {
            Merchandise => "merchandise",
            Service => "service",
        }
    }
}

impl std::str::FromStr for UpdateIssuingDisputeEvidenceNotReceivedProductType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateIssuingDisputeEvidenceNotReceivedProductType::*;
        match s {
            "merchandise" => Ok(Merchandise),
            "service" => Ok(Service),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UpdateIssuingDisputeEvidenceNotReceivedProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateIssuingDisputeEvidenceNotReceivedProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateIssuingDisputeEvidenceNotReceivedProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateIssuingDisputeEvidenceNotReceivedProductType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateIssuingDisputeEvidenceNotReceivedProductType",
            )
        })
    }
}
/// Evidence provided when `reason` is 'other'.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateIssuingDisputeEvidenceOther<'a> {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<&'a str>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<&'a str>,
    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<&'a str>,
    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<UpdateIssuingDisputeEvidenceOtherProductType>,
}
impl<'a> UpdateIssuingDisputeEvidenceOther<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether the product was a merchandise or service.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateIssuingDisputeEvidenceOtherProductType {
    Merchandise,
    Service,
}
impl UpdateIssuingDisputeEvidenceOtherProductType {
    pub fn as_str(self) -> &'static str {
        use UpdateIssuingDisputeEvidenceOtherProductType::*;
        match self {
            Merchandise => "merchandise",
            Service => "service",
        }
    }
}

impl std::str::FromStr for UpdateIssuingDisputeEvidenceOtherProductType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateIssuingDisputeEvidenceOtherProductType::*;
        match s {
            "merchandise" => Ok(Merchandise),
            "service" => Ok(Service),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UpdateIssuingDisputeEvidenceOtherProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateIssuingDisputeEvidenceOtherProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateIssuingDisputeEvidenceOtherProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateIssuingDisputeEvidenceOtherProductType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateIssuingDisputeEvidenceOtherProductType",
            )
        })
    }
}
/// The reason for filing the dispute. The evidence should be submitted in the field of the same name.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateIssuingDisputeEvidenceReason {
    Canceled,
    Duplicate,
    Fraudulent,
    MerchandiseNotAsDescribed,
    NotReceived,
    Other,
    ServiceNotAsDescribed,
}
impl UpdateIssuingDisputeEvidenceReason {
    pub fn as_str(self) -> &'static str {
        use UpdateIssuingDisputeEvidenceReason::*;
        match self {
            Canceled => "canceled",
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            MerchandiseNotAsDescribed => "merchandise_not_as_described",
            NotReceived => "not_received",
            Other => "other",
            ServiceNotAsDescribed => "service_not_as_described",
        }
    }
}

impl std::str::FromStr for UpdateIssuingDisputeEvidenceReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateIssuingDisputeEvidenceReason::*;
        match s {
            "canceled" => Ok(Canceled),
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "merchandise_not_as_described" => Ok(MerchandiseNotAsDescribed),
            "not_received" => Ok(NotReceived),
            "other" => Ok(Other),
            "service_not_as_described" => Ok(ServiceNotAsDescribed),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UpdateIssuingDisputeEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateIssuingDisputeEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateIssuingDisputeEvidenceReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateIssuingDisputeEvidenceReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateIssuingDisputeEvidenceReason")
        })
    }
}
impl<'a> UpdateIssuingDispute<'a> {
    /// Updates the specified Issuing `Dispute` object by setting the values of the parameters passed.
    /// Any parameters not provided will be left unchanged.
    /// Properties on the `evidence` object can be unset by passing in an empty string.
    pub fn send(
        &self,
        client: &stripe::Client,
        dispute: &stripe_shared::IssuingDisputeId,
    ) -> stripe::Response<stripe_shared::IssuingDispute> {
        client.send_form(&format!("/issuing/disputes/{dispute}"), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct SubmitIssuingDispute<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> SubmitIssuingDispute<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> SubmitIssuingDispute<'a> {
    /// Submits an Issuing `Dispute` to the card network.
    /// Stripe validates that all evidence fields required for the dispute’s reason are present.
    /// For more details, see [Dispute reasons and evidence](https://stripe.com/docs/issuing/purchases/disputes#dispute-reasons-and-evidence).
    pub fn send(
        &self,
        client: &stripe::Client,
        dispute: &stripe_shared::IssuingDisputeId,
    ) -> stripe::Response<stripe_shared::IssuingDispute> {
        client.send_form(
            &format!("/issuing/disputes/{dispute}/submit"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct Duplicate<'a> {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the card statement showing that the product had already been paid for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_statement: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the receipt showing that the product had been paid for in cash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_receipt: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Image of the front and back of the check that was used to pay for the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_image: Option<&'a str>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<&'a str>,
    /// Transaction (e.g., ipi_...) that the disputed transaction is a duplicate of.
    /// Of the two or more transactions that are copies of each other, this is original undisputed one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_transaction: Option<&'a str>,
}
impl<'a> Duplicate<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct Fraudulent<'a> {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<&'a str>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<&'a str>,
}
impl<'a> Fraudulent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ServiceNotAsDescribed<'a> {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<&'a str>,
    /// Date when order was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Reason for canceling the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<&'a str>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<&'a str>,
    /// Date when the product was received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<stripe_types::Timestamp>,
}
impl<'a> ServiceNotAsDescribed<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
