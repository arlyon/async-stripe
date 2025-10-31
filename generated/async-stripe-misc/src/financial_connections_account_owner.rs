/// Describes an owner of an account.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FinancialConnectionsAccountOwner {
    /// The email address of the owner.
    pub email: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_misc::FinancialConnectionsAccountOwnerId,
    /// The full name of the owner.
    pub name: String,
    /// The ownership object that this owner belongs to.
    pub ownership: String,
    /// The raw phone number of the owner.
    pub phone: Option<String>,
    /// The raw physical address of the owner.
    pub raw_address: Option<String>,
    /// The timestamp of the refresh that updated this owner.
    pub refreshed_at: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct FinancialConnectionsAccountOwnerBuilder {
    email: Option<Option<String>>,
    id: Option<stripe_misc::FinancialConnectionsAccountOwnerId>,
    name: Option<String>,
    ownership: Option<String>,
    phone: Option<Option<String>>,
    raw_address: Option<Option<String>>,
    refreshed_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for FinancialConnectionsAccountOwner {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FinancialConnectionsAccountOwner>,
        builder: FinancialConnectionsAccountOwnerBuilder,
    }

    impl Visitor for Place<FinancialConnectionsAccountOwner> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FinancialConnectionsAccountOwnerBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for FinancialConnectionsAccountOwnerBuilder {
        type Out = FinancialConnectionsAccountOwner;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "email" => Deserialize::begin(&mut self.email),
                "id" => Deserialize::begin(&mut self.id),
                "name" => Deserialize::begin(&mut self.name),
                "ownership" => Deserialize::begin(&mut self.ownership),
                "phone" => Deserialize::begin(&mut self.phone),
                "raw_address" => Deserialize::begin(&mut self.raw_address),
                "refreshed_at" => Deserialize::begin(&mut self.refreshed_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                email: Deserialize::default(),
                id: Deserialize::default(),
                name: Deserialize::default(),
                ownership: Deserialize::default(),
                phone: Deserialize::default(),
                raw_address: Deserialize::default(),
                refreshed_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(email),
                Some(id),
                Some(name),
                Some(ownership),
                Some(phone),
                Some(raw_address),
                Some(refreshed_at),
            ) = (
                self.email.take(),
                self.id.take(),
                self.name.take(),
                self.ownership.take(),
                self.phone.take(),
                self.raw_address.take(),
                self.refreshed_at,
            )
            else {
                return None;
            };
            Some(Self::Out { email, id, name, ownership, phone, raw_address, refreshed_at })
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

    impl ObjectDeser for FinancialConnectionsAccountOwner {
        type Builder = FinancialConnectionsAccountOwnerBuilder;
    }

    impl FromValueOpt for FinancialConnectionsAccountOwner {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FinancialConnectionsAccountOwnerBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "email" => b.email = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "ownership" => b.ownership = FromValueOpt::from_value(v),
                    "phone" => b.phone = FromValueOpt::from_value(v),
                    "raw_address" => b.raw_address = FromValueOpt::from_value(v),
                    "refreshed_at" => b.refreshed_at = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccountOwner {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("FinancialConnectionsAccountOwner", 8)?;
        s.serialize_field("email", &self.email)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("ownership", &self.ownership)?;
        s.serialize_field("phone", &self.phone)?;
        s.serialize_field("raw_address", &self.raw_address)?;
        s.serialize_field("refreshed_at", &self.refreshed_at)?;

        s.serialize_field("object", "financial_connections.account_owner")?;
        s.end()
    }
}
impl stripe_types::Object for FinancialConnectionsAccountOwner {
    type Id = stripe_misc::FinancialConnectionsAccountOwnerId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(FinancialConnectionsAccountOwnerId);
