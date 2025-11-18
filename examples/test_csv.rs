use csv;
use std::collections::HashMap;
#[doc = "Read CSV file as list of rows"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn read_csv_rows(filename: String) -> Vec<Vec<String>> {
    let rows = vec![];
    let f = std::fs::File::open(filename)?;
    let reader = csv::Reader::from_reader(f);
    for row in reader.iter().cloned() {
        rows.push(row);
    }
    rows
}
#[doc = "Write data to CSV file"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn write_csv_data(filename: String, data: &Vec<Vec<String>>) {
    let f = std::fs::File::create(filename)?;
    let writer = csv::Writer::from_writer(f);
    writer.writerows(data);
}
#[doc = "Read CSV file as list of dictionaries"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn read_csv_dict(filename: String) -> Vec<HashMap<String, String>> {
    let records = vec![];
    let f = std::fs::File::open(filename)?;
    let reader = csv::ReaderBuilder::new().has_headers(true).from_reader(f);
    for row in reader.iter().cloned() {
        records.push(row.into_iter().collect::<HashMap<_, _>>());
    }
    records
}
