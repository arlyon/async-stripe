/// Describes an owner of an account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsAccountOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FinancialConnectionsAccountOwner").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: FinancialConnectionsAccountOwnerBuilder {
                    email: Deserialize::default(),
                    id: Deserialize::default(),
                    name: Deserialize::default(),
                    ownership: Deserialize::default(),
                    phone: Deserialize::default(),
                    raw_address: Deserialize::default(),
                    refreshed_at: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "email" => Deserialize::begin(&mut self.builder.email),
                "id" => Deserialize::begin(&mut self.builder.id),
                "name" => Deserialize::begin(&mut self.builder.name),
                "ownership" => Deserialize::begin(&mut self.builder.ownership),
                "phone" => Deserialize::begin(&mut self.builder.phone),
                "raw_address" => Deserialize::begin(&mut self.builder.raw_address),
                "refreshed_at" => Deserialize::begin(&mut self.builder.refreshed_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(email),
                Some(id),
                Some(name),
                Some(ownership),
                Some(phone),
                Some(raw_address),
                Some(refreshed_at),
            ) = (
                self.builder.email.take(),
                self.builder.id.take(),
                self.builder.name.take(),
                self.builder.ownership.take(),
                self.builder.phone.take(),
                self.builder.raw_address.take(),
                self.builder.refreshed_at,
            )
            else {
                return Ok(());
            };
            *self.out = Some(FinancialConnectionsAccountOwner {
                email,
                id,
                name,
                ownership,
                phone,
                raw_address,
                refreshed_at,
            });
            Ok(())
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
