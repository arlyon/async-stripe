#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteTerminalReader {}
impl DeleteTerminalReader {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteTerminalReader {
    /// Deletes a `Reader` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &stripe_terminal::TerminalReaderId,
    ) -> stripe::Response<stripe_terminal::DeletedTerminalReader> {
        client.send_form(&format!("/terminal/readers/{reader}"), self, http_types::Method::Delete)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListTerminalReader<'a> {
    /// Filters readers by device type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<stripe_terminal::TerminalReaderDeviceType>,
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
    /// A location ID to filter the response list to only readers at the specific location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<&'a str>,
    /// Filters readers by serial number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<&'a str>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// A status filter to filter readers to only offline or online readers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_terminal::TerminalReaderStatus>,
}
impl<'a> ListTerminalReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListTerminalReader<'a> {
    /// Returns a list of `Reader` objects.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_terminal::TerminalReader>> {
        client.get_query("/terminal/readers", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_terminal::TerminalReader>> {
        stripe::ListPaginator::from_list_params("/terminal/readers", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTerminalReader<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTerminalReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTerminalReader<'a> {
    /// Retrieves a `Reader` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &stripe_terminal::TerminalReaderId,
    ) -> stripe::Response<RetrieveTerminalReaderReturned> {
        client.get_query(&format!("/terminal/readers/{reader}"), self)
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
pub struct CreateTerminalReader<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Custom label given to the reader for easier identification.
    /// If no label is specified, the registration code will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<&'a str>,
    /// The location to assign the reader to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// A code generated by the reader used for registering to an account.
    pub registration_code: &'a str,
}
impl<'a> CreateTerminalReader<'a> {
    pub fn new(registration_code: &'a str) -> Self {
        Self { expand: None, label: None, location: None, metadata: None, registration_code }
    }
}
impl<'a> CreateTerminalReader<'a> {
    /// Creates a new `Reader` object.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_terminal::TerminalReader> {
        client.send_form("/terminal/readers", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalReader<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The new label of the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateTerminalReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateTerminalReader<'a> {
    /// Updates a `Reader` object by setting the values of the parameters passed.
    /// Any parameters not provided will be left unchanged.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &stripe_terminal::TerminalReaderId,
    ) -> stripe::Response<UpdateTerminalReaderReturned> {
        client.send_form(&format!("/terminal/readers/{reader}"), self, http_types::Method::Post)
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

#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelActionTerminalReader<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelActionTerminalReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CancelActionTerminalReader<'a> {
    /// Cancels the current reader action.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &stripe_terminal::TerminalReaderId,
    ) -> stripe::Response<stripe_terminal::TerminalReader> {
        client.send_form(
            &format!("/terminal/readers/{reader}/cancel_action"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ProcessPaymentIntentTerminalReader<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// PaymentIntent ID
    pub payment_intent: &'a str,
    /// Configuration overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_config: Option<ProcessPaymentIntentTerminalReaderProcessConfig>,
}
impl<'a> ProcessPaymentIntentTerminalReader<'a> {
    pub fn new(payment_intent: &'a str) -> Self {
        Self { expand: None, payment_intent, process_config: None }
    }
}
/// Configuration overrides
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ProcessPaymentIntentTerminalReaderProcessConfig {
    /// Override showing a tipping selection screen on this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_tipping: Option<bool>,
    /// Tipping configuration for this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<ProcessPaymentIntentTerminalReaderProcessConfigTipping>,
}
impl ProcessPaymentIntentTerminalReaderProcessConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for this transaction.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ProcessPaymentIntentTerminalReaderProcessConfigTipping {
    /// Amount used to calculate tip suggestions on tipping selection screen for this transaction.
    /// Must be a positive integer in the smallest currency unit (e.g., 100 cents to represent $1.00 or 100 to represent ¥100, a zero-decimal currency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eligible: Option<i64>,
}
impl ProcessPaymentIntentTerminalReaderProcessConfigTipping {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ProcessPaymentIntentTerminalReader<'a> {
    /// Initiates a payment flow on a Reader.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &stripe_terminal::TerminalReaderId,
    ) -> stripe::Response<stripe_terminal::TerminalReader> {
        client.send_form(
            &format!("/terminal/readers/{reader}/process_payment_intent"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ProcessSetupIntentTerminalReader<'a> {
    /// Customer Consent Collected
    pub customer_consent_collected: bool,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Configuration overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub process_config: Option<miniserde::json::Value>,
    /// SetupIntent ID
    pub setup_intent: &'a str,
}
impl<'a> ProcessSetupIntentTerminalReader<'a> {
    pub fn new(customer_consent_collected: bool, setup_intent: &'a str) -> Self {
        Self { customer_consent_collected, expand: None, process_config: None, setup_intent }
    }
}
impl<'a> ProcessSetupIntentTerminalReader<'a> {
    /// Initiates a setup intent flow on a Reader.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &stripe_terminal::TerminalReaderId,
    ) -> stripe::Response<stripe_terminal::TerminalReader> {
        client.send_form(
            &format!("/terminal/readers/{reader}/process_setup_intent"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RefundPaymentTerminalReader<'a> {
    /// A positive integer in __cents__ representing how much of this charge to refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ID of the Charge to refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// ID of the PaymentIntent to refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<&'a str>,
    /// Boolean indicating whether the application fee should be refunded when refunding this charge.
    /// If a full charge refund is given, the full application fee will be refunded.
    /// Otherwise, the application fee will be refunded in an amount proportional to the amount of the charge refunded.
    /// An application fee can be refunded only by the application that created the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,
    /// Boolean indicating whether the transfer should be reversed when refunding this charge.
    /// The transfer will be reversed proportionally to the amount being refunded (either the entire or partial amount).
    /// A transfer can be reversed only by the application that created the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_transfer: Option<bool>,
}
impl<'a> RefundPaymentTerminalReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RefundPaymentTerminalReader<'a> {
    /// Initiates a refund on a Reader
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &stripe_terminal::TerminalReaderId,
    ) -> stripe::Response<stripe_terminal::TerminalReader> {
        client.send_form(
            &format!("/terminal/readers/{reader}/refund_payment"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SetReaderDisplayTerminalReader<'a> {
    /// Cart
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart: Option<SetReaderDisplayTerminalReaderCart<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Type
    #[serde(rename = "type")]
    pub type_: SetReaderDisplayTerminalReaderType,
}
impl<'a> SetReaderDisplayTerminalReader<'a> {
    pub fn new(type_: SetReaderDisplayTerminalReaderType) -> Self {
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
impl<'a> SetReaderDisplayTerminalReader<'a> {
    /// Sets reader display to show cart details.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &stripe_terminal::TerminalReaderId,
    ) -> stripe::Response<stripe_terminal::TerminalReader> {
        client.send_form(
            &format!("/terminal/readers/{reader}/set_reader_display"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PresentPaymentMethodTerminalReader<'a> {
    /// Simulated on-reader tip amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tip: Option<i64>,
    /// Simulated data for the card_present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<PresentPaymentMethodTerminalReaderCardPresent<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Simulated data for the interac_present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<PresentPaymentMethodTerminalReaderInteracPresent<'a>>,
    /// Simulated payment type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PresentPaymentMethodTerminalReaderType>,
}
impl<'a> PresentPaymentMethodTerminalReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Simulated data for the card_present payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PresentPaymentMethodTerminalReaderCardPresent<'a> {
    /// The card number, as a string without any separators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<&'a str>,
}
impl<'a> PresentPaymentMethodTerminalReaderCardPresent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Simulated data for the interac_present payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PresentPaymentMethodTerminalReaderInteracPresent<'a> {
    /// Card Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<&'a str>,
}
impl<'a> PresentPaymentMethodTerminalReaderInteracPresent<'a> {
    pub fn new() -> Self {
        Self::default()
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
impl<'a> PresentPaymentMethodTerminalReader<'a> {
    /// Presents a payment method on a simulated reader.
    /// Can be used to simulate accepting a payment, saving a card or refunding a transaction.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &str,
    ) -> stripe::Response<stripe_terminal::TerminalReader> {
        client.send_form(
            &format!("/test_helpers/terminal/readers/{reader}/present_payment_method"),
            self,
            http_types::Method::Post,
        )
    }
}
