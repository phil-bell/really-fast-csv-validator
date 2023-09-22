use pyo3::prelude::*;
use csv::{Reader, Error};
use pyo3::exceptions::PyValueError;

// use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// struct Record {
//     name: String,
//     place: String,
//     #[serde(deserialize_with = "csv::invalid_option")]
//     id: Option<u64>,
// }

struct CsvError(Error);

impl From<CsvError> for PyErr {
    fn from(_error: CsvError) -> Self {
        PyValueError::new_err("Invalid CSV file")
    }
}

impl From<Error> for CsvError {
    fn from(other: Error) -> Self {
        Self(other)
    }
}


#[pyfunction]
fn validate_csv(csv_path: &str) -> Result<(), CsvError> {
    let mut reader = Reader::from_path(csv_path)?;
    for record in reader.records() {
        let record = record?;
        println!(
            "{} -{} - {}.",
            &record[0],
            &record[1],
            &record[2],
        );
    }
    Ok(())
}


#[pymodule]
fn really_fast_csv_validator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate_csv, m)?)?;
    Ok(())
}