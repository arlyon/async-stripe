/// Describes a snapshot of the owners of an account at a particular point in time.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FinancialConnectionsAccountOwnership {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_misc::FinancialConnectionsAccountOwnershipId,
    /// A paginated list of owners for this account.
    pub owners: stripe_types::List<stripe_misc::FinancialConnectionsAccountOwner>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsAccountOwnership {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FinancialConnectionsAccountOwnership").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct FinancialConnectionsAccountOwnershipBuilder {
    created: Option<stripe_types::Timestamp>,
    id: Option<stripe_misc::FinancialConnectionsAccountOwnershipId>,
    owners: Option<stripe_types::List<stripe_misc::FinancialConnectionsAccountOwner>>,
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

    impl Deserialize for FinancialConnectionsAccountOwnership {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FinancialConnectionsAccountOwnership>,
        builder: FinancialConnectionsAccountOwnershipBuilder,
    }

    impl Visitor for Place<FinancialConnectionsAccountOwnership> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FinancialConnectionsAccountOwnershipBuilder {
                    created: Deserialize::default(),
                    id: Deserialize::default(),
                    owners: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "id" => Deserialize::begin(&mut self.builder.id),
                "owners" => Deserialize::begin(&mut self.builder.owners),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(created), Some(id), Some(owners)) =
                (self.builder.created, self.builder.id.take(), self.builder.owners.take())
            else {
                return Ok(());
            };
            *self.out = Some(FinancialConnectionsAccountOwnership { created, id, owners });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsAccountOwnership {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("FinancialConnectionsAccountOwnership", 4)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("owners", &self.owners)?;

        s.serialize_field("object", "financial_connections.account_ownership")?;
        s.end()
    }
}
impl stripe_types::Object for FinancialConnectionsAccountOwnership {
    type Id = stripe_misc::FinancialConnectionsAccountOwnershipId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(FinancialConnectionsAccountOwnershipId);
