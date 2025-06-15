#[derive(Clone, Debug)]
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
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: ConnectCollectionTransferBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConnectCollectionTransferBuilder {
        type Out = ConnectCollectionTransfer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "currency" => Deserialize::begin(&mut self.currency),
                "destination" => Deserialize::begin(&mut self.destination),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                currency: Deserialize::default(),
                destination: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(currency), Some(destination), Some(id), Some(livemode)) = (
                self.amount,
                self.currency.take(),
                self.destination.take(),
                self.id.take(),
                self.livemode,
            ) else {
                return None;
            };
            Some(Self::Out { amount, currency, destination, id, livemode })
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

    impl ObjectDeser for ConnectCollectionTransfer {
        type Builder = ConnectCollectionTransferBuilder;
    }

    impl FromValueOpt for ConnectCollectionTransfer {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConnectCollectionTransferBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "destination" => b.destination = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
