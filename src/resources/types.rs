use serde::{Deserialize, Serialize};

use crate::params::Timestamp;

/// An enum representing the versions of the Stripe API.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ApiVersion {
    #[serde(rename = "2011-01-01")]
    V2011_01_01,
    #[serde(rename = "2011-06-21")]
    V2011_06_21,
    #[serde(rename = "2011-06-28")]
    V2011_06_28,
    #[serde(rename = "2011-08-01")]
    V2011_08_01,
    #[serde(rename = "2011-09-15")]
    V2011_09_15,
    #[serde(rename = "2011-11-17")]
    V2011_11_17,
    #[serde(rename = "2012-02-23")]
    V2012_02_23,
    #[serde(rename = "2012-03-25")]
    V2012_03_25,
    #[serde(rename = "2012-06-18")]
    V2012_06_18,
    #[serde(rename = "2012-06-28")]
    V2012_06_28,
    #[serde(rename = "2012-07-09")]
    V2012_07_09,
    #[serde(rename = "2012-09-24")]
    V2012_09_24,
    #[serde(rename = "2012-10-26")]
    V2012_10_26,
    #[serde(rename = "2012-11-07")]
    V2012_11_07,
    #[serde(rename = "2013-02-11")]
    V2013_02_11,
    #[serde(rename = "2013-02-13")]
    V2013_02_13,
    #[serde(rename = "2013-07-05")]
    V2013_07_05,
    #[serde(rename = "2013-08-12")]
    V2013_08_12,
    #[serde(rename = "2013-08-13")]
    V2013_08_13,
    #[serde(rename = "2013-10-29")]
    V2013_10_29,
    #[serde(rename = "2013-12-03")]
    V2013_12_03,
    #[serde(rename = "2014-01-31")]
    V2014_01_31,
    #[serde(rename = "2014-03-13")]
    V2014_03_13,
    #[serde(rename = "2014-03-28")]
    V2014_03_28,
    #[serde(rename = "2014-05-19")]
    V2014_05_19,
    #[serde(rename = "2014-06-13")]
    V2014_06_13,
    #[serde(rename = "2014-06-17")]
    V2014_06_17,
    #[serde(rename = "2014-07-22")]
    V2014_07_22,
    #[serde(rename = "2014-07-26")]
    V2014_07_26,
    #[serde(rename = "2014-08-04")]
    V2014_08_04,
    #[serde(rename = "2014-08-20")]
    V2014_08_20,
    #[serde(rename = "2014-09-08")]
    V2014_09_08,
    #[serde(rename = "2014-10-07")]
    V2014_10_07,
    #[serde(rename = "2014-11-05")]
    V2014_11_05,
    #[serde(rename = "2014-11-20")]
    V2014_11_20,
    #[serde(rename = "2014-12-08")]
    V2014_12_08,
    #[serde(rename = "2014-12-17")]
    V2014_12_17,
    #[serde(rename = "2014-12-22")]
    V2014_12_22,
    #[serde(rename = "2015-01-11")]
    V2015_01_11,
    #[serde(rename = "2015-01-26")]
    V2015_01_26,
    #[serde(rename = "2015-02-10")]
    V2015_02_10,
    #[serde(rename = "2015-02-16")]
    V2015_02_16,
    #[serde(rename = "2015-02-18")]
    V2015_02_18,
    #[serde(rename = "2015-03-24")]
    V2015_03_24,
    #[serde(rename = "2015-04-07")]
    V2015_04_07,
    #[serde(rename = "2015-06-15")]
    V2015_06_15,
    #[serde(rename = "2015-07-07")]
    V2015_07_07,
    #[serde(rename = "2015-07-13")]
    V2015_07_13,
    #[serde(rename = "2015-07-28")]
    V2015_07_28,
    #[serde(rename = "2015-08-07")]
    V2015_08_07,
    #[serde(rename = "2015-08-19")]
    V2015_08_19,
    #[serde(rename = "2015-09-03")]
    V2015_09_03,
    #[serde(rename = "2015-09-08")]
    V2015_09_08,
    #[serde(rename = "2015-09-23")]
    V2015_09_23,
    #[serde(rename = "2015-10-01")]
    V2015_10_01,
    #[serde(rename = "2015-10-12")]
    V2015_10_12,
    #[serde(rename = "2015-10-16")]
    V2015_10_16,
    #[serde(rename = "2016-02-03")]
    V2016_02_03,
    #[serde(rename = "2016-02-19")]
    V2016_02_19,
    #[serde(rename = "2016-02-22")]
    V2016_02_22,
    #[serde(rename = "2016-02-23")]
    V2016_02_23,
    #[serde(rename = "2016-02-29")]
    V2016_02_29,
    #[serde(rename = "2016-03-07")]
    V2016_03_07,
    #[serde(rename = "2016-06-15")]
    V2016_06_15,
    #[serde(rename = "2016-07-06")]
    V2016_07_06,
    #[serde(rename = "2016-10-19")]
    V2016_10_19,
    #[serde(rename = "2017-01-27")]
    V2017_01_27,
    #[serde(rename = "2017-02-14")]
    V2017_02_14,
    #[serde(rename = "2017-04-06")]
    V2017_04_06,
    #[serde(rename = "2017-05-25")]
    V2017_05_25,
    #[serde(rename = "2017-06-05")]
    V2017_06_05,
    #[serde(rename = "2017-08-15")]
    V2017_08_15,
    #[serde(rename = "2017-12-14")]
    V2017_12_14,
    #[serde(rename = "2018-01-23")]
    V2018_01_23,
    #[serde(rename = "2018-02-05")]
    V2018_02_05,
    #[serde(rename = "2018-02-06")]
    V2018_02_06,
    #[serde(rename = "2018-02-28")]
    V2018_02_28,
    #[serde(rename = "2018-05-21")]
    V2018_05_21,
    #[serde(rename = "2018-07-27")]
    V2018_07_27,
    #[serde(rename = "2018-08-23")]
    V2018_08_23,
    #[serde(rename = "2018-09-06")]
    V2018_09_06,
    #[serde(rename = "2018-09-24")]
    V2018_09_24,
    #[serde(rename = "2018-10-31")]
    V2018_10_31,
    #[serde(rename = "2018-11-08")]
    V2018_11_08,
    #[serde(rename = "2019-02-11")]
    V2019_02_11,
    #[serde(rename = "2019-02-19")]
    V2019_02_19,
    #[serde(rename = "2019-03-14")]
    V2019_03_14,
    #[serde(rename = "2019-05-16")]
    V2019_05_16,
    #[serde(rename = "2019-08-14")]
    V2019_08_14,
    #[serde(rename = "2019-09-09")]
    V2019_09_09,
    #[serde(rename = "2020-08-27")]
    V2020_08_27,
    #[serde(rename = "2022-08-01")]
    V2022_08_01,
    #[serde(rename = "2022-11-15")]
    V2022_11_15,
    #[serde(rename = "2023-08-16")]
    V2023_08_16,
    #[serde(rename = "2023-10-16")]
    V2023_10_16,
}

