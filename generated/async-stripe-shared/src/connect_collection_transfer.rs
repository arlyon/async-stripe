#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectCollectionTransfer {
    /// Amount transferred, in cents (or local equivalent).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the account that funds are being collected for.
    pub destination: stripe_types::Expandable<stripe_shared::Account>,
    /// Unique identifier for the object.
    pub id: stripe_shared::ConnectCollectionTransferId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConnectCollectionTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConnectCollectionTransfer").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConnectCollectionTransferBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
    destination: Option<stripe_types::Expandable<stripe_shared::Account>>,
    id: Option<stripe_shared::ConnectCollectionTransferId>,
    livemode: Option<bool>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for ConnectCollectionTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectCollectionTransfer>,
        builder: ConnectCollectionTransferBuilder,
    }

    impl Visitor for Place<ConnectCollectionTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectCollectionTransferBuilder {
                    amount: Deserialize::default(),
                    currency: Deserialize::default(),
                    destination: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "destination" => Deserialize::begin(&mut self.builder.destination),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(currency), Some(destination), Some(id), Some(livemode)) = (
                self.builder.amount,
                self.builder.currency.take(),
                self.builder.destination.take(),
                self.builder.id.take(),
                self.builder.livemode,
            ) else {
                return Ok(());
            };
            *self.out =
                Some(ConnectCollectionTransfer { amount, currency, destination, id, livemode });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ConnectCollectionTransfer {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ConnectCollectionTransfer", 6)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("destination", &self.destination)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;

        s.serialize_field("object", "connect_collection_transfer")?;
        s.end()
    }
}
impl stripe_types::Object for ConnectCollectionTransfer {
    type Id = stripe_shared::ConnectCollectionTransferId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ConnectCollectionTransferId);
