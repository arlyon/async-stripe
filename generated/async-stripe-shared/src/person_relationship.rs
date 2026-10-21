#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PersonRelationship {
    /// Whether the person is the authorizer of the account's representative.
    pub authorizer: Option<bool>,
    /// Whether the person is a director of the account's legal entity.
    /// Directors are typically members of the governing board of the company, or responsible for ensuring the company meets its regulatory obligations.
    pub director: Option<bool>,
    /// Whether the person has significant responsibility to control, manage, or direct the organization.
    pub executive: Option<bool>,
    /// Whether the person is the legal guardian of the account's representative.
    pub legal_guardian: Option<bool>,
    /// Whether the person is an owner of the account’s legal entity.
    pub owner: Option<bool>,
    /// The percent owned by the person of the account's legal entity.
    pub percent_ownership: Option<f64>,
    /// Whether the person is authorized as the primary representative of the account.
    /// This is the person nominated by the business to provide information about themselves, and general information about the account.
    /// There can only be one representative at any given time.
    /// At the time the account is created, this person should be set to the person responsible for opening the account.
    pub representative: Option<bool>,
    /// The person's title (e.g., CEO, Support Engineer).
    pub title: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonRelationship").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PersonRelationshipBuilder {
    authorizer: Option<Option<bool>>,
    director: Option<Option<bool>>,
    executive: Option<Option<bool>>,
    legal_guardian: Option<Option<bool>>,
    owner: Option<Option<bool>>,
    percent_ownership: Option<Option<f64>>,
    representative: Option<Option<bool>>,
    title: Option<Option<String>>,
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

    impl Deserialize for PersonRelationship {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PersonRelationship>,
        builder: PersonRelationshipBuilder,
    }

    impl Visitor for Place<PersonRelationship> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PersonRelationshipBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PersonRelationshipBuilder {
        type Out = PersonRelationship;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "authorizer" => Deserialize::begin(&mut self.authorizer),
                "director" => Deserialize::begin(&mut self.director),
                "executive" => Deserialize::begin(&mut self.executive),
                "legal_guardian" => Deserialize::begin(&mut self.legal_guardian),
                "owner" => Deserialize::begin(&mut self.owner),
                "percent_ownership" => Deserialize::begin(&mut self.percent_ownership),
                "representative" => Deserialize::begin(&mut self.representative),
                "title" => Deserialize::begin(&mut self.title),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                authorizer: Some(None),
                director: Some(None),
                executive: Some(None),
                legal_guardian: Some(None),
                owner: Some(None),
                percent_ownership: Some(None),
                representative: Some(None),
                title: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    authorizer: Some(authorizer),
                    director: Some(director),
                    executive: Some(executive),
                    legal_guardian: Some(legal_guardian),
                    owner: Some(owner),
                    percent_ownership: Some(percent_ownership),
                    representative: Some(representative),
                    title: Some(title),
                } => Some(Self::Out {
                    authorizer,
                    director,
                    executive,
                    legal_guardian,
                    owner,
                    percent_ownership,
                    representative,
                    title,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PersonRelationship {
        type Builder = PersonRelationshipBuilder;
    }

    impl FromValueOpt for PersonRelationship {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
