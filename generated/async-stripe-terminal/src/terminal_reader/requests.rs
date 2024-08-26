use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes a `Reader` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteTerminalReader {
    reader: stripe_terminal::TerminalReaderId,
}
impl DeleteTerminalReader {
    /// Construct a new `DeleteTerminalReader`.
    pub fn new(reader: impl Into<stripe_terminal::TerminalReaderId>) -> Self {
        Self { reader: reader.into() }
    }
}
impl DeleteTerminalReader {
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

impl StripeRequest for DeleteTerminalReader {
    type Output = stripe_terminal::DeletedTerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = &self.reader;
        RequestBuilder::new(StripeMethod::Delete, format!("/terminal/readers/{reader}"))
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ListTerminalReaderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type: Option<stripe_terminal::TerminalReaderDeviceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_terminal::TerminalReaderStatus>,
}
impl ListTerminalReaderBuilder {
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
pub struct ListTerminalReader {
    inner: ListTerminalReaderBuilder,
}
impl ListTerminalReader {
    /// Construct a new `ListTerminalReader`.
    pub fn new() -> Self {
        Self { inner: ListTerminalReaderBuilder::new() }
    }
    /// Filters readers by device type
    pub fn device_type(
        mut self,
        device_type: impl Into<stripe_terminal::TerminalReaderDeviceType>,
    ) -> Self {
        self.inner.device_type = Some(device_type.into());
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
    /// A location ID to filter the response list to only readers at the specific location
    pub fn location(mut self, location: impl Into<String>) -> Self {
        self.inner.location = Some(location.into());
        self
    }
    /// Filters readers by serial number
    pub fn serial_number(mut self, serial_number: impl Into<String>) -> Self {
        self.inner.serial_number = Some(serial_number.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// A status filter to filter readers to only offline or online readers
    pub fn status(mut self, status: impl Into<stripe_terminal::TerminalReaderStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl Default for ListTerminalReader {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTerminalReader {
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
        stripe_client_core::ListPaginator::new_list("/terminal/readers", &self.inner)
    }
}

impl StripeRequest for ListTerminalReader {
    type Output = stripe_types::List<stripe_terminal::TerminalReader>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/terminal/readers").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveTerminalReaderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTerminalReaderBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a `Reader` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTerminalReader {
    inner: RetrieveTerminalReaderBuilder,
    reader: stripe_terminal::TerminalReaderId,
}
impl RetrieveTerminalReader {
    /// Construct a new `RetrieveTerminalReader`.
    pub fn new(reader: impl Into<stripe_terminal::TerminalReaderId>) -> Self {
        Self { reader: reader.into(), inner: RetrieveTerminalReaderBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTerminalReader {
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

impl StripeRequest for RetrieveTerminalReader {
    type Output = RetrieveTerminalReaderReturned;

    fn build(&self) -> RequestBuilder {
        let reader = &self.reader;
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

#[derive(Clone, Debug, serde::Serialize)]
struct CreateTerminalReaderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    registration_code: String,
}
impl CreateTerminalReaderBuilder {
    fn new(registration_code: impl Into<String>) -> Self {
        Self {
            expand: None,
            label: None,
            location: None,
            metadata: None,
            registration_code: registration_code.into(),
        }
    }
}
/// Creates a new `Reader` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTerminalReader {
    inner: CreateTerminalReaderBuilder,
}
impl CreateTerminalReader {
    /// Construct a new `CreateTerminalReader`.
    pub fn new(registration_code: impl Into<String>) -> Self {
        Self { inner: CreateTerminalReaderBuilder::new(registration_code.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Custom label given to the reader for easier identification.
    /// If no label is specified, the registration code will be used.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.inner.label = Some(label.into());
        self
    }
    /// The location to assign the reader to.
    pub fn location(mut self, location: impl Into<String>) -> Self {
        self.inner.location = Some(location.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
impl CreateTerminalReader {
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

impl StripeRequest for CreateTerminalReader {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/terminal/readers").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateTerminalReaderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl UpdateTerminalReaderBuilder {
    fn new() -> Self {
        Self { expand: None, label: None, metadata: None }
    }
}
/// Updates a `Reader` object by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalReader {
    inner: UpdateTerminalReaderBuilder,
    reader: stripe_terminal::TerminalReaderId,
}
impl UpdateTerminalReader {
    /// Construct a new `UpdateTerminalReader`.
    pub fn new(reader: impl Into<stripe_terminal::TerminalReaderId>) -> Self {
        Self { reader: reader.into(), inner: UpdateTerminalReaderBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The new label of the reader.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.inner.label = Some(label.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
impl UpdateTerminalReader {
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

impl StripeRequest for UpdateTerminalReader {
    type Output = UpdateTerminalReaderReturned;

    fn build(&self) -> RequestBuilder {
        let reader = &self.reader;
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

#[derive(Clone, Debug, serde::Serialize)]
struct CancelActionTerminalReaderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CancelActionTerminalReaderBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Cancels the current reader action.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CancelActionTerminalReader {
    inner: CancelActionTerminalReaderBuilder,
    reader: stripe_terminal::TerminalReaderId,
}
impl CancelActionTerminalReader {
    /// Construct a new `CancelActionTerminalReader`.
    pub fn new(reader: impl Into<stripe_terminal::TerminalReaderId>) -> Self {
        Self { reader: reader.into(), inner: CancelActionTerminalReaderBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CancelActionTerminalReader {
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

impl StripeRequest for CancelActionTerminalReader {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = &self.reader;
        RequestBuilder::new(StripeMethod::Post, format!("/terminal/readers/{reader}/cancel_action"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ProcessPaymentIntentTerminalReaderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    payment_intent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    process_config: Option<ProcessPaymentIntentTerminalReaderProcessConfig>,
}
impl ProcessPaymentIntentTerminalReaderBuilder {
    fn new(payment_intent: impl Into<String>) -> Self {
        Self { expand: None, payment_intent: payment_intent.into(), process_config: None }
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
pub struct ProcessPaymentIntentTerminalReader {
    inner: ProcessPaymentIntentTerminalReaderBuilder,
    reader: stripe_terminal::TerminalReaderId,
}
impl ProcessPaymentIntentTerminalReader {
    /// Construct a new `ProcessPaymentIntentTerminalReader`.
    pub fn new(
        reader: impl Into<stripe_terminal::TerminalReaderId>,
        payment_intent: impl Into<String>,
    ) -> Self {
        Self {
            reader: reader.into(),
            inner: ProcessPaymentIntentTerminalReaderBuilder::new(payment_intent.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Configuration overrides
    pub fn process_config(
        mut self,
        process_config: impl Into<ProcessPaymentIntentTerminalReaderProcessConfig>,
    ) -> Self {
        self.inner.process_config = Some(process_config.into());
        self
    }
}
impl ProcessPaymentIntentTerminalReader {
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

impl StripeRequest for ProcessPaymentIntentTerminalReader {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = &self.reader;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/terminal/readers/{reader}/process_payment_intent"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ProcessSetupIntentTerminalReaderBuilder {
    customer_consent_collected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    process_config: Option<ProcessSetupIntentTerminalReaderProcessConfig>,
    setup_intent: String,
}
impl ProcessSetupIntentTerminalReaderBuilder {
    fn new(customer_consent_collected: impl Into<bool>, setup_intent: impl Into<String>) -> Self {
        Self {
            customer_consent_collected: customer_consent_collected.into(),
            expand: None,
            process_config: None,
            setup_intent: setup_intent.into(),
        }
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
pub struct ProcessSetupIntentTerminalReader {
    inner: ProcessSetupIntentTerminalReaderBuilder,
    reader: stripe_terminal::TerminalReaderId,
}
impl ProcessSetupIntentTerminalReader {
    /// Construct a new `ProcessSetupIntentTerminalReader`.
    pub fn new(
        reader: impl Into<stripe_terminal::TerminalReaderId>,
        customer_consent_collected: impl Into<bool>,
        setup_intent: impl Into<String>,
    ) -> Self {
        Self {
            reader: reader.into(),
            inner: ProcessSetupIntentTerminalReaderBuilder::new(
                customer_consent_collected.into(),
                setup_intent.into(),
            ),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Configuration overrides
    pub fn process_config(
        mut self,
        process_config: impl Into<ProcessSetupIntentTerminalReaderProcessConfig>,
    ) -> Self {
        self.inner.process_config = Some(process_config.into());
        self
    }
}
impl ProcessSetupIntentTerminalReader {
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

impl StripeRequest for ProcessSetupIntentTerminalReader {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = &self.reader;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/terminal/readers/{reader}/process_setup_intent"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RefundPaymentTerminalReaderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_application_fee: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_payment_config: Option<RefundPaymentTerminalReaderRefundPaymentConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reverse_transfer: Option<bool>,
}
impl RefundPaymentTerminalReaderBuilder {
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
pub struct RefundPaymentTerminalReader {
    inner: RefundPaymentTerminalReaderBuilder,
    reader: stripe_terminal::TerminalReaderId,
}
impl RefundPaymentTerminalReader {
    /// Construct a new `RefundPaymentTerminalReader`.
    pub fn new(reader: impl Into<stripe_terminal::TerminalReaderId>) -> Self {
        Self { reader: reader.into(), inner: RefundPaymentTerminalReaderBuilder::new() }
    }
    /// A positive integer in __cents__ representing how much of this charge to refund.
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// ID of the Charge to refund.
    pub fn charge(mut self, charge: impl Into<String>) -> Self {
        self.inner.charge = Some(charge.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
    /// ID of the PaymentIntent to refund.
    pub fn payment_intent(mut self, payment_intent: impl Into<String>) -> Self {
        self.inner.payment_intent = Some(payment_intent.into());
        self
    }
    /// Boolean indicating whether the application fee should be refunded when refunding this charge.
    /// If a full charge refund is given, the full application fee will be refunded.
    /// Otherwise, the application fee will be refunded in an amount proportional to the amount of the charge refunded.
    /// An application fee can be refunded only by the application that created the charge.
    pub fn refund_application_fee(mut self, refund_application_fee: impl Into<bool>) -> Self {
        self.inner.refund_application_fee = Some(refund_application_fee.into());
        self
    }
    /// Configuration overrides
    pub fn refund_payment_config(
        mut self,
        refund_payment_config: impl Into<RefundPaymentTerminalReaderRefundPaymentConfig>,
    ) -> Self {
        self.inner.refund_payment_config = Some(refund_payment_config.into());
        self
    }
    /// Boolean indicating whether the transfer should be reversed when refunding this charge.
    /// The transfer will be reversed proportionally to the amount being refunded (either the entire or partial amount).
    /// A transfer can be reversed only by the application that created the charge.
    pub fn reverse_transfer(mut self, reverse_transfer: impl Into<bool>) -> Self {
        self.inner.reverse_transfer = Some(reverse_transfer.into());
        self
    }
}
impl RefundPaymentTerminalReader {
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

impl StripeRequest for RefundPaymentTerminalReader {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = &self.reader;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/terminal/readers/{reader}/refund_payment"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct SetReaderDisplayTerminalReaderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    cart: Option<SetReaderDisplayTerminalReaderCart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(rename = "type")]
    type_: SetReaderDisplayTerminalReaderType,
}
impl SetReaderDisplayTerminalReaderBuilder {
    fn new(type_: impl Into<SetReaderDisplayTerminalReaderType>) -> Self {
        Self { cart: None, expand: None, type_: type_.into() }
    }
}
/// Cart
#[derive(Clone, Debug, serde::Serialize)]
pub struct SetReaderDisplayTerminalReaderCart {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Array of line items that were purchased.
    pub line_items: Vec<SetReaderDisplayTerminalReaderCartLineItems>,
    /// The amount of tax in cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<i64>,
    /// Total balance of cart due in cents.
    pub total: i64,
}
impl SetReaderDisplayTerminalReaderCart {
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        line_items: impl Into<Vec<SetReaderDisplayTerminalReaderCartLineItems>>,
        total: impl Into<i64>,
    ) -> Self {
        Self {
            currency: currency.into(),
            line_items: line_items.into(),
            tax: None,
            total: total.into(),
        }
    }
}
/// Array of line items that were purchased.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SetReaderDisplayTerminalReaderCartLineItems {
    /// The price of the item in cents.
    pub amount: i64,
    /// The description or name of the item.
    pub description: String,
    /// The quantity of the line item being purchased.
    pub quantity: u64,
}
impl SetReaderDisplayTerminalReaderCartLineItems {
    pub fn new(
        amount: impl Into<i64>,
        description: impl Into<String>,
        quantity: impl Into<u64>,
    ) -> Self {
        Self { amount: amount.into(), description: description.into(), quantity: quantity.into() }
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
pub struct SetReaderDisplayTerminalReader {
    inner: SetReaderDisplayTerminalReaderBuilder,
    reader: stripe_terminal::TerminalReaderId,
}
impl SetReaderDisplayTerminalReader {
    /// Construct a new `SetReaderDisplayTerminalReader`.
    pub fn new(
        reader: impl Into<stripe_terminal::TerminalReaderId>,
        type_: impl Into<SetReaderDisplayTerminalReaderType>,
    ) -> Self {
        Self {
            reader: reader.into(),
            inner: SetReaderDisplayTerminalReaderBuilder::new(type_.into()),
        }
    }
    /// Cart
    pub fn cart(mut self, cart: impl Into<SetReaderDisplayTerminalReaderCart>) -> Self {
        self.inner.cart = Some(cart.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl SetReaderDisplayTerminalReader {
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

impl StripeRequest for SetReaderDisplayTerminalReader {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = &self.reader;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/terminal/readers/{reader}/set_reader_display"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct PresentPaymentMethodTerminalReaderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_tip: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_present: Option<PresentPaymentMethodTerminalReaderCardPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interac_present: Option<PresentPaymentMethodTerminalReaderInteracPresent>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<PresentPaymentMethodTerminalReaderType>,
}
impl PresentPaymentMethodTerminalReaderBuilder {
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct PresentPaymentMethodTerminalReaderCardPresent {
    /// The card number, as a string without any separators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
}
impl PresentPaymentMethodTerminalReaderCardPresent {
    pub fn new() -> Self {
        Self { number: None }
    }
}
impl Default for PresentPaymentMethodTerminalReaderCardPresent {
    fn default() -> Self {
        Self::new()
    }
}
/// Simulated data for the interac_present payment method.
#[derive(Clone, Debug, serde::Serialize)]
pub struct PresentPaymentMethodTerminalReaderInteracPresent {
    /// Card Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
}
impl PresentPaymentMethodTerminalReaderInteracPresent {
    pub fn new() -> Self {
        Self { number: None }
    }
}
impl Default for PresentPaymentMethodTerminalReaderInteracPresent {
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
pub struct PresentPaymentMethodTerminalReader {
    inner: PresentPaymentMethodTerminalReaderBuilder,
    reader: String,
}
impl PresentPaymentMethodTerminalReader {
    /// Construct a new `PresentPaymentMethodTerminalReader`.
    pub fn new(reader: impl Into<String>) -> Self {
        Self { reader: reader.into(), inner: PresentPaymentMethodTerminalReaderBuilder::new() }
    }
    /// Simulated on-reader tip amount.
    pub fn amount_tip(mut self, amount_tip: impl Into<i64>) -> Self {
        self.inner.amount_tip = Some(amount_tip.into());
        self
    }
    /// Simulated data for the card_present payment method.
    pub fn card_present(
        mut self,
        card_present: impl Into<PresentPaymentMethodTerminalReaderCardPresent>,
    ) -> Self {
        self.inner.card_present = Some(card_present.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Simulated data for the interac_present payment method.
    pub fn interac_present(
        mut self,
        interac_present: impl Into<PresentPaymentMethodTerminalReaderInteracPresent>,
    ) -> Self {
        self.inner.interac_present = Some(interac_present.into());
        self
    }
    /// Simulated payment type.
    pub fn type_(mut self, type_: impl Into<PresentPaymentMethodTerminalReaderType>) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
}
impl PresentPaymentMethodTerminalReader {
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

impl StripeRequest for PresentPaymentMethodTerminalReader {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = &self.reader;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/terminal/readers/{reader}/present_payment_method"),
        )
        .form(&self.inner)
    }
}
