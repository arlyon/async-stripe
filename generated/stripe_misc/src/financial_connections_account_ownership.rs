/// Describes a snapshot of the owners of an account at a particular point in time.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FinancialConnectionsAccountOwnership {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_misc::FinancialConnectionsAccountOwnershipId,
    /// A paginated list of owners for this account.
    pub owners: stripe_types::List<stripe_misc::FinancialConnectionsAccountOwner>,
}
#[cfg(feature = "min-ser")]
pub struct FinancialConnectionsAccountOwnershipBuilder {
    created: Option<stripe_types::Timestamp>,
    id: Option<stripe_misc::FinancialConnectionsAccountOwnershipId>,
    owners: Option<stripe_types::List<stripe_misc::FinancialConnectionsAccountOwner>>,
}

#[cfg(feature = "min-ser")]
#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
            Ok(Box::new(Builder { out: &mut self.out, builder: FinancialConnectionsAccountOwnershipBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FinancialConnectionsAccountOwnershipBuilder {
        type Out = FinancialConnectionsAccountOwnership;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "id" => Deserialize::begin(&mut self.id),
                "owners" => Deserialize::begin(&mut self.owners),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { created: Deserialize::default(), id: Deserialize::default(), owners: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let created = self.created.take()?;
            let id = self.id.take()?;
            let owners = self.owners.take()?;

            Some(Self::Out { created, id, owners })
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

    impl ObjectDeser for FinancialConnectionsAccountOwnership {
        type Builder = FinancialConnectionsAccountOwnershipBuilder;
    }

    impl FromValueOpt for FinancialConnectionsAccountOwnership {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FinancialConnectionsAccountOwnershipBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "owners" => b.owners = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
impl stripe_types::Object for FinancialConnectionsAccountOwnership {
    type Id = stripe_misc::FinancialConnectionsAccountOwnershipId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(FinancialConnectionsAccountOwnershipId);
