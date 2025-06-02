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
    m.add_class::<api::ratchet_tree_in::PyRatchetTreeIn>()?;
    m.add_class::<api::signature_key_pair::PySignatureKeyPair>()?;
    m.add_class::<api::storage_provider::PyStorageProvider>()?;
    m.add_class::<api::key_packages::PyKeyPackage>()?;
    m.add_class::<api::key_packages::PyKeyPackageBuilder>()?;
    m.add_class::<api::key_packages::PyKeyPackageBundle>()?;
    m.add_class::<api::mls_group::PyMlsGroup>()?;
    m.add_class::<api::mls_group::PyMlsGroupCreateConfig>()?;
    m.add_class::<api::mls_group::PyMlsGroupJoinConfig>()?;
    m.add_class::<api::mls_group::PyMlsMessageIn>()?;
    m.add_class::<api::mls_group::PyMlsMessageBodyIn>()?;
    m.add_class::<api::staged_welcome::PyStagedWelcome>()?;
    m.add_class::<api::messages::PyPublicMessageIn>()?;
    m.add_class::<api::messages::PyPrivateMessageIn>()?;
    m.add_class::<api::messages::PyWelcome>()?;
    m.add_class::<api::messages::PyVerifiableGroupInfo>()?;
    m.add_class::<api::messages::PyKeyPackageMsgIn>()?;
    Ok(())
}
