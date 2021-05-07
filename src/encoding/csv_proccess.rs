use error_chain::error_chain;

error_chain!{
    foreign_links {
        CsvErr(csv::Error);
    }
}

pub fn run() {
    simple_read().unwrap();
    simple_read_deserialize().unwrap();
}

use serde::Deserialize;
#[derive(Deserialize)]
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