/// An issuing token object is created when an issued card is added to a digital wallet.
/// As a [card issuer](https://docs.stripe.com/issuing), you can [view and manage these tokens](https://docs.stripe.com/issuing/controls/token-management) through Stripe.
///
/// For more details see <<https://stripe.com/docs/api/issuing/tokens/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingToken {
    /// Card associated with this token.
    pub card: stripe_types::Expandable<stripe_shared::IssuingCard>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The hashed ID derived from the device ID from the card network associated with the token.
    pub device_fingerprint: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::IssuingTokenId,
    /// The last four digits of the token.
    pub last4: Option<String>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The token service provider / card network associated with the token.
    pub network: stripe_shared::IssuingTokenNetwork,
    pub network_data: Option<stripe_shared::IssuingNetworkTokenNetworkData>,
    /// Time at which the token was last updated by the card network.
    /// Measured in seconds since the Unix epoch.
    pub network_updated_at: stripe_types::Timestamp,
    /// The usage state of the token.
    pub status: stripe_shared::IssuingTokenStatus,
    /// The digital wallet for this token, if one was used.
    pub wallet_provider: Option<stripe_shared::IssuingTransactionWallet>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingToken").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingTokenBuilder {
    card: Option<stripe_types::Expandable<stripe_shared::IssuingCard>>,
    created: Option<stripe_types::Timestamp>,
    device_fingerprint: Option<Option<String>>,
    id: Option<stripe_shared::IssuingTokenId>,
    last4: Option<Option<String>>,
    livemode: Option<bool>,
    network: Option<stripe_shared::IssuingTokenNetwork>,
    network_data: Option<Option<stripe_shared::IssuingNetworkTokenNetworkData>>,
    network_updated_at: Option<stripe_types::Timestamp>,
    status: Option<stripe_shared::IssuingTokenStatus>,
    wallet_provider: Option<Option<stripe_shared::IssuingTransactionWallet>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingToken {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingToken>,
        builder: IssuingTokenBuilder,
    }

    impl Visitor for Place<IssuingToken> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTokenBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingTokenBuilder {
        type Out = IssuingToken;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card" => Deserialize::begin(&mut self.card),
                "created" => Deserialize::begin(&mut self.created),
                "device_fingerprint" => Deserialize::begin(&mut self.device_fingerprint),
                "id" => Deserialize::begin(&mut self.id),
                "last4" => Deserialize::begin(&mut self.last4),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "network" => Deserialize::begin(&mut self.network),
                "network_data" => Deserialize::begin(&mut self.network_data),
                "network_updated_at" => Deserialize::begin(&mut self.network_updated_at),
                "status" => Deserialize::begin(&mut self.status),
                "wallet_provider" => Deserialize::begin(&mut self.wallet_provider),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                card: None,
                created: None,
                device_fingerprint: Some(None),
                id: None,
                last4: Some(None),
                livemode: None,
                network: None,
                network_data: Some(None),
                network_updated_at: None,
                status: None,
                wallet_provider: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(card),
                Some(created),
                Some(device_fingerprint),
                Some(id),
                Some(last4),
                Some(livemode),
                Some(network),
                Some(network_data),
                Some(network_updated_at),
                Some(status),
                Some(wallet_provider),
            ) = (
                self.card.take(),
                self.created,
                self.device_fingerprint.take(),
                self.id.take(),
                self.last4.take(),
                self.livemode,
                self.network.take(),
                self.network_data.take(),
                self.network_updated_at,
                self.status.take(),
                self.wallet_provider.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                card,
                created,
                device_fingerprint,
                id,
                last4,
                livemode,
                network,
                network_data,
                network_updated_at,
                status,
                wallet_provider,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for IssuingToken {
        type Builder = IssuingTokenBuilder;
    }

    impl FromValueOpt for IssuingToken {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingTokenBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card" => b.card = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "device_fingerprint" => b.device_fingerprint = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "network" => b.network = FromValueOpt::from_value(v),
                    "network_data" => b.network_data = FromValueOpt::from_value(v),
                    "network_updated_at" => b.network_updated_at = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "wallet_provider" => b.wallet_provider = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingToken {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("IssuingToken", 12)?;
        s.serialize_field("card", &self.card)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("device_fingerprint", &self.device_fingerprint)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("last4", &self.last4)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("network", &self.network)?;
        s.serialize_field("network_data", &self.network_data)?;
        s.serialize_field("network_updated_at", &self.network_updated_at)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("wallet_provider", &self.wallet_provider)?;

        s.serialize_field("object", "issuing.token")?;
        s.end()
    }
}
impl stripe_types::Object for IssuingToken {
    type Id = stripe_shared::IssuingTokenId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(IssuingTokenId);
/// The token service provider / card network associated with the token.
pub type IssuingTokenNetwork = stripe_shared::IssuingTokenNetwork;
/// The usage state of the token.
pub type IssuingTokenStatus = stripe_shared::IssuingTokenStatus;
/// The digital wallet for this token, if one was used.
pub type IssuingTokenWalletProvider = stripe_shared::IssuingTransactionWallet;
