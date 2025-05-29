#![allow(unused)]
use pyo3::prelude::{*};

mod api;


#[pymodule]
fn pyopenmls(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(api::sum_as_string::sum_as_string, m)?)?;
    m.add_class::<api::basic_credential::PyBasicCredential>()?;
    m.add_class::<api::cipher_suite::PyCiphersuite>()?;
    m.add_class::<api::signature_scheme::PySignatureScheme>()?;
    Ok(())
}
