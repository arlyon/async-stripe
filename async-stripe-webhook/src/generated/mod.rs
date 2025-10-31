#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum AccountExternalAccountCreated {
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "bank_account"))]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct AccountExternalAccountCreatedBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::MapBuilder;
    use stripe_types::miniserde_helpers::FromValueOpt;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<AccountExternalAccountCreated>,
        builder: AccountExternalAccountCreatedBuilder,
    }

    impl Deserialize for AccountExternalAccountCreated {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<AccountExternalAccountCreated> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for AccountExternalAccountCreatedBuilder {
        type Out = AccountExternalAccountCreated;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            AccountExternalAccountCreated::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for AccountExternalAccountCreated {
        type Builder = AccountExternalAccountCreatedBuilder;
    }
    impl AccountExternalAccountCreated {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "bank_account" => Self::BankAccount(FromValueOpt::from_value(Value::Object(o))?),
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for AccountExternalAccountCreated {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum AccountExternalAccountDeleted {
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "bank_account"))]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct AccountExternalAccountDeletedBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::MapBuilder;
    use stripe_types::miniserde_helpers::FromValueOpt;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<AccountExternalAccountDeleted>,
        builder: AccountExternalAccountDeletedBuilder,
    }

    impl Deserialize for AccountExternalAccountDeleted {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<AccountExternalAccountDeleted> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for AccountExternalAccountDeletedBuilder {
        type Out = AccountExternalAccountDeleted;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            AccountExternalAccountDeleted::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for AccountExternalAccountDeleted {
        type Builder = AccountExternalAccountDeletedBuilder;
    }
    impl AccountExternalAccountDeleted {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "bank_account" => Self::BankAccount(FromValueOpt::from_value(Value::Object(o))?),
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for AccountExternalAccountDeleted {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum AccountExternalAccountUpdated {
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "bank_account"))]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct AccountExternalAccountUpdatedBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::MapBuilder;
    use stripe_types::miniserde_helpers::FromValueOpt;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<AccountExternalAccountUpdated>,
        builder: AccountExternalAccountUpdatedBuilder,
    }

    impl Deserialize for AccountExternalAccountUpdated {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<AccountExternalAccountUpdated> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for AccountExternalAccountUpdatedBuilder {
        type Out = AccountExternalAccountUpdated;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            AccountExternalAccountUpdated::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for AccountExternalAccountUpdated {
        type Builder = AccountExternalAccountUpdatedBuilder;
    }
    impl AccountExternalAccountUpdated {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "bank_account" => Self::BankAccount(FromValueOpt::from_value(Value::Object(o))?),
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for AccountExternalAccountUpdated {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum CustomerSourceCreated {
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "bank_account"))]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct CustomerSourceCreatedBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::MapBuilder;
    use stripe_types::miniserde_helpers::FromValueOpt;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<CustomerSourceCreated>,
        builder: CustomerSourceCreatedBuilder,
    }

    impl Deserialize for CustomerSourceCreated {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<CustomerSourceCreated> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for CustomerSourceCreatedBuilder {
        type Out = CustomerSourceCreated;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            CustomerSourceCreated::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for CustomerSourceCreated {
        type Builder = CustomerSourceCreatedBuilder;
    }
    impl CustomerSourceCreated {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "bank_account" => Self::BankAccount(FromValueOpt::from_value(Value::Object(o))?),
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for CustomerSourceCreated {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum CustomerSourceDeleted {
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "bank_account"))]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct CustomerSourceDeletedBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::MapBuilder;
    use stripe_types::miniserde_helpers::FromValueOpt;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<CustomerSourceDeleted>,
        builder: CustomerSourceDeletedBuilder,
    }

    impl Deserialize for CustomerSourceDeleted {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<CustomerSourceDeleted> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for CustomerSourceDeletedBuilder {
        type Out = CustomerSourceDeleted;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            CustomerSourceDeleted::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for CustomerSourceDeleted {
        type Builder = CustomerSourceDeletedBuilder;
    }
    impl CustomerSourceDeleted {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "bank_account" => Self::BankAccount(FromValueOpt::from_value(Value::Object(o))?),
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for CustomerSourceDeleted {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum CustomerSourceExpiring {
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct CustomerSourceExpiringBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::MapBuilder;
    use stripe_types::miniserde_helpers::FromValueOpt;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<CustomerSourceExpiring>,
        builder: CustomerSourceExpiringBuilder,
    }

    impl Deserialize for CustomerSourceExpiring {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<CustomerSourceExpiring> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for CustomerSourceExpiringBuilder {
        type Out = CustomerSourceExpiring;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            CustomerSourceExpiring::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for CustomerSourceExpiring {
        type Builder = CustomerSourceExpiringBuilder;
    }
    impl CustomerSourceExpiring {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for CustomerSourceExpiring {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum CustomerSourceUpdated {
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "bank_account"))]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct CustomerSourceUpdatedBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::MapBuilder;
    use stripe_types::miniserde_helpers::FromValueOpt;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<CustomerSourceUpdated>,
        builder: CustomerSourceUpdatedBuilder,
    }

    impl Deserialize for CustomerSourceUpdated {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<CustomerSourceUpdated> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for CustomerSourceUpdatedBuilder {
        type Out = CustomerSourceUpdated;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            CustomerSourceUpdated::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for CustomerSourceUpdated {
        type Builder = CustomerSourceUpdatedBuilder;
    }
    impl CustomerSourceUpdated {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "bank_account" => Self::BankAccount(FromValueOpt::from_value(Value::Object(o))?),
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for CustomerSourceUpdated {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[non_exhaustive]
/// The event data for a webhook event.
pub enum EventObject {
    /// Occurs whenever a user authorizes an application. Sent to the related application only.
    AccountApplicationAuthorized(stripe_shared::Application),
    /// Occurs whenever a user deauthorizes an application. Sent to the related application only.
    AccountApplicationDeauthorized(stripe_shared::Application),
    /// Occurs whenever an external account is created.
    AccountExternalAccountCreated(AccountExternalAccountCreated),
    /// Occurs whenever an external account is deleted.
    AccountExternalAccountDeleted(AccountExternalAccountDeleted),
    /// Occurs whenever an external account is updated.
    AccountExternalAccountUpdated(AccountExternalAccountUpdated),
    /// Occurs whenever an account status or property has changed.
    AccountUpdated(stripe_shared::Account),
    /// Occurs whenever an application fee is created on a charge.
    ApplicationFeeCreated(stripe_shared::ApplicationFee),
    /// Occurs whenever an application fee refund is updated.
    ApplicationFeeRefundUpdated(stripe_shared::ApplicationFeeRefund),
    /// Occurs whenever an application fee is refunded, whether from refunding a charge or from [refunding the application fee directly](#fee_refunds).
    /// This includes partial refunds.
    ApplicationFeeRefunded(stripe_shared::ApplicationFee),
    /// Occurs whenever your Stripe balance has been updated (e.g., when a charge is available to be paid out).
    /// By default, Stripe automatically transfers funds in your balance to your bank account on a daily basis.
    /// This event is not fired for negative transactions.
    #[cfg(feature = "async-stripe-core")]
    BalanceAvailable(stripe_core::Balance),
    /// Occurs whenever a balance settings status or property has changed.
    #[cfg(feature = "async-stripe-core")]
    BalanceSettingsUpdated(stripe_core::BalanceSettings),
    /// Occurs whenever your custom alert threshold is met.
    #[cfg(feature = "async-stripe-billing")]
    BillingAlertTriggered(stripe_billing::BillingAlertTriggered),
    /// Occurs when a credit balance transaction is created
    BillingCreditBalanceTransactionCreated(stripe_shared::BillingCreditBalanceTransaction),
    /// Occurs when a credit grant is created
    BillingCreditGrantCreated(stripe_shared::BillingCreditGrant),
    /// Occurs when a credit grant is updated
    BillingCreditGrantUpdated(stripe_shared::BillingCreditGrant),
    /// Occurs when a meter is created
    #[cfg(feature = "async-stripe-billing")]
    BillingMeterCreated(stripe_billing::BillingMeter),
    /// Occurs when a meter is deactivated
    #[cfg(feature = "async-stripe-billing")]
    BillingMeterDeactivated(stripe_billing::BillingMeter),
    /// Occurs when a meter is reactivated
    #[cfg(feature = "async-stripe-billing")]
    BillingMeterReactivated(stripe_billing::BillingMeter),
    /// Occurs when a meter is updated
    #[cfg(feature = "async-stripe-billing")]
    BillingMeterUpdated(stripe_billing::BillingMeter),
    /// Occurs whenever a portal configuration is created.
    #[cfg(feature = "async-stripe-billing")]
    BillingPortalConfigurationCreated(stripe_billing::BillingPortalConfiguration),
    /// Occurs whenever a portal configuration is updated.
    #[cfg(feature = "async-stripe-billing")]
    BillingPortalConfigurationUpdated(stripe_billing::BillingPortalConfiguration),
    /// Occurs whenever a portal session is created.
    #[cfg(feature = "async-stripe-billing")]
    BillingPortalSessionCreated(stripe_billing::BillingPortalSession),
    /// Occurs whenever a capability has new requirements or a new status.
    CapabilityUpdated(stripe_shared::Capability),
    /// Occurs whenever there is a positive remaining cash balance after Stripe automatically reconciles new funds into the cash balance.
    /// If you enabled manual reconciliation, this webhook will fire whenever there are new funds into the cash balance.
    CashBalanceFundsAvailable(stripe_shared::CashBalance),
    /// Occurs whenever a previously uncaptured charge is captured.
    ChargeCaptured(stripe_shared::Charge),
    /// Occurs when a dispute is closed and the dispute status changes to `lost`, `warning_closed`, or `won`.
    ChargeDisputeClosed(stripe_shared::Dispute),
    /// Occurs whenever a customer disputes a charge with their bank.
    ChargeDisputeCreated(stripe_shared::Dispute),
    /// Occurs when funds are reinstated to your account after a dispute is closed.
    /// This includes [partially refunded payments](https://docs.stripe.com/disputes#disputes-on-partially-refunded-payments).
    ChargeDisputeFundsReinstated(stripe_shared::Dispute),
    /// Occurs when funds are removed from your account due to a dispute.
    ChargeDisputeFundsWithdrawn(stripe_shared::Dispute),
    /// Occurs when the dispute is updated (usually with evidence).
    ChargeDisputeUpdated(stripe_shared::Dispute),
    /// Occurs whenever an uncaptured charge expires.
    ChargeExpired(stripe_shared::Charge),
    /// Occurs whenever a failed charge attempt occurs.
    ChargeFailed(stripe_shared::Charge),
    /// Occurs whenever a pending charge is created.
    ChargePending(stripe_shared::Charge),
    /// Occurs whenever a refund is updated on selected payment methods.
    /// For updates on all refunds, listen to `refund.updated` instead.
    ChargeRefundUpdated(stripe_shared::Refund),
    /// Occurs whenever a charge is refunded, including partial refunds.
    /// Listen to `refund.created` for information about the refund.
    ChargeRefunded(stripe_shared::Charge),
    /// Occurs whenever a charge is successful.
    ChargeSucceeded(stripe_shared::Charge),
    /// Occurs whenever a charge description or metadata is updated, or upon an asynchronous capture.
    ChargeUpdated(stripe_shared::Charge),
    /// Occurs when a payment intent using a delayed payment method fails.
    CheckoutSessionAsyncPaymentFailed(stripe_shared::CheckoutSession),
    /// Occurs when a payment intent using a delayed payment method finally succeeds.
    CheckoutSessionAsyncPaymentSucceeded(stripe_shared::CheckoutSession),
    /// Occurs when a Checkout Session has been successfully completed.
    CheckoutSessionCompleted(stripe_shared::CheckoutSession),
    /// Occurs when a Checkout Session is expired.
    CheckoutSessionExpired(stripe_shared::CheckoutSession),
    /// Occurs when a Climate order is canceled.
    #[cfg(feature = "async-stripe-misc")]
    ClimateOrderCanceled(stripe_misc::ClimateOrder),
    /// Occurs when a Climate order is created.
    #[cfg(feature = "async-stripe-misc")]
    ClimateOrderCreated(stripe_misc::ClimateOrder),
    /// Occurs when a Climate order is delayed.
    #[cfg(feature = "async-stripe-misc")]
    ClimateOrderDelayed(stripe_misc::ClimateOrder),
    /// Occurs when a Climate order is delivered.
    #[cfg(feature = "async-stripe-misc")]
    ClimateOrderDelivered(stripe_misc::ClimateOrder),
    /// Occurs when a Climate order's product is substituted for another.
    #[cfg(feature = "async-stripe-misc")]
    ClimateOrderProductSubstituted(stripe_misc::ClimateOrder),
    /// Occurs when a Climate product is created.
    #[cfg(feature = "async-stripe-misc")]
    ClimateProductCreated(stripe_misc::ClimateProduct),
    /// Occurs when a Climate product is updated.
    #[cfg(feature = "async-stripe-misc")]
    ClimateProductPricingUpdated(stripe_misc::ClimateProduct),
    /// Occurs whenever a coupon is created.
    CouponCreated(stripe_shared::Coupon),
    /// Occurs whenever a coupon is deleted.
    CouponDeleted(stripe_shared::Coupon),
    /// Occurs whenever a coupon is updated.
    CouponUpdated(stripe_shared::Coupon),
    /// Occurs whenever a credit note is created.
    CreditNoteCreated(stripe_shared::CreditNote),
    /// Occurs whenever a credit note is updated.
    CreditNoteUpdated(stripe_shared::CreditNote),
    /// Occurs whenever a credit note is voided.
    CreditNoteVoided(stripe_shared::CreditNote),
    /// Occurs whenever a new customer is created.
    CustomerCreated(stripe_shared::Customer),
    /// Occurs whenever a customer is deleted.
    CustomerDeleted(stripe_shared::Customer),
    /// Occurs whenever a coupon is attached to a customer.
    CustomerDiscountCreated(stripe_shared::Discount),
    /// Occurs whenever a coupon is removed from a customer.
    CustomerDiscountDeleted(stripe_shared::Discount),
    /// Occurs whenever a customer is switched from one coupon to another.
    CustomerDiscountUpdated(stripe_shared::Discount),
    /// Occurs whenever a new source is created for a customer.
    CustomerSourceCreated(CustomerSourceCreated),
    /// Occurs whenever a source is removed from a customer.
    CustomerSourceDeleted(CustomerSourceDeleted),
    /// Occurs whenever a card or source will expire at the end of the month.
    /// This event only works with legacy integrations using Card or Source objects.
    /// If you use the PaymentMethod API, this event won't occur.
    CustomerSourceExpiring(CustomerSourceExpiring),
    /// Occurs whenever a source's details are changed.
    CustomerSourceUpdated(CustomerSourceUpdated),
    /// Occurs whenever a customer is signed up for a new plan.
    CustomerSubscriptionCreated(stripe_shared::Subscription),
    /// Occurs whenever a customer's subscription ends.
    CustomerSubscriptionDeleted(stripe_shared::Subscription),
    /// Occurs whenever a customer's subscription is paused.
    /// Only applies when subscriptions enter `status=paused`, not when [payment collection](https://docs.stripe.com/billing/subscriptions/pause) is paused.
    CustomerSubscriptionPaused(stripe_shared::Subscription),
    /// Occurs whenever a customer's subscription's pending update is applied, and the subscription is updated.
    CustomerSubscriptionPendingUpdateApplied(stripe_shared::Subscription),
    /// Occurs whenever a customer's subscription's pending update expires before the related invoice is paid.
    CustomerSubscriptionPendingUpdateExpired(stripe_shared::Subscription),
    /// Occurs whenever a customer's subscription is no longer paused.
    /// Only applies when a `status=paused` subscription is [resumed](https://docs.stripe.com/api/subscriptions/resume), not when [payment collection](https://docs.stripe.com/billing/subscriptions/pause) is resumed.
    CustomerSubscriptionResumed(stripe_shared::Subscription),
    /// Occurs three days before a subscription's trial period is scheduled to end, or when a trial is ended immediately (using `trial_end=now`).
    CustomerSubscriptionTrialWillEnd(stripe_shared::Subscription),
    /// Occurs whenever a subscription changes (e.g., switching from one plan to another, or changing the status from trial to active).
    CustomerSubscriptionUpdated(stripe_shared::Subscription),
    /// Occurs whenever a tax ID is created for a customer.
    CustomerTaxIdCreated(stripe_shared::TaxId),
    /// Occurs whenever a tax ID is deleted from a customer.
    CustomerTaxIdDeleted(stripe_shared::TaxId),
    /// Occurs whenever a customer's tax ID is updated.
    CustomerTaxIdUpdated(stripe_shared::TaxId),
    /// Occurs whenever any property of a customer changes.
    CustomerUpdated(stripe_shared::Customer),
    /// Occurs whenever a new customer cash balance transactions is created.
    CustomerCashBalanceTransactionCreated(stripe_shared::CustomerCashBalanceTransaction),
    /// Occurs whenever a customer's entitlements change.
    #[cfg(feature = "async-stripe-misc")]
    EntitlementsActiveEntitlementSummaryUpdated(stripe_misc::EntitlementsActiveEntitlementSummary),
    /// Occurs whenever a new Stripe-generated file is available for your account.
    FileCreated(stripe_shared::File),
    /// Occurs when a new Financial Connections account is created.
    #[cfg(feature = "async-stripe-misc")]
    FinancialConnectionsAccountCreated(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when a Financial Connections account's status is updated from `active` to `inactive`.
    #[cfg(feature = "async-stripe-misc")]
    FinancialConnectionsAccountDeactivated(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when a Financial Connections account is disconnected.
    #[cfg(feature = "async-stripe-misc")]
    FinancialConnectionsAccountDisconnected(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when a Financial Connections account's status is updated from `inactive` to `active`.
    #[cfg(feature = "async-stripe-misc")]
    FinancialConnectionsAccountReactivated(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when an Account’s `balance_refresh` status transitions from `pending` to either `succeeded` or `failed`.
    #[cfg(feature = "async-stripe-misc")]
    FinancialConnectionsAccountRefreshedBalance(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when an Account’s `ownership_refresh` status transitions from `pending` to either `succeeded` or `failed`.
    #[cfg(feature = "async-stripe-misc")]
    FinancialConnectionsAccountRefreshedOwnership(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when an Account’s `transaction_refresh` status transitions from `pending` to either `succeeded` or `failed`.
    #[cfg(feature = "async-stripe-misc")]
    FinancialConnectionsAccountRefreshedTransactions(stripe_misc::FinancialConnectionsAccount),
    /// Occurs whenever a VerificationSession is canceled
    #[cfg(feature = "async-stripe-misc")]
    IdentityVerificationSessionCanceled(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a VerificationSession is created
    #[cfg(feature = "async-stripe-misc")]
    IdentityVerificationSessionCreated(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a VerificationSession transitions to processing
    #[cfg(feature = "async-stripe-misc")]
    IdentityVerificationSessionProcessing(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a VerificationSession is redacted.
    #[cfg(feature = "async-stripe-misc")]
    IdentityVerificationSessionRedacted(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a VerificationSession transitions to require user input
    #[cfg(feature = "async-stripe-misc")]
    IdentityVerificationSessionRequiresInput(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a VerificationSession transitions to verified
    #[cfg(feature = "async-stripe-misc")]
    IdentityVerificationSessionVerified(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a new invoice is created.
    /// To learn how webhooks can be used with this event, and how they can affect it, see [Using Webhooks with Subscriptions](https://docs.stripe.com/subscriptions/webhooks).
    InvoiceCreated(stripe_shared::Invoice),
    /// Occurs whenever a draft invoice is deleted.
    /// Note: This event is not sent for [invoice previews](https://docs.stripe.com/api/invoices/create_preview).
    InvoiceDeleted(stripe_shared::Invoice),
    /// Occurs whenever a draft invoice cannot be finalized.
    /// See the invoice’s [last finalization error](https://docs.stripe.com/api/invoices/object#invoice_object-last_finalization_error) for details.
    InvoiceFinalizationFailed(stripe_shared::Invoice),
    /// Occurs whenever a draft invoice is finalized and updated to be an open invoice.
    InvoiceFinalized(stripe_shared::Invoice),
    /// Occurs whenever an invoice is marked uncollectible.
    InvoiceMarkedUncollectible(stripe_shared::Invoice),
    /// Occurs X number of days after an invoice becomes due&mdash;where X is determined by Automations
    InvoiceOverdue(stripe_shared::Invoice),
    /// Occurs when an invoice transitions to paid with a non-zero amount_overpaid.
    InvoiceOverpaid(stripe_shared::Invoice),
    /// Occurs whenever an invoice payment attempt succeeds or an invoice is marked as paid out-of-band.
    InvoicePaid(stripe_shared::Invoice),
    /// Occurs whenever an invoice payment attempt requires further user action to complete.
    InvoicePaymentActionRequired(stripe_shared::Invoice),
    /// Occurs when an invoice requires a payment using a payment method that cannot be processed by Stripe.
    InvoicePaymentAttemptRequired(stripe_shared::Invoice),
    /// Occurs whenever an invoice payment attempt fails, due to either a declined payment, including soft decline, or to the lack of a stored payment method.
    InvoicePaymentFailed(stripe_shared::Invoice),
    /// Occurs whenever an invoice payment attempt succeeds.
    InvoicePaymentSucceeded(stripe_shared::Invoice),
    /// Occurs whenever an invoice email is sent out.
    InvoiceSent(stripe_shared::Invoice),
    /// Occurs X number of days before a subscription is scheduled to create an invoice that is automatically charged&mdash;where X is determined by your [subscriptions settings](https://dashboard.stripe.com/account/billing/automatic).
    /// Note: The received `Invoice` object will not have an invoice ID.
    InvoiceUpcoming(stripe_shared::Invoice),
    /// Occurs whenever an invoice changes (e.g., the invoice amount).
    InvoiceUpdated(stripe_shared::Invoice),
    /// Occurs whenever an invoice is voided.
    InvoiceVoided(stripe_shared::Invoice),
    /// Occurs X number of days before an invoice becomes due&mdash;where X is determined by Automations
    InvoiceWillBeDue(stripe_shared::Invoice),
    /// Occurs when an InvoicePayment is successfully paid.
    InvoicePaymentPaid(stripe_shared::InvoicePayment),
    /// Occurs whenever an invoice item is created.
    #[cfg(feature = "async-stripe-billing")]
    InvoiceitemCreated(stripe_billing::InvoiceItem),
    /// Occurs whenever an invoice item is deleted.
    #[cfg(feature = "async-stripe-billing")]
    InvoiceitemDeleted(stripe_billing::InvoiceItem),
    /// Occurs whenever an authorization is created.
    IssuingAuthorizationCreated(stripe_shared::IssuingAuthorization),
    /// Represents a synchronous request for authorization, see [Using your integration to handle authorization requests](https://docs.stripe.com/issuing/purchases/authorizations#authorization-handling).
    IssuingAuthorizationRequest(stripe_shared::IssuingAuthorization),
    /// Occurs whenever an authorization is updated.
    IssuingAuthorizationUpdated(stripe_shared::IssuingAuthorization),
    /// Occurs whenever a card is created.
    IssuingCardCreated(stripe_shared::IssuingCard),
    /// Occurs whenever a card is updated.
    IssuingCardUpdated(stripe_shared::IssuingCard),
    /// Occurs whenever a cardholder is created.
    IssuingCardholderCreated(stripe_shared::IssuingCardholder),
    /// Occurs whenever a cardholder is updated.
    IssuingCardholderUpdated(stripe_shared::IssuingCardholder),
    /// Occurs whenever a dispute is won, lost or expired.
    IssuingDisputeClosed(stripe_shared::IssuingDispute),
    /// Occurs whenever a dispute is created.
    IssuingDisputeCreated(stripe_shared::IssuingDispute),
    /// Occurs whenever funds are reinstated to your account for an Issuing dispute.
    IssuingDisputeFundsReinstated(stripe_shared::IssuingDispute),
    /// Occurs whenever funds are deducted from your account for an Issuing dispute.
    IssuingDisputeFundsRescinded(stripe_shared::IssuingDispute),
    /// Occurs whenever a dispute is submitted.
    IssuingDisputeSubmitted(stripe_shared::IssuingDispute),
    /// Occurs whenever a dispute is updated.
    IssuingDisputeUpdated(stripe_shared::IssuingDispute),
    /// Occurs whenever a personalization design is activated following the activation of the physical bundle that belongs to it.
    IssuingPersonalizationDesignActivated(stripe_shared::IssuingPersonalizationDesign),
    /// Occurs whenever a personalization design is deactivated following the deactivation of the physical bundle that belongs to it.
    IssuingPersonalizationDesignDeactivated(stripe_shared::IssuingPersonalizationDesign),
    /// Occurs whenever a personalization design is rejected by design review.
    IssuingPersonalizationDesignRejected(stripe_shared::IssuingPersonalizationDesign),
    /// Occurs whenever a personalization design is updated.
    IssuingPersonalizationDesignUpdated(stripe_shared::IssuingPersonalizationDesign),
    /// Occurs whenever an issuing digital wallet token is created.
    IssuingTokenCreated(stripe_shared::IssuingToken),
    /// Occurs whenever an issuing digital wallet token is updated.
    IssuingTokenUpdated(stripe_shared::IssuingToken),
    /// Occurs whenever an issuing transaction is created.
    IssuingTransactionCreated(stripe_shared::IssuingTransaction),
    /// Occurs whenever an issuing transaction is updated with receipt data.
    IssuingTransactionPurchaseDetailsReceiptUpdated(stripe_shared::IssuingTransaction),
    /// Occurs whenever an issuing transaction is updated.
    IssuingTransactionUpdated(stripe_shared::IssuingTransaction),
    /// Occurs whenever a Mandate is updated.
    MandateUpdated(stripe_shared::Mandate),
    /// Occurs when a PaymentIntent has funds to be captured.
    /// Check the `amount_capturable` property on the PaymentIntent to determine the amount that can be captured.
    /// You may capture the PaymentIntent with an `amount_to_capture` value up to the specified amount.
    /// [Learn more about capturing PaymentIntents.](https://docs.stripe.com/api/payment_intents/capture).
    PaymentIntentAmountCapturableUpdated(stripe_shared::PaymentIntent),
    /// Occurs when a PaymentIntent is canceled.
    PaymentIntentCanceled(stripe_shared::PaymentIntent),
    /// Occurs when a new PaymentIntent is created.
    PaymentIntentCreated(stripe_shared::PaymentIntent),
    /// Occurs when funds are applied to a customer_balance PaymentIntent and the 'amount_remaining' changes.
    PaymentIntentPartiallyFunded(stripe_shared::PaymentIntent),
    /// Occurs when a PaymentIntent has failed the attempt to create a payment method or a payment.
    PaymentIntentPaymentFailed(stripe_shared::PaymentIntent),
    /// Occurs when a PaymentIntent has started processing.
    PaymentIntentProcessing(stripe_shared::PaymentIntent),
    /// Occurs when a PaymentIntent transitions to requires_action state
    PaymentIntentRequiresAction(stripe_shared::PaymentIntent),
    /// Occurs when a PaymentIntent has successfully completed payment.
    PaymentIntentSucceeded(stripe_shared::PaymentIntent),
    /// Occurs when a payment link is created.
    PaymentLinkCreated(stripe_shared::PaymentLink),
    /// Occurs when a payment link is updated.
    PaymentLinkUpdated(stripe_shared::PaymentLink),
    /// Occurs whenever a new payment method is attached to a customer.
    PaymentMethodAttached(stripe_shared::PaymentMethod),
    /// Occurs whenever a payment method's details are automatically updated by the network.
    PaymentMethodAutomaticallyUpdated(stripe_shared::PaymentMethod),
    /// Occurs whenever a payment method is detached from a customer.
    PaymentMethodDetached(stripe_shared::PaymentMethod),
    /// Occurs whenever a payment method is updated via the [PaymentMethod update API](https://docs.stripe.com/api/payment_methods/update).
    PaymentMethodUpdated(stripe_shared::PaymentMethod),
    /// Occurs whenever a payout is canceled.
    PayoutCanceled(stripe_shared::Payout),
    /// Occurs whenever a payout is created.
    PayoutCreated(stripe_shared::Payout),
    /// Occurs whenever a payout attempt fails.
    PayoutFailed(stripe_shared::Payout),
    /// Occurs whenever a payout is *expected* to be available in the destination account.
    /// If the payout fails, a `payout.failed` notification is also sent, at a later time.
    PayoutPaid(stripe_shared::Payout),
    /// Occurs whenever balance transactions paid out in an automatic payout can be queried.
    PayoutReconciliationCompleted(stripe_shared::Payout),
    /// Occurs whenever a payout is updated.
    PayoutUpdated(stripe_shared::Payout),
    /// Occurs whenever a person associated with an account is created.
    PersonCreated(stripe_shared::Person),
    /// Occurs whenever a person associated with an account is deleted.
    PersonDeleted(stripe_shared::Person),
    /// Occurs whenever a person associated with an account is updated.
    PersonUpdated(stripe_shared::Person),
    /// Occurs whenever a plan is created.
    PlanCreated(stripe_shared::Plan),
    /// Occurs whenever a plan is deleted.
    PlanDeleted(stripe_shared::Plan),
    /// Occurs whenever a plan is updated.
    PlanUpdated(stripe_shared::Plan),
    /// Occurs whenever a price is created.
    PriceCreated(stripe_shared::Price),
    /// Occurs whenever a price is deleted.
    PriceDeleted(stripe_shared::Price),
    /// Occurs whenever a price is updated.
    PriceUpdated(stripe_shared::Price),
    /// Occurs whenever a product is created.
    ProductCreated(stripe_shared::Product),
    /// Occurs whenever a product is deleted.
    ProductDeleted(stripe_shared::Product),
    /// Occurs whenever a product is updated.
    ProductUpdated(stripe_shared::Product),
    /// Occurs whenever a promotion code is created.
    PromotionCodeCreated(stripe_shared::PromotionCode),
    /// Occurs whenever a promotion code is updated.
    PromotionCodeUpdated(stripe_shared::PromotionCode),
    /// Occurs whenever a quote is accepted.
    #[cfg(feature = "async-stripe-billing")]
    QuoteAccepted(stripe_billing::Quote),
    /// Occurs whenever a quote is canceled.
    #[cfg(feature = "async-stripe-billing")]
    QuoteCanceled(stripe_billing::Quote),
    /// Occurs whenever a quote is created.
    #[cfg(feature = "async-stripe-billing")]
    QuoteCreated(stripe_billing::Quote),
    /// Occurs whenever a quote is finalized.
    #[cfg(feature = "async-stripe-billing")]
    QuoteFinalized(stripe_billing::Quote),
    /// Occurs whenever an early fraud warning is created.
    #[cfg(feature = "async-stripe-fraud")]
    RadarEarlyFraudWarningCreated(stripe_fraud::RadarEarlyFraudWarning),
    /// Occurs whenever an early fraud warning is updated.
    #[cfg(feature = "async-stripe-fraud")]
    RadarEarlyFraudWarningUpdated(stripe_fraud::RadarEarlyFraudWarning),
    /// Occurs whenever a refund is created.
    RefundCreated(stripe_shared::Refund),
    /// Occurs whenever a refund has failed.
    RefundFailed(stripe_shared::Refund),
    /// Occurs whenever a refund is updated.
    RefundUpdated(stripe_shared::Refund),
    /// Occurs whenever a requested `ReportRun` failed to complete.
    #[cfg(feature = "async-stripe-misc")]
    ReportingReportRunFailed(stripe_misc::ReportingReportRun),
    /// Occurs whenever a requested `ReportRun` completed successfully.
    #[cfg(feature = "async-stripe-misc")]
    ReportingReportRunSucceeded(stripe_misc::ReportingReportRun),
    /// Occurs whenever a `ReportType` is updated (typically to indicate that a new day's data has come available).
    #[cfg(feature = "async-stripe-misc")]
    ReportingReportTypeUpdated(stripe_misc::ReportingReportType),
    /// Occurs whenever a review is closed.
    /// The review's `reason` field indicates why: `approved`, `disputed`, `refunded`, `refunded_as_fraud`, or `canceled`.
    ReviewClosed(stripe_shared::Review),
    /// Occurs whenever a review is opened.
    ReviewOpened(stripe_shared::Review),
    /// Occurs when a SetupIntent is canceled.
    SetupIntentCanceled(stripe_shared::SetupIntent),
    /// Occurs when a new SetupIntent is created.
    SetupIntentCreated(stripe_shared::SetupIntent),
    /// Occurs when a SetupIntent is in requires_action state.
    SetupIntentRequiresAction(stripe_shared::SetupIntent),
    /// Occurs when a SetupIntent has failed the attempt to setup a payment method.
    SetupIntentSetupFailed(stripe_shared::SetupIntent),
    /// Occurs when an SetupIntent has successfully setup a payment method.
    SetupIntentSucceeded(stripe_shared::SetupIntent),
    /// Occurs whenever a Sigma scheduled query run finishes.
    #[cfg(feature = "async-stripe-misc")]
    SigmaScheduledQueryRunCreated(stripe_misc::ScheduledQueryRun),
    /// Occurs whenever a source is canceled.
    SourceCanceled(stripe_shared::Source),
    /// Occurs whenever a source transitions to chargeable.
    SourceChargeable(stripe_shared::Source),
    /// Occurs whenever a source fails.
    SourceFailed(stripe_shared::Source),
    /// Occurs whenever a source mandate notification method is set to manual.
    #[cfg(feature = "async-stripe-payment")]
    SourceMandateNotification(stripe_payment::SourceMandateNotification),
    /// Occurs whenever the refund attributes are required on a receiver source to process a refund or a mispayment.
    SourceRefundAttributesRequired(stripe_shared::Source),
    /// Occurs whenever a source transaction is created.
    SourceTransactionCreated(stripe_shared::SourceTransaction),
    /// Occurs whenever a source transaction is updated.
    SourceTransactionUpdated(stripe_shared::SourceTransaction),
    /// Occurs whenever a subscription schedule is canceled due to the underlying subscription being canceled because of delinquency.
    SubscriptionScheduleAborted(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever a subscription schedule is canceled.
    SubscriptionScheduleCanceled(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever a new subscription schedule is completed.
    SubscriptionScheduleCompleted(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever a new subscription schedule is created.
    SubscriptionScheduleCreated(stripe_shared::SubscriptionSchedule),
    /// Occurs 7 days before a subscription schedule will expire.
    SubscriptionScheduleExpiring(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever a new subscription schedule is released.
    SubscriptionScheduleReleased(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever a subscription schedule is updated.
    SubscriptionScheduleUpdated(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever tax settings is updated.
    #[cfg(feature = "async-stripe-misc")]
    TaxSettingsUpdated(stripe_misc::TaxSettings),
    /// Occurs whenever a new tax rate is created.
    TaxRateCreated(stripe_shared::TaxRate),
    /// Occurs whenever a tax rate is updated.
    TaxRateUpdated(stripe_shared::TaxRate),
    /// Occurs whenever an action sent to a Terminal reader failed.
    #[cfg(feature = "async-stripe-terminal")]
    TerminalReaderActionFailed(stripe_terminal::TerminalReader),
    /// Occurs whenever an action sent to a Terminal reader was successful.
    #[cfg(feature = "async-stripe-terminal")]
    TerminalReaderActionSucceeded(stripe_terminal::TerminalReader),
    /// Occurs whenever an action sent to a Terminal reader is updated.
    #[cfg(feature = "async-stripe-terminal")]
    TerminalReaderActionUpdated(stripe_terminal::TerminalReader),
    /// Occurs whenever a test clock starts advancing.
    TestHelpersTestClockAdvancing(stripe_shared::TestHelpersTestClock),
    /// Occurs whenever a test clock is created.
    TestHelpersTestClockCreated(stripe_shared::TestHelpersTestClock),
    /// Occurs whenever a test clock is deleted.
    TestHelpersTestClockDeleted(stripe_shared::TestHelpersTestClock),
    /// Occurs whenever a test clock fails to advance its frozen time.
    TestHelpersTestClockInternalFailure(stripe_shared::TestHelpersTestClock),
    /// Occurs whenever a test clock transitions to a ready status.
    TestHelpersTestClockReady(stripe_shared::TestHelpersTestClock),
    /// Occurs whenever a top-up is canceled.
    TopupCanceled(stripe_shared::Topup),
    /// Occurs whenever a top-up is created.
    TopupCreated(stripe_shared::Topup),
    /// Occurs whenever a top-up fails.
    TopupFailed(stripe_shared::Topup),
    /// Occurs whenever a top-up is reversed.
    TopupReversed(stripe_shared::Topup),
    /// Occurs whenever a top-up succeeds.
    TopupSucceeded(stripe_shared::Topup),
    /// Occurs whenever a transfer is created.
    TransferCreated(stripe_shared::Transfer),
    /// Occurs whenever a transfer is reversed, including partial reversals.
    TransferReversed(stripe_shared::Transfer),
    /// Occurs whenever a transfer's description or metadata is updated.
    TransferUpdated(stripe_shared::Transfer),
    /// Occurs whenever an CreditReversal is submitted and created.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryCreditReversalCreated(stripe_treasury::TreasuryCreditReversal),
    /// Occurs whenever an CreditReversal post is posted.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryCreditReversalPosted(stripe_treasury::TreasuryCreditReversal),
    /// Occurs whenever a DebitReversal is completed.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryDebitReversalCompleted(stripe_treasury::TreasuryDebitReversal),
    /// Occurs whenever a DebitReversal is created.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryDebitReversalCreated(stripe_treasury::TreasuryDebitReversal),
    /// Occurs whenever an initial credit is granted on a DebitReversal.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryDebitReversalInitialCreditGranted(stripe_treasury::TreasuryDebitReversal),
    /// Occurs whenever the status of the FinancialAccount becomes closed.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryFinancialAccountClosed(stripe_treasury::TreasuryFinancialAccount),
    /// Occurs whenever a new FinancialAccount is created.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryFinancialAccountCreated(stripe_treasury::TreasuryFinancialAccount),
    /// Occurs whenever the statuses of any features within an existing FinancialAccount are updated.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryFinancialAccountFeaturesStatusUpdated(stripe_treasury::TreasuryFinancialAccount),
    /// Occurs whenever an InboundTransfer is canceled.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryInboundTransferCanceled(stripe_treasury::TreasuryInboundTransfer),
    /// Occurs whenever an InboundTransfer is created.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryInboundTransferCreated(stripe_treasury::TreasuryInboundTransfer),
    /// Occurs whenever an InboundTransfer has failed.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryInboundTransferFailed(stripe_treasury::TreasuryInboundTransfer),
    /// Occurs whenever an InboundTransfer has succeeded.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryInboundTransferSucceeded(stripe_treasury::TreasuryInboundTransfer),
    /// Occurs whenever an OutboundPayment is canceled.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryOutboundPaymentCanceled(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever a new OutboundPayment is successfully created.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryOutboundPaymentCreated(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever the arrival date on an OutboundPayment updates.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryOutboundPaymentExpectedArrivalDateUpdated(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever an OutboundPayment fails.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryOutboundPaymentFailed(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever an OutboundPayment posts.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryOutboundPaymentPosted(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever an OutboundPayment was returned.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryOutboundPaymentReturned(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever tracking_details on an OutboundPayment is updated.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryOutboundPaymentTrackingDetailsUpdated(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever an OutboundTransfer is canceled.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryOutboundTransferCanceled(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever an OutboundTransfer is created.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryOutboundTransferCreated(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever the arrival date on an OutboundTransfer updates.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryOutboundTransferExpectedArrivalDateUpdated(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever an OutboundTransfer has failed.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryOutboundTransferFailed(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever an OutboundTransfer is posted.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryOutboundTransferPosted(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever an OutboundTransfer is returned.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryOutboundTransferReturned(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever tracking_details on an OutboundTransfer is updated.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryOutboundTransferTrackingDetailsUpdated(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever a received_credit is created as a result of funds being pushed by another account.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryReceivedCreditCreated(stripe_treasury::TreasuryReceivedCredit),
    /// Occurs whenever a received_credit transitions to failed state. Only applicable for check deposits.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryReceivedCreditFailed(stripe_treasury::TreasuryReceivedCredit),
    /// Occurs whenever a received_credit transitions to succeeded state.
    /// Only applicable for check deposits.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryReceivedCreditSucceeded(stripe_treasury::TreasuryReceivedCredit),
    /// Occurs whenever a received_debit is created as a result of funds being pulled by another account.
    #[cfg(feature = "async-stripe-treasury")]
    TreasuryReceivedDebitCreated(stripe_treasury::TreasuryReceivedDebit),
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(with = "stripe_types::with_serde_json")
    )]
    Unknown(miniserde::json::Value),
}
impl EventObject {
    pub(crate) fn from_raw_data(typ: &str, data: miniserde::json::Value) -> Option<Self> {
        use stripe_types::miniserde_helpers::FromValueOpt;
        if typ == "account.application.authorized" {
            return FromValueOpt::from_value(data).map(Self::AccountApplicationAuthorized);
        }
        if typ == "account.application.deauthorized" {
            return FromValueOpt::from_value(data).map(Self::AccountApplicationDeauthorized);
        }
        if typ == "account.external_account.created" {
            return FromValueOpt::from_value(data).map(Self::AccountExternalAccountCreated);
        }
        if typ == "account.external_account.deleted" {
            return FromValueOpt::from_value(data).map(Self::AccountExternalAccountDeleted);
        }
        if typ == "account.external_account.updated" {
            return FromValueOpt::from_value(data).map(Self::AccountExternalAccountUpdated);
        }
        if typ == "account.updated" {
            return FromValueOpt::from_value(data).map(Self::AccountUpdated);
        }
        if typ == "application_fee.created" {
            return FromValueOpt::from_value(data).map(Self::ApplicationFeeCreated);
        }
        if typ == "application_fee.refund.updated" {
            return FromValueOpt::from_value(data).map(Self::ApplicationFeeRefundUpdated);
        }
        if typ == "application_fee.refunded" {
            return FromValueOpt::from_value(data).map(Self::ApplicationFeeRefunded);
        }
        #[cfg(feature = "async-stripe-core")]
        if typ == "balance.available" {
            return FromValueOpt::from_value(data).map(Self::BalanceAvailable);
        }
        #[cfg(feature = "async-stripe-core")]
        if typ == "balance_settings.updated" {
            return FromValueOpt::from_value(data).map(Self::BalanceSettingsUpdated);
        }
        #[cfg(feature = "async-stripe-billing")]
        if typ == "billing.alert.triggered" {
            return FromValueOpt::from_value(data).map(Self::BillingAlertTriggered);
        }
        if typ == "billing.credit_balance_transaction.created" {
            return FromValueOpt::from_value(data)
                .map(Self::BillingCreditBalanceTransactionCreated);
        }
        if typ == "billing.credit_grant.created" {
            return FromValueOpt::from_value(data).map(Self::BillingCreditGrantCreated);
        }
        if typ == "billing.credit_grant.updated" {
            return FromValueOpt::from_value(data).map(Self::BillingCreditGrantUpdated);
        }
        #[cfg(feature = "async-stripe-billing")]
        if typ == "billing.meter.created" {
            return FromValueOpt::from_value(data).map(Self::BillingMeterCreated);
        }
        #[cfg(feature = "async-stripe-billing")]
        if typ == "billing.meter.deactivated" {
            return FromValueOpt::from_value(data).map(Self::BillingMeterDeactivated);
        }
        #[cfg(feature = "async-stripe-billing")]
        if typ == "billing.meter.reactivated" {
            return FromValueOpt::from_value(data).map(Self::BillingMeterReactivated);
        }
        #[cfg(feature = "async-stripe-billing")]
        if typ == "billing.meter.updated" {
            return FromValueOpt::from_value(data).map(Self::BillingMeterUpdated);
        }
        #[cfg(feature = "async-stripe-billing")]
        if typ == "billing_portal.configuration.created" {
            return FromValueOpt::from_value(data).map(Self::BillingPortalConfigurationCreated);
        }
        #[cfg(feature = "async-stripe-billing")]
        if typ == "billing_portal.configuration.updated" {
            return FromValueOpt::from_value(data).map(Self::BillingPortalConfigurationUpdated);
        }
        #[cfg(feature = "async-stripe-billing")]
        if typ == "billing_portal.session.created" {
            return FromValueOpt::from_value(data).map(Self::BillingPortalSessionCreated);
        }
        if typ == "capability.updated" {
            return FromValueOpt::from_value(data).map(Self::CapabilityUpdated);
        }
        if typ == "cash_balance.funds_available" {
            return FromValueOpt::from_value(data).map(Self::CashBalanceFundsAvailable);
        }
        if typ == "charge.captured" {
            return FromValueOpt::from_value(data).map(Self::ChargeCaptured);
        }
        if typ == "charge.dispute.closed" {
            return FromValueOpt::from_value(data).map(Self::ChargeDisputeClosed);
        }
        if typ == "charge.dispute.created" {
            return FromValueOpt::from_value(data).map(Self::ChargeDisputeCreated);
        }
        if typ == "charge.dispute.funds_reinstated" {
            return FromValueOpt::from_value(data).map(Self::ChargeDisputeFundsReinstated);
        }
        if typ == "charge.dispute.funds_withdrawn" {
            return FromValueOpt::from_value(data).map(Self::ChargeDisputeFundsWithdrawn);
        }
        if typ == "charge.dispute.updated" {
            return FromValueOpt::from_value(data).map(Self::ChargeDisputeUpdated);
        }
        if typ == "charge.expired" {
            return FromValueOpt::from_value(data).map(Self::ChargeExpired);
        }
        if typ == "charge.failed" {
            return FromValueOpt::from_value(data).map(Self::ChargeFailed);
        }
        if typ == "charge.pending" {
            return FromValueOpt::from_value(data).map(Self::ChargePending);
        }
        if typ == "charge.refund.updated" {
            return FromValueOpt::from_value(data).map(Self::ChargeRefundUpdated);
        }
        if typ == "charge.refunded" {
            return FromValueOpt::from_value(data).map(Self::ChargeRefunded);
        }
        if typ == "charge.succeeded" {
            return FromValueOpt::from_value(data).map(Self::ChargeSucceeded);
        }
        if typ == "charge.updated" {
            return FromValueOpt::from_value(data).map(Self::ChargeUpdated);
        }
        if typ == "checkout.session.async_payment_failed" {
            return FromValueOpt::from_value(data).map(Self::CheckoutSessionAsyncPaymentFailed);
        }
        if typ == "checkout.session.async_payment_succeeded" {
            return FromValueOpt::from_value(data).map(Self::CheckoutSessionAsyncPaymentSucceeded);
        }
        if typ == "checkout.session.completed" {
            return FromValueOpt::from_value(data).map(Self::CheckoutSessionCompleted);
        }
        if typ == "checkout.session.expired" {
            return FromValueOpt::from_value(data).map(Self::CheckoutSessionExpired);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "climate.order.canceled" {
            return FromValueOpt::from_value(data).map(Self::ClimateOrderCanceled);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "climate.order.created" {
            return FromValueOpt::from_value(data).map(Self::ClimateOrderCreated);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "climate.order.delayed" {
            return FromValueOpt::from_value(data).map(Self::ClimateOrderDelayed);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "climate.order.delivered" {
            return FromValueOpt::from_value(data).map(Self::ClimateOrderDelivered);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "climate.order.product_substituted" {
            return FromValueOpt::from_value(data).map(Self::ClimateOrderProductSubstituted);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "climate.product.created" {
            return FromValueOpt::from_value(data).map(Self::ClimateProductCreated);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "climate.product.pricing_updated" {
            return FromValueOpt::from_value(data).map(Self::ClimateProductPricingUpdated);
        }
        if typ == "coupon.created" {
            return FromValueOpt::from_value(data).map(Self::CouponCreated);
        }
        if typ == "coupon.deleted" {
            return FromValueOpt::from_value(data).map(Self::CouponDeleted);
        }
        if typ == "coupon.updated" {
            return FromValueOpt::from_value(data).map(Self::CouponUpdated);
        }
        if typ == "credit_note.created" {
            return FromValueOpt::from_value(data).map(Self::CreditNoteCreated);
        }
        if typ == "credit_note.updated" {
            return FromValueOpt::from_value(data).map(Self::CreditNoteUpdated);
        }
        if typ == "credit_note.voided" {
            return FromValueOpt::from_value(data).map(Self::CreditNoteVoided);
        }
        if typ == "customer.created" {
            return FromValueOpt::from_value(data).map(Self::CustomerCreated);
        }
        if typ == "customer.deleted" {
            return FromValueOpt::from_value(data).map(Self::CustomerDeleted);
        }
        if typ == "customer.discount.created" {
            return FromValueOpt::from_value(data).map(Self::CustomerDiscountCreated);
        }
        if typ == "customer.discount.deleted" {
            return FromValueOpt::from_value(data).map(Self::CustomerDiscountDeleted);
        }
        if typ == "customer.discount.updated" {
            return FromValueOpt::from_value(data).map(Self::CustomerDiscountUpdated);
        }
        if typ == "customer.source.created" {
            return FromValueOpt::from_value(data).map(Self::CustomerSourceCreated);
        }
        if typ == "customer.source.deleted" {
            return FromValueOpt::from_value(data).map(Self::CustomerSourceDeleted);
        }
        if typ == "customer.source.expiring" {
            return FromValueOpt::from_value(data).map(Self::CustomerSourceExpiring);
        }
        if typ == "customer.source.updated" {
            return FromValueOpt::from_value(data).map(Self::CustomerSourceUpdated);
        }
        if typ == "customer.subscription.created" {
            return FromValueOpt::from_value(data).map(Self::CustomerSubscriptionCreated);
        }
        if typ == "customer.subscription.deleted" {
            return FromValueOpt::from_value(data).map(Self::CustomerSubscriptionDeleted);
        }
        if typ == "customer.subscription.paused" {
            return FromValueOpt::from_value(data).map(Self::CustomerSubscriptionPaused);
        }
        if typ == "customer.subscription.pending_update_applied" {
            return FromValueOpt::from_value(data)
                .map(Self::CustomerSubscriptionPendingUpdateApplied);
        }
        if typ == "customer.subscription.pending_update_expired" {
            return FromValueOpt::from_value(data)
                .map(Self::CustomerSubscriptionPendingUpdateExpired);
        }
        if typ == "customer.subscription.resumed" {
            return FromValueOpt::from_value(data).map(Self::CustomerSubscriptionResumed);
        }
        if typ == "customer.subscription.trial_will_end" {
            return FromValueOpt::from_value(data).map(Self::CustomerSubscriptionTrialWillEnd);
        }
        if typ == "customer.subscription.updated" {
            return FromValueOpt::from_value(data).map(Self::CustomerSubscriptionUpdated);
        }
        if typ == "customer.tax_id.created" {
            return FromValueOpt::from_value(data).map(Self::CustomerTaxIdCreated);
        }
        if typ == "customer.tax_id.deleted" {
            return FromValueOpt::from_value(data).map(Self::CustomerTaxIdDeleted);
        }
        if typ == "customer.tax_id.updated" {
            return FromValueOpt::from_value(data).map(Self::CustomerTaxIdUpdated);
        }
        if typ == "customer.updated" {
            return FromValueOpt::from_value(data).map(Self::CustomerUpdated);
        }
        if typ == "customer_cash_balance_transaction.created" {
            return FromValueOpt::from_value(data).map(Self::CustomerCashBalanceTransactionCreated);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "entitlements.active_entitlement_summary.updated" {
            return FromValueOpt::from_value(data)
                .map(Self::EntitlementsActiveEntitlementSummaryUpdated);
        }
        if typ == "file.created" {
            return FromValueOpt::from_value(data).map(Self::FileCreated);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "financial_connections.account.created" {
            return FromValueOpt::from_value(data).map(Self::FinancialConnectionsAccountCreated);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "financial_connections.account.deactivated" {
            return FromValueOpt::from_value(data)
                .map(Self::FinancialConnectionsAccountDeactivated);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "financial_connections.account.disconnected" {
            return FromValueOpt::from_value(data)
                .map(Self::FinancialConnectionsAccountDisconnected);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "financial_connections.account.reactivated" {
            return FromValueOpt::from_value(data)
                .map(Self::FinancialConnectionsAccountReactivated);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "financial_connections.account.refreshed_balance" {
            return FromValueOpt::from_value(data)
                .map(Self::FinancialConnectionsAccountRefreshedBalance);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "financial_connections.account.refreshed_ownership" {
            return FromValueOpt::from_value(data)
                .map(Self::FinancialConnectionsAccountRefreshedOwnership);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "financial_connections.account.refreshed_transactions" {
            return FromValueOpt::from_value(data)
                .map(Self::FinancialConnectionsAccountRefreshedTransactions);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "identity.verification_session.canceled" {
            return FromValueOpt::from_value(data).map(Self::IdentityVerificationSessionCanceled);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "identity.verification_session.created" {
            return FromValueOpt::from_value(data).map(Self::IdentityVerificationSessionCreated);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "identity.verification_session.processing" {
            return FromValueOpt::from_value(data).map(Self::IdentityVerificationSessionProcessing);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "identity.verification_session.redacted" {
            return FromValueOpt::from_value(data).map(Self::IdentityVerificationSessionRedacted);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "identity.verification_session.requires_input" {
            return FromValueOpt::from_value(data)
                .map(Self::IdentityVerificationSessionRequiresInput);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "identity.verification_session.verified" {
            return FromValueOpt::from_value(data).map(Self::IdentityVerificationSessionVerified);
        }
        if typ == "invoice.created" {
            return FromValueOpt::from_value(data).map(Self::InvoiceCreated);
        }
        if typ == "invoice.deleted" {
            return FromValueOpt::from_value(data).map(Self::InvoiceDeleted);
        }
        if typ == "invoice.finalization_failed" {
            return FromValueOpt::from_value(data).map(Self::InvoiceFinalizationFailed);
        }
        if typ == "invoice.finalized" {
            return FromValueOpt::from_value(data).map(Self::InvoiceFinalized);
        }
        if typ == "invoice.marked_uncollectible" {
            return FromValueOpt::from_value(data).map(Self::InvoiceMarkedUncollectible);
        }
        if typ == "invoice.overdue" {
            return FromValueOpt::from_value(data).map(Self::InvoiceOverdue);
        }
        if typ == "invoice.overpaid" {
            return FromValueOpt::from_value(data).map(Self::InvoiceOverpaid);
        }
        if typ == "invoice.paid" {
            return FromValueOpt::from_value(data).map(Self::InvoicePaid);
        }
        if typ == "invoice.payment_action_required" {
            return FromValueOpt::from_value(data).map(Self::InvoicePaymentActionRequired);
        }
        if typ == "invoice.payment_attempt_required" {
            return FromValueOpt::from_value(data).map(Self::InvoicePaymentAttemptRequired);
        }
        if typ == "invoice.payment_failed" {
            return FromValueOpt::from_value(data).map(Self::InvoicePaymentFailed);
        }
        if typ == "invoice.payment_succeeded" {
            return FromValueOpt::from_value(data).map(Self::InvoicePaymentSucceeded);
        }
        if typ == "invoice.sent" {
            return FromValueOpt::from_value(data).map(Self::InvoiceSent);
        }
        if typ == "invoice.upcoming" {
            return FromValueOpt::from_value(data).map(Self::InvoiceUpcoming);
        }
        if typ == "invoice.updated" {
            return FromValueOpt::from_value(data).map(Self::InvoiceUpdated);
        }
        if typ == "invoice.voided" {
            return FromValueOpt::from_value(data).map(Self::InvoiceVoided);
        }
        if typ == "invoice.will_be_due" {
            return FromValueOpt::from_value(data).map(Self::InvoiceWillBeDue);
        }
        if typ == "invoice_payment.paid" {
            return FromValueOpt::from_value(data).map(Self::InvoicePaymentPaid);
        }
        #[cfg(feature = "async-stripe-billing")]
        if typ == "invoiceitem.created" {
            return FromValueOpt::from_value(data).map(Self::InvoiceitemCreated);
        }
        #[cfg(feature = "async-stripe-billing")]
        if typ == "invoiceitem.deleted" {
            return FromValueOpt::from_value(data).map(Self::InvoiceitemDeleted);
        }
        if typ == "issuing_authorization.created" {
            return FromValueOpt::from_value(data).map(Self::IssuingAuthorizationCreated);
        }
        if typ == "issuing_authorization.request" {
            return FromValueOpt::from_value(data).map(Self::IssuingAuthorizationRequest);
        }
        if typ == "issuing_authorization.updated" {
            return FromValueOpt::from_value(data).map(Self::IssuingAuthorizationUpdated);
        }
        if typ == "issuing_card.created" {
            return FromValueOpt::from_value(data).map(Self::IssuingCardCreated);
        }
        if typ == "issuing_card.updated" {
            return FromValueOpt::from_value(data).map(Self::IssuingCardUpdated);
        }
        if typ == "issuing_cardholder.created" {
            return FromValueOpt::from_value(data).map(Self::IssuingCardholderCreated);
        }
        if typ == "issuing_cardholder.updated" {
            return FromValueOpt::from_value(data).map(Self::IssuingCardholderUpdated);
        }
        if typ == "issuing_dispute.closed" {
            return FromValueOpt::from_value(data).map(Self::IssuingDisputeClosed);
        }
        if typ == "issuing_dispute.created" {
            return FromValueOpt::from_value(data).map(Self::IssuingDisputeCreated);
        }
        if typ == "issuing_dispute.funds_reinstated" {
            return FromValueOpt::from_value(data).map(Self::IssuingDisputeFundsReinstated);
        }
        if typ == "issuing_dispute.funds_rescinded" {
            return FromValueOpt::from_value(data).map(Self::IssuingDisputeFundsRescinded);
        }
        if typ == "issuing_dispute.submitted" {
            return FromValueOpt::from_value(data).map(Self::IssuingDisputeSubmitted);
        }
        if typ == "issuing_dispute.updated" {
            return FromValueOpt::from_value(data).map(Self::IssuingDisputeUpdated);
        }
        if typ == "issuing_personalization_design.activated" {
            return FromValueOpt::from_value(data).map(Self::IssuingPersonalizationDesignActivated);
        }
        if typ == "issuing_personalization_design.deactivated" {
            return FromValueOpt::from_value(data)
                .map(Self::IssuingPersonalizationDesignDeactivated);
        }
        if typ == "issuing_personalization_design.rejected" {
            return FromValueOpt::from_value(data).map(Self::IssuingPersonalizationDesignRejected);
        }
        if typ == "issuing_personalization_design.updated" {
            return FromValueOpt::from_value(data).map(Self::IssuingPersonalizationDesignUpdated);
        }
        if typ == "issuing_token.created" {
            return FromValueOpt::from_value(data).map(Self::IssuingTokenCreated);
        }
        if typ == "issuing_token.updated" {
            return FromValueOpt::from_value(data).map(Self::IssuingTokenUpdated);
        }
        if typ == "issuing_transaction.created" {
            return FromValueOpt::from_value(data).map(Self::IssuingTransactionCreated);
        }
        if typ == "issuing_transaction.purchase_details_receipt_updated" {
            return FromValueOpt::from_value(data)
                .map(Self::IssuingTransactionPurchaseDetailsReceiptUpdated);
        }
        if typ == "issuing_transaction.updated" {
            return FromValueOpt::from_value(data).map(Self::IssuingTransactionUpdated);
        }
        if typ == "mandate.updated" {
            return FromValueOpt::from_value(data).map(Self::MandateUpdated);
        }
        if typ == "payment_intent.amount_capturable_updated" {
            return FromValueOpt::from_value(data).map(Self::PaymentIntentAmountCapturableUpdated);
        }
        if typ == "payment_intent.canceled" {
            return FromValueOpt::from_value(data).map(Self::PaymentIntentCanceled);
        }
        if typ == "payment_intent.created" {
            return FromValueOpt::from_value(data).map(Self::PaymentIntentCreated);
        }
        if typ == "payment_intent.partially_funded" {
            return FromValueOpt::from_value(data).map(Self::PaymentIntentPartiallyFunded);
        }
        if typ == "payment_intent.payment_failed" {
            return FromValueOpt::from_value(data).map(Self::PaymentIntentPaymentFailed);
        }
        if typ == "payment_intent.processing" {
            return FromValueOpt::from_value(data).map(Self::PaymentIntentProcessing);
        }
        if typ == "payment_intent.requires_action" {
            return FromValueOpt::from_value(data).map(Self::PaymentIntentRequiresAction);
        }
        if typ == "payment_intent.succeeded" {
            return FromValueOpt::from_value(data).map(Self::PaymentIntentSucceeded);
        }
        if typ == "payment_link.created" {
            return FromValueOpt::from_value(data).map(Self::PaymentLinkCreated);
        }
        if typ == "payment_link.updated" {
            return FromValueOpt::from_value(data).map(Self::PaymentLinkUpdated);
        }
        if typ == "payment_method.attached" {
            return FromValueOpt::from_value(data).map(Self::PaymentMethodAttached);
        }
        if typ == "payment_method.automatically_updated" {
            return FromValueOpt::from_value(data).map(Self::PaymentMethodAutomaticallyUpdated);
        }
        if typ == "payment_method.detached" {
            return FromValueOpt::from_value(data).map(Self::PaymentMethodDetached);
        }
        if typ == "payment_method.updated" {
            return FromValueOpt::from_value(data).map(Self::PaymentMethodUpdated);
        }
        if typ == "payout.canceled" {
            return FromValueOpt::from_value(data).map(Self::PayoutCanceled);
        }
        if typ == "payout.created" {
            return FromValueOpt::from_value(data).map(Self::PayoutCreated);
        }
        if typ == "payout.failed" {
            return FromValueOpt::from_value(data).map(Self::PayoutFailed);
        }
        if typ == "payout.paid" {
            return FromValueOpt::from_value(data).map(Self::PayoutPaid);
        }
        if typ == "payout.reconciliation_completed" {
            return FromValueOpt::from_value(data).map(Self::PayoutReconciliationCompleted);
        }
        if typ == "payout.updated" {
            return FromValueOpt::from_value(data).map(Self::PayoutUpdated);
        }
        if typ == "person.created" {
            return FromValueOpt::from_value(data).map(Self::PersonCreated);
        }
        if typ == "person.deleted" {
            return FromValueOpt::from_value(data).map(Self::PersonDeleted);
        }
        if typ == "person.updated" {
            return FromValueOpt::from_value(data).map(Self::PersonUpdated);
        }
        if typ == "plan.created" {
            return FromValueOpt::from_value(data).map(Self::PlanCreated);
        }
        if typ == "plan.deleted" {
            return FromValueOpt::from_value(data).map(Self::PlanDeleted);
        }
        if typ == "plan.updated" {
            return FromValueOpt::from_value(data).map(Self::PlanUpdated);
        }
        if typ == "price.created" {
            return FromValueOpt::from_value(data).map(Self::PriceCreated);
        }
        if typ == "price.deleted" {
            return FromValueOpt::from_value(data).map(Self::PriceDeleted);
        }
        if typ == "price.updated" {
            return FromValueOpt::from_value(data).map(Self::PriceUpdated);
        }
        if typ == "product.created" {
            return FromValueOpt::from_value(data).map(Self::ProductCreated);
        }
        if typ == "product.deleted" {
            return FromValueOpt::from_value(data).map(Self::ProductDeleted);
        }
        if typ == "product.updated" {
            return FromValueOpt::from_value(data).map(Self::ProductUpdated);
        }
        if typ == "promotion_code.created" {
            return FromValueOpt::from_value(data).map(Self::PromotionCodeCreated);
        }
        if typ == "promotion_code.updated" {
            return FromValueOpt::from_value(data).map(Self::PromotionCodeUpdated);
        }
        #[cfg(feature = "async-stripe-billing")]
        if typ == "quote.accepted" {
            return FromValueOpt::from_value(data).map(Self::QuoteAccepted);
        }
        #[cfg(feature = "async-stripe-billing")]
        if typ == "quote.canceled" {
            return FromValueOpt::from_value(data).map(Self::QuoteCanceled);
        }
        #[cfg(feature = "async-stripe-billing")]
        if typ == "quote.created" {
            return FromValueOpt::from_value(data).map(Self::QuoteCreated);
        }
        #[cfg(feature = "async-stripe-billing")]
        if typ == "quote.finalized" {
            return FromValueOpt::from_value(data).map(Self::QuoteFinalized);
        }
        #[cfg(feature = "async-stripe-fraud")]
        if typ == "radar.early_fraud_warning.created" {
            return FromValueOpt::from_value(data).map(Self::RadarEarlyFraudWarningCreated);
        }
        #[cfg(feature = "async-stripe-fraud")]
        if typ == "radar.early_fraud_warning.updated" {
            return FromValueOpt::from_value(data).map(Self::RadarEarlyFraudWarningUpdated);
        }
        if typ == "refund.created" {
            return FromValueOpt::from_value(data).map(Self::RefundCreated);
        }
        if typ == "refund.failed" {
            return FromValueOpt::from_value(data).map(Self::RefundFailed);
        }
        if typ == "refund.updated" {
            return FromValueOpt::from_value(data).map(Self::RefundUpdated);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "reporting.report_run.failed" {
            return FromValueOpt::from_value(data).map(Self::ReportingReportRunFailed);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "reporting.report_run.succeeded" {
            return FromValueOpt::from_value(data).map(Self::ReportingReportRunSucceeded);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "reporting.report_type.updated" {
            return FromValueOpt::from_value(data).map(Self::ReportingReportTypeUpdated);
        }
        if typ == "review.closed" {
            return FromValueOpt::from_value(data).map(Self::ReviewClosed);
        }
        if typ == "review.opened" {
            return FromValueOpt::from_value(data).map(Self::ReviewOpened);
        }
        if typ == "setup_intent.canceled" {
            return FromValueOpt::from_value(data).map(Self::SetupIntentCanceled);
        }
        if typ == "setup_intent.created" {
            return FromValueOpt::from_value(data).map(Self::SetupIntentCreated);
        }
        if typ == "setup_intent.requires_action" {
            return FromValueOpt::from_value(data).map(Self::SetupIntentRequiresAction);
        }
        if typ == "setup_intent.setup_failed" {
            return FromValueOpt::from_value(data).map(Self::SetupIntentSetupFailed);
        }
        if typ == "setup_intent.succeeded" {
            return FromValueOpt::from_value(data).map(Self::SetupIntentSucceeded);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "sigma.scheduled_query_run.created" {
            return FromValueOpt::from_value(data).map(Self::SigmaScheduledQueryRunCreated);
        }
        if typ == "source.canceled" {
            return FromValueOpt::from_value(data).map(Self::SourceCanceled);
        }
        if typ == "source.chargeable" {
            return FromValueOpt::from_value(data).map(Self::SourceChargeable);
        }
        if typ == "source.failed" {
            return FromValueOpt::from_value(data).map(Self::SourceFailed);
        }
        #[cfg(feature = "async-stripe-payment")]
        if typ == "source.mandate_notification" {
            return FromValueOpt::from_value(data).map(Self::SourceMandateNotification);
        }
        if typ == "source.refund_attributes_required" {
            return FromValueOpt::from_value(data).map(Self::SourceRefundAttributesRequired);
        }
        if typ == "source.transaction.created" {
            return FromValueOpt::from_value(data).map(Self::SourceTransactionCreated);
        }
        if typ == "source.transaction.updated" {
            return FromValueOpt::from_value(data).map(Self::SourceTransactionUpdated);
        }
        if typ == "subscription_schedule.aborted" {
            return FromValueOpt::from_value(data).map(Self::SubscriptionScheduleAborted);
        }
        if typ == "subscription_schedule.canceled" {
            return FromValueOpt::from_value(data).map(Self::SubscriptionScheduleCanceled);
        }
        if typ == "subscription_schedule.completed" {
            return FromValueOpt::from_value(data).map(Self::SubscriptionScheduleCompleted);
        }
        if typ == "subscription_schedule.created" {
            return FromValueOpt::from_value(data).map(Self::SubscriptionScheduleCreated);
        }
        if typ == "subscription_schedule.expiring" {
            return FromValueOpt::from_value(data).map(Self::SubscriptionScheduleExpiring);
        }
        if typ == "subscription_schedule.released" {
            return FromValueOpt::from_value(data).map(Self::SubscriptionScheduleReleased);
        }
        if typ == "subscription_schedule.updated" {
            return FromValueOpt::from_value(data).map(Self::SubscriptionScheduleUpdated);
        }
        #[cfg(feature = "async-stripe-misc")]
        if typ == "tax.settings.updated" {
            return FromValueOpt::from_value(data).map(Self::TaxSettingsUpdated);
        }
        if typ == "tax_rate.created" {
            return FromValueOpt::from_value(data).map(Self::TaxRateCreated);
        }
        if typ == "tax_rate.updated" {
            return FromValueOpt::from_value(data).map(Self::TaxRateUpdated);
        }
        #[cfg(feature = "async-stripe-terminal")]
        if typ == "terminal.reader.action_failed" {
            return FromValueOpt::from_value(data).map(Self::TerminalReaderActionFailed);
        }
        #[cfg(feature = "async-stripe-terminal")]
        if typ == "terminal.reader.action_succeeded" {
            return FromValueOpt::from_value(data).map(Self::TerminalReaderActionSucceeded);
        }
        #[cfg(feature = "async-stripe-terminal")]
        if typ == "terminal.reader.action_updated" {
            return FromValueOpt::from_value(data).map(Self::TerminalReaderActionUpdated);
        }
        if typ == "test_helpers.test_clock.advancing" {
            return FromValueOpt::from_value(data).map(Self::TestHelpersTestClockAdvancing);
        }
        if typ == "test_helpers.test_clock.created" {
            return FromValueOpt::from_value(data).map(Self::TestHelpersTestClockCreated);
        }
        if typ == "test_helpers.test_clock.deleted" {
            return FromValueOpt::from_value(data).map(Self::TestHelpersTestClockDeleted);
        }
        if typ == "test_helpers.test_clock.internal_failure" {
            return FromValueOpt::from_value(data).map(Self::TestHelpersTestClockInternalFailure);
        }
        if typ == "test_helpers.test_clock.ready" {
            return FromValueOpt::from_value(data).map(Self::TestHelpersTestClockReady);
        }
        if typ == "topup.canceled" {
            return FromValueOpt::from_value(data).map(Self::TopupCanceled);
        }
        if typ == "topup.created" {
            return FromValueOpt::from_value(data).map(Self::TopupCreated);
        }
        if typ == "topup.failed" {
            return FromValueOpt::from_value(data).map(Self::TopupFailed);
        }
        if typ == "topup.reversed" {
            return FromValueOpt::from_value(data).map(Self::TopupReversed);
        }
        if typ == "topup.succeeded" {
            return FromValueOpt::from_value(data).map(Self::TopupSucceeded);
        }
        if typ == "transfer.created" {
            return FromValueOpt::from_value(data).map(Self::TransferCreated);
        }
        if typ == "transfer.reversed" {
            return FromValueOpt::from_value(data).map(Self::TransferReversed);
        }
        if typ == "transfer.updated" {
            return FromValueOpt::from_value(data).map(Self::TransferUpdated);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.credit_reversal.created" {
            return FromValueOpt::from_value(data).map(Self::TreasuryCreditReversalCreated);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.credit_reversal.posted" {
            return FromValueOpt::from_value(data).map(Self::TreasuryCreditReversalPosted);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.debit_reversal.completed" {
            return FromValueOpt::from_value(data).map(Self::TreasuryDebitReversalCompleted);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.debit_reversal.created" {
            return FromValueOpt::from_value(data).map(Self::TreasuryDebitReversalCreated);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.debit_reversal.initial_credit_granted" {
            return FromValueOpt::from_value(data)
                .map(Self::TreasuryDebitReversalInitialCreditGranted);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.financial_account.closed" {
            return FromValueOpt::from_value(data).map(Self::TreasuryFinancialAccountClosed);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.financial_account.created" {
            return FromValueOpt::from_value(data).map(Self::TreasuryFinancialAccountCreated);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.financial_account.features_status_updated" {
            return FromValueOpt::from_value(data)
                .map(Self::TreasuryFinancialAccountFeaturesStatusUpdated);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.inbound_transfer.canceled" {
            return FromValueOpt::from_value(data).map(Self::TreasuryInboundTransferCanceled);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.inbound_transfer.created" {
            return FromValueOpt::from_value(data).map(Self::TreasuryInboundTransferCreated);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.inbound_transfer.failed" {
            return FromValueOpt::from_value(data).map(Self::TreasuryInboundTransferFailed);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.inbound_transfer.succeeded" {
            return FromValueOpt::from_value(data).map(Self::TreasuryInboundTransferSucceeded);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.outbound_payment.canceled" {
            return FromValueOpt::from_value(data).map(Self::TreasuryOutboundPaymentCanceled);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.outbound_payment.created" {
            return FromValueOpt::from_value(data).map(Self::TreasuryOutboundPaymentCreated);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.outbound_payment.expected_arrival_date_updated" {
            return FromValueOpt::from_value(data)
                .map(Self::TreasuryOutboundPaymentExpectedArrivalDateUpdated);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.outbound_payment.failed" {
            return FromValueOpt::from_value(data).map(Self::TreasuryOutboundPaymentFailed);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.outbound_payment.posted" {
            return FromValueOpt::from_value(data).map(Self::TreasuryOutboundPaymentPosted);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.outbound_payment.returned" {
            return FromValueOpt::from_value(data).map(Self::TreasuryOutboundPaymentReturned);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.outbound_payment.tracking_details_updated" {
            return FromValueOpt::from_value(data)
                .map(Self::TreasuryOutboundPaymentTrackingDetailsUpdated);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.outbound_transfer.canceled" {
            return FromValueOpt::from_value(data).map(Self::TreasuryOutboundTransferCanceled);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.outbound_transfer.created" {
            return FromValueOpt::from_value(data).map(Self::TreasuryOutboundTransferCreated);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.outbound_transfer.expected_arrival_date_updated" {
            return FromValueOpt::from_value(data)
                .map(Self::TreasuryOutboundTransferExpectedArrivalDateUpdated);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.outbound_transfer.failed" {
            return FromValueOpt::from_value(data).map(Self::TreasuryOutboundTransferFailed);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.outbound_transfer.posted" {
            return FromValueOpt::from_value(data).map(Self::TreasuryOutboundTransferPosted);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.outbound_transfer.returned" {
            return FromValueOpt::from_value(data).map(Self::TreasuryOutboundTransferReturned);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.outbound_transfer.tracking_details_updated" {
            return FromValueOpt::from_value(data)
                .map(Self::TreasuryOutboundTransferTrackingDetailsUpdated);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.received_credit.created" {
            return FromValueOpt::from_value(data).map(Self::TreasuryReceivedCreditCreated);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.received_credit.failed" {
            return FromValueOpt::from_value(data).map(Self::TreasuryReceivedCreditFailed);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.received_credit.succeeded" {
            return FromValueOpt::from_value(data).map(Self::TreasuryReceivedCreditSucceeded);
        }
        #[cfg(feature = "async-stripe-treasury")]
        if typ == "treasury.received_debit.created" {
            return FromValueOpt::from_value(data).map(Self::TreasuryReceivedDebitCreated);
        }

        Some(Self::Unknown(data))
    }
}
