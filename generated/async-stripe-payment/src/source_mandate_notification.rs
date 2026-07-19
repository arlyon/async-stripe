/// Source mandate notifications should be created when a notification related to
/// a source mandate must be sent to the payer. They will trigger a webhook or
/// deliver an email to the customer.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceMandateNotification {
    pub acss_debit: Option<stripe_payment::SourceMandateNotificationAcssDebitData>,
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount associated with the mandate notification.
    /// The amount is expressed in the currency of the underlying source.
    /// Required if the notification type is `debit_initiated`.
    pub amount: Option<i64>,
    pub bacs_debit: Option<stripe_payment::SourceMandateNotificationBacsDebitData>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_payment::SourceMandateNotificationId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The reason of the mandate notification. Valid reasons are `mandate_confirmed` or `debit_initiated`.
    pub reason: String,
    pub sepa_debit: Option<stripe_payment::SourceMandateNotificationSepaDebitData>,
    pub source: stripe_shared::Source,
    /// The status of the mandate notification. Valid statuses are `pending` or `submitted`.
    pub status: String,
    /// The type of source this mandate notification is attached to.
    /// Should be the source type identifier code for the payment method, such as `three_d_secure`.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceMandateNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceMandateNotification").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceMandateNotificationBuilder {
    acss_debit: Option<Option<stripe_payment::SourceMandateNotificationAcssDebitData>>,
    amount: Option<Option<i64>>,
    bacs_debit: Option<Option<stripe_payment::SourceMandateNotificationBacsDebitData>>,
    created: Option<stripe_types::Timestamp>,
    id: Option<stripe_payment::SourceMandateNotificationId>,
    livemode: Option<bool>,
    reason: Option<String>,
    sepa_debit: Option<Option<stripe_payment::SourceMandateNotificationSepaDebitData>>,
    source: Option<stripe_shared::Source>,
    status: Option<String>,
    type_: Option<String>,
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

    impl Deserialize for SourceMandateNotification {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceMandateNotification>,
        builder: SourceMandateNotificationBuilder,
    }

    impl Visitor for Place<SourceMandateNotification> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceMandateNotificationBuilder {
                    acss_debit: Deserialize::default(),
                    amount: Deserialize::default(),
                    bacs_debit: Deserialize::default(),
                    created: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    reason: Deserialize::default(),
                    sepa_debit: Deserialize::default(),
                    source: Deserialize::default(),
                    status: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => Deserialize::begin(&mut self.builder.acss_debit),
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "bacs_debit" => Deserialize::begin(&mut self.builder.bacs_debit),
                "created" => Deserialize::begin(&mut self.builder.created),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "reason" => Deserialize::begin(&mut self.builder.reason),
                "sepa_debit" => Deserialize::begin(&mut self.builder.sepa_debit),
                "source" => Deserialize::begin(&mut self.builder.source),
                "status" => Deserialize::begin(&mut self.builder.status),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(acss_debit),
                Some(amount),
                Some(bacs_debit),
                Some(created),
                Some(id),
                Some(livemode),
                Some(reason),
                Some(sepa_debit),
                Some(source),
                Some(status),
                Some(type_),
            ) = (
                self.builder.acss_debit.take(),
                self.builder.amount,
                self.builder.bacs_debit.take(),
                self.builder.created,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.reason.take(),
                self.builder.sepa_debit.take(),
                self.builder.source.take(),
                self.builder.status.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SourceMandateNotification {
                acss_debit,
                amount,
                bacs_debit,
                created,
                id,
                livemode,
                reason,
                sepa_debit,
                source,
                status,
                type_,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for SourceMandateNotification {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("SourceMandateNotification", 12)?;
        s.serialize_field("acss_debit", &self.acss_debit)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("bacs_debit", &self.bacs_debit)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("reason", &self.reason)?;
        s.serialize_field("sepa_debit", &self.sepa_debit)?;
        s.serialize_field("source", &self.source)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "source_mandate_notification")?;
        s.end()
    }
}
impl stripe_types::Object for SourceMandateNotification {
    type Id = stripe_payment::SourceMandateNotificationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(SourceMandateNotificationId);
