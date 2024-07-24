/// You can store multiple cards on a customer in order to charge the customer
/// later. You can also store multiple debit cards on a recipient in order to
/// transfer to those cards later.
///
/// Related guide: [Card payments with Sources](https://stripe.com/docs/sources/cards)
///
/// For more details see <<https://stripe.com/docs/api/cards/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Card {
    /// The account this card belongs to.
    /// This attribute will not be in the card object if the card belongs to a customer or recipient instead.
    /// This property is only available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    pub account: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// City/District/Suburb/Town/Village.
    pub address_city: Option<String>,
    /// Billing address country, if provided when creating card.
    pub address_country: Option<String>,
    /// Address line 1 (Street address/PO Box/Company name).
    pub address_line1: Option<String>,
    /// If `address_line1` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_line1_check: Option<String>,
    /// Address line 2 (Apartment/Suite/Unit/Building).
    pub address_line2: Option<String>,
    /// State/County/Province/Region.
    pub address_state: Option<String>,
    /// ZIP or postal code.
    pub address_zip: Option<String>,
    /// If `address_zip` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_zip_check: Option<String>,
    /// A set of available payout methods for this card.
    /// Only values from this set should be passed as the `method` when creating a payout.
    pub available_payout_methods: Option<Vec<CardAvailablePayoutMethods>>,
    /// Card brand.
    /// Can be `American Express`, `Diners Club`, `Discover`, `Eftpos Australia`, `JCB`, `MasterCard`, `UnionPay`, `Visa`, or `Unknown`.
    pub brand: String,
    /// Two-letter ISO code representing the country of the card.
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,
    /// Three-letter [ISO code for currency](https://stripe.com/docs/payouts).
    /// Only applicable on accounts (not customers or recipients).
    /// The card can be used as a transfer destination for funds in this currency.
    /// This property is only available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    pub currency: Option<stripe_types::Currency>,
    /// The customer that this card belongs to.
    /// This attribute will not be in the card object if the card belongs to an account or recipient instead.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// If a CVC was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`.
    /// A result of unchecked indicates that CVC was provided but hasn't been checked yet.
    /// Checks are typically performed when attaching a card to a Customer object, or when creating a charge.
    /// For more details, see [Check if a card is valid without a charge](https://support.stripe.com/questions/check-if-a-card-is-valid-without-a-charge).
    pub cvc_check: Option<String>,
    /// Whether this card is the default external account for its currency.
    /// This property is only available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    pub default_for_currency: Option<bool>,
    /// A high-level description of the type of cards issued in this range.
    /// (For internal use only and not typically available in standard API requests.).
    pub description: Option<String>,
    /// (For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,
    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,
    /// Uniquely identifies this particular card number.
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.
    ///
    /// *As of May 1, 2021, card fingerprint in India for Connect changed to allow two fingerprints for the same card---one for India and one for the rest of the world.*.
    pub fingerprint: Option<String>,
    /// Card funding type. Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: String,
    /// Unique identifier for the object.
    pub id: stripe_shared::CardId,
    /// Issuer identification number of the card.
    /// (For internal use only and not typically available in standard API requests.).
    pub iin: Option<String>,
    /// The name of the card's issuing bank.
    /// (For internal use only and not typically available in standard API requests.).
    pub issuer: Option<String>,
    /// The last four digits of the card.
    pub last4: String,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Cardholder name.
    pub name: Option<String>,
    pub networks: Option<stripe_shared::TokenCardNetworks>,
    /// For external accounts that are cards, possible values are `new` and `errored`.
    /// If a payout fails, the status is set to `errored` and [scheduled payouts](https://stripe.com/docs/payouts#payout-schedule) are stopped until account details are updated.
    pub status: Option<String>,
    /// If the card number is tokenized, this is the method that was used.
    /// Can be `android_pay` (includes Google Pay), `apple_pay`, `masterpass`, `visa_checkout`, or null.
    pub tokenization_method: Option<String>,
}
#[doc(hidden)]
pub struct CardBuilder {
    account: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    address_city: Option<Option<String>>,
    address_country: Option<Option<String>>,
    address_line1: Option<Option<String>>,
    address_line1_check: Option<Option<String>>,
    address_line2: Option<Option<String>>,
    address_state: Option<Option<String>>,
    address_zip: Option<Option<String>>,
    address_zip_check: Option<Option<String>>,
    available_payout_methods: Option<Option<Vec<CardAvailablePayoutMethods>>>,
    brand: Option<String>,
    country: Option<Option<String>>,
    currency: Option<Option<stripe_types::Currency>>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    cvc_check: Option<Option<String>>,
    default_for_currency: Option<Option<bool>>,
    description: Option<Option<String>>,
    dynamic_last4: Option<Option<String>>,
    exp_month: Option<i64>,
    exp_year: Option<i64>,
    fingerprint: Option<Option<String>>,
    funding: Option<String>,
    id: Option<stripe_shared::CardId>,
    iin: Option<Option<String>>,
    issuer: Option<Option<String>>,
    last4: Option<String>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    name: Option<Option<String>>,
    networks: Option<Option<stripe_shared::TokenCardNetworks>>,
    status: Option<Option<String>>,
    tokenization_method: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Card {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Card>,
        builder: CardBuilder,
    }

