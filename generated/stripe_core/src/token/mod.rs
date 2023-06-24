/// Tokenization is the process Stripe uses to collect sensitive card or bank
/// account details, or personally identifiable information (PII), directly from
/// your customers in a secure manner.
///
/// A token representing this information is returned to your server to use.
/// You should use our [recommended payments integrations](https://stripe.com/docs/payments) to perform this process client-side.
/// This ensures that no sensitive card data touches your server, and allows your integration to operate in a PCI-compliant way.  If you cannot use client-side tokenization, you can also create tokens using the API with either your publishable or secret API key.
/// Keep in mind that if your integration uses this method, you are responsible for any PCI compliance that may be required, and you must keep your secret API key safe.
/// Unlike with client-side tokenization, your customer's information is not sent directly to Stripe, so we cannot determine how it is handled or stored.  Tokens cannot be stored or used more than once.
/// To store card or bank account information for later use, you can create [Customer](https://stripe.com/docs/api#customers) objects or [Custom accounts](https://stripe.com/docs/api#external_accounts).
/// Note that [Radar](https://stripe.com/docs/radar), our integrated solution for automatic fraud protection, performs best with integrations that use client-side tokenization.  Related guide: [Accept a payment](https://stripe.com/docs/payments/accept-a-payment-charges#web-create-token).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Token {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<stripe_core::bank_account::BankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_core::card::Card>,
    /// IP address of the client that generated the token.
    pub client_ip: Option<String>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_core::token::TokenId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TokenObject,
    /// Type of the token: `account`, `bank_account`, `card`, or `pii`.
    #[serde(rename = "type")]
    pub type_: String,
    /// Whether this token has already been used (tokens can be used only once).
    pub used: bool,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Token {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TokenObject {
    Token,
}

impl TokenObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Token => "token",
        }
    }
}

impl std::str::FromStr for TokenObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "token" => Ok(Self::Token),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for TokenObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TokenObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TokenObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TokenObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TokenObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TokenObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<TokenObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TokenObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for Token {
    type Id = stripe_core::token::TokenId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TokenId);
pub mod requests;
