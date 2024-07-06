#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ApiVersion {
    V2011_01_01,
    V2011_06_21,
    V2011_06_28,
    V2011_08_01,
    V2011_09_15,
    V2011_11_17,
    V2012_02_23,
    V2012_03_25,
    V2012_06_18,
    V2012_06_28,
    V2012_07_09,
    V2012_09_24,
    V2012_10_26,
    V2012_11_07,
    V2013_02_11,
    V2013_02_13,
    V2013_07_05,
    V2013_08_12,
    V2013_08_13,
    V2013_10_29,
    V2013_12_03,
    V2014_01_31,
    V2014_03_13,
    V2014_03_28,
    V2014_05_19,
    V2014_06_13,
    V2014_06_17,
    V2014_07_22,
    V2014_07_26,
    V2014_08_04,
    V2014_08_20,
    V2014_09_08,
    V2014_10_07,
    V2014_11_05,
    V2014_11_20,
    V2014_12_08,
    V2014_12_17,
    V2014_12_22,
    V2015_01_11,
    V2015_01_26,
    V2015_02_10,
    V2015_02_16,
    V2015_02_18,
    V2015_03_24,
    V2015_04_07,
    V2015_06_15,
    V2015_07_07,
    V2015_07_13,
    V2015_07_28,
    V2015_08_07,
    V2015_08_19,
    V2015_09_03,
    V2015_09_08,
    V2015_09_23,
    V2015_10_01,
    V2015_10_12,
    V2015_10_16,
    V2016_02_03,
    V2016_02_19,
    V2016_02_22,
    V2016_02_23,
    V2016_02_29,
    V2016_03_07,
    V2016_06_15,
    V2016_07_06,
    V2016_10_19,
    V2017_01_27,
    V2017_02_14,
    V2017_04_06,
    V2017_05_25,
    V2017_06_05,
    V2017_08_15,
    V2017_12_14,
    V2018_01_23,
    V2018_02_05,
    V2018_02_06,
    V2018_02_28,
    V2018_05_21,
    V2018_07_27,
    V2018_08_23,
    V2018_09_06,
    V2018_09_24,
    V2018_10_31,
    V2018_11_08,
    V2019_02_11,
    V2019_02_19,
    V2019_03_14,
    V2019_05_16,
    V2019_08_14,
    V2019_09_09,
    V2019_10_08,
    V2019_10_17,
    V2019_11_05,
    V2019_12_03,
    V2020_03_02,
    V2020_08_27,
    V2022_08_01,
    V2022_11_15,
    V2023_08_16,
    V2023_10_16,
    V2024_04_10,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl ApiVersion {
    pub fn as_str(self) -> &'static str {
        use ApiVersion::*;
        match self {
            V2011_01_01 => "2011-01-01",
            V2011_06_21 => "2011-06-21",
            V2011_06_28 => "2011-06-28",
            V2011_08_01 => "2011-08-01",
            V2011_09_15 => "2011-09-15",
            V2011_11_17 => "2011-11-17",
            V2012_02_23 => "2012-02-23",
            V2012_03_25 => "2012-03-25",
            V2012_06_18 => "2012-06-18",
            V2012_06_28 => "2012-06-28",
            V2012_07_09 => "2012-07-09",
            V2012_09_24 => "2012-09-24",
            V2012_10_26 => "2012-10-26",
            V2012_11_07 => "2012-11-07",
            V2013_02_11 => "2013-02-11",
            V2013_02_13 => "2013-02-13",
            V2013_07_05 => "2013-07-05",
            V2013_08_12 => "2013-08-12",
            V2013_08_13 => "2013-08-13",
            V2013_10_29 => "2013-10-29",
            V2013_12_03 => "2013-12-03",
            V2014_01_31 => "2014-01-31",
            V2014_03_13 => "2014-03-13",
            V2014_03_28 => "2014-03-28",
            V2014_05_19 => "2014-05-19",
            V2014_06_13 => "2014-06-13",
            V2014_06_17 => "2014-06-17",
            V2014_07_22 => "2014-07-22",
            V2014_07_26 => "2014-07-26",
            V2014_08_04 => "2014-08-04",
            V2014_08_20 => "2014-08-20",
            V2014_09_08 => "2014-09-08",
            V2014_10_07 => "2014-10-07",
            V2014_11_05 => "2014-11-05",
            V2014_11_20 => "2014-11-20",
            V2014_12_08 => "2014-12-08",
            V2014_12_17 => "2014-12-17",
            V2014_12_22 => "2014-12-22",
            V2015_01_11 => "2015-01-11",
            V2015_01_26 => "2015-01-26",
            V2015_02_10 => "2015-02-10",
            V2015_02_16 => "2015-02-16",
            V2015_02_18 => "2015-02-18",
            V2015_03_24 => "2015-03-24",
            V2015_04_07 => "2015-04-07",
            V2015_06_15 => "2015-06-15",
            V2015_07_07 => "2015-07-07",
            V2015_07_13 => "2015-07-13",
            V2015_07_28 => "2015-07-28",
            V2015_08_07 => "2015-08-07",
            V2015_08_19 => "2015-08-19",
            V2015_09_03 => "2015-09-03",
            V2015_09_08 => "2015-09-08",
            V2015_09_23 => "2015-09-23",
            V2015_10_01 => "2015-10-01",
            V2015_10_12 => "2015-10-12",
            V2015_10_16 => "2015-10-16",
            V2016_02_03 => "2016-02-03",
            V2016_02_19 => "2016-02-19",
            V2016_02_22 => "2016-02-22",
            V2016_02_23 => "2016-02-23",
            V2016_02_29 => "2016-02-29",
            V2016_03_07 => "2016-03-07",
            V2016_06_15 => "2016-06-15",
            V2016_07_06 => "2016-07-06",
            V2016_10_19 => "2016-10-19",
            V2017_01_27 => "2017-01-27",
            V2017_02_14 => "2017-02-14",
            V2017_04_06 => "2017-04-06",
            V2017_05_25 => "2017-05-25",
            V2017_06_05 => "2017-06-05",
            V2017_08_15 => "2017-08-15",
            V2017_12_14 => "2017-12-14",
            V2018_01_23 => "2018-01-23",
            V2018_02_05 => "2018-02-05",
            V2018_02_06 => "2018-02-06",
            V2018_02_28 => "2018-02-28",
            V2018_05_21 => "2018-05-21",
            V2018_07_27 => "2018-07-27",
            V2018_08_23 => "2018-08-23",
            V2018_09_06 => "2018-09-06",
            V2018_09_24 => "2018-09-24",
            V2018_10_31 => "2018-10-31",
            V2018_11_08 => "2018-11-08",
            V2019_02_11 => "2019-02-11",
            V2019_02_19 => "2019-02-19",
            V2019_03_14 => "2019-03-14",
            V2019_05_16 => "2019-05-16",
            V2019_08_14 => "2019-08-14",
            V2019_09_09 => "2019-09-09",
            V2019_10_08 => "2019-10-08",
            V2019_10_17 => "2019-10-17",
            V2019_11_05 => "2019-11-05",
            V2019_12_03 => "2019-12-03",
            V2020_03_02 => "2020-03-02",
            V2020_08_27 => "2020-08-27",
            V2022_08_01 => "2022-08-01",
            V2022_11_15 => "2022-11-15",
            V2023_08_16 => "2023-08-16",
            V2023_10_16 => "2023-10-16",
            V2024_04_10 => "2024-04-10",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for ApiVersion {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ApiVersion::*;
        match s {
            "2011-01-01" => Ok(V2011_01_01),
            "2011-06-21" => Ok(V2011_06_21),
            "2011-06-28" => Ok(V2011_06_28),
            "2011-08-01" => Ok(V2011_08_01),
            "2011-09-15" => Ok(V2011_09_15),
            "2011-11-17" => Ok(V2011_11_17),
            "2012-02-23" => Ok(V2012_02_23),
            "2012-03-25" => Ok(V2012_03_25),
            "2012-06-18" => Ok(V2012_06_18),
            "2012-06-28" => Ok(V2012_06_28),
            "2012-07-09" => Ok(V2012_07_09),
            "2012-09-24" => Ok(V2012_09_24),
            "2012-10-26" => Ok(V2012_10_26),
            "2012-11-07" => Ok(V2012_11_07),
            "2013-02-11" => Ok(V2013_02_11),
            "2013-02-13" => Ok(V2013_02_13),
            "2013-07-05" => Ok(V2013_07_05),
            "2013-08-12" => Ok(V2013_08_12),
            "2013-08-13" => Ok(V2013_08_13),
            "2013-10-29" => Ok(V2013_10_29),
            "2013-12-03" => Ok(V2013_12_03),
            "2014-01-31" => Ok(V2014_01_31),
            "2014-03-13" => Ok(V2014_03_13),
            "2014-03-28" => Ok(V2014_03_28),
            "2014-05-19" => Ok(V2014_05_19),
            "2014-06-13" => Ok(V2014_06_13),
            "2014-06-17" => Ok(V2014_06_17),
            "2014-07-22" => Ok(V2014_07_22),
            "2014-07-26" => Ok(V2014_07_26),
            "2014-08-04" => Ok(V2014_08_04),
            "2014-08-20" => Ok(V2014_08_20),
            "2014-09-08" => Ok(V2014_09_08),
            "2014-10-07" => Ok(V2014_10_07),
            "2014-11-05" => Ok(V2014_11_05),
            "2014-11-20" => Ok(V2014_11_20),
            "2014-12-08" => Ok(V2014_12_08),
            "2014-12-17" => Ok(V2014_12_17),
            "2014-12-22" => Ok(V2014_12_22),
            "2015-01-11" => Ok(V2015_01_11),
            "2015-01-26" => Ok(V2015_01_26),
            "2015-02-10" => Ok(V2015_02_10),
            "2015-02-16" => Ok(V2015_02_16),
            "2015-02-18" => Ok(V2015_02_18),
            "2015-03-24" => Ok(V2015_03_24),
            "2015-04-07" => Ok(V2015_04_07),
            "2015-06-15" => Ok(V2015_06_15),
            "2015-07-07" => Ok(V2015_07_07),
            "2015-07-13" => Ok(V2015_07_13),
            "2015-07-28" => Ok(V2015_07_28),
            "2015-08-07" => Ok(V2015_08_07),
            "2015-08-19" => Ok(V2015_08_19),
            "2015-09-03" => Ok(V2015_09_03),
            "2015-09-08" => Ok(V2015_09_08),
            "2015-09-23" => Ok(V2015_09_23),
            "2015-10-01" => Ok(V2015_10_01),
            "2015-10-12" => Ok(V2015_10_12),
            "2015-10-16" => Ok(V2015_10_16),
            "2016-02-03" => Ok(V2016_02_03),
            "2016-02-19" => Ok(V2016_02_19),
            "2016-02-22" => Ok(V2016_02_22),
            "2016-02-23" => Ok(V2016_02_23),
            "2016-02-29" => Ok(V2016_02_29),
            "2016-03-07" => Ok(V2016_03_07),
            "2016-06-15" => Ok(V2016_06_15),
            "2016-07-06" => Ok(V2016_07_06),
            "2016-10-19" => Ok(V2016_10_19),
            "2017-01-27" => Ok(V2017_01_27),
            "2017-02-14" => Ok(V2017_02_14),
            "2017-04-06" => Ok(V2017_04_06),
            "2017-05-25" => Ok(V2017_05_25),
            "2017-06-05" => Ok(V2017_06_05),
            "2017-08-15" => Ok(V2017_08_15),
            "2017-12-14" => Ok(V2017_12_14),
            "2018-01-23" => Ok(V2018_01_23),
            "2018-02-05" => Ok(V2018_02_05),
            "2018-02-06" => Ok(V2018_02_06),
            "2018-02-28" => Ok(V2018_02_28),
            "2018-05-21" => Ok(V2018_05_21),
            "2018-07-27" => Ok(V2018_07_27),
            "2018-08-23" => Ok(V2018_08_23),
            "2018-09-06" => Ok(V2018_09_06),
            "2018-09-24" => Ok(V2018_09_24),
            "2018-10-31" => Ok(V2018_10_31),
            "2018-11-08" => Ok(V2018_11_08),
            "2019-02-11" => Ok(V2019_02_11),
            "2019-02-19" => Ok(V2019_02_19),
            "2019-03-14" => Ok(V2019_03_14),
            "2019-05-16" => Ok(V2019_05_16),
            "2019-08-14" => Ok(V2019_08_14),
            "2019-09-09" => Ok(V2019_09_09),
            "2019-10-08" => Ok(V2019_10_08),
            "2019-10-17" => Ok(V2019_10_17),
            "2019-11-05" => Ok(V2019_11_05),
            "2019-12-03" => Ok(V2019_12_03),
            "2020-03-02" => Ok(V2020_03_02),
            "2020-08-27" => Ok(V2020_08_27),
            "2022-08-01" => Ok(V2022_08_01),
            "2022-11-15" => Ok(V2022_11_15),
            "2023-08-16" => Ok(V2023_08_16),
            "2023-10-16" => Ok(V2023_10_16),
            "2024-04-10" => Ok(V2024_04_10),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ApiVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ApiVersion {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ApiVersion> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ApiVersion::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ApiVersion);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ApiVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
