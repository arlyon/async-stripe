#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Application {
    /// Unique identifier for the object.
    pub id: stripe_shared::ApplicationId,
    /// The name of the application.
    pub name: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Application").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ApplicationBuilder {
    id: Option<stripe_shared::ApplicationId>,
    name: Option<Option<String>>,
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

    impl Deserialize for Application {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Application>,
        builder: ApplicationBuilder,
    }

    impl Visitor for Place<Application> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ApplicationBuilder {
                    id: Deserialize::default(),
                    name: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.builder.id),
                "name" => Deserialize::begin(&mut self.builder.name),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(id), Some(name)) = (self.builder.id.take(), self.builder.name.take()) else {
                return Ok(());
            };
            *self.out = Some(Application { id, name });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Application {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Application", 3)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("name", &self.name)?;

        s.serialize_field("object", "application")?;
        s.end()
    }
}
impl stripe_types::Object for Application {
    type Id = stripe_shared::ApplicationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ApplicationId);
