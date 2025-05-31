use std::default::{Default};
use pyo3::prelude::{*};
use openmls::prelude::{*};
use super::openmls_rust_crypto_provider::PyOpenMlsRustCrypto;
use super::signature_key_pair::PySignatureKeyPair;
use super::credential_with_key::PyCredentialWithKey;
use super::key_packages::PyKeyPackageBundle;


#[derive(Debug, Default)]
#[pyclass(name="MlsGroupCreateConfig")]
pub struct PyMlsGroupCreateConfig {
    pub wrapped : MlsGroupCreateConfig,
}

#[pymethods]
impl PyMlsGroupCreateConfig {
    #[new]
    pub fn new() -> Self {
        Self {
            wrapped: Default::default(),
        }
    }
}


#[derive(Debug)]
#[pyclass(name="MlsGroup")]
pub struct PyMlsGroup {
    pub wrapped : MlsGroup,

}

#[pymethods]
impl PyMlsGroup {
    #[new]
    pub fn new(
        py_provider: &PyOpenMlsRustCrypto,
        signer: &PySignatureKeyPair,
        config: &PyMlsGroupCreateConfig,
        credential_with_key: PyCredentialWithKey,
    
    ) -> Self {
        Self {
            wrapped: MlsGroup::new(
                        &py_provider.wrapped,
                        &signer.wrapped,
                        &config.wrapped,
                        credential_with_key.wrapped,
                        ).expect("An unexpected error occurred creating MlsGroup."),
        }
    }

    // pub fn add_members(
    //     &mut self,
    //     provider: &PyOpenMlsRustCrypto,
    //     signer: &PySignatureKeyPair,
    //     key_packages: &[PyKeyPackageBundle],
    // ) -> (MlsMessageOut, MlsMessageOut, Option<GroupInfo>) {

    // }
}