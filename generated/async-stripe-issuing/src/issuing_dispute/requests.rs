use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListIssuingDisputeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::IssuingDisputeStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transaction: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListIssuingDisputeBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListIssuingDisputeBuilder").finish_non_exhaustive()
    }
}
impl ListIssuingDisputeBuilder {
    fn new() -> Self {
        Self {
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
            transaction: None,
        }
    }
}
/// Returns a list of Issuing `Dispute` objects.
/// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListIssuingDispute {
    inner: ListIssuingDisputeBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListIssuingDispute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListIssuingDispute").finish_non_exhaustive()
    }
}
impl ListIssuingDispute {
    /// Construct a new `ListIssuingDispute`.
    pub fn new() -> Self {
        Self { inner: ListIssuingDisputeBuilder::new() }
    }
    /// Only return Issuing disputes that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Select Issuing disputes with the given status.
    pub fn status(mut self, status: impl Into<stripe_shared::IssuingDisputeStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
    /// Select the Issuing dispute for the given transaction.
    pub fn transaction(mut self, transaction: impl Into<String>) -> Self {
        self.inner.transaction = Some(transaction.into());
        self
    }
}
impl Default for ListIssuingDispute {
    fn default() -> Self {
        Self::new()
    }
}
impl ListIssuingDispute {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::IssuingDispute>> {
        stripe_client_core::ListPaginator::new_list("/issuing/disputes", &self.inner)
    }
}

impl StripeRequest for ListIssuingDispute {
    type Output = stripe_types::List<stripe_shared::IssuingDispute>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/issuing/disputes").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveIssuingDisputeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveIssuingDisputeBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveIssuingDisputeBuilder").finish_non_exhaustive()
    }
}
impl RetrieveIssuingDisputeBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves an Issuing `Dispute` object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveIssuingDispute {
    inner: RetrieveIssuingDisputeBuilder,
    dispute: stripe_shared::IssuingDisputeId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveIssuingDispute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveIssuingDispute").finish_non_exhaustive()
    }
}
impl RetrieveIssuingDispute {
    /// Construct a new `RetrieveIssuingDispute`.
    pub fn new(dispute: impl Into<stripe_shared::IssuingDisputeId>) -> Self {
        Self { dispute: dispute.into(), inner: RetrieveIssuingDisputeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveIssuingDispute {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveIssuingDispute {
    type Output = stripe_shared::IssuingDispute;

    fn build(&self) -> RequestBuilder {
        let dispute = &self.dispute;
        RequestBuilder::new(StripeMethod::Get, format!("/issuing/disputes/{dispute}"))
            .query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateIssuingDisputeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence: Option<CreateIssuingDisputeEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transaction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    treasury: Option<CreateIssuingDisputeTreasury>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingDisputeBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingDisputeBuilder").finish_non_exhaustive()
    }
}
impl CreateIssuingDisputeBuilder {
    fn new() -> Self {
        Self {
            amount: None,
            evidence: None,
            expand: None,
            metadata: None,
            transaction: None,
            treasury: None,
        }
    }
}
/// Evidence provided for the dispute.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingDisputeEvidence {
    /// Evidence provided when `reason` is 'canceled'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<Canceled>,
    /// Evidence provided when `reason` is 'duplicate'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<Duplicate>,
    /// Evidence provided when `reason` is 'fraudulent'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<CreateIssuingDisputeEvidenceFraudulent>,
    /// Evidence provided when `reason` is 'merchandise_not_as_described'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_not_as_described: Option<MerchandiseNotAsDescribed>,
    /// Evidence provided when `reason` is 'no_valid_authorization'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_valid_authorization: Option<CreateIssuingDisputeEvidenceNoValidAuthorization>,
    /// Evidence provided when `reason` is 'not_received'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_received: Option<NotReceived>,
    /// Evidence provided when `reason` is 'other'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Other>,
    /// The reason for filing the dispute. The evidence should be submitted in the field of the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<stripe_shared::IssuingDisputeEvidenceReason>,
    /// Evidence provided when `reason` is 'service_not_as_described'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_not_as_described: Option<ServiceNotAsDescribed>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingDisputeEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingDisputeEvidence").finish_non_exhaustive()
    }
}
impl CreateIssuingDisputeEvidence {
    pub fn new() -> Self {
        Self {
            canceled: None,
            duplicate: None,
            fraudulent: None,
            merchandise_not_as_described: None,
            no_valid_authorization: None,
            not_received: None,
            other: None,
            reason: None,
            service_not_as_described: None,
        }
    }
}
impl Default for CreateIssuingDisputeEvidence {
    fn default() -> Self {
        Self::new()
    }
}
/// Evidence provided when `reason` is 'fraudulent'.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingDisputeEvidenceFraudulent {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingDisputeEvidenceFraudulent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingDisputeEvidenceFraudulent").finish_non_exhaustive()
    }
}
impl CreateIssuingDisputeEvidenceFraudulent {
    pub fn new() -> Self {
        Self { additional_documentation: None, explanation: None }
    }
}
impl Default for CreateIssuingDisputeEvidenceFraudulent {
    fn default() -> Self {
        Self::new()
    }
}
/// Evidence provided when `reason` is 'no_valid_authorization'.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingDisputeEvidenceNoValidAuthorization {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingDisputeEvidenceNoValidAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingDisputeEvidenceNoValidAuthorization").finish_non_exhaustive()
    }
}
impl CreateIssuingDisputeEvidenceNoValidAuthorization {
    pub fn new() -> Self {
        Self { additional_documentation: None, explanation: None }
    }
}
impl Default for CreateIssuingDisputeEvidenceNoValidAuthorization {
    fn default() -> Self {
        Self::new()
    }
}
/// Params for disputes related to Treasury FinancialAccounts
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingDisputeTreasury {
    /// The ID of the ReceivedDebit to initiate an Issuings dispute for.
    pub received_debit: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingDisputeTreasury {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingDisputeTreasury").finish_non_exhaustive()
    }
}
impl CreateIssuingDisputeTreasury {
    pub fn new(received_debit: impl Into<String>) -> Self {
        Self { received_debit: received_debit.into() }
    }
}
/// Creates an Issuing `Dispute` object.
/// Individual pieces of evidence within the `evidence` object are optional at this point.
/// Stripe only validates that required evidence is present during submission.
/// Refer to [Dispute reasons and evidence](https://stripe.com/docs/issuing/purchases/disputes#dispute-reasons-and-evidence) for more details about evidence requirements.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingDispute {
    inner: CreateIssuingDisputeBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingDispute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingDispute").finish_non_exhaustive()
    }
}
impl CreateIssuingDispute {
    /// Construct a new `CreateIssuingDispute`.
    pub fn new() -> Self {
        Self { inner: CreateIssuingDisputeBuilder::new() }
    }
    /// The dispute amount in the card's currency and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    /// If not set, defaults to the full transaction amount.
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// Evidence provided for the dispute.
    pub fn evidence(mut self, evidence: impl Into<CreateIssuingDisputeEvidence>) -> Self {
        self.inner.evidence = Some(evidence.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The ID of the issuing transaction to create a dispute for.
    /// For transaction on Treasury FinancialAccounts, use `treasury.received_debit`.
    pub fn transaction(mut self, transaction: impl Into<String>) -> Self {
        self.inner.transaction = Some(transaction.into());
        self
    }
    /// Params for disputes related to Treasury FinancialAccounts
    pub fn treasury(mut self, treasury: impl Into<CreateIssuingDisputeTreasury>) -> Self {
        self.inner.treasury = Some(treasury.into());
        self
    }
}
impl Default for CreateIssuingDispute {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateIssuingDispute {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateIssuingDispute {
    type Output = stripe_shared::IssuingDispute;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/issuing/disputes").form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateIssuingDisputeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence: Option<UpdateIssuingDisputeEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIssuingDisputeBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateIssuingDisputeBuilder").finish_non_exhaustive()
    }
}
impl UpdateIssuingDisputeBuilder {
    fn new() -> Self {
        Self { amount: None, evidence: None, expand: None, metadata: None }
    }
}
/// Evidence provided for the dispute.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateIssuingDisputeEvidence {
    /// Evidence provided when `reason` is 'canceled'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<Canceled>,
    /// Evidence provided when `reason` is 'duplicate'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<Duplicate>,
    /// Evidence provided when `reason` is 'fraudulent'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<UpdateIssuingDisputeEvidenceFraudulent>,
    /// Evidence provided when `reason` is 'merchandise_not_as_described'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_not_as_described: Option<MerchandiseNotAsDescribed>,
    /// Evidence provided when `reason` is 'no_valid_authorization'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_valid_authorization: Option<UpdateIssuingDisputeEvidenceNoValidAuthorization>,
    /// Evidence provided when `reason` is 'not_received'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_received: Option<NotReceived>,
    /// Evidence provided when `reason` is 'other'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Other>,
    /// The reason for filing the dispute. The evidence should be submitted in the field of the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<stripe_shared::IssuingDisputeEvidenceReason>,
    /// Evidence provided when `reason` is 'service_not_as_described'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_not_as_described: Option<ServiceNotAsDescribed>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIssuingDisputeEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateIssuingDisputeEvidence").finish_non_exhaustive()
    }
}
impl UpdateIssuingDisputeEvidence {
    pub fn new() -> Self {
        Self {
            canceled: None,
            duplicate: None,
            fraudulent: None,
            merchandise_not_as_described: None,
            no_valid_authorization: None,
            not_received: None,
            other: None,
            reason: None,
            service_not_as_described: None,
        }
    }
}
impl Default for UpdateIssuingDisputeEvidence {
    fn default() -> Self {
        Self::new()
    }
}
/// Evidence provided when `reason` is 'fraudulent'.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateIssuingDisputeEvidenceFraudulent {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIssuingDisputeEvidenceFraudulent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateIssuingDisputeEvidenceFraudulent").finish_non_exhaustive()
    }
}
impl UpdateIssuingDisputeEvidenceFraudulent {
    pub fn new() -> Self {
        Self { additional_documentation: None, explanation: None }
    }
}
impl Default for UpdateIssuingDisputeEvidenceFraudulent {
    fn default() -> Self {
        Self::new()
    }
}
/// Evidence provided when `reason` is 'no_valid_authorization'.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateIssuingDisputeEvidenceNoValidAuthorization {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIssuingDisputeEvidenceNoValidAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateIssuingDisputeEvidenceNoValidAuthorization").finish_non_exhaustive()
    }
}
impl UpdateIssuingDisputeEvidenceNoValidAuthorization {
    pub fn new() -> Self {
        Self { additional_documentation: None, explanation: None }
    }
}
impl Default for UpdateIssuingDisputeEvidenceNoValidAuthorization {
    fn default() -> Self {
        Self::new()
    }
}
/// Updates the specified Issuing `Dispute` object by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
/// Properties on the `evidence` object can be unset by passing in an empty string.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateIssuingDispute {
    inner: UpdateIssuingDisputeBuilder,
    dispute: stripe_shared::IssuingDisputeId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIssuingDispute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateIssuingDispute").finish_non_exhaustive()
    }
}
impl UpdateIssuingDispute {
    /// Construct a new `UpdateIssuingDispute`.
    pub fn new(dispute: impl Into<stripe_shared::IssuingDisputeId>) -> Self {
        Self { dispute: dispute.into(), inner: UpdateIssuingDisputeBuilder::new() }
    }
    /// The dispute amount in the card's currency and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// Evidence provided for the dispute.
    pub fn evidence(mut self, evidence: impl Into<UpdateIssuingDisputeEvidence>) -> Self {
        self.inner.evidence = Some(evidence.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl UpdateIssuingDispute {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for UpdateIssuingDispute {
    type Output = stripe_shared::IssuingDispute;

    fn build(&self) -> RequestBuilder {
        let dispute = &self.dispute;
        RequestBuilder::new(StripeMethod::Post, format!("/issuing/disputes/{dispute}"))
            .form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct SubmitIssuingDisputeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubmitIssuingDisputeBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubmitIssuingDisputeBuilder").finish_non_exhaustive()
    }
}
impl SubmitIssuingDisputeBuilder {
    fn new() -> Self {
        Self { expand: None, metadata: None }
    }
}
/// Submits an Issuing `Dispute` to the card network.
/// Stripe validates that all evidence fields required for the dispute’s reason are present.
/// For more details, see [Dispute reasons and evidence](https://stripe.com/docs/issuing/purchases/disputes#dispute-reasons-and-evidence).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SubmitIssuingDispute {
    inner: SubmitIssuingDisputeBuilder,
    dispute: stripe_shared::IssuingDisputeId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubmitIssuingDispute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubmitIssuingDispute").finish_non_exhaustive()
    }
}
impl SubmitIssuingDispute {
    /// Construct a new `SubmitIssuingDispute`.
    pub fn new(dispute: impl Into<stripe_shared::IssuingDisputeId>) -> Self {
        Self { dispute: dispute.into(), inner: SubmitIssuingDisputeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl SubmitIssuingDispute {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for SubmitIssuingDispute {
    type Output = stripe_shared::IssuingDispute;

    fn build(&self) -> RequestBuilder {
        let dispute = &self.dispute;
        RequestBuilder::new(StripeMethod::Post, format!("/issuing/disputes/{dispute}/submit"))
            .form(&self.inner)
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct Canceled {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,
    /// Date when order was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Whether the cardholder was provided with a cancellation policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_provided: Option<bool>,
    /// Reason for canceling the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    /// Date when the cardholder expected to receive the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<stripe_types::Timestamp>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<stripe_shared::IssuingDisputeOtherEvidenceProductType>,
    /// Result of cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_status: Option<stripe_shared::ReturnStatus>,
    /// Date when the product was returned or attempted to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Canceled {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Canceled").finish_non_exhaustive()
    }
}
impl Canceled {
    pub fn new() -> Self {
        Self {
            additional_documentation: None,
            canceled_at: None,
            cancellation_policy_provided: None,
            cancellation_reason: None,
            expected_at: None,
            explanation: None,
            product_description: None,
            product_type: None,
            return_status: None,
            returned_at: None,
        }
    }
}
impl Default for Canceled {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct Duplicate {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the card statement showing that the product had already been paid for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_statement: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the receipt showing that the product had been paid for in cash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_receipt: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Image of the front and back of the check that was used to pay for the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_image: Option<String>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Transaction (e.g., ipi_...) that the disputed transaction is a duplicate of.
    /// Of the two or more transactions that are copies of each other, this is original undisputed one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_transaction: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Duplicate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Duplicate").finish_non_exhaustive()
    }
}
impl Duplicate {
    pub fn new() -> Self {
        Self {
            additional_documentation: None,
            card_statement: None,
            cash_receipt: None,
            check_image: None,
            explanation: None,
            original_transaction: None,
        }
    }
}
impl Default for Duplicate {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct MerchandiseNotAsDescribed {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Date when the product was received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<stripe_types::Timestamp>,
    /// Description of the cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_description: Option<String>,
    /// Result of cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_status: Option<stripe_shared::ReturnStatus>,
    /// Date when the product was returned or attempted to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MerchandiseNotAsDescribed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MerchandiseNotAsDescribed").finish_non_exhaustive()
    }
}
impl MerchandiseNotAsDescribed {
    pub fn new() -> Self {
        Self {
            additional_documentation: None,
            explanation: None,
            received_at: None,
            return_description: None,
            return_status: None,
            returned_at: None,
        }
    }
}
impl Default for MerchandiseNotAsDescribed {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct NotReceived {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,
    /// Date when the cardholder expected to receive the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<stripe_types::Timestamp>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<stripe_shared::IssuingDisputeOtherEvidenceProductType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for NotReceived {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("NotReceived").finish_non_exhaustive()
    }
}
impl NotReceived {
    pub fn new() -> Self {
        Self {
            additional_documentation: None,
            expected_at: None,
            explanation: None,
            product_description: None,
            product_type: None,
        }
    }
}
impl Default for NotReceived {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct Other {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<stripe_shared::IssuingDisputeOtherEvidenceProductType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Other {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Other").finish_non_exhaustive()
    }
}
impl Other {
    pub fn new() -> Self {
        Self {
            additional_documentation: None,
            explanation: None,
            product_description: None,
            product_type: None,
        }
    }
}
impl Default for Other {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ServiceNotAsDescribed {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,
    /// Date when order was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Reason for canceling the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Date when the product was received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ServiceNotAsDescribed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ServiceNotAsDescribed").finish_non_exhaustive()
    }
}
impl ServiceNotAsDescribed {
    pub fn new() -> Self {
        Self {
            additional_documentation: None,
            canceled_at: None,
            cancellation_reason: None,
            explanation: None,
            received_at: None,
        }
    }
}
impl Default for ServiceNotAsDescribed {
    fn default() -> Self {
        Self::new()
    }
}
/// Select Issuing disputes with the given status.
pub type ListIssuingDisputeStatus = stripe_shared::IssuingDisputeStatus;
/// Whether the product was a merchandise or service.
pub type CreateIssuingDisputeEvidenceCanceledProductType =
    stripe_shared::IssuingDisputeOtherEvidenceProductType;
/// Result of cardholder's attempt to return the product.
pub type CreateIssuingDisputeEvidenceCanceledReturnStatus = stripe_shared::ReturnStatus;
/// Result of cardholder's attempt to return the product.
pub type CreateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus =
    stripe_shared::ReturnStatus;
/// Whether the product was a merchandise or service.
pub type CreateIssuingDisputeEvidenceNotReceivedProductType =
    stripe_shared::IssuingDisputeOtherEvidenceProductType;
/// Whether the product was a merchandise or service.
pub type CreateIssuingDisputeEvidenceOtherProductType =
    stripe_shared::IssuingDisputeOtherEvidenceProductType;
/// The reason for filing the dispute. The evidence should be submitted in the field of the same name.
pub type CreateIssuingDisputeEvidenceReason = stripe_shared::IssuingDisputeEvidenceReason;
/// Whether the product was a merchandise or service.
pub type UpdateIssuingDisputeEvidenceCanceledProductType =
    stripe_shared::IssuingDisputeOtherEvidenceProductType;
/// Result of cardholder's attempt to return the product.
pub type UpdateIssuingDisputeEvidenceCanceledReturnStatus = stripe_shared::ReturnStatus;
/// Result of cardholder's attempt to return the product.
pub type UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribedReturnStatus =
    stripe_shared::ReturnStatus;
/// Whether the product was a merchandise or service.
pub type UpdateIssuingDisputeEvidenceNotReceivedProductType =
    stripe_shared::IssuingDisputeOtherEvidenceProductType;
/// Whether the product was a merchandise or service.
pub type UpdateIssuingDisputeEvidenceOtherProductType =
    stripe_shared::IssuingDisputeOtherEvidenceProductType;
/// The reason for filing the dispute. The evidence should be submitted in the field of the same name.
pub type UpdateIssuingDisputeEvidenceReason = stripe_shared::IssuingDisputeEvidenceReason;
/// Evidence provided when `reason` is 'canceled'.
pub type CreateIssuingDisputeEvidenceCanceled = Canceled;
/// Evidence provided when `reason` is 'duplicate'.
pub type CreateIssuingDisputeEvidenceDuplicate = Duplicate;
/// Evidence provided when `reason` is 'merchandise_not_as_described'.
pub type CreateIssuingDisputeEvidenceMerchandiseNotAsDescribed = MerchandiseNotAsDescribed;
/// Evidence provided when `reason` is 'not_received'.
pub type CreateIssuingDisputeEvidenceNotReceived = NotReceived;
/// Evidence provided when `reason` is 'other'.
pub type CreateIssuingDisputeEvidenceOther = Other;
/// Evidence provided when `reason` is 'service_not_as_described'.
pub type CreateIssuingDisputeEvidenceServiceNotAsDescribed = ServiceNotAsDescribed;
/// Evidence provided when `reason` is 'canceled'.
pub type UpdateIssuingDisputeEvidenceCanceled = Canceled;
/// Evidence provided when `reason` is 'duplicate'.
pub type UpdateIssuingDisputeEvidenceDuplicate = Duplicate;
/// Evidence provided when `reason` is 'merchandise_not_as_described'.
pub type UpdateIssuingDisputeEvidenceMerchandiseNotAsDescribed = MerchandiseNotAsDescribed;
/// Evidence provided when `reason` is 'not_received'.
pub type UpdateIssuingDisputeEvidenceNotReceived = NotReceived;
/// Evidence provided when `reason` is 'other'.
pub type UpdateIssuingDisputeEvidenceOther = Other;
/// Evidence provided when `reason` is 'service_not_as_described'.
pub type UpdateIssuingDisputeEvidenceServiceNotAsDescribed = ServiceNotAsDescribed;
