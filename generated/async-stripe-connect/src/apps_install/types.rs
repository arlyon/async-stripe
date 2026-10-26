/// An app install represents a Stripe App that is installed on an account. It reports the permissions,
/// content security policy entries, and endpoints that the installing account has authorized, along with any.
/// that the app's latest version requests but the account has not authorized yet.
/// Use the Install API to.
/// install, reauthorize, and uninstall apps, and to check the state of existing installs.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AppsInstall {
    /// The ID of the account that the app install belongs to.
    pub account: String,
    /// The ID of the app installed.
    pub app: String,
    /// Whether the installer must authorize pending permissions, content security policy entries, or endpoints.
    /// For private apps, `approval_required` stays `false`; creating or reauthorizing the install through the API installs the newest completed upload and grants its permissions.
    pub approval_required: bool,
    /// The authorization code for an oauth app install.
    pub auth_code: Option<String>,
    /// The distribution channel associated with the app install.
    pub channel: AppsInstallChannel,
    pub content_security_policy_granted:
        stripe_connect::AppServiceResourceInstallContentSecurityPolicy,
    pub content_security_policy_pending:
        stripe_connect::AppServiceResourceInstallContentSecurityPolicy,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The ID of the embedding platform that created the install, if applicable.
    pub created_by: Option<String>,
    /// The endpoint URLs authorized by the installer.
    pub endpoints_granted: Vec<String>,
    /// The endpoint URLs requested by the latest app version that the installer has not authorized.
    pub endpoints_pending: Vec<String>,
    /// Unique identifier for the object.
    pub id: stripe_connect::AppsInstallId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The permissions authorized by the installer.
    pub permissions_granted: Vec<String>,
    /// The permissions requested by the latest app version that the installer has not authorized.
    pub permissions_pending: Vec<String>,
    /// The status of the app install.
    pub status: stripe_connect::AppsInstallStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AppsInstall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AppsInstall").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AppsInstallBuilder {
    account: Option<String>,
    app: Option<String>,
    approval_required: Option<bool>,
    auth_code: Option<Option<String>>,
    channel: Option<AppsInstallChannel>,
    content_security_policy_granted:
        Option<stripe_connect::AppServiceResourceInstallContentSecurityPolicy>,
    content_security_policy_pending:
        Option<stripe_connect::AppServiceResourceInstallContentSecurityPolicy>,
    created: Option<stripe_types::Timestamp>,
    created_by: Option<Option<String>>,
    endpoints_granted: Option<Vec<String>>,
    endpoints_pending: Option<Vec<String>>,
    id: Option<stripe_connect::AppsInstallId>,
    livemode: Option<bool>,
    permissions_granted: Option<Vec<String>>,
    permissions_pending: Option<Vec<String>>,
    status: Option<stripe_connect::AppsInstallStatus>,
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

    impl Deserialize for AppsInstall {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AppsInstall>,
        builder: AppsInstallBuilder,
    }

    impl Visitor for Place<AppsInstall> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AppsInstallBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AppsInstallBuilder {
        type Out = AppsInstall;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),
                "app" => Deserialize::begin(&mut self.app),
                "approval_required" => Deserialize::begin(&mut self.approval_required),
                "auth_code" => Deserialize::begin(&mut self.auth_code),
                "channel" => Deserialize::begin(&mut self.channel),
                "content_security_policy_granted" => {
                    Deserialize::begin(&mut self.content_security_policy_granted)
                }
                "content_security_policy_pending" => {
                    Deserialize::begin(&mut self.content_security_policy_pending)
                }
                "created" => Deserialize::begin(&mut self.created),
                "created_by" => Deserialize::begin(&mut self.created_by),
                "endpoints_granted" => Deserialize::begin(&mut self.endpoints_granted),
                "endpoints_pending" => Deserialize::begin(&mut self.endpoints_pending),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "permissions_granted" => Deserialize::begin(&mut self.permissions_granted),
                "permissions_pending" => Deserialize::begin(&mut self.permissions_pending),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account: None,
                app: None,
                approval_required: None,
                auth_code: Some(None),
                channel: None,
                content_security_policy_granted: None,
                content_security_policy_pending: None,
                created: None,
                created_by: Some(None),
                endpoints_granted: None,
                endpoints_pending: None,
                id: None,
                livemode: None,
                permissions_granted: None,
                permissions_pending: None,
                status: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account),
                Some(app),
                Some(approval_required),
                Some(auth_code),
                Some(channel),
                Some(content_security_policy_granted),
                Some(content_security_policy_pending),
                Some(created),
                Some(created_by),
                Some(endpoints_granted),
                Some(endpoints_pending),
                Some(id),
                Some(livemode),
                Some(permissions_granted),
                Some(permissions_pending),
                Some(status),
            ) = (
                self.account.take(),
                self.app.take(),
                self.approval_required,
                self.auth_code.take(),
                self.channel.take(),
                self.content_security_policy_granted.take(),
                self.content_security_policy_pending.take(),
                self.created,
                self.created_by.take(),
                self.endpoints_granted.take(),
                self.endpoints_pending.take(),
                self.id.take(),
                self.livemode,
                self.permissions_granted.take(),
                self.permissions_pending.take(),
                self.status.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                account,
                app,
                approval_required,
                auth_code,
                channel,
                content_security_policy_granted,
                content_security_policy_pending,
                created,
                created_by,
                endpoints_granted,
                endpoints_pending,
                id,
                livemode,
                permissions_granted,
                permissions_pending,
                status,
            })
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

    impl ObjectDeser for AppsInstall {
        type Builder = AppsInstallBuilder;
    }

    impl FromValueOpt for AppsInstall {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AppsInstallBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = FromValueOpt::from_value(v),
                    "app" => b.app = FromValueOpt::from_value(v),
                    "approval_required" => b.approval_required = FromValueOpt::from_value(v),
                    "auth_code" => b.auth_code = FromValueOpt::from_value(v),
                    "channel" => b.channel = FromValueOpt::from_value(v),
                    "content_security_policy_granted" => {
                        b.content_security_policy_granted = FromValueOpt::from_value(v)
                    }
                    "content_security_policy_pending" => {
                        b.content_security_policy_pending = FromValueOpt::from_value(v)
                    }
                    "created" => b.created = FromValueOpt::from_value(v),
                    "created_by" => b.created_by = FromValueOpt::from_value(v),
                    "endpoints_granted" => b.endpoints_granted = FromValueOpt::from_value(v),
                    "endpoints_pending" => b.endpoints_pending = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "permissions_granted" => b.permissions_granted = FromValueOpt::from_value(v),
                    "permissions_pending" => b.permissions_pending = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for AppsInstall {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("AppsInstall", 17)?;
        s.serialize_field("account", &self.account)?;
        s.serialize_field("app", &self.app)?;
        s.serialize_field("approval_required", &self.approval_required)?;
        s.serialize_field("auth_code", &self.auth_code)?;
        s.serialize_field("channel", &self.channel)?;
        s.serialize_field(
            "content_security_policy_granted",
            &self.content_security_policy_granted,
        )?;
        s.serialize_field(
            "content_security_policy_pending",
            &self.content_security_policy_pending,
        )?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("created_by", &self.created_by)?;
        s.serialize_field("endpoints_granted", &self.endpoints_granted)?;
        s.serialize_field("endpoints_pending", &self.endpoints_pending)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("permissions_granted", &self.permissions_granted)?;
        s.serialize_field("permissions_pending", &self.permissions_pending)?;
        s.serialize_field("status", &self.status)?;

        s.serialize_field("object", "apps.install")?;
        s.end()
    }
}
/// The distribution channel associated with the app install.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AppsInstallChannel {
    PrivateLive,
    PrivateTest,
    Public,
    Review,
    Testing,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AppsInstallChannel {
    pub fn as_str(&self) -> &str {
        use AppsInstallChannel::*;
        match self {
            PrivateLive => "private_live",
            PrivateTest => "private_test",
            Public => "public",
            Review => "review",
            Testing => "testing",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AppsInstallChannel {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AppsInstallChannel::*;
        match s {
            "private_live" => Ok(PrivateLive),
            "private_test" => Ok(PrivateTest),
            "public" => Ok(Public),
            "review" => Ok(Review),
            "testing" => Ok(Testing),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "AppsInstallChannel");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AppsInstallChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AppsInstallChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AppsInstallChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AppsInstallChannel)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AppsInstallChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AppsInstallChannel {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AppsInstallChannel> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AppsInstallChannel::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AppsInstallChannel);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AppsInstallChannel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for AppsInstall {
    type Id = stripe_connect::AppsInstallId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(AppsInstallId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AppsInstallStatus {
    InstallFailed,
    Installed,
    Installing,
    UninstallFailed,
    Uninstalling,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AppsInstallStatus {
    pub fn as_str(&self) -> &str {
        use AppsInstallStatus::*;
        match self {
            InstallFailed => "install_failed",
            Installed => "installed",
            Installing => "installing",
            UninstallFailed => "uninstall_failed",
            Uninstalling => "uninstalling",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AppsInstallStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AppsInstallStatus::*;
        match s {
            "install_failed" => Ok(InstallFailed),
            "installed" => Ok(Installed),
            "installing" => Ok(Installing),
            "uninstall_failed" => Ok(UninstallFailed),
            "uninstalling" => Ok(Uninstalling),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "AppsInstallStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AppsInstallStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AppsInstallStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AppsInstallStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AppsInstallStatus)).finish_non_exhaustive()
    }
}
impl serde::Serialize for AppsInstallStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AppsInstallStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AppsInstallStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AppsInstallStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AppsInstallStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AppsInstallStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
