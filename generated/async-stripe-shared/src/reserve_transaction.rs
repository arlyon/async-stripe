#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ReserveTransaction {
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::ReserveTransactionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReserveTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReserveTransaction").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ReserveTransactionBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    id: Option<stripe_shared::ReserveTransactionId>,
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

    impl Deserialize for ReserveTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ReserveTransaction>,
        builder: ReserveTransactionBuilder,
    }

    impl Visitor for Place<ReserveTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ReserveTransactionBuilder {
                    amount: Deserialize::default(),
                    currency: Deserialize::default(),
                    description: Deserialize::default(),
                    id: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "description" => Deserialize::begin(&mut self.builder.description),
                "id" => Deserialize::begin(&mut self.builder.id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(currency), Some(description), Some(id)) = (
                self.builder.amount,
                self.builder.currency.take(),
                self.builder.description.take(),
                self.builder.id.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(ReserveTransaction { amount, currency, description, id });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ReserveTransaction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ReserveTransaction", 5)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("id", &self.id)?;

        s.serialize_field("object", "reserve_transaction")?;
        s.end()
    }
}
impl stripe_types::Object for ReserveTransaction {
    type Id = stripe_shared::ReserveTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ReserveTransactionId);
