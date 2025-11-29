use csv;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
#[doc = "Read CSV file as list of rows"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn read_csv_rows(filename: String) -> Result<Vec<Vec<String>>, std::io::Error> {
    let mut rows = vec![];
    let mut f = std::fs::File::open(&filename)?;
    let mut reader = csv::Reader::from_reader(f);
    for result in reader.deserialize::<HashMap<String, String>>() {
        let row = result?;
        rows.push(row);
    }
    Ok(rows)
}
#[doc = "Write data to CSV file"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn write_csv_data(filename: String, data: &Vec<Vec<String>>) -> Result<(), std::io::Error> {
    let mut f = std::fs::File::create(&filename)?;
    let mut writer = csv::Writer::from_writer(f);
    writer.writerows(data);
    Ok(())
}
#[doc = "Read CSV file as list of dictionaries"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn read_csv_dict(filename: String) -> Result<Vec<HashMap<String, String>>, std::io::Error> {
    let mut records = vec![];
    let mut f = std::fs::File::open(&filename)?;
    let mut reader = csv::ReaderBuilder::new().has_headers(true).from_reader(f);
    for result in reader.deserialize::<HashMap<String, String>>() {
        let row = result?;
        records.push(row.into_iter().collect::<HashMap<_, _>>());
    }
    Ok(records)
}