impl ApiVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            ApiVersion::V2011_01_01 => "2011-01-01",
            ApiVersion::V2011_06_21 => "2011-06-21",
            ApiVersion::V2011_06_28 => "2011-06-28",
            ApiVersion::V2011_08_01 => "2011-08-01",
            ApiVersion::V2011_09_15 => "2011-09-15",
            ApiVersion::V2011_11_17 => "2011-11-17",
            ApiVersion::V2012_02_23 => "2012-02-23",
            ApiVersion::V2012_03_25 => "2012-03-25",
            ApiVersion::V2012_06_18 => "2012-06-18",
            ApiVersion::V2012_06_28 => "2012-06-28",
            ApiVersion::V2012_07_09 => "2012-07-09",
            ApiVersion::V2012_09_24 => "2012-09-24",
            ApiVersion::V2012_10_26 => "2012-10-26",
            ApiVersion::V2012_11_07 => "2012-11-07",
            ApiVersion::V2013_02_11 => "2013-02-11",
            ApiVersion::V2013_02_13 => "2013-02-13",
            ApiVersion::V2013_07_05 => "2013-07-05",
            ApiVersion::V2013_08_12 => "2013-08-12",
            ApiVersion::V2013_08_13 => "2013-08-13",
            ApiVersion::V2013_10_29 => "2013-10-29",
            ApiVersion::V2013_12_03 => "2013-12-03",
            ApiVersion::V2014_01_31 => "2014-01-31",
            ApiVersion::V2014_03_13 => "2014-03-13",
            ApiVersion::V2014_03_28 => "2014-03-28",
            ApiVersion::V2014_05_19 => "2014-05-19",
            ApiVersion::V2014_06_13 => "2014-06-13",
            ApiVersion::V2014_06_17 => "2014-06-17",
            ApiVersion::V2014_07_22 => "2014-07-22",
            ApiVersion::V2014_07_26 => "2014-07-26",
            ApiVersion::V2014_08_04 => "2014-08-04",
            ApiVersion::V2014_08_20 => "2014-08-20",
            ApiVersion::V2014_09_08 => "2014-09-08",
            ApiVersion::V2014_10_07 => "2014-10-07",
            ApiVersion::V2014_11_05 => "2014-11-05",
            ApiVersion::V2014_11_20 => "2014-11-20",
            ApiVersion::V2014_12_08 => "2014-12-08",
            ApiVersion::V2014_12_17 => "2014-12-17",
            ApiVersion::V2014_12_22 => "2014-12-22",
            ApiVersion::V2015_01_11 => "2015-01-11",
            ApiVersion::V2015_01_26 => "2015-01-26",
            ApiVersion::V2015_02_10 => "2015-02-10",
            ApiVersion::V2015_02_16 => "2015-02-16",
            ApiVersion::V2015_02_18 => "2015-02-18",
            ApiVersion::V2015_03_24 => "2015-03-24",
            ApiVersion::V2015_04_07 => "2015-04-07",
            ApiVersion::V2015_06_15 => "2015-06-15",
            ApiVersion::V2015_07_07 => "2015-07-07",
            ApiVersion::V2015_07_13 => "2015-07-13",
            ApiVersion::V2015_07_28 => "2015-07-28",
            ApiVersion::V2015_08_07 => "2015-08-07",
            ApiVersion::V2015_08_19 => "2015-08-19",
            ApiVersion::V2015_09_03 => "2015-09-03",
            ApiVersion::V2015_09_08 => "2015-09-08",
            ApiVersion::V2015_09_23 => "2015-09-23",
            ApiVersion::V2015_10_01 => "2015-10-01",
            ApiVersion::V2015_10_12 => "2015-10-12",
            ApiVersion::V2015_10_16 => "2015-10-16",
            ApiVersion::V2016_02_03 => "2016-02-03",
            ApiVersion::V2016_02_19 => "2016-02-19",
            ApiVersion::V2016_02_22 => "2016-02-22",
            ApiVersion::V2016_02_23 => "2016-02-23",
            ApiVersion::V2016_02_29 => "2016-02-29",
            ApiVersion::V2016_03_07 => "2016-03-07",
            ApiVersion::V2016_06_15 => "2016-06-15",
            ApiVersion::V2016_07_06 => "2016-07-06",
            ApiVersion::V2016_10_19 => "2016-10-19",
            ApiVersion::V2017_01_27 => "2017-01-27",
            ApiVersion::V2017_02_14 => "2017-02-14",
            ApiVersion::V2017_04_06 => "2017-04-06",
            ApiVersion::V2017_05_25 => "2017-05-25",
            ApiVersion::V2017_06_05 => "2017-06-05",
            ApiVersion::V2017_08_15 => "2017-08-15",
            ApiVersion::V2017_12_14 => "2017-12-14",
            ApiVersion::V2018_01_23 => "2018-01-23",
            ApiVersion::V2018_02_05 => "2018-02-05",
            ApiVersion::V2018_02_06 => "2018-02-06",
            ApiVersion::V2018_02_28 => "2018-02-28",
            ApiVersion::V2018_05_21 => "2018-05-21",
            ApiVersion::V2018_07_27 => "2018-07-27",
            ApiVersion::V2018_08_23 => "2018-08-23",
            ApiVersion::V2018_09_06 => "2018-09-06",
            ApiVersion::V2018_09_24 => "2018-09-24",
            ApiVersion::V2018_10_31 => "2018-10-31",
            ApiVersion::V2018_11_08 => "2018-11-08",
            ApiVersion::V2019_02_11 => "2019-02-11",
            ApiVersion::V2019_02_19 => "2019-02-19",
            ApiVersion::V2019_03_14 => "2019-03-14",
            ApiVersion::V2019_05_16 => "2019-05-16",
            ApiVersion::V2019_08_14 => "2019-08-14",
            ApiVersion::V2019_09_09 => "2019-09-09",
            ApiVersion::V2020_08_27 => "2020-08-27",
            ApiVersion::V2022_08_01 => "2022-08-01",
            ApiVersion::V2022_11_15 => "2022-11-15",
            ApiVersion::V2023_08_16 => "2023-08-16",
            ApiVersion::V2023_10_16 => "2023-10-16",
        }
    }
}

