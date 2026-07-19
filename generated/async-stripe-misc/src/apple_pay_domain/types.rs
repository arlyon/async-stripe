#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ApplePayDomain {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    pub domain_name: String,
    /// Unique identifier for the object.
    pub id: stripe_misc::ApplePayDomainId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ApplePayDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ApplePayDomain").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ApplePayDomainBuilder {
    created: Option<stripe_types::Timestamp>,
    domain_name: Option<String>,
    id: Option<stripe_misc::ApplePayDomainId>,
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

    impl Deserialize for ApplePayDomain {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ApplePayDomain>,
        builder: ApplePayDomainBuilder,
    }

    impl Visitor for Place<ApplePayDomain> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ApplePayDomainBuilder {
                    created: Deserialize::default(),
                    domain_name: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "domain_name" => Deserialize::begin(&mut self.builder.domain_name),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(created), Some(domain_name), Some(id), Some(livemode)) = (
                self.builder.created,
                self.builder.domain_name.take(),
                self.builder.id.take(),
                self.builder.livemode,
            ) else {
                return Ok(());
            };
            *self.out = Some(ApplePayDomain { created, domain_name, id, livemode });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ApplePayDomain {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ApplePayDomain", 5)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("domain_name", &self.domain_name)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;

        s.serialize_field("object", "apple_pay_domain")?;
        s.end()
    }
}
impl stripe_types::Object for ApplePayDomain {
    type Id = stripe_misc::ApplePayDomainId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ApplePayDomainId);
