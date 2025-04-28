
use core::fmt;
use std::{collections::HashMap, iter::Sum};
use rust_decimal::{prelude::{FromPrimitive, ToPrimitive}, Decimal};
use rust_decimal_macros::dec;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Visitor};

#[derive(Debug, Clone, PartialEq)]
pub enum VatRate {
    Percentage(Decimal),
    Exempt,
}

impl fmt::Display for VatRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VatRate::Percentage(rate) => write!(f, "{}%", rate.normalize()),
            VatRate::Exempt => write!(f,"zw."),
        }
    }
}


// SERIALIZACJA
impl Serialize for VatRate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            VatRate::Percentage(rate) => {

                // TODO consider normalize to str rate.normalize().to_string()
                // serializer.serialize_str(&rate.normalize().to_string())

                if rate.round() == *rate {
                    serializer.serialize_u64(rate.to_u64().expect("Decimalt to u64 went wrong"))
                } else {
                    serializer.serialize_f64(rate.to_f64().expect("Decimal to f64 went wrong"))
                }
            },
            VatRate::Exempt => serializer.serialize_str("zw."),
        }
    }
}

impl<'de> Deserialize<'de> for VatRate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VatRateVisitor;

        impl<'de> Visitor<'de> for VatRateVisitor {
            type Value = VatRate;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a VAT rate as a decimal number or 'zw.' string")
            }

            fn visit_str<E>(self, raw_value: &str) -> Result<VatRate, E>
            where
                E: de::Error,
            {
                println!("raw_value {}",raw_value);
                if raw_value == "zw." {
                    Ok(VatRate::Exempt)
                } else {

                    let value = if let Some(stripped) = raw_value.strip_suffix("%") {
                        stripped.trim()
                    }
                    else {
                        raw_value
                    };


                    Decimal::from_str_exact(value)
                        .map(VatRate::Percentage)
                        .map_err(|_| E::custom(format!("Invalid VAT percentage: {}", raw_value)))
                }
            }

            fn visit_f64<E>(self, value: f64) -> Result<VatRate, E>
            where
                E: de::Error,
            {
                Ok(VatRate::Percentage(Decimal::from_f64(value).ok_or_else(|| {
                    E::custom(format!("Invalid VAT percentage from f64: {}", value))
                })?))
            }

            fn visit_u64<E>(self, value: u64) -> Result<VatRate, E>
            where
                E: de::Error,
            {
                Ok(VatRate::Percentage(Decimal::from(value)))
            }

            fn visit_i64<E>(self, value: i64) -> Result<VatRate, E>
            where
                E: de::Error,
            {
                Ok(VatRate::Percentage(Decimal::from(value)))
            }
        }

        deserializer.deserialize_any(VatRateVisitor)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_display_percentage() {
        let vat = VatRate::Percentage(dec!(23.00));
        assert_eq!(vat.to_string(), "23%");
    }

    #[test]
    fn test_display_exempt() {
        let vat = VatRate::Exempt;
        assert_eq!(vat.to_string(), "zw.");
    }

    #[test]
    fn test_serialize_percentage() {
        let vat = VatRate::Percentage(dec!(8.00));
        let json = serde_json::to_string(&vat).unwrap();
        assert_eq!(json, "8");
    }

    #[test]
    fn test_serialize_exempt() {
        let vat = VatRate::Exempt;
        let json = serde_json::to_string(&vat).unwrap();
        assert_eq!(json, "\"zw.\"");
    }

    #[test]
    fn test_deserialize_percentage_from_number_with_stripp_pchar() {
        let json = r#""5%""#;
        let vat: VatRate = serde_json::from_str(json).unwrap();
        assert_eq!(vat, VatRate::Percentage(dec!(5.0)));
    }


    #[test]
    fn test_deserialize_percentage_from_number() {
        let json = "5";
        let vat: VatRate = serde_json::from_str(json).unwrap();
        assert_eq!(vat, VatRate::Percentage(dec!(5.0)));
    }

    #[test]
    fn test_deserialize_percentage_from_float() {
        let json = "5.5";
        let vat: VatRate = serde_json::from_str(json).unwrap();
        assert_eq!(vat, VatRate::Percentage(dec!(5.5)));
    }

    #[test]
    fn test_deserialize_percentage_from_string() {
        let json = "\"7.5\"";
        let vat: VatRate = serde_json::from_str(json).unwrap();
        assert_eq!(vat, VatRate::Percentage(dec!(7.5)));
    }

    #[test]
    fn test_deserialize_percentage_from_float_with_stripp_pchar() {
        let json = r#""5.5%""#;
        let vat: VatRate = serde_json::from_str(json).unwrap();
        assert_eq!(vat, VatRate::Percentage(dec!(5.5)));
    }


    #[test]
    fn test_deserialize_exempt_from_string() {
        let json = "\"zw.\"";
        let vat: VatRate = serde_json::from_str(json).unwrap();
        assert_eq!(vat, VatRate::Exempt);
    }

    #[test]
    fn test_invalid_string_deserialization() {
        let json = "\"invalid\"";
        let result: Result<VatRate, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_number_deserialization() {
        let json = "999999999999999999999999999999";
        let result: Result<VatRate, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}