/// You can [create physical or virtual cards](https://stripe.com/docs/issuing/cards) that are issued to cardholders.
///
/// For more details see <<https://stripe.com/docs/api/issuing/cards/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCard {
    /// The brand of the card.
    pub brand: String,
    /// The reason why the card was canceled.
    pub cancellation_reason: Option<IssuingCardCancellationReason>,
    pub cardholder: stripe_shared::IssuingCardholder,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Supported currencies are `usd` in the US, `eur` in the EU, and `gbp` in the UK.
    pub currency: stripe_types::Currency,
    /// The card's CVC.
    /// For security reasons, this is only available for virtual cards, and will be omitted unless you explicitly request it with [the `expand` parameter](https://stripe.com/docs/api/expanding_objects).
    /// Additionally, it's only available via the ["Retrieve a card" endpoint](https://stripe.com/docs/api/issuing/cards/retrieve), not via "List all cards" or any other endpoint.
    pub cvc: Option<String>,
    /// The expiration month of the card.
    pub exp_month: i64,
    /// The expiration year of the card.
    pub exp_year: i64,
    /// The financial account this card is attached to.
    pub financial_account: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::IssuingCardId,
    /// The last 4 digits of the card number.
    pub last4: String,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The full unredacted card number.
    /// For security reasons, this is only available for virtual cards, and will be omitted unless you explicitly request it with [the `expand` parameter](https://stripe.com/docs/api/expanding_objects).
    /// Additionally, it's only available via the ["Retrieve a card" endpoint](https://stripe.com/docs/api/issuing/cards/retrieve), not via "List all cards" or any other endpoint.
    pub number: Option<String>,
    /// The latest card that replaces this card, if any.
    pub replaced_by: Option<stripe_types::Expandable<stripe_shared::IssuingCard>>,
    /// The card this card replaces, if any.
    pub replacement_for: Option<stripe_types::Expandable<stripe_shared::IssuingCard>>,
    /// The reason why the previous card needed to be replaced.
    pub replacement_reason: Option<stripe_shared::IssuingCardReplacementReason>,
    /// Where and how the card will be shipped.
    pub shipping: Option<stripe_shared::IssuingCardShipping>,
    pub spending_controls: stripe_shared::IssuingCardAuthorizationControls,
    /// Whether authorizations can be approved on this card.
    /// May be blocked from activating cards depending on past-due Cardholder requirements.
    /// Defaults to `inactive`.
    pub status: stripe_shared::IssuingCardStatus,
    /// The type of the card.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: stripe_shared::IssuingCardType,
    /// Information relating to digital wallets (like Apple Pay and Google Pay).
    pub wallets: Option<stripe_shared::IssuingCardWallets>,
}
#[doc(hidden)]
pub struct IssuingCardBuilder {
    brand: Option<String>,
    cancellation_reason: Option<Option<IssuingCardCancellationReason>>,
    cardholder: Option<stripe_shared::IssuingCardholder>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    cvc: Option<Option<String>>,
    exp_month: Option<i64>,
    exp_year: Option<i64>,
    financial_account: Option<Option<String>>,
    id: Option<stripe_shared::IssuingCardId>,
    last4: Option<String>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    number: Option<Option<String>>,
    replaced_by: Option<Option<stripe_types::Expandable<stripe_shared::IssuingCard>>>,
    replacement_for: Option<Option<stripe_types::Expandable<stripe_shared::IssuingCard>>>,
    replacement_reason: Option<Option<stripe_shared::IssuingCardReplacementReason>>,
    shipping: Option<Option<stripe_shared::IssuingCardShipping>>,
    spending_controls: Option<stripe_shared::IssuingCardAuthorizationControls>,
    status: Option<stripe_shared::IssuingCardStatus>,
    type_: Option<stripe_shared::IssuingCardType>,
    wallets: Option<Option<stripe_shared::IssuingCardWallets>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCard>,
        builder: IssuingCardBuilder,
    }

    impl Visitor for Place<IssuingCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardBuilder {
        type Out = IssuingCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "brand" => Deserialize::begin(&mut self.brand),
                "cancellation_reason" => Deserialize::begin(&mut self.cancellation_reason),
                "cardholder" => Deserialize::begin(&mut self.cardholder),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "cvc" => Deserialize::begin(&mut self.cvc),
                "exp_month" => Deserialize::begin(&mut self.exp_month),
                "exp_year" => Deserialize::begin(&mut self.exp_year),
                "financial_account" => Deserialize::begin(&mut self.financial_account),
                "id" => Deserialize::begin(&mut self.id),
                "last4" => Deserialize::begin(&mut self.last4),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "number" => Deserialize::begin(&mut self.number),
                "replaced_by" => Deserialize::begin(&mut self.replaced_by),
                "replacement_for" => Deserialize::begin(&mut self.replacement_for),
                "replacement_reason" => Deserialize::begin(&mut self.replacement_reason),
                "shipping" => Deserialize::begin(&mut self.shipping),
                "spending_controls" => Deserialize::begin(&mut self.spending_controls),
                "status" => Deserialize::begin(&mut self.status),
                "type" => Deserialize::begin(&mut self.type_),
                "wallets" => Deserialize::begin(&mut self.wallets),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                brand: Deserialize::default(),
                cancellation_reason: Deserialize::default(),
                cardholder: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                cvc: Deserialize::default(),
                exp_month: Deserialize::default(),
                exp_year: Deserialize::default(),
                financial_account: Deserialize::default(),
                id: Deserialize::default(),
                last4: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                number: Deserialize::default(),
                replaced_by: Deserialize::default(),
                replacement_for: Deserialize::default(),
                replacement_reason: Deserialize::default(),
                shipping: Deserialize::default(),
                spending_controls: Deserialize::default(),
                status: Deserialize::default(),
                type_: Deserialize::default(),
                wallets: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                brand: self.brand.take()?,
                cancellation_reason: self.cancellation_reason?,
                cardholder: self.cardholder.take()?,
                created: self.created?,
                currency: self.currency?,
                cvc: self.cvc.take()?,
                exp_month: self.exp_month?,
                exp_year: self.exp_year?,
                financial_account: self.financial_account.take()?,
                id: self.id.take()?,
                last4: self.last4.take()?,
                livemode: self.livemode?,
                metadata: self.metadata.take()?,
                number: self.number.take()?,
                replaced_by: self.replaced_by.take()?,
                replacement_for: self.replacement_for.take()?,
                replacement_reason: self.replacement_reason?,
                shipping: self.shipping.take()?,
                spending_controls: self.spending_controls.take()?,
                status: self.status?,
                type_: self.type_?,
                wallets: self.wallets.take()?,
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

    impl ObjectDeser for IssuingCard {
        type Builder = IssuingCardBuilder;
    }

    impl FromValueOpt for IssuingCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "brand" => b.brand = Some(FromValueOpt::from_value(v)?),
                    "cancellation_reason" => {
                        b.cancellation_reason = Some(FromValueOpt::from_value(v)?)
                    }
                    "cardholder" => b.cardholder = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "cvc" => b.cvc = Some(FromValueOpt::from_value(v)?),
                    "exp_month" => b.exp_month = Some(FromValueOpt::from_value(v)?),
                    "exp_year" => b.exp_year = Some(FromValueOpt::from_value(v)?),
                    "financial_account" => b.financial_account = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "last4" => b.last4 = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "number" => b.number = Some(FromValueOpt::from_value(v)?),
                    "replaced_by" => b.replaced_by = Some(FromValueOpt::from_value(v)?),
                    "replacement_for" => b.replacement_for = Some(FromValueOpt::from_value(v)?),
                    "replacement_reason" => {
                        b.replacement_reason = Some(FromValueOpt::from_value(v)?)
                    }
                    "shipping" => b.shipping = Some(FromValueOpt::from_value(v)?),
                    "spending_controls" => b.spending_controls = Some(FromValueOpt::from_value(v)?),
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),
                    "wallets" => b.wallets = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingCard {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("IssuingCard", 23)?;
        s.serialize_field("brand", &self.brand)?;
        s.serialize_field("cancellation_reason", &self.cancellation_reason)?;
        s.serialize_field("cardholder", &self.cardholder)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("cvc", &self.cvc)?;
        s.serialize_field("exp_month", &self.exp_month)?;
        s.serialize_field("exp_year", &self.exp_year)?;
        s.serialize_field("financial_account", &self.financial_account)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("last4", &self.last4)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("number", &self.number)?;
        s.serialize_field("replaced_by", &self.replaced_by)?;
        s.serialize_field("replacement_for", &self.replacement_for)?;
        s.serialize_field("replacement_reason", &self.replacement_reason)?;
        s.serialize_field("shipping", &self.shipping)?;
        s.serialize_field("spending_controls", &self.spending_controls)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("wallets", &self.wallets)?;

        s.serialize_field("object", "issuing.card")?;
        s.end()
    }
}
/// The reason why the card was canceled.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardCancellationReason {
    DesignRejected,
    Lost,
    Stolen,
}
impl IssuingCardCancellationReason {
    pub fn as_str(self) -> &'static str {
        use IssuingCardCancellationReason::*;
        match self {
            DesignRejected => "design_rejected",
            Lost => "lost",
            Stolen => "stolen",
        }
    }
}