    impl Visitor for Place<Card> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CardBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CardBuilder {
        type Out = Card;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),
                "address_city" => Deserialize::begin(&mut self.address_city),
                "address_country" => Deserialize::begin(&mut self.address_country),
                "address_line1" => Deserialize::begin(&mut self.address_line1),
                "address_line1_check" => Deserialize::begin(&mut self.address_line1_check),
                "address_line2" => Deserialize::begin(&mut self.address_line2),
                "address_state" => Deserialize::begin(&mut self.address_state),
                "address_zip" => Deserialize::begin(&mut self.address_zip),
                "address_zip_check" => Deserialize::begin(&mut self.address_zip_check),
                "available_payout_methods" => {
                    Deserialize::begin(&mut self.available_payout_methods)
                }
                "brand" => Deserialize::begin(&mut self.brand),
                "country" => Deserialize::begin(&mut self.country),
                "currency" => Deserialize::begin(&mut self.currency),
                "customer" => Deserialize::begin(&mut self.customer),
                "cvc_check" => Deserialize::begin(&mut self.cvc_check),
                "default_for_currency" => Deserialize::begin(&mut self.default_for_currency),
                "description" => Deserialize::begin(&mut self.description),
                "dynamic_last4" => Deserialize::begin(&mut self.dynamic_last4),
                "exp_month" => Deserialize::begin(&mut self.exp_month),
                "exp_year" => Deserialize::begin(&mut self.exp_year),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "funding" => Deserialize::begin(&mut self.funding),
                "id" => Deserialize::begin(&mut self.id),
                "iin" => Deserialize::begin(&mut self.iin),
                "issuer" => Deserialize::begin(&mut self.issuer),
                "last4" => Deserialize::begin(&mut self.last4),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "name" => Deserialize::begin(&mut self.name),
                "networks" => Deserialize::begin(&mut self.networks),
                "status" => Deserialize::begin(&mut self.status),
                "tokenization_method" => Deserialize::begin(&mut self.tokenization_method),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account: Deserialize::default(),
                address_city: Deserialize::default(),
                address_country: Deserialize::default(),
                address_line1: Deserialize::default(),
                address_line1_check: Deserialize::default(),
                address_line2: Deserialize::default(),
                address_state: Deserialize::default(),
                address_zip: Deserialize::default(),
                address_zip_check: Deserialize::default(),
                available_payout_methods: Deserialize::default(),
                brand: Deserialize::default(),
                country: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                cvc_check: Deserialize::default(),
                default_for_currency: Deserialize::default(),
                description: Deserialize::default(),
                dynamic_last4: Deserialize::default(),
                exp_month: Deserialize::default(),
                exp_year: Deserialize::default(),
                fingerprint: Deserialize::default(),
                funding: Deserialize::default(),
                id: Deserialize::default(),
                iin: Deserialize::default(),
                issuer: Deserialize::default(),
                last4: Deserialize::default(),
                metadata: Deserialize::default(),
                name: Deserialize::default(),
                networks: Deserialize::default(),
                status: Deserialize::default(),
                tokenization_method: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                account: self.account.take()?,
                address_city: self.address_city.take()?,
                address_country: self.address_country.take()?,
                address_line1: self.address_line1.take()?,
                address_line1_check: self.address_line1_check.take()?,
                address_line2: self.address_line2.take()?,
                address_state: self.address_state.take()?,
                address_zip: self.address_zip.take()?,
                address_zip_check: self.address_zip_check.take()?,
                available_payout_methods: self.available_payout_methods.take()?,
                brand: self.brand.take()?,
                country: self.country.take()?,
                currency: self.currency?,
                customer: self.customer.take()?,
                cvc_check: self.cvc_check.take()?,
                default_for_currency: self.default_for_currency?,
                description: self.description.take()?,
                dynamic_last4: self.dynamic_last4.take()?,
                exp_month: self.exp_month?,
                exp_year: self.exp_year?,
                fingerprint: self.fingerprint.take()?,
                funding: self.funding.take()?,
                id: self.id.take()?,
                iin: self.iin.take()?,
                issuer: self.issuer.take()?,
                last4: self.last4.take()?,
                metadata: self.metadata.take()?,
                name: self.name.take()?,
                networks: self.networks.take()?,
                status: self.status.take()?,
                tokenization_method: self.tokenization_method.take()?,
            })
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

    impl ObjectDeser for Card {
        type Builder = CardBuilder;
    }

    impl FromValueOpt for Card {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = Some(FromValueOpt::from_value(v)?),
                    "address_city" => b.address_city = Some(FromValueOpt::from_value(v)?),
                    "address_country" => b.address_country = Some(FromValueOpt::from_value(v)?),
                    "address_line1" => b.address_line1 = Some(FromValueOpt::from_value(v)?),
                    "address_line1_check" => {
                        b.address_line1_check = Some(FromValueOpt::from_value(v)?)
                    }
                    "address_line2" => b.address_line2 = Some(FromValueOpt::from_value(v)?),
                    "address_state" => b.address_state = Some(FromValueOpt::from_value(v)?),
                    "address_zip" => b.address_zip = Some(FromValueOpt::from_value(v)?),
                    "address_zip_check" => b.address_zip_check = Some(FromValueOpt::from_value(v)?),
                    "available_payout_methods" => {
                        b.available_payout_methods = Some(FromValueOpt::from_value(v)?)
                    }
                    "brand" => b.brand = Some(FromValueOpt::from_value(v)?),
                    "country" => b.country = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "customer" => b.customer = Some(FromValueOpt::from_value(v)?),
                    "cvc_check" => b.cvc_check = Some(FromValueOpt::from_value(v)?),
                    "default_for_currency" => {
                        b.default_for_currency = Some(FromValueOpt::from_value(v)?)
                    }
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "dynamic_last4" => b.dynamic_last4 = Some(FromValueOpt::from_value(v)?),
                    "exp_month" => b.exp_month = Some(FromValueOpt::from_value(v)?),
                    "exp_year" => b.exp_year = Some(FromValueOpt::from_value(v)?),
                    "fingerprint" => b.fingerprint = Some(FromValueOpt::from_value(v)?),
                    "funding" => b.funding = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "iin" => b.iin = Some(FromValueOpt::from_value(v)?),
                    "issuer" => b.issuer = Some(FromValueOpt::from_value(v)?),
                    "last4" => b.last4 = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "name" => b.name = Some(FromValueOpt::from_value(v)?),
                    "networks" => b.networks = Some(FromValueOpt::from_value(v)?),
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "tokenization_method" => {
                        b.tokenization_method = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Card {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Card", 32)?;
        s.serialize_field("account", &self.account)?;
        s.serialize_field("address_city", &self.address_city)?;
        s.serialize_field("address_country", &self.address_country)?;
        s.serialize_field("address_line1", &self.address_line1)?;
        s.serialize_field("address_line1_check", &self.address_line1_check)?;
        s.serialize_field("address_line2", &self.address_line2)?;
        s.serialize_field("address_state", &self.address_state)?;
        s.serialize_field("address_zip", &self.address_zip)?;
        s.serialize_field("address_zip_check", &self.address_zip_check)?;
        s.serialize_field("available_payout_methods", &self.available_payout_methods)?;
        s.serialize_field("brand", &self.brand)?;
        s.serialize_field("country", &self.country)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("cvc_check", &self.cvc_check)?;
        s.serialize_field("default_for_currency", &self.default_for_currency)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("dynamic_last4", &self.dynamic_last4)?;
        s.serialize_field("exp_month", &self.exp_month)?;
        s.serialize_field("exp_year", &self.exp_year)?;
        s.serialize_field("fingerprint", &self.fingerprint)?;
        s.serialize_field("funding", &self.funding)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("iin", &self.iin)?;
        s.serialize_field("issuer", &self.issuer)?;
        s.serialize_field("last4", &self.last4)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("networks", &self.networks)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("tokenization_method", &self.tokenization_method)?;

        s.serialize_field("object", "card")?;
        s.end()
    }
}
/// A set of available payout methods for this card.
/// Only values from this set should be passed as the `method` when creating a payout.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CardAvailablePayoutMethods {
    Instant,
    Standard,
}
impl CardAvailablePayoutMethods {
    pub fn as_str(self) -> &'static str {
        use CardAvailablePayoutMethods::*;
        match self {
            Instant => "instant",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CardAvailablePayoutMethods {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardAvailablePayoutMethods::*;
        match s {
            "instant" => Ok(Instant),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CardAvailablePayoutMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CardAvailablePayoutMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CardAvailablePayoutMethods {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CardAvailablePayoutMethods {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CardAvailablePayoutMethods> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CardAvailablePayoutMethods::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CardAvailablePayoutMethods);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CardAvailablePayoutMethods {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CardAvailablePayoutMethods"))
    }
}
impl stripe_types::Object for Card {
    type Id = stripe_shared::CardId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(CardId);
