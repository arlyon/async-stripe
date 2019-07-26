use crate::config::{Client, Response};
use crate::ids::{BankAccountId, CardId, ChargeId, SourceId, TokenId};
use crate::params::Object;
use crate::resources::{Charge, Rule};
use serde_derive::{Deserialize, Serialize};

/// The set of PaymentSource parameters that can be used to create a charge.
/// 
/// For more details see [https://stripe.com/docs/api/charges/create#create_charge-source](https://stripe.com/docs/api/charges/create#create_charge-source).
#[derive(Clone, Debug)]
pub enum PaymentChargeParams {
    Token(TokenId),
    Source(SourceId),
    Card(CardId),
    BankAccount(BankAccountId),
}

impl<'de> ::serde::Deserialize<'de> for PaymentChargeParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        use serde::de::{Deserialize, Error};
        use serde::private::de::{Content, ContentRefDeserializer};

        let content = <Content<'_> as Deserialize>::deserialize(deserializer)?;
        let deserializer = ContentRefDeserializer::<D::Error>::new(&content);
        if let Ok(ok) = <SourceId as Deserialize>::deserialize(deserializer) {
            return Ok(PaymentChargeParams::Source(ok));
        }
        let deserializer = ContentRefDeserializer::<D::Error>::new(&content);
        if let Ok(ok) = <TokenId as Deserialize>::deserialize(deserializer) {
            return Ok(PaymentChargeParams::Token(ok));
        }
        let deserializer = ContentRefDeserializer::<D::Error>::new(&content);
        if let Ok(ok) = <CardId as Deserialize>::deserialize(deserializer) {
            return Ok(PaymentChargeParams::Card(ok));
        }
        let deserializer = ContentRefDeserializer::<D::Error>::new(&content);
        if let Ok(ok) = <BankAccountId as Deserialize>::deserialize(deserializer) {
            return Ok(PaymentChargeParams::BankAccount(ok));
        }

        Err(Error::custom("data did not match any variant of enum PaymentChargeParams"))
    }
}

impl<'a> ::serde::Serialize for PaymentChargeParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
    {
        match self {
            PaymentChargeParams::Source(id) => id.serialize(serializer),
            PaymentChargeParams::Token(id) => id.serialize(serializer),
            PaymentChargeParams::Card(id) => id.serialize(serializer),
            PaymentChargeParams::BankAccount(id) => id.serialize(serializer),
        }
    }
}

/// The set of parameters that can be used when capturing a charge object.
///
/// For more details see [https://stripe.com/docs/api#charge_capture](https://stripe.com/docs/api#charge_capture).
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CaptureCharge<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}

impl Charge {
    /// Capture captures a previously created charge with capture set to false.
    ///
    /// For more details see [https://stripe.com/docs/api#charge_capture](https://stripe.com/docs/api#charge_capture).
    pub fn capture(
        client: &Client,
        charge_id: &ChargeId,
        params: CaptureCharge<'_>,
    ) -> Response<Charge> {
        client.post_form(&format!("/charges/{}/capture", charge_id), params)
    }
}

impl Object for Rule {
    type Id = String;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        ""
    }
}
