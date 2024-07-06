use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes a `Reader` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteTerminalReader<'a> {
    reader: &'a stripe_terminal::TerminalReaderId,
}
impl<'a> DeleteTerminalReader<'a> {
    /// Construct a new `DeleteTerminalReader`.
    pub fn new(reader: &'a stripe_terminal::TerminalReaderId) -> Self {
        Self { reader }
    }
}
impl DeleteTerminalReader<'_> {
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

impl StripeRequest for DeleteTerminalReader<'_> {
    type Output = stripe_terminal::DeletedTerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = self.reader;
        RequestBuilder::new(StripeMethod::Delete, format!("/terminal/readers/{reader}"))
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListTerminalReaderBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type: Option<stripe_terminal::TerminalReaderDeviceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serial_number: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_terminal::TerminalReaderStatus>,
}
impl<'a> ListTerminalReaderBuilder<'a> {
    fn new() -> Self {
        Self {
            device_type: None,
            ending_before: None,
            expand: None,
            limit: None,
            location: None,
            serial_number: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Returns a list of `Reader` objects.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTerminalReader<'a> {
    inner: ListTerminalReaderBuilder<'a>,
}
impl<'a> ListTerminalReader<'a> {
    /// Construct a new `ListTerminalReader`.
    pub fn new() -> Self {
        Self { inner: ListTerminalReaderBuilder::new() }
    }
    /// Filters readers by device type
    pub fn device_type(mut self, device_type: stripe_terminal::TerminalReaderDeviceType) -> Self {
        self.inner.device_type = Some(device_type);
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A location ID to filter the response list to only readers at the specific location
    pub fn location(mut self, location: &'a str) -> Self {
        self.inner.location = Some(location);
        self
    }
    /// Filters readers by serial number
    pub fn serial_number(mut self, serial_number: &'a str) -> Self {
        self.inner.serial_number = Some(serial_number);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
    /// A status filter to filter readers to only offline or online readers
    pub fn status(mut self, status: stripe_terminal::TerminalReaderStatus) -> Self {
        self.inner.status = Some(status);
        self
    }
}
impl<'a> Default for ListTerminalReader<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTerminalReader<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_terminal::TerminalReader>>
    {
        stripe_client_core::ListPaginator::new_list("/terminal/readers", self.inner)
    }
}

impl StripeRequest for ListTerminalReader<'_> {
    type Output = stripe_types::List<stripe_terminal::TerminalReader>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/terminal/readers").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveTerminalReaderBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTerminalReaderBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a `Reader` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTerminalReader<'a> {
    inner: RetrieveTerminalReaderBuilder<'a>,
    reader: &'a stripe_terminal::TerminalReaderId,
}
impl<'a> RetrieveTerminalReader<'a> {
    /// Construct a new `RetrieveTerminalReader`.
    pub fn new(reader: &'a stripe_terminal::TerminalReaderId) -> Self {
        Self { reader, inner: RetrieveTerminalReaderBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveTerminalReader<'_> {
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

impl StripeRequest for RetrieveTerminalReader<'_> {
    type Output = RetrieveTerminalReaderReturned;

    fn build(&self) -> RequestBuilder {
        let reader = self.reader;
        RequestBuilder::new(StripeMethod::Get, format!("/terminal/readers/{reader}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(untagged))]
pub enum RetrieveTerminalReaderReturned {
    TerminalReader(stripe_terminal::TerminalReader),
    DeletedTerminalReader(stripe_terminal::DeletedTerminalReader),
}

#[derive(Default)]
pub struct RetrieveTerminalReaderReturnedBuilder {
    inner: stripe_types::miniserde_helpers::MaybeDeletedBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<RetrieveTerminalReaderReturned>,
        builder: RetrieveTerminalReaderReturnedBuilder,
    }

    impl Deserialize for RetrieveTerminalReaderReturned {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<RetrieveTerminalReaderReturned> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for RetrieveTerminalReaderReturnedBuilder {
        type Out = RetrieveTerminalReaderReturned;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (deleted, o) = self.inner.finish_inner()?;
            Some(if deleted {
                RetrieveTerminalReaderReturned::DeletedTerminalReader(FromValueOpt::from_value(
                    Value::Object(o),
                )?)
            } else {
                RetrieveTerminalReaderReturned::TerminalReader(FromValueOpt::from_value(
                    Value::Object(o),
                )?)
            })
        }
    }

    impl stripe_types::ObjectDeser for RetrieveTerminalReaderReturned {
        type Builder = RetrieveTerminalReaderReturnedBuilder;
    }
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateTerminalReaderBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    registration_code: &'a str,
}
impl<'a> CreateTerminalReaderBuilder<'a> {
    fn new(registration_code: &'a str) -> Self {
        Self { expand: None, label: None, location: None, metadata: None, registration_code }
    }
}
/// Creates a new `Reader` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTerminalReader<'a> {
    inner: CreateTerminalReaderBuilder<'a>,
}
impl<'a> CreateTerminalReader<'a> {
    /// Construct a new `CreateTerminalReader`.
    pub fn new(registration_code: &'a str) -> Self {
        Self { inner: CreateTerminalReaderBuilder::new(registration_code) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Custom label given to the reader for easier identification.
    /// If no label is specified, the registration code will be used.
    pub fn label(mut self, label: &'a str) -> Self {
        self.inner.label = Some(label);
        self
    }
    /// The location to assign the reader to.
    pub fn location(mut self, location: &'a str) -> Self {
        self.inner.location = Some(location);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
}
impl CreateTerminalReader<'_> {
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

impl StripeRequest for CreateTerminalReader<'_> {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/terminal/readers").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateTerminalReaderBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateTerminalReaderBuilder<'a> {
    fn new() -> Self {
        Self { expand: None, label: None, metadata: None }
    }
}
/// Updates a `Reader` object by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalReader<'a> {
    inner: UpdateTerminalReaderBuilder<'a>,
    reader: &'a stripe_terminal::TerminalReaderId,
}
impl<'a> UpdateTerminalReader<'a> {
    /// Construct a new `UpdateTerminalReader`.
    pub fn new(reader: &'a stripe_terminal::TerminalReaderId) -> Self {
        Self { reader, inner: UpdateTerminalReaderBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The new label of the reader.
    pub fn label(mut self, label: &'a str) -> Self {
        self.inner.label = Some(label);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
}
impl UpdateTerminalReader<'_> {
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

impl StripeRequest for UpdateTerminalReader<'_> {
    type Output = UpdateTerminalReaderReturned;

    fn build(&self) -> RequestBuilder {
        let reader = self.reader;
        RequestBuilder::new(StripeMethod::Post, format!("/terminal/readers/{reader}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(untagged))]
pub enum UpdateTerminalReaderReturned {
    TerminalReader(stripe_terminal::TerminalReader),
    DeletedTerminalReader(stripe_terminal::DeletedTerminalReader),
}

#[derive(Default)]
pub struct UpdateTerminalReaderReturnedBuilder {
    inner: stripe_types::miniserde_helpers::MaybeDeletedBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<UpdateTerminalReaderReturned>,
        builder: UpdateTerminalReaderReturnedBuilder,
    }

    impl Deserialize for UpdateTerminalReaderReturned {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<UpdateTerminalReaderReturned> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for UpdateTerminalReaderReturnedBuilder {
        type Out = UpdateTerminalReaderReturned;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (deleted, o) = self.inner.finish_inner()?;
            Some(if deleted {
                UpdateTerminalReaderReturned::DeletedTerminalReader(FromValueOpt::from_value(
                    Value::Object(o),
                )?)
            } else {
                UpdateTerminalReaderReturned::TerminalReader(FromValueOpt::from_value(
                    Value::Object(o),
                )?)
            })
        }
    }

    impl stripe_types::ObjectDeser for UpdateTerminalReaderReturned {
        type Builder = UpdateTerminalReaderReturnedBuilder;
    }
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CancelActionTerminalReaderBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> CancelActionTerminalReaderBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Cancels the current reader action.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CancelActionTerminalReader<'a> {
    inner: CancelActionTerminalReaderBuilder<'a>,
    reader: &'a stripe_terminal::TerminalReaderId,
}
impl<'a> CancelActionTerminalReader<'a> {
    /// Construct a new `CancelActionTerminalReader`.
    pub fn new(reader: &'a stripe_terminal::TerminalReaderId) -> Self {
        Self { reader, inner: CancelActionTerminalReaderBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl CancelActionTerminalReader<'_> {
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

impl StripeRequest for CancelActionTerminalReader<'_> {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = self.reader;
        RequestBuilder::new(StripeMethod::Post, format!("/terminal/readers/{reader}/cancel_action"))
            .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ProcessPaymentIntentTerminalReaderBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    payment_intent: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    process_config: Option<ProcessPaymentIntentTerminalReaderProcessConfig>,
}
impl<'a> ProcessPaymentIntentTerminalReaderBuilder<'a> {
    fn new(payment_intent: &'a str) -> Self {
        Self { expand: None, payment_intent, process_config: None }
    }
}
/// Configuration overrides
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ProcessPaymentIntentTerminalReaderProcessConfig {
    /// Enables cancel button on transaction screens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_customer_cancellation: Option<bool>,
    /// Override showing a tipping selection screen on this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_tipping: Option<bool>,
    /// Tipping configuration for this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<ProcessPaymentIntentTerminalReaderProcessConfigTipping>,
}
impl ProcessPaymentIntentTerminalReaderProcessConfig {
    pub fn new() -> Self {
        Self { enable_customer_cancellation: None, skip_tipping: None, tipping: None }
    }
}
impl Default for ProcessPaymentIntentTerminalReaderProcessConfig {
    fn default() -> Self {
        Self::new()
    }
}
/// Tipping configuration for this transaction.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ProcessPaymentIntentTerminalReaderProcessConfigTipping {
    /// Amount used to calculate tip suggestions on tipping selection screen for this transaction.
    /// Must be a positive integer in the smallest currency unit (e.g., 100 cents to represent $1.00 or 100 to represent ¥100, a zero-decimal currency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eligible: Option<i64>,
}
impl ProcessPaymentIntentTerminalReaderProcessConfigTipping {
    pub fn new() -> Self {
        Self { amount_eligible: None }
    }
}
impl Default for ProcessPaymentIntentTerminalReaderProcessConfigTipping {
    fn default() -> Self {
        Self::new()
    }
}
/// Initiates a payment flow on a Reader.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ProcessPaymentIntentTerminalReader<'a> {
    inner: ProcessPaymentIntentTerminalReaderBuilder<'a>,
    reader: &'a stripe_terminal::TerminalReaderId,
}
impl<'a> ProcessPaymentIntentTerminalReader<'a> {
    /// Construct a new `ProcessPaymentIntentTerminalReader`.
    pub fn new(reader: &'a stripe_terminal::TerminalReaderId, payment_intent: &'a str) -> Self {
        Self { reader, inner: ProcessPaymentIntentTerminalReaderBuilder::new(payment_intent) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Configuration overrides
    pub fn process_config(
        mut self,
        process_config: ProcessPaymentIntentTerminalReaderProcessConfig,
    ) -> Self {
        self.inner.process_config = Some(process_config);
        self
    }
}
impl ProcessPaymentIntentTerminalReader<'_> {
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

impl StripeRequest for ProcessPaymentIntentTerminalReader<'_> {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = self.reader;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/terminal/readers/{reader}/process_payment_intent"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ProcessSetupIntentTerminalReaderBuilder<'a> {
    customer_consent_collected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    process_config: Option<ProcessSetupIntentTerminalReaderProcessConfig>,
    setup_intent: &'a str,
}
impl<'a> ProcessSetupIntentTerminalReaderBuilder<'a> {
    fn new(customer_consent_collected: bool, setup_intent: &'a str) -> Self {
        Self { customer_consent_collected, expand: None, process_config: None, setup_intent }
    }
}
/// Configuration overrides
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ProcessSetupIntentTerminalReaderProcessConfig {
    /// Enables cancel button on transaction screens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_customer_cancellation: Option<bool>,
}
impl ProcessSetupIntentTerminalReaderProcessConfig {
    pub fn new() -> Self {
        Self { enable_customer_cancellation: None }
    }
}
impl Default for ProcessSetupIntentTerminalReaderProcessConfig {
    fn default() -> Self {
        Self::new()
    }
}
/// Initiates a setup intent flow on a Reader.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ProcessSetupIntentTerminalReader<'a> {
    inner: ProcessSetupIntentTerminalReaderBuilder<'a>,
    reader: &'a stripe_terminal::TerminalReaderId,
}
impl<'a> ProcessSetupIntentTerminalReader<'a> {
    /// Construct a new `ProcessSetupIntentTerminalReader`.
    pub fn new(
        reader: &'a stripe_terminal::TerminalReaderId,
        customer_consent_collected: bool,
        setup_intent: &'a str,
    ) -> Self {
        Self {
            reader,
            inner: ProcessSetupIntentTerminalReaderBuilder::new(
                customer_consent_collected,
                setup_intent,
            ),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Configuration overrides
    pub fn process_config(
        mut self,
        process_config: ProcessSetupIntentTerminalReaderProcessConfig,
    ) -> Self {
        self.inner.process_config = Some(process_config);
        self
    }
}
impl ProcessSetupIntentTerminalReader<'_> {
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

impl StripeRequest for ProcessSetupIntentTerminalReader<'_> {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = self.reader;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/terminal/readers/{reader}/process_setup_intent"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RefundPaymentTerminalReaderBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_application_fee: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_payment_config: Option<RefundPaymentTerminalReaderRefundPaymentConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reverse_transfer: Option<bool>,
}
impl<'a> RefundPaymentTerminalReaderBuilder<'a> {
    fn new() -> Self {
        Self {
            amount: None,
            charge: None,
            expand: None,
            metadata: None,
            payment_intent: None,
            refund_application_fee: None,
            refund_payment_config: None,
            reverse_transfer: None,
        }
    }
}
/// Configuration overrides
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RefundPaymentTerminalReaderRefundPaymentConfig {
    /// Enables cancel button on transaction screens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_customer_cancellation: Option<bool>,
}
impl RefundPaymentTerminalReaderRefundPaymentConfig {
    pub fn new() -> Self {
        Self { enable_customer_cancellation: None }
    }
}
impl Default for RefundPaymentTerminalReaderRefundPaymentConfig {
    fn default() -> Self {
        Self::new()
    }
}
/// Initiates a refund on a Reader
#[derive(Clone, Debug, serde::Serialize)]
pub struct RefundPaymentTerminalReader<'a> {
    inner: RefundPaymentTerminalReaderBuilder<'a>,
    reader: &'a stripe_terminal::TerminalReaderId,
}
impl<'a> RefundPaymentTerminalReader<'a> {
    /// Construct a new `RefundPaymentTerminalReader`.
    pub fn new(reader: &'a stripe_terminal::TerminalReaderId) -> Self {
        Self { reader, inner: RefundPaymentTerminalReaderBuilder::new() }
    }
    /// A positive integer in __cents__ representing how much of this charge to refund.
    pub fn amount(mut self, amount: i64) -> Self {
        self.inner.amount = Some(amount);
        self
    }
    /// ID of the Charge to refund.
    pub fn charge(mut self, charge: &'a str) -> Self {
        self.inner.charge = Some(charge);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// ID of the PaymentIntent to refund.
    pub fn payment_intent(mut self, payment_intent: &'a str) -> Self {
        self.inner.payment_intent = Some(payment_intent);
        self
    }
    /// Boolean indicating whether the application fee should be refunded when refunding this charge.
    /// If a full charge refund is given, the full application fee will be refunded.
    /// Otherwise, the application fee will be refunded in an amount proportional to the amount of the charge refunded.
    /// An application fee can be refunded only by the application that created the charge.
    pub fn refund_application_fee(mut self, refund_application_fee: bool) -> Self {
        self.inner.refund_application_fee = Some(refund_application_fee);
        self
    }
    /// Configuration overrides
    pub fn refund_payment_config(
        mut self,
        refund_payment_config: RefundPaymentTerminalReaderRefundPaymentConfig,
    ) -> Self {
        self.inner.refund_payment_config = Some(refund_payment_config);
        self
    }
    /// Boolean indicating whether the transfer should be reversed when refunding this charge.
    /// The transfer will be reversed proportionally to the amount being refunded (either the entire or partial amount).
    /// A transfer can be reversed only by the application that created the charge.
    pub fn reverse_transfer(mut self, reverse_transfer: bool) -> Self {
        self.inner.reverse_transfer = Some(reverse_transfer);
        self
    }
}
impl RefundPaymentTerminalReader<'_> {
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

impl StripeRequest for RefundPaymentTerminalReader<'_> {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = self.reader;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/terminal/readers/{reader}/refund_payment"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct SetReaderDisplayTerminalReaderBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    cart: Option<SetReaderDisplayTerminalReaderCart<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(rename = "type")]
    type_: SetReaderDisplayTerminalReaderType,
}
impl<'a> SetReaderDisplayTerminalReaderBuilder<'a> {
    fn new(type_: SetReaderDisplayTerminalReaderType) -> Self {
        Self { cart: None, expand: None, type_ }
    }
}
/// Cart
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SetReaderDisplayTerminalReaderCart<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Array of line items that were purchased.
    pub line_items: &'a [SetReaderDisplayTerminalReaderCartLineItems<'a>],
    /// The amount of tax in cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<i64>,
    /// Total balance of cart due in cents.
    pub total: i64,
}
impl<'a> SetReaderDisplayTerminalReaderCart<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        line_items: &'a [SetReaderDisplayTerminalReaderCartLineItems<'a>],
        total: i64,
    ) -> Self {
        Self { currency, line_items, tax: None, total }
    }
}
/// Array of line items that were purchased.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SetReaderDisplayTerminalReaderCartLineItems<'a> {
    /// The price of the item in cents.
    pub amount: i64,
    /// The description or name of the item.
    pub description: &'a str,
    /// The quantity of the line item being purchased.
    pub quantity: u64,
}
impl<'a> SetReaderDisplayTerminalReaderCartLineItems<'a> {
    pub fn new(amount: i64, description: &'a str, quantity: u64) -> Self {
        Self { amount, description, quantity }
    }
}
/// Type
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetReaderDisplayTerminalReaderType {
    Cart,
}
impl SetReaderDisplayTerminalReaderType {
    pub fn as_str(self) -> &'static str {
        use SetReaderDisplayTerminalReaderType::*;
        match self {
            Cart => "cart",
        }
    }
}

impl std::str::FromStr for SetReaderDisplayTerminalReaderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetReaderDisplayTerminalReaderType::*;
        match s {
            "cart" => Ok(Cart),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SetReaderDisplayTerminalReaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetReaderDisplayTerminalReaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetReaderDisplayTerminalReaderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SetReaderDisplayTerminalReaderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SetReaderDisplayTerminalReaderType")
        })
    }
}
/// Sets reader display to show cart details.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SetReaderDisplayTerminalReader<'a> {
    inner: SetReaderDisplayTerminalReaderBuilder<'a>,
    reader: &'a stripe_terminal::TerminalReaderId,
}
impl<'a> SetReaderDisplayTerminalReader<'a> {
    /// Construct a new `SetReaderDisplayTerminalReader`.
    pub fn new(
        reader: &'a stripe_terminal::TerminalReaderId,
        type_: SetReaderDisplayTerminalReaderType,
    ) -> Self {
        Self { reader, inner: SetReaderDisplayTerminalReaderBuilder::new(type_) }
    }
    /// Cart
    pub fn cart(mut self, cart: SetReaderDisplayTerminalReaderCart<'a>) -> Self {
        self.inner.cart = Some(cart);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl SetReaderDisplayTerminalReader<'_> {
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

impl StripeRequest for SetReaderDisplayTerminalReader<'_> {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = self.reader;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/terminal/readers/{reader}/set_reader_display"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct PresentPaymentMethodTerminalReaderBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_tip: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_present: Option<PresentPaymentMethodTerminalReaderCardPresent<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interac_present: Option<PresentPaymentMethodTerminalReaderInteracPresent<'a>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<PresentPaymentMethodTerminalReaderType>,
}
impl<'a> PresentPaymentMethodTerminalReaderBuilder<'a> {
    fn new() -> Self {
        Self {
            amount_tip: None,
            card_present: None,
            expand: None,
            interac_present: None,
            type_: None,
        }
    }
}
/// Simulated data for the card_present payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PresentPaymentMethodTerminalReaderCardPresent<'a> {
    /// The card number, as a string without any separators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<&'a str>,
}
impl<'a> PresentPaymentMethodTerminalReaderCardPresent<'a> {
    pub fn new() -> Self {
        Self { number: None }
    }
}
impl<'a> Default for PresentPaymentMethodTerminalReaderCardPresent<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Simulated data for the interac_present payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PresentPaymentMethodTerminalReaderInteracPresent<'a> {
    /// Card Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<&'a str>,
}
impl<'a> PresentPaymentMethodTerminalReaderInteracPresent<'a> {
    pub fn new() -> Self {
        Self { number: None }
    }
}
impl<'a> Default for PresentPaymentMethodTerminalReaderInteracPresent<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Simulated payment type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PresentPaymentMethodTerminalReaderType {
    CardPresent,
    InteracPresent,
}
impl PresentPaymentMethodTerminalReaderType {
    pub fn as_str(self) -> &'static str {
        use PresentPaymentMethodTerminalReaderType::*;
        match self {
            CardPresent => "card_present",
            InteracPresent => "interac_present",
        }
    }
}

impl std::str::FromStr for PresentPaymentMethodTerminalReaderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PresentPaymentMethodTerminalReaderType::*;
        match s {
            "card_present" => Ok(CardPresent),
            "interac_present" => Ok(InteracPresent),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PresentPaymentMethodTerminalReaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PresentPaymentMethodTerminalReaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PresentPaymentMethodTerminalReaderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PresentPaymentMethodTerminalReaderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PresentPaymentMethodTerminalReaderType")
        })
    }
}
/// Presents a payment method on a simulated reader.
/// Can be used to simulate accepting a payment, saving a card or refunding a transaction.
#[derive(Clone, Debug, serde::Serialize)]
pub struct PresentPaymentMethodTerminalReader<'a> {
    inner: PresentPaymentMethodTerminalReaderBuilder<'a>,
    reader: &'a str,
}
impl<'a> PresentPaymentMethodTerminalReader<'a> {
    /// Construct a new `PresentPaymentMethodTerminalReader`.
    pub fn new(reader: &'a str) -> Self {
        Self { reader, inner: PresentPaymentMethodTerminalReaderBuilder::new() }
    }
    /// Simulated on-reader tip amount.
    pub fn amount_tip(mut self, amount_tip: i64) -> Self {
        self.inner.amount_tip = Some(amount_tip);
        self
    }
    /// Simulated data for the card_present payment method.
    pub fn card_present(
        mut self,
        card_present: PresentPaymentMethodTerminalReaderCardPresent<'a>,
    ) -> Self {
        self.inner.card_present = Some(card_present);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Simulated data for the interac_present payment method.
    pub fn interac_present(
        mut self,
        interac_present: PresentPaymentMethodTerminalReaderInteracPresent<'a>,
    ) -> Self {
        self.inner.interac_present = Some(interac_present);
        self
    }
    /// Simulated payment type.
    pub fn type_(mut self, type_: PresentPaymentMethodTerminalReaderType) -> Self {
        self.inner.type_ = Some(type_);
        self
    }
}
impl PresentPaymentMethodTerminalReader<'_> {
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

impl StripeRequest for PresentPaymentMethodTerminalReader<'_> {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = self.reader;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/terminal/readers/{reader}/present_payment_method"),
        )
        .form(&self.inner)
    }
}
