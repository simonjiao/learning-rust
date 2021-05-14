use anyhow::Result;

pub fn run() {
    simple_read().unwrap();
    simple_read_deserialize().unwrap();
    read_csv_delimeter().unwrap();
    filter_csv_lines().unwrap();
    serialize_simple_type().unwrap();
    serialize_with_serde().unwrap();
    transform_csv_column().unwrap();
}

use serde::{de, Deserialize, Deserializer};
#[derive(Debug, Deserialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String,
}

fn simple_read() -> Result<()> {
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;
        println!("In {}, {} built the {} model. It is a {}.",
            &record[0], &record[1], &record[2], &record[3]);
    }
    Ok(())
}

fn simple_read_deserialize() -> Result<()> {
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.deserialize() {
        let record:Record = record?;
        println!("In {}, {} built the {} model. It is a {}.",
                 record.year, record.make, record.model, record.description);
    }
    Ok(())
}

use csv::ReaderBuilder;

#[derive(Debug, Deserialize)]
struct NewRecord {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

fn read_csv_delimeter() -> Result<()> {
    let data = "name\tplace\tid
        Mark\tMelbourne\t46
        akshat\tdelphi\t37
        alisha\tcolomb\txyz
        Ashley\tZurich\t92";

    let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(data.as_bytes());
    for result in reader.deserialize::<NewRecord>() {
        println!("{:?}", result?);
    }

    Ok(())
}

use std::io;

fn filter_csv_lines() -> Result<()> {
    let query = "CA";
    let data = "\
City,State,Population,Latitude,Longitude
Kenai,AK,7610,60.5544444,-151.2583333
Oakman,AL,,33.7133333,-87.3886111
Sandfort,AL,,32.3380556,-85.2233333
West Hollywood,CA,37031,34.0900000,-118.3608333";

    let mut rdr = csv::ReaderBuilder::new().from_reader(data.as_bytes());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(rdr.headers()?)?;

    for result in rdr.records() {
        let record = result?;
        if record.iter().any(|field| field == query) {
            wtr.write_record(&record)?;
        }
    }
    wtr.flush()?;

    Ok(())
}

fn serialize_simple_type() -> Result<()> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(&["Name", "Place", "Id"])?;
    wtr.serialize(("Mark", "Sydeny", 87))?;
    wtr.serialize(("Ashley", "Dublin", 67))?;

    wtr.flush()?;
    Ok(())
}

use serde::Serialize;
#[derive(Serialize)]
struct RecordTwo<'a> {
    name: &'a str,
    place: &'a str,
    id: usize,
}

fn serialize_with_serde() -> Result<()> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    let rec1 = RecordTwo {name: "Makr", place:"USA", id:12};
    let rec2 = RecordTwo {name: "Simon", place:"BeiJing", id:143};
    let rec3 = RecordTwo {name: "cc",place:"UK", id:23};

    wtr.serialize(&rec1)?;
    wtr.serialize(&rec2)?;
    wtr.serialize(&rec3)?;

    wtr.flush()?;
    Ok(())
}

#[derive(Debug)]
struct HexColor {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Deserialize)]
struct Row {
    color_name: String,
    color: HexColor,
}
use std::str::FromStr;

impl FromStr for HexColor {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let trimmed = s.trim_matches('#');
        if trimmed.len() != 6 {
            Err("Invalid length of hex string".into())
        } else {
            Ok(HexColor {
               red:u8::from_str_radix(&trimmed[..2], 16)?,
               green:u8::from_str_radix(&trimmed[2..4], 16)?,
               blue:u8::from_str_radix(&trimmed[4..6], 16)?,
            })
        }
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer:D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

fn transform_csv_column() -> Result<()> {
    let data = "color_name,color
red,#ff0000
green,#00ff00
blue,#0000FF
periwinkle,#ccccff
magenta,#ff00ff".to_owned();

    let mut wtr = csv::Writer::from_writer(vec![]);
    let mut rdr = csv::Reader::from_reader(data.as_bytes());

    for result in rdr.deserialize::<Row>() {
        let row = result?;
        wtr.serialize((
            row.color_name,
            row.color.red,
            row.color.green,
            row.color.blue,
        ))?;
    }
    let written = String::from_utf8(wtr.into_inner()?)?;
    assert_eq!(Some("magenta,255,0,255"), written.lines().last());
    println!("{}", written);

    Ok(())
}