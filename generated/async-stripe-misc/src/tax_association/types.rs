/// A Tax Association exposes the Tax Transactions that Stripe attempted to create on your behalf based on the PaymentIntent input.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxAssociation {
    /// The [Tax Calculation](https://docs.stripe.com/api/tax/calculations/object) that was included in PaymentIntent.
    pub calculation: String,
    /// Unique identifier for the object.
    pub id: stripe_misc::TaxAssociationId,
    /// The [PaymentIntent](https://docs.stripe.com/api/payment_intents/object) that this Tax Association is tracking.
    pub payment_intent: String,
    /// Information about the tax transactions linked to this payment intent
    pub tax_transaction_attempts:
        Option<Vec<stripe_misc::TaxProductResourceTaxAssociationTransactionAttempts>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxAssociation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxAssociation").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxAssociationBuilder {
    calculation: Option<String>,
    id: Option<stripe_misc::TaxAssociationId>,
    payment_intent: Option<String>,
    tax_transaction_attempts:
        Option<Option<Vec<stripe_misc::TaxProductResourceTaxAssociationTransactionAttempts>>>,
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

    impl Deserialize for TaxAssociation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxAssociation>,
        builder: TaxAssociationBuilder,
    }

    impl Visitor for Place<TaxAssociation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxAssociationBuilder {
                    calculation: Deserialize::default(),
                    id: Deserialize::default(),
                    payment_intent: Deserialize::default(),
                    tax_transaction_attempts: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "calculation" => Deserialize::begin(&mut self.builder.calculation),
                "id" => Deserialize::begin(&mut self.builder.id),
                "payment_intent" => Deserialize::begin(&mut self.builder.payment_intent),
                "tax_transaction_attempts" => {
                    Deserialize::begin(&mut self.builder.tax_transaction_attempts)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(calculation), Some(id), Some(payment_intent), Some(tax_transaction_attempts)) = (
                self.builder.calculation.take(),
                self.builder.id.take(),
                self.builder.payment_intent.take(),
                self.builder.tax_transaction_attempts.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(TaxAssociation { calculation, id, payment_intent, tax_transaction_attempts });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxAssociation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TaxAssociation", 5)?;
        s.serialize_field("calculation", &self.calculation)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("payment_intent", &self.payment_intent)?;
        s.serialize_field("tax_transaction_attempts", &self.tax_transaction_attempts)?;

        s.serialize_field("object", "tax.association")?;
        s.end()
    }
}
impl stripe_types::Object for TaxAssociation {
    type Id = stripe_misc::TaxAssociationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TaxAssociationId);
