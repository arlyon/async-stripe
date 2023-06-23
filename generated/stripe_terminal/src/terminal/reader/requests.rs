use stripe::{Client, Response};

impl stripe_terminal::terminal::reader::Reader {
    /// Updates a `Reader` object by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    pub fn update(client: &Client, reader: &str, params: UpdateReader) -> Response<UpdateReturned> {
        client.send_form(
            &format!("/terminal/readers/{reader}", reader = reader),
            params,
            http_types::Method::Post,
        )
    }
    /// Retrieves a `Reader` object.
    pub fn retrieve(
        client: &Client,
        reader: &str,
        params: RetrieveReader,
    ) -> Response<RetrieveReturned> {
        client.get_query(&format!("/terminal/readers/{reader}", reader = reader), params)
    }
    /// Creates a new `Reader` object.
    pub fn create(
        client: &Client,
        params: CreateReader,
    ) -> Response<stripe_terminal::terminal::reader::Reader> {
        client.send_form("/terminal/readers", params, http_types::Method::Post)
    }
    /// Returns a list of `Reader` objects.
    pub fn list(
        client: &Client,
        params: ListReader,
    ) -> Response<stripe_types::List<stripe_terminal::terminal::reader::Reader>> {
        client.get_query("/terminal/readers", params)
    }
    /// Deletes a `Reader` object.
    pub fn delete(
        client: &Client,
        reader: &str,
    ) -> Response<stripe_terminal::terminal::reader::DeletedReader> {
        client.send(
            &format!("/terminal/readers/{reader}", reader = reader),
            http_types::Method::Delete,
        )
    }
    /// Initiates a payment flow on a Reader.
    pub fn process_payment_intent(
        client: &Client,
        reader: &str,
        params: ProcessPaymentIntentReader,
    ) -> Response<stripe_terminal::terminal::reader::Reader> {
        client.send_form(
            &format!("/terminal/readers/{reader}/process_payment_intent", reader = reader),
            params,
            http_types::Method::Post,
        )
    }
    /// Initiates a setup intent flow on a Reader.
    pub fn process_setup_intent(
        client: &Client,
        reader: &str,
        params: ProcessSetupIntentReader,
    ) -> Response<stripe_terminal::terminal::reader::Reader> {
        client.send_form(
            &format!("/terminal/readers/{reader}/process_setup_intent", reader = reader),
            params,
            http_types::Method::Post,
        )
    }
    /// Cancels the current reader action.
    pub fn cancel_action(
        client: &Client,
        reader: &str,
        params: CancelActionReader,
    ) -> Response<stripe_terminal::terminal::reader::Reader> {
        client.send_form(
            &format!("/terminal/readers/{reader}/cancel_action", reader = reader),
            params,
            http_types::Method::Post,
        )
    }
    /// Sets reader display to show cart details.
    pub fn set_reader_display(
        client: &Client,
        reader: &str,
        params: SetReaderDisplayReader,
    ) -> Response<stripe_terminal::terminal::reader::Reader> {
        client.send_form(
            &format!("/terminal/readers/{reader}/set_reader_display", reader = reader),
            params,
            http_types::Method::Post,
        )
    }
    /// Presents a payment method on a simulated reader.
    ///
    /// Can be used to simulate accepting a payment, saving a card or refunding a transaction.
    pub fn present_payment_method(
        client: &Client,
        reader: &str,
        params: PresentPaymentMethodReader,
    ) -> Response<stripe_terminal::terminal::reader::Reader> {
        client.send_form(
            &format!(
                "/test_helpers/terminal/readers/{reader}/present_payment_method",
                reader = reader
            ),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum UpdateReturned {
    TerminalReader(stripe_terminal::terminal::reader::Reader),
    DeletedTerminalReader(stripe_terminal::terminal::reader::DeletedReader),
}
#[cfg(feature = "min-ser")]
impl crate::deser::StripeDeserialize for UpdateReturned {
    fn deserialize(str: &str) -> Result<Self, crate::StripeError> {
        use miniserde::json::from_str;

        use crate::deser::StripeDeserialize;
        let maybe_deleted: crate::deser::MaybeDeleted = from_str(str)?;
        let deleted = maybe_deleted.deleted.unwrap_or(false);
        if deleted {
            Ok(Self::DeletedTerminalReader(StripeDeserialize::deserialize(str)?))
        } else {
            Ok(Self::TerminalReader(StripeDeserialize::deserialize(str)?))
        }
    }
}

#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateReader<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The new label of the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum RetrieveReturned {
    TerminalReader(stripe_terminal::terminal::reader::Reader),
    DeletedTerminalReader(stripe_terminal::terminal::reader::DeletedReader),
}
#[cfg(feature = "min-ser")]
impl crate::deser::StripeDeserialize for RetrieveReturned {
    fn deserialize(str: &str) -> Result<Self, crate::StripeError> {
        use miniserde::json::from_str;

        use crate::deser::StripeDeserialize;
        let maybe_deleted: crate::deser::MaybeDeleted = from_str(str)?;
        let deleted = maybe_deleted.deleted.unwrap_or(false);
        if deleted {
            Ok(Self::DeletedTerminalReader(StripeDeserialize::deserialize(str)?))
        } else {
            Ok(Self::TerminalReader(StripeDeserialize::deserialize(str)?))
        }
    }
}

#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveReader<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateReader<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Custom label given to the reader for easier identification.
    ///
    /// If no label is specified, the registration code will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<&'a str>,
    /// The location to assign the reader to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// A code generated by the reader used for registering to an account.
    pub registration_code: &'a str,
}
impl<'a> CreateReader<'a> {
    pub fn new(registration_code: &'a str) -> Self {
        Self {
            expand: Default::default(),
            label: Default::default(),
            location: Default::default(),
            metadata: Default::default(),
            registration_code,
        }
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListReader<'a> {
    /// Filters readers by device type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<ListReaderDeviceType>,
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
    /// A location ID to filter the response list to only readers at the specific location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// A status filter to filter readers to only offline or online readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListReaderStatus>,
}
impl<'a> ListReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Filters readers by device type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListReaderDeviceType {
    BbposChipper2x,
    BbposWisepad3,
    BbposWiseposE,
    SimulatedWiseposE,
    StripeM2,
    #[serde(rename = "verifone_P400")]
    VerifoneP400,
}

