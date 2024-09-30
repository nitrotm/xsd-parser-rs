use std::{fmt, str::FromStr};

use rust_decimal::{Decimal as RDecimal, Error as RError};
use xsd_macro_utils::UtilsDefaultSerde;

#[derive(Default, Clone, PartialEq, PartialOrd, Debug, UtilsDefaultSerde)]
pub struct Decimal(pub RDecimal);

impl Decimal {
    pub fn from_bigdecimal(bigdecimal: RDecimal) -> Self {
        Decimal(bigdecimal)
    }

    pub fn to_bigdecimal(&self) -> RDecimal {
        self.0.clone()
    }
}

impl FromStr for Decimal {
    type Err = RError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Decimal(RDecimal::from_str(s)?))
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use yaserde_derive::{YaDeserialize, YaSerialize};

    use super::*;
    use crate::utils::xml_eq::assert_xml_eq;

    #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespace = "t: test")]
    pub struct DecimalPair {
        #[yaserde(prefix = "t", rename = "First")]
        pub first: Decimal,

        #[yaserde(prefix = "t", rename = "Second")]
        pub second: Decimal,
    }

    #[test]
    fn integer_serialize_test() {
        let expected = r#"<?xml version="1.0" encoding="utf-8"?>
            <t:DecimalPair xmlns:t="test">
                <t:First>0.01234</t:First>
                <t:Second>-12.34</t:Second>
            </t:DecimalPair>
            "#;
        let i = DecimalPair {
            first: Decimal::from_bigdecimal(RDecimal::new(1234, 5)),
            second: Decimal::from_bigdecimal(RDecimal::new(-1234, 2)),
        };
        let actual = yaserde::ser::to_string(&i).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn integer_deserialize_test() {
        // Value "+0.01234" is used to check optional plus sign deserialization.
        let s = r#"<?xml version="1.0" encoding="utf-8"?>
            <t:DecimalPair xmlns:t="test">
                <t:First>+0.01234</t:First>
                <t:Second>-12.34</t:Second>
            </t:DecimalPair>
            "#;
        let i: DecimalPair = yaserde::de::from_str(s).unwrap();
        assert_eq!(i.first.to_bigdecimal(), RDecimal::new(1234, 5));
        assert_eq!(i.second.to_bigdecimal(), RDecimal::new(-1234, 2));
    }
}
