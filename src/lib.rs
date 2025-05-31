use pyo3::prelude::{*};

mod api;


#[pymodule]
fn pyopenmls(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<api::basic_credential::PyBasicCredential>()?;
    m.add_class::<api::credential_type::PyCredentialType>()?;
    m.add_class::<api::credential_with_key::PyCredentialWithKey>()?;
    m.add_class::<api::cipher_suite::PyCiphersuite>()?;
    m.add_class::<api::signature_scheme::PySignatureScheme>()?;
    m.add_class::<api::openmls_rust_crypto_provider::PyOpenMlsRustCrypto>()?;
    m.add_class::<api::signature_key_pair::PySignatureKeyPair>()?;
    m.add_class::<api::storage_provider::PyStorageProvider>()?;
    m.add_class::<api::key_packages::PyKeyPackage>()?;
    m.add_class::<api::key_packages::PyKeyPackageBuilder>()?;
    m.add_class::<api::key_packages::PyKeyPackageBundle>()?;
    Ok(())
}
