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

    impl Map for Builder<'_> {
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

    impl Map for Builder<'_> {
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
struct CollectInputsTerminalReaderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    inputs: Vec<CollectInputsTerminalReaderInputs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl CollectInputsTerminalReaderBuilder {
    fn new(inputs: impl Into<Vec<CollectInputsTerminalReaderInputs>>) -> Self {
        Self { expand: None, inputs: inputs.into(), metadata: None }
    }
}
/// List of inputs to be collected using the Reader
#[derive(Clone, Debug, serde::Serialize)]
pub struct CollectInputsTerminalReaderInputs {
    /// Customize the text which will be displayed while collecting this input
    pub custom_text: CollectInputsTerminalReaderInputsCustomText,
    /// Indicate that this input is required, disabling the skip button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Options for the `selection` input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<CollectInputsTerminalReaderInputsSelection>,
    /// List of toggles to be displayed and customization for the toggles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggles: Option<Vec<CollectInputsTerminalReaderInputsToggles>>,
    /// The type of input to collect
    #[serde(rename = "type")]
    pub type_: CollectInputsTerminalReaderInputsType,
}
impl CollectInputsTerminalReaderInputs {
    pub fn new(
        custom_text: impl Into<CollectInputsTerminalReaderInputsCustomText>,
        type_: impl Into<CollectInputsTerminalReaderInputsType>,
    ) -> Self {
        Self {
            custom_text: custom_text.into(),
            required: None,
            selection: None,
            toggles: None,
            type_: type_.into(),
        }
    }
}
/// Customize the text which will be displayed while collecting this input
#[derive(Clone, Debug, serde::Serialize)]
pub struct CollectInputsTerminalReaderInputsCustomText {
    /// The description which will be displayed when collecting this input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The skip button text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_button: Option<String>,
    /// The submit button text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_button: Option<String>,
    /// The title which will be displayed when collecting this input
    pub title: String,
}
impl CollectInputsTerminalReaderInputsCustomText {
    pub fn new(title: impl Into<String>) -> Self {
        Self { description: None, skip_button: None, submit_button: None, title: title.into() }
    }
}
/// Options for the `selection` input
#[derive(Clone, Debug, serde::Serialize)]
pub struct CollectInputsTerminalReaderInputsSelection {
    /// List of choices for the `selection` input
    pub choices: Vec<CollectInputsTerminalReaderInputsSelectionChoices>,
}
impl CollectInputsTerminalReaderInputsSelection {
    pub fn new(choices: impl Into<Vec<CollectInputsTerminalReaderInputsSelectionChoices>>) -> Self {
        Self { choices: choices.into() }
    }
}
/// List of choices for the `selection` input
#[derive(Clone, Debug, serde::Serialize)]
pub struct CollectInputsTerminalReaderInputsSelectionChoices {
    /// The unique identifier for this choice
    pub id: String,
    /// The style of the button which will be shown for this choice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CollectInputsTerminalReaderInputsSelectionChoicesStyle>,
    /// The text which will be shown on the button for this choice
    pub text: String,
}
impl CollectInputsTerminalReaderInputsSelectionChoices {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self { id: id.into(), style: None, text: text.into() }
    }
}
/// The style of the button which will be shown for this choice
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CollectInputsTerminalReaderInputsSelectionChoicesStyle {
    Primary,
    Secondary,
}
impl CollectInputsTerminalReaderInputsSelectionChoicesStyle {
    pub fn as_str(self) -> &'static str {
        use CollectInputsTerminalReaderInputsSelectionChoicesStyle::*;
        match self {
            Primary => "primary",
            Secondary => "secondary",
        }
    }
}

