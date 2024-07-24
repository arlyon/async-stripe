/// Source mandate notifications should be created when a notification related to
/// a source mandate must be sent to the payer. They will trigger a webhook or
/// deliver an email to the customer.
#[derive(Clone, Debug)]
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
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
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

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: SourceMandateNotificationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceMandateNotificationBuilder {
        type Out = SourceMandateNotification;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => Deserialize::begin(&mut self.acss_debit),
                "amount" => Deserialize::begin(&mut self.amount),
                "bacs_debit" => Deserialize::begin(&mut self.bacs_debit),
                "created" => Deserialize::begin(&mut self.created),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "reason" => Deserialize::begin(&mut self.reason),
                "sepa_debit" => Deserialize::begin(&mut self.sepa_debit),
                "source" => Deserialize::begin(&mut self.source),
                "status" => Deserialize::begin(&mut self.status),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                acss_debit: self.acss_debit.take()?,
                amount: self.amount?,
                bacs_debit: self.bacs_debit.take()?,
                created: self.created?,
                id: self.id.take()?,
                livemode: self.livemode?,
                reason: self.reason.take()?,
                sepa_debit: self.sepa_debit.take()?,
                source: self.source.take()?,
                status: self.status.take()?,
                type_: self.type_.take()?,
            })
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

    impl ObjectDeser for SourceMandateNotification {
        type Builder = SourceMandateNotificationBuilder;
    }

    impl FromValueOpt for SourceMandateNotification {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceMandateNotificationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "acss_debit" => b.acss_debit = Some(FromValueOpt::from_value(v)?),
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "bacs_debit" => b.bacs_debit = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "reason" => b.reason = Some(FromValueOpt::from_value(v)?),
                    "sepa_debit" => b.sepa_debit = Some(FromValueOpt::from_value(v)?),
                    "source" => b.source = Some(FromValueOpt::from_value(v)?),
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
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