impl std::str::FromStr for IssuingCardCancellationReason {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardCancellationReason::*;
        match s {
            "design_rejected" => Ok(DesignRejected),
            "lost" => Ok(Lost),
            "stolen" => Ok(Stolen),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingCardCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingCardCancellationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingCardCancellationReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardCancellationReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardCancellationReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardCancellationReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardCancellationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingCardCancellationReason")
        })
    }
}
impl stripe_types::Object for IssuingCard {
    type Id = stripe_shared::IssuingCardId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(IssuingCardId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardReplacementReason {
    Damaged,
    Expired,
    Lost,
    Stolen,
}
impl IssuingCardReplacementReason {
    pub fn as_str(self) -> &'static str {
        use IssuingCardReplacementReason::*;
        match self {
            Damaged => "damaged",
            Expired => "expired",
            Lost => "lost",
            Stolen => "stolen",
        }
    }
}

impl std::str::FromStr for IssuingCardReplacementReason {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardReplacementReason::*;
        match s {
            "damaged" => Ok(Damaged),
            "expired" => Ok(Expired),
            "lost" => Ok(Lost),
            "stolen" => Ok(Stolen),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingCardReplacementReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardReplacementReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardReplacementReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingCardReplacementReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardReplacementReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardReplacementReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardReplacementReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardReplacementReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardReplacementReason"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardStatus {
    Active,
    Canceled,
    Inactive,
}
impl IssuingCardStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingCardStatus::*;
        match self {
            Active => "active",
            Canceled => "canceled",
            Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for IssuingCardStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardStatus::*;
        match s {
            "active" => Ok(Active),
            "canceled" => Ok(Canceled),
            "inactive" => Ok(Inactive),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingCardStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingCardStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardStatus"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardType {
    Physical,
    Virtual,
}
impl IssuingCardType {
    pub fn as_str(self) -> &'static str {
        use IssuingCardType::*;
        match self {
            Physical => "physical",
            Virtual => "virtual",
        }
    }
}

impl std::str::FromStr for IssuingCardType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardType::*;
        match s {
            "physical" => Ok(Physical),
            "virtual" => Ok(Virtual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingCardType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardType"))
    }
}
