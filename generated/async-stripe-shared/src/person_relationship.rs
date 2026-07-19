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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PersonRelationshipBuilder {
                    authorizer: Deserialize::default(),
                    director: Deserialize::default(),
                    executive: Deserialize::default(),
                    legal_guardian: Deserialize::default(),
                    owner: Deserialize::default(),
                    percent_ownership: Deserialize::default(),
                    representative: Deserialize::default(),
                    title: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "authorizer" => Deserialize::begin(&mut self.builder.authorizer),
                "director" => Deserialize::begin(&mut self.builder.director),
                "executive" => Deserialize::begin(&mut self.builder.executive),
                "legal_guardian" => Deserialize::begin(&mut self.builder.legal_guardian),
                "owner" => Deserialize::begin(&mut self.builder.owner),
                "percent_ownership" => Deserialize::begin(&mut self.builder.percent_ownership),
                "representative" => Deserialize::begin(&mut self.builder.representative),
                "title" => Deserialize::begin(&mut self.builder.title),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(authorizer),
                Some(director),
                Some(executive),
                Some(legal_guardian),
                Some(owner),
                Some(percent_ownership),
                Some(representative),
                Some(title),
            ) = (
                self.builder.authorizer,
                self.builder.director,
                self.builder.executive,
                self.builder.legal_guardian,
                self.builder.owner,
                self.builder.percent_ownership,
                self.builder.representative,
                self.builder.title.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PersonRelationship {
                authorizer,
                director,
                executive,
                legal_guardian,
                owner,
                percent_ownership,
                representative,
                title,
            });
            Ok(())
        }
    }
};
