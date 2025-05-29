use pyo3::prelude::{*};

mod api;


#[pymodule]
fn pyopenmls(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<api::basic_credential::PyBasicCredential>()?;
    m.add_class::<api::cipher_suite::PyCiphersuite>()?;
    m.add_class::<api::signature_scheme::PySignatureScheme>()?;
    m.add_class::<api::openmls_rust_crypto::PyOpenMlsRustCrypto>()?;
    Ok(())
}