impl std::str::FromStr for CollectInputsTerminalReaderInputsSelectionChoicesStyle {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CollectInputsTerminalReaderInputsSelectionChoicesStyle::*;
        match s {
            "primary" => Ok(Primary),
            "secondary" => Ok(Secondary),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CollectInputsTerminalReaderInputsSelectionChoicesStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CollectInputsTerminalReaderInputsSelectionChoicesStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CollectInputsTerminalReaderInputsSelectionChoicesStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CollectInputsTerminalReaderInputsSelectionChoicesStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CollectInputsTerminalReaderInputsSelectionChoicesStyle",
            )
        })
    }
}
/// List of toggles to be displayed and customization for the toggles
#[derive(Clone, Debug, serde::Serialize)]
pub struct CollectInputsTerminalReaderInputsToggles {
    /// The default value of the toggle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<CollectInputsTerminalReaderInputsTogglesDefaultValue>,
    /// The description which will be displayed for the toggle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The title which will be displayed for the toggle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl CollectInputsTerminalReaderInputsToggles {
    pub fn new() -> Self {
        Self { default_value: None, description: None, title: None }
    }
}
impl Default for CollectInputsTerminalReaderInputsToggles {
    fn default() -> Self {
        Self::new()
    }
}
/// The default value of the toggle
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CollectInputsTerminalReaderInputsTogglesDefaultValue {
    Disabled,
    Enabled,
}
impl CollectInputsTerminalReaderInputsTogglesDefaultValue {
    pub fn as_str(self) -> &'static str {
        use CollectInputsTerminalReaderInputsTogglesDefaultValue::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr for CollectInputsTerminalReaderInputsTogglesDefaultValue {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CollectInputsTerminalReaderInputsTogglesDefaultValue::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CollectInputsTerminalReaderInputsTogglesDefaultValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CollectInputsTerminalReaderInputsTogglesDefaultValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CollectInputsTerminalReaderInputsTogglesDefaultValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CollectInputsTerminalReaderInputsTogglesDefaultValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CollectInputsTerminalReaderInputsTogglesDefaultValue",
            )
        })
    }
}
/// The type of input to collect
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CollectInputsTerminalReaderInputsType {
    Email,
    Numeric,
    Phone,
    Selection,
    Signature,
    Text,
}
impl CollectInputsTerminalReaderInputsType {
    pub fn as_str(self) -> &'static str {
        use CollectInputsTerminalReaderInputsType::*;
        match self {
            Email => "email",
            Numeric => "numeric",
            Phone => "phone",
            Selection => "selection",
            Signature => "signature",
            Text => "text",
        }
    }
}

