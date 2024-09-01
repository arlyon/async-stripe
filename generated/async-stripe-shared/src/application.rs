#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Application {
    /// Unique identifier for the object.
    pub id: stripe_shared::ApplicationId,
    /// The name of the application.
    pub name: Option<String>,
}
#[doc(hidden)]
pub struct ApplicationBuilder {
    id: Option<stripe_shared::ApplicationId>,
    name: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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
                builder: ApplicationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ApplicationBuilder {
        type Out = Application;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.id),
                "name" => Deserialize::begin(&mut self.name),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { id: Deserialize::default(), name: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(id), Some(name)) = (self.id.take(), self.name.take()) else {
                return None;
            };
            Some(Self::Out { id, name })
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

    impl ObjectDeser for Application {
        type Builder = ApplicationBuilder;
    }

    impl FromValueOpt for Application {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ApplicationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "id" => b.id = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
