use core::fmt;
use std::{collections::HashMap, iter::Sum};
use rust_decimal::{prelude::{FromPrimitive, ToPrimitive}, Decimal};
use rust_decimal_macros::dec;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Visitor};
//use std::fmt;


// #[derive(Debug, Deserialize)]
// pub struct Invoice {
//     pub numer: String,
//     pub miejsce_wystawienia: String,
//     pub data_wystawienia: String,
//     pub data_realizacji: String,
//     pub forma_platnosci: String,
//     pub sprzedawca: Company,
//     pub nabywca: Company,
//     pub pozycje: Vec<InvoiceItem>,
//     pub summary: Option<Summary>,
// }


#[derive(Debug, Deserialize)]
pub struct Party {
    pub name: String,
    pub nip: String,
    pub enip: String,
    pub banck_account: String,
    pub address: Address,
}

#[derive(Debug, Deserialize)]
pub struct Address {
    pub ulica: String,
    pub kod_pocztowy: String,
    pub miasto: String,
}

#[derive(Debug,Deserialize)]
pub struct InvoiceItem {
    pub nazwa: String,
    pub ilosc: u32,
    pub jednostka: String,
    pub cena_jednostkowa: Decimal,
    pub stawka_vat: u8,
    pub rabat: Option<u8>,
    pub cena_netto: Option<Decimal>,
    pub vat:Option<Decimal>,
    pub cena_brutto: Option<Decimal>,
}


#[derive(Debug, Deserialize)]
pub struct SummaryRow {
    pub stawka_vat: u8,
    pub wartosc_netto: Decimal,
    pub wartosc_vat: Decimal,
    pub wartosc_brutto: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct Summary {
    pub rows: Vec<SummaryRow>,
    pub suma_netto: Decimal,
    pub suma_vat: Decimal,
    pub suma_brutto: Decimal,
}

// impl Invoice {
//     pub fn from_str(faktura_json: &str) -> Self {
//         let mut faktura: Invoice = serde_json::from_str(&faktura_json)
//             .expect("Błąd parsowania JSON do Invoice");
// 
//         if faktura.summary.is_none() {
//             faktura.pozycje
//                         .iter_mut()
//                         .for_each(|p|{
//                             p.cena_netto = Some(match p.cena_netto {
//                                 Some(cena_netto) => cena_netto,
//                                 None => p.cena_jednostkowa * Decimal::from_u32(p.ilosc).unwrap(),
//                             });
//                             p.vat = Some(match p.vat {
//                                 Some(vat) => vat,
//                                 None => p.cena_netto.unwrap() * Decimal::from_u8(p.stawka_vat).unwrap() / Decimal::from_u16(100).unwrap(),
//                             });
//                             p.cena_brutto = Some(match p.cena_brutto {
//                                 Some(cena_brutto) => cena_brutto,
//                                 None => p.cena_netto.unwrap() + p.vat.unwrap(),
//                             });
//                         });
//             faktura.summary = Some(Summary::from_items(&faktura.pozycje));
//         }
// 
//         faktura
//     }
// }

impl Summary {
    fn from_items(items: &Vec<InvoiceItem>) -> Self {

        let mut rows: HashMap<u8,(Decimal, Decimal, Decimal)> = HashMap::new();

        for item in items {
            let entry = rows.entry(item.stawka_vat).or_insert((dec!(0),dec!(0),dec!(0)));
            // netto
            entry.0 += item.cena_netto.unwrap();
            // VAT
            entry.1 += item.vat.unwrap();
            // brutto
            entry.2 += item.cena_brutto.unwrap();
            // todo check?

            if entry.0 + entry.1 != entry.2 {
                print!("diff");
            }
        }

        Self {
            rows: rows.iter().by_ref()
                .map(|(stawka_vat,(wartosc_netto, wartosc_vat, wartosc_brutto))|
                         SummaryRow {
                            stawka_vat:*stawka_vat,
                            wartosc_netto:*wartosc_netto,
                            wartosc_vat:*wartosc_vat,
                            wartosc_brutto:*wartosc_brutto,
                        })
                .collect(),
            suma_netto: rows.iter().map(|item| item.1.0).sum(),
            suma_vat: rows.iter().map(|item| item.1.1).sum(),
            suma_brutto: rows.iter().map(|item| item.1.2).sum(),
        }
    }
}