impl std::str::FromStr for CollectInputsTerminalReaderInputsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CollectInputsTerminalReaderInputsType::*;
        match s {
            "email" => Ok(Email),
            "numeric" => Ok(Numeric),
            "phone" => Ok(Phone),
            "selection" => Ok(Selection),
            "signature" => Ok(Signature),
            "text" => Ok(Text),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CollectInputsTerminalReaderInputsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CollectInputsTerminalReaderInputsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CollectInputsTerminalReaderInputsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CollectInputsTerminalReaderInputsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CollectInputsTerminalReaderInputsType")
        })
    }
}
/// Initiates an input collection flow on a Reader.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CollectInputsTerminalReader {
    inner: CollectInputsTerminalReaderBuilder,
    reader: stripe_terminal::TerminalReaderId,
}
impl CollectInputsTerminalReader {
    /// Construct a new `CollectInputsTerminalReader`.
    pub fn new(
        reader: impl Into<stripe_terminal::TerminalReaderId>,
        inputs: impl Into<Vec<CollectInputsTerminalReaderInputs>>,
    ) -> Self {
        Self {
            reader: reader.into(),
            inner: CollectInputsTerminalReaderBuilder::new(inputs.into()),
        }
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
}
impl CollectInputsTerminalReader {
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

impl StripeRequest for CollectInputsTerminalReader {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = &self.reader;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/terminal/readers/{reader}/collect_inputs"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CollectPaymentMethodTerminalReaderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    collect_config: Option<CollectPaymentMethodTerminalReaderCollectConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    payment_intent: String,
}
impl CollectPaymentMethodTerminalReaderBuilder {
    fn new(payment_intent: impl Into<String>) -> Self {
        Self { collect_config: None, expand: None, payment_intent: payment_intent.into() }
    }
}
/// Configuration overrides.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CollectPaymentMethodTerminalReaderCollectConfig {
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<CollectPaymentMethodTerminalReaderCollectConfigAllowRedisplay>,
    /// Enables cancel button on transaction screens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_customer_cancellation: Option<bool>,
    /// Override showing a tipping selection screen on this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_tipping: Option<bool>,
    /// Tipping configuration for this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<TippingConfig>,
}
impl CollectPaymentMethodTerminalReaderCollectConfig {
    pub fn new() -> Self {
        Self {
            allow_redisplay: None,
            enable_customer_cancellation: None,
            skip_tipping: None,
            tipping: None,
        }
    }
}
impl Default for CollectPaymentMethodTerminalReaderCollectConfig {
    fn default() -> Self {
        Self::new()
    }
}
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CollectPaymentMethodTerminalReaderCollectConfigAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}
impl CollectPaymentMethodTerminalReaderCollectConfigAllowRedisplay {
    pub fn as_str(self) -> &'static str {
        use CollectPaymentMethodTerminalReaderCollectConfigAllowRedisplay::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CollectPaymentMethodTerminalReaderCollectConfigAllowRedisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CollectPaymentMethodTerminalReaderCollectConfigAllowRedisplay::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CollectPaymentMethodTerminalReaderCollectConfigAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CollectPaymentMethodTerminalReaderCollectConfigAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CollectPaymentMethodTerminalReaderCollectConfigAllowRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CollectPaymentMethodTerminalReaderCollectConfigAllowRedisplay
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CollectPaymentMethodTerminalReaderCollectConfigAllowRedisplay",
            )
        })
    }
}
/// Initiates a payment flow on a Reader and updates the PaymentIntent with card details before manual confirmation.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CollectPaymentMethodTerminalReader {
    inner: CollectPaymentMethodTerminalReaderBuilder,
    reader: stripe_terminal::TerminalReaderId,
}
impl CollectPaymentMethodTerminalReader {
    /// Construct a new `CollectPaymentMethodTerminalReader`.
    pub fn new(
        reader: impl Into<stripe_terminal::TerminalReaderId>,
        payment_intent: impl Into<String>,
    ) -> Self {
        Self {
            reader: reader.into(),
            inner: CollectPaymentMethodTerminalReaderBuilder::new(payment_intent.into()),
        }
    }
    /// Configuration overrides.
    pub fn collect_config(
        mut self,
        collect_config: impl Into<CollectPaymentMethodTerminalReaderCollectConfig>,
    ) -> Self {
        self.inner.collect_config = Some(collect_config.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CollectPaymentMethodTerminalReader {
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

impl StripeRequest for CollectPaymentMethodTerminalReader {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = &self.reader;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/terminal/readers/{reader}/collect_payment_method"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ConfirmPaymentIntentTerminalReaderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm_config: Option<ConfirmPaymentIntentTerminalReaderConfirmConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    payment_intent: String,
}
impl ConfirmPaymentIntentTerminalReaderBuilder {
    fn new(payment_intent: impl Into<String>) -> Self {
        Self { confirm_config: None, expand: None, payment_intent: payment_intent.into() }
    }
}
/// Configuration overrides.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentTerminalReaderConfirmConfig {
    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app or site.
    /// If you'd prefer to redirect to a mobile application, you can alternatively supply an application URI scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}
impl ConfirmPaymentIntentTerminalReaderConfirmConfig {
    pub fn new() -> Self {
        Self { return_url: None }
    }
}
impl Default for ConfirmPaymentIntentTerminalReaderConfirmConfig {
    fn default() -> Self {
        Self::new()
    }
}
/// Finalizes a payment on a Reader.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentTerminalReader {
    inner: ConfirmPaymentIntentTerminalReaderBuilder,
    reader: stripe_terminal::TerminalReaderId,
}
impl ConfirmPaymentIntentTerminalReader {
    /// Construct a new `ConfirmPaymentIntentTerminalReader`.
    pub fn new(
        reader: impl Into<stripe_terminal::TerminalReaderId>,
        payment_intent: impl Into<String>,
    ) -> Self {
        Self {
            reader: reader.into(),
            inner: ConfirmPaymentIntentTerminalReaderBuilder::new(payment_intent.into()),
        }
    }
    /// Configuration overrides.
    pub fn confirm_config(
        mut self,
        confirm_config: impl Into<ConfirmPaymentIntentTerminalReaderConfirmConfig>,
    ) -> Self {
        self.inner.confirm_config = Some(confirm_config.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl ConfirmPaymentIntentTerminalReader {
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

impl StripeRequest for ConfirmPaymentIntentTerminalReader {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = &self.reader;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/terminal/readers/{reader}/confirm_payment_intent"),
        )
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct ProcessPaymentIntentTerminalReaderProcessConfig {
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<ProcessPaymentIntentTerminalReaderProcessConfigAllowRedisplay>,
    /// Enables cancel button on transaction screens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_customer_cancellation: Option<bool>,
    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app or site.
    /// If you'd prefer to redirect to a mobile application, you can alternatively supply an application URI scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Override showing a tipping selection screen on this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_tipping: Option<bool>,
    /// Tipping configuration for this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<TippingConfig>,
}
impl ProcessPaymentIntentTerminalReaderProcessConfig {
    pub fn new() -> Self {
        Self {
            allow_redisplay: None,
            enable_customer_cancellation: None,
            return_url: None,
            skip_tipping: None,
            tipping: None,
        }
    }
}
impl Default for ProcessPaymentIntentTerminalReaderProcessConfig {
    fn default() -> Self {
        Self::new()
    }
}
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ProcessPaymentIntentTerminalReaderProcessConfigAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}
impl ProcessPaymentIntentTerminalReaderProcessConfigAllowRedisplay {
    pub fn as_str(self) -> &'static str {
        use ProcessPaymentIntentTerminalReaderProcessConfigAllowRedisplay::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for ProcessPaymentIntentTerminalReaderProcessConfigAllowRedisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ProcessPaymentIntentTerminalReaderProcessConfigAllowRedisplay::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ProcessPaymentIntentTerminalReaderProcessConfigAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ProcessPaymentIntentTerminalReaderProcessConfigAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ProcessPaymentIntentTerminalReaderProcessConfigAllowRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ProcessPaymentIntentTerminalReaderProcessConfigAllowRedisplay
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ProcessPaymentIntentTerminalReaderProcessConfigAllowRedisplay",
            )
        })
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
    allow_redisplay: ProcessSetupIntentTerminalReaderAllowRedisplay,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    process_config: Option<ProcessSetupIntentTerminalReaderProcessConfig>,
    setup_intent: String,
}
impl ProcessSetupIntentTerminalReaderBuilder {
    fn new(
        allow_redisplay: impl Into<ProcessSetupIntentTerminalReaderAllowRedisplay>,
        setup_intent: impl Into<String>,
    ) -> Self {
        Self {
            allow_redisplay: allow_redisplay.into(),
            expand: None,
            process_config: None,
            setup_intent: setup_intent.into(),
        }
    }
}
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ProcessSetupIntentTerminalReaderAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}
impl ProcessSetupIntentTerminalReaderAllowRedisplay {
    pub fn as_str(self) -> &'static str {
        use ProcessSetupIntentTerminalReaderAllowRedisplay::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for ProcessSetupIntentTerminalReaderAllowRedisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ProcessSetupIntentTerminalReaderAllowRedisplay::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ProcessSetupIntentTerminalReaderAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ProcessSetupIntentTerminalReaderAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ProcessSetupIntentTerminalReaderAllowRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ProcessSetupIntentTerminalReaderAllowRedisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ProcessSetupIntentTerminalReaderAllowRedisplay",
            )
        })
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
        allow_redisplay: impl Into<ProcessSetupIntentTerminalReaderAllowRedisplay>,
        setup_intent: impl Into<String>,
    ) -> Self {
        Self {
            reader: reader.into(),
            inner: ProcessSetupIntentTerminalReaderBuilder::new(
                allow_redisplay.into(),
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
    card: Option<PresentPaymentMethodTerminalReaderCard>,
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
            card: None,
            card_present: None,
            expand: None,
            interac_present: None,
            type_: None,
        }
    }
}
/// Simulated data for the card payment method.
#[derive(Clone, Debug, serde::Serialize)]
pub struct PresentPaymentMethodTerminalReaderCard {
    /// Card security code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,
    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    /// Two- or four-digit number representing the card's expiration year.
    pub exp_year: i64,
    /// The card number, as a string without any separators.
    pub number: String,
}
impl PresentPaymentMethodTerminalReaderCard {
    pub fn new(
        exp_month: impl Into<i64>,
        exp_year: impl Into<i64>,
        number: impl Into<String>,
    ) -> Self {
        Self {
            cvc: None,
            exp_month: exp_month.into(),
            exp_year: exp_year.into(),
            number: number.into(),
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
    Card,
    CardPresent,
    InteracPresent,
}
impl PresentPaymentMethodTerminalReaderType {
    pub fn as_str(self) -> &'static str {
        use PresentPaymentMethodTerminalReaderType::*;
        match self {
            Card => "card",
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
            "card" => Ok(Card),
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
    /// Simulated data for the card payment method.
    pub fn card(mut self, card: impl Into<PresentPaymentMethodTerminalReaderCard>) -> Self {
        self.inner.card = Some(card.into());
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
#[derive(Clone, Debug, serde::Serialize)]
struct SucceedInputCollectionTerminalReaderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_non_required_inputs: Option<SucceedInputCollectionTerminalReaderSkipNonRequiredInputs>,
}
impl SucceedInputCollectionTerminalReaderBuilder {
    fn new() -> Self {
        Self { expand: None, skip_non_required_inputs: None }
    }
}
/// This parameter defines the skip behavior for input collection.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SucceedInputCollectionTerminalReaderSkipNonRequiredInputs {
    All,
    None,
}
impl SucceedInputCollectionTerminalReaderSkipNonRequiredInputs {
    pub fn as_str(self) -> &'static str {
        use SucceedInputCollectionTerminalReaderSkipNonRequiredInputs::*;
        match self {
            All => "all",
            None => "none",
        }
    }
}

impl std::str::FromStr for SucceedInputCollectionTerminalReaderSkipNonRequiredInputs {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SucceedInputCollectionTerminalReaderSkipNonRequiredInputs::*;
        match s {
            "all" => Ok(All),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SucceedInputCollectionTerminalReaderSkipNonRequiredInputs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SucceedInputCollectionTerminalReaderSkipNonRequiredInputs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SucceedInputCollectionTerminalReaderSkipNonRequiredInputs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SucceedInputCollectionTerminalReaderSkipNonRequiredInputs {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SucceedInputCollectionTerminalReaderSkipNonRequiredInputs",
            )
        })
    }
}
/// Use this endpoint to trigger a successful input collection on a simulated reader.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SucceedInputCollectionTerminalReader {
    inner: SucceedInputCollectionTerminalReaderBuilder,
    reader: String,
}
impl SucceedInputCollectionTerminalReader {
    /// Construct a new `SucceedInputCollectionTerminalReader`.
    pub fn new(reader: impl Into<String>) -> Self {
        Self { reader: reader.into(), inner: SucceedInputCollectionTerminalReaderBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// This parameter defines the skip behavior for input collection.
    pub fn skip_non_required_inputs(
        mut self,
        skip_non_required_inputs: impl Into<SucceedInputCollectionTerminalReaderSkipNonRequiredInputs>,
    ) -> Self {
        self.inner.skip_non_required_inputs = Some(skip_non_required_inputs.into());
        self
    }
}
impl SucceedInputCollectionTerminalReader {
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

impl StripeRequest for SucceedInputCollectionTerminalReader {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = &self.reader;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/terminal/readers/{reader}/succeed_input_collection"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct TimeoutInputCollectionTerminalReaderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl TimeoutInputCollectionTerminalReaderBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Use this endpoint to complete an input collection with a timeout error on a simulated reader.
#[derive(Clone, Debug, serde::Serialize)]
pub struct TimeoutInputCollectionTerminalReader {
    inner: TimeoutInputCollectionTerminalReaderBuilder,
    reader: String,
}
impl TimeoutInputCollectionTerminalReader {
    /// Construct a new `TimeoutInputCollectionTerminalReader`.
    pub fn new(reader: impl Into<String>) -> Self {
        Self { reader: reader.into(), inner: TimeoutInputCollectionTerminalReaderBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl TimeoutInputCollectionTerminalReader {
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

impl StripeRequest for TimeoutInputCollectionTerminalReader {
    type Output = stripe_terminal::TerminalReader;

    fn build(&self) -> RequestBuilder {
        let reader = &self.reader;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/terminal/readers/{reader}/timeout_input_collection"),
        )
        .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct TippingConfig {
    /// Amount used to calculate tip suggestions on tipping selection screen for this transaction.
    /// Must be a positive integer in the smallest currency unit (e.g., 100 cents to represent $1.00 or 100 to represent ¥100, a zero-decimal currency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eligible: Option<i64>,
}
impl TippingConfig {
    pub fn new() -> Self {
        Self { amount_eligible: None }
    }
}
impl Default for TippingConfig {
    fn default() -> Self {
        Self::new()
    }
}
