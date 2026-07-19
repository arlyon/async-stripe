#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {
    /// Bank name.
    pub bank_name: Option<String>,
    /// The last four digits of the bank account number.
    pub last4: Option<String>,
    /// The routing number for the bank account.
    pub routing_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccountBuilder {
    bank_name: Option<Option<String>>,
    last4: Option<Option<String>>,
    routing_number: Option<Option<String>>,
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

    impl Deserialize for TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount>,
        builder: TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccountBuilder,
    }

    impl Visitor for Place<TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccountBuilder {
                    bank_name: Deserialize::default(),
                    last4: Deserialize::default(),
                    routing_number: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_name" => Deserialize::begin(&mut self.builder.bank_name),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "routing_number" => Deserialize::begin(&mut self.builder.routing_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(bank_name), Some(last4), Some(routing_number)) = (
                self.builder.bank_name.take(),
                self.builder.last4.take(),
                self.builder.routing_number.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {
                bank_name,
                last4,
                routing_number,
            });
            Ok(())
        }
    }
};
