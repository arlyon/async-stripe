/// Encodes whether a FinancialAccount has access to a particular Feature, with a `status` enum and associated `status_details`.
/// Stripe or the platform can control Features via the requested field.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountFeatures {
    pub card_issuing: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    pub deposit_insurance: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    pub financial_addresses:
        Option<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAddressesFeatures>,
    pub inbound_transfers:
        Option<stripe_treasury::TreasuryFinancialAccountsResourceInboundTransfers>,
    pub intra_stripe_flows:
        Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    pub outbound_payments:
        Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundPayments>,
    pub outbound_transfers:
        Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundTransfers>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryFinancialAccountFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryFinancialAccountFeatures").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountFeaturesBuilder {
    card_issuing: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>>,
    deposit_insurance:
        Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>>,
    financial_addresses: Option<
        Option<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAddressesFeatures>,
    >,
    inbound_transfers:
        Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceInboundTransfers>>,
    intra_stripe_flows:
        Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>>,
    outbound_payments:
        Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundPayments>>,
    outbound_transfers:
        Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundTransfers>>,
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

    impl Deserialize for TreasuryFinancialAccountFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountFeatures>,
        builder: TreasuryFinancialAccountFeaturesBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountFeaturesBuilder {
                    card_issuing: Deserialize::default(),
                    deposit_insurance: Deserialize::default(),
                    financial_addresses: Deserialize::default(),
                    inbound_transfers: Deserialize::default(),
                    intra_stripe_flows: Deserialize::default(),
                    outbound_payments: Deserialize::default(),
                    outbound_transfers: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_issuing" => Deserialize::begin(&mut self.builder.card_issuing),
                "deposit_insurance" => Deserialize::begin(&mut self.builder.deposit_insurance),
                "financial_addresses" => Deserialize::begin(&mut self.builder.financial_addresses),
                "inbound_transfers" => Deserialize::begin(&mut self.builder.inbound_transfers),
                "intra_stripe_flows" => Deserialize::begin(&mut self.builder.intra_stripe_flows),
                "outbound_payments" => Deserialize::begin(&mut self.builder.outbound_payments),
                "outbound_transfers" => Deserialize::begin(&mut self.builder.outbound_transfers),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(card_issuing),
                Some(deposit_insurance),
                Some(financial_addresses),
                Some(inbound_transfers),
                Some(intra_stripe_flows),
                Some(outbound_payments),
                Some(outbound_transfers),
            ) = (
                self.builder.card_issuing.take(),
                self.builder.deposit_insurance.take(),
                self.builder.financial_addresses.take(),
                self.builder.inbound_transfers.take(),
                self.builder.intra_stripe_flows.take(),
                self.builder.outbound_payments.take(),
                self.builder.outbound_transfers.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TreasuryFinancialAccountFeatures {
                card_issuing,
                deposit_insurance,
                financial_addresses,
                inbound_transfers,
                intra_stripe_flows,
                outbound_payments,
                outbound_transfers,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountFeatures {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TreasuryFinancialAccountFeatures", 8)?;
        s.serialize_field("card_issuing", &self.card_issuing)?;
        s.serialize_field("deposit_insurance", &self.deposit_insurance)?;
        s.serialize_field("financial_addresses", &self.financial_addresses)?;
        s.serialize_field("inbound_transfers", &self.inbound_transfers)?;
        s.serialize_field("intra_stripe_flows", &self.intra_stripe_flows)?;
        s.serialize_field("outbound_payments", &self.outbound_payments)?;
        s.serialize_field("outbound_transfers", &self.outbound_transfers)?;

        s.serialize_field("object", "treasury.financial_account_features")?;
        s.end()
    }
}