impl AsRef<str> for ApiVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/* Developers note -- DelayDays and DelayDaysOther are not worth the trouble
 * to automate.  Recommend letting the mapping stand*/
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum DelayDays {
    Days(u32),
    Other(DelayDaysOther),
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DelayDaysOther {
    Minimum,
}

impl DelayDays {
    pub fn days(n: u32) -> Self {
        DelayDays::Days(n)
    }
    pub fn minimum() -> Self {
        DelayDays::Other(DelayDaysOther::Minimum)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Scheduled {
    Timestamp(Timestamp),
    Other(ScheduledOther),
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ScheduledOther {
    Now,
}

impl Scheduled {
    pub fn at(ts: Timestamp) -> Self {
        Scheduled::Timestamp(ts)
    }
    pub fn now() -> Self {
        Scheduled::Other(ScheduledOther::Now)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum UpTo {
    Max(u64),
    Other(UpToOther),
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpToOther {
    Inf,
}

impl UpTo {
    pub fn max(n: u64) -> Self {
        UpTo::Max(n)
    }
    pub fn now() -> Self {
        UpTo::Other(UpToOther::Inf)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum PaymentIntentOffSession {
    Exists(bool),
    Other(OffSessionOther),
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OffSessionOther {
    #[serde(rename = "one_off")]
    OneOff,
    #[serde(rename = "recurring")]
    Recurring,
}

impl PaymentIntentOffSession {
    pub fn exists(n: bool) -> Self {
        PaymentIntentOffSession::Exists(n)
    }
    pub fn frequency(n: OffSessionOther) -> Self {
        match n {
            OffSessionOther::OneOff => PaymentIntentOffSession::Other(OffSessionOther::OneOff),
            OffSessionOther::Recurring => {
                PaymentIntentOffSession::Other(OffSessionOther::Recurring)
            }
        }
    }
}