impl ListReaderDeviceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BbposChipper2x => "bbpos_chipper2x",
            Self::BbposWisepad3 => "bbpos_wisepad3",
            Self::BbposWiseposE => "bbpos_wisepos_e",
            Self::SimulatedWiseposE => "simulated_wisepos_e",
            Self::StripeM2 => "stripe_m2",
            Self::VerifoneP400 => "verifone_P400",
        }
    }
}

impl AsRef<str> for ListReaderDeviceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListReaderDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// A status filter to filter readers to only offline or online readers.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListReaderStatus {
    Offline,
    Online,
}

impl ListReaderStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Online => "online",
        }
    }
}

impl AsRef<str> for ListReaderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListReaderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ProcessPaymentIntentReader<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// PaymentIntent ID.
    pub payment_intent: &'a str,
    /// Configuration overrides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_config: Option<ProcessPaymentIntentReaderProcessConfig>,
}
impl<'a> ProcessPaymentIntentReader<'a> {
    pub fn new(payment_intent: &'a str) -> Self {
        Self { expand: Default::default(), payment_intent, process_config: Default::default() }
    }
}
/// Configuration overrides.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ProcessPaymentIntentReaderProcessConfig {
    /// Override showing a tipping selection screen on this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_tipping: Option<bool>,
    /// Tipping configuration for this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<ProcessPaymentIntentReaderProcessConfigTipping>,
}
impl ProcessPaymentIntentReaderProcessConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for this transaction.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ProcessPaymentIntentReaderProcessConfigTipping {
    /// Amount used to calculate tip suggestions on tipping selection screen for this transaction.
    ///
    /// Must be a positive integer in the smallest currency unit (e.g., 100 cents to represent $1.00 or 100 to represent ¥100, a zero-decimal currency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eligible: Option<i64>,
}
impl ProcessPaymentIntentReaderProcessConfigTipping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ProcessSetupIntentReader<'a> {
    /// Customer Consent Collected.
    pub customer_consent_collected: bool,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// SetupIntent ID.
    pub setup_intent: &'a str,
}
impl<'a> ProcessSetupIntentReader<'a> {
    pub fn new(customer_consent_collected: bool, setup_intent: &'a str) -> Self {
        Self { customer_consent_collected, expand: Default::default(), setup_intent }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelActionReader<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelActionReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SetReaderDisplayReader<'a> {
    /// Cart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart: Option<SetReaderDisplayReaderCart<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Type.
    #[serde(rename = "type")]
    pub type_: SetReaderDisplayReaderType,
}
impl<'a> SetReaderDisplayReader<'a> {
    pub fn new(type_: SetReaderDisplayReaderType) -> Self {
        Self { cart: Default::default(), expand: Default::default(), type_ }
    }
}
/// Cart.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SetReaderDisplayReaderCart<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Array of line items that were purchased.
    pub line_items: &'a [SetReaderDisplayReaderCartLineItems<'a>],
    /// The amount of tax in cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<i64>,
    /// Total balance of cart due in cents.
    pub total: i64,
}
impl<'a> SetReaderDisplayReaderCart<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        line_items: &'a [SetReaderDisplayReaderCartLineItems<'a>],
        total: i64,
    ) -> Self {
        Self { currency, line_items, tax: Default::default(), total }
    }
}
/// Array of line items that were purchased.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SetReaderDisplayReaderCartLineItems<'a> {
    /// The price of the item in cents.
    pub amount: i64,
    /// The description or name of the item.
    pub description: &'a str,
    /// The quantity of the line item being purchased.
    pub quantity: u64,
}
impl<'a> SetReaderDisplayReaderCartLineItems<'a> {
    pub fn new(amount: i64, description: &'a str, quantity: u64) -> Self {
        Self { amount, description, quantity }
    }
}
/// Type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SetReaderDisplayReaderType {
    Cart,
}

impl SetReaderDisplayReaderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cart => "cart",
        }
    }
}

impl AsRef<str> for SetReaderDisplayReaderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetReaderDisplayReaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PresentPaymentMethodReader<'a> {
    /// Simulated data for the card_present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<PresentPaymentMethodReaderCardPresent<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Simulated payment type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PresentPaymentMethodReaderType>,
}
impl<'a> PresentPaymentMethodReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Simulated data for the card_present payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PresentPaymentMethodReaderCardPresent<'a> {
    /// Card Number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<&'a str>,
}
impl<'a> PresentPaymentMethodReaderCardPresent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Simulated payment type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentPaymentMethodReaderType {
    CardPresent,
}

impl PresentPaymentMethodReaderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CardPresent => "card_present",
        }
    }
}

impl AsRef<str> for PresentPaymentMethodReaderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PresentPaymentMethodReaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
