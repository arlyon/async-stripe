impl stripe_core::issuing::dispute::Dispute {
    /// Returns a list of Issuing `Dispute` objects.
    ///
    /// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
    pub fn list(
        client: &stripe::Client,
        params: ListDispute,
    ) -> stripe::Response<stripe_types::List<stripe_core::issuing::dispute::Dispute>> {
        client.get_query("/issuing/disputes", params)
    }
    /// Creates an Issuing `Dispute` object.
    ///
    /// Individual pieces of evidence within the `evidence` object are optional at this point.
    /// Stripe only validates that required evidence is present during submission.
    /// Refer to [Dispute reasons and evidence](https://stripe.com/docs/issuing/purchases/disputes#dispute-reasons-and-evidence) for more details about evidence requirements.
    pub fn create(
        client: &stripe::Client,
        params: CreateDispute,
    ) -> stripe::Response<stripe_core::issuing::dispute::Dispute> {
        client.send_form("/issuing/disputes", params, http_types::Method::Post)
    }
    /// Updates the specified Issuing `Dispute` object by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    /// Properties on the `evidence` object can be unset by passing in an empty string.
    pub fn update(
        client: &stripe::Client,
        dispute: &stripe_core::dispute::DisputeId,
        params: UpdateDispute,
    ) -> stripe::Response<stripe_core::issuing::dispute::Dispute> {
        client.send_form(
            &format!("/issuing/disputes/{dispute}", dispute = dispute),
            params,
            http_types::Method::Post,
        )
    }
    /// Retrieves an Issuing `Dispute` object.
    pub fn retrieve(
        client: &stripe::Client,
        dispute: &stripe_core::dispute::DisputeId,
        params: RetrieveDispute,
    ) -> stripe::Response<stripe_core::issuing::dispute::Dispute> {
        client.get_query(&format!("/issuing/disputes/{dispute}", dispute = dispute), params)
    }
    /// Submits an Issuing `Dispute` to the card network.
    ///
    /// Stripe validates that all evidence fields required for the dispute’s reason are present.
    /// For more details, see [Dispute reasons and evidence](https://stripe.com/docs/issuing/purchases/disputes#dispute-reasons-and-evidence).
    pub fn submit(
        client: &stripe::Client,
        dispute: &stripe_core::dispute::DisputeId,
        params: SubmitDispute,
    ) -> stripe::Response<stripe_core::issuing::dispute::Dispute> {
        client.send_form(
            &format!("/issuing/disputes/{dispute}/submit", dispute = dispute),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListDispute<'a> {
    /// Select Issuing disputes that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Select Issuing disputes with the given status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListDisputeStatus>,
    /// Select the Issuing dispute for the given transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<&'a str>,
}
impl<'a> ListDispute<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Select Issuing disputes with the given status.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ListDisputeStatus {
    Expired,
    Lost,
    Submitted,
    Unsubmitted,
    Won,
}

impl ListDisputeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Expired => "expired",
            Self::Lost => "lost",
            Self::Submitted => "submitted",
            Self::Unsubmitted => "unsubmitted",
            Self::Won => "won",
        }
    }
}

impl std::str::FromStr for ListDisputeStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "expired" => Ok(Self::Expired),
            "lost" => Ok(Self::Lost),
            "submitted" => Ok(Self::Submitted),
            "unsubmitted" => Ok(Self::Unsubmitted),
            "won" => Ok(Self::Won),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListDisputeStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListDisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ListDisputeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateDispute<'a> {
    /// The dispute amount in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// If not set, defaults to the full transaction amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Evidence provided for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<EvidenceParam<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The ID of the issuing transaction to create a dispute for.
    ///
    /// For transaction on Treasury FinancialAccounts, use `treasury.received_debit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<&'a str>,
    /// Params for disputes related to Treasury FinancialAccounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<CreateDisputeTreasury<'a>>,
}
impl<'a> CreateDispute<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Params for disputes related to Treasury FinancialAccounts.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateDisputeTreasury<'a> {
    /// The ID of the ReceivedDebit to initiate an Issuings dispute for.
    pub received_debit: &'a str,
}
impl<'a> CreateDisputeTreasury<'a> {
    pub fn new(received_debit: &'a str) -> Self {
        Self { received_debit }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateDispute<'a> {
    /// The dispute amount in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Evidence provided for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<EvidenceParam<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateDispute<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveDispute<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveDispute<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct SubmitDispute<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> SubmitDispute<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProductType {
    Merchandise,
    Service,
}

impl ProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Merchandise => "merchandise",
            Self::Service => "service",
        }
    }
}

impl std::str::FromStr for ProductType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "merchandise" => Ok(Self::Merchandise),
            "service" => Ok(Self::Service),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReturnStatus {
    MerchantRejected,
    Successful,
}

impl ReturnStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MerchantRejected => "merchant_rejected",
            Self::Successful => "successful",
        }
    }
}

impl std::str::FromStr for ReturnStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "merchant_rejected" => Ok(Self::MerchantRejected),
            "successful" => Ok(Self::Successful),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ReturnStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ReturnStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    ///
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Reason {
    Canceled,
    Duplicate,
    Fraudulent,
    MerchandiseNotAsDescribed,
    NotReceived,
    Other,
    ServiceNotAsDescribed,
}

impl Reason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::MerchandiseNotAsDescribed => "merchandise_not_as_described",
            Self::NotReceived => "not_received",
            Self::Other => "other",
            Self::ServiceNotAsDescribed => "service_not_as_described",
        }
    }
}

impl std::str::FromStr for Reason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "canceled" => Ok(Self::Canceled),
            "duplicate" => Ok(Self::Duplicate),
            "fraudulent" => Ok(Self::Fraudulent),
            "merchandise_not_as_described" => Ok(Self::MerchandiseNotAsDescribed),
            "not_received" => Ok(Self::NotReceived),
            "other" => Ok(Self::Other),
            "service_not_as_described" => Ok(Self::ServiceNotAsDescribed),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for Reason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Reason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct Canceled<'a> {
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
    pub product_type: Option<ProductType>,
    /// Result of cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_status: Option<ReturnStatus>,
    /// Date when the product was returned or attempted to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<stripe_types::Timestamp>,
}
impl<'a> Canceled<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct MerchandiseNotAsDescribed<'a> {
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
    pub return_status: Option<ReturnStatus>,
    /// Date when the product was returned or attempted to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<stripe_types::Timestamp>,
}
impl<'a> MerchandiseNotAsDescribed<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct NotReceived<'a> {
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
    pub product_type: Option<ProductType>,
}
impl<'a> NotReceived<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct Other<'a> {
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
    pub product_type: Option<ProductType>,
}
impl<'a> Other<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct EvidenceParam<'a> {
    /// Evidence provided when `reason` is 'canceled'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<Canceled<'a>>,
    /// Evidence provided when `reason` is 'duplicate'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<Duplicate<'a>>,
    /// Evidence provided when `reason` is 'fraudulent'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<Fraudulent<'a>>,
    /// Evidence provided when `reason` is 'merchandise_not_as_described'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_not_as_described: Option<MerchandiseNotAsDescribed<'a>>,
    /// Evidence provided when `reason` is 'not_received'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_received: Option<NotReceived<'a>>,
    /// Evidence provided when `reason` is 'other'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Other<'a>>,
    /// The reason for filing the dispute.
    ///
    /// The evidence should be submitted in the field of the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
    /// Evidence provided when `reason` is 'service_not_as_described'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_not_as_described: Option<ServiceNotAsDescribed<'a>>,
}
impl<'a> EvidenceParam<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
