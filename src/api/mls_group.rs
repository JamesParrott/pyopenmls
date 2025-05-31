use pyo3::prelude::{*};
use openmls::prelude::tls_codec::Serialize;
use openmls::prelude::{*,group_info::{GroupInfo}};
use super::openmls_rust_crypto_provider::PyOpenMlsRustCrypto;
use super::signature_key_pair::PySignatureKeyPair;
use super::credential_with_key::PyCredentialWithKey;
use super::key_packages::PyKeyPackage;


#[derive(Debug)]
#[pyclass(name="MlsGroupCreateConfig")]
pub struct PyMlsGroupCreateConfig {
    pub wrapped : MlsGroupCreateConfig,
}

#[pymethods]
impl PyMlsGroupCreateConfig {
    #[new]
    pub fn new() -> Self {
        Self {
            wrapped: MlsGroupCreateConfig::default(),
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

    pub fn add_member(
        &mut self,
        provider: &PyOpenMlsRustCrypto,
        signer: &PySignatureKeyPair,
        key_package: &PyKeyPackage,
    ) -> (PyMlsMessageOut, PyMlsMessageOut, PyOptionalGroupInfo) {
        
        let (mls_message_out, welcome_out, optional_group_info) = self.wrapped
            .add_members(&provider.wrapped, &signer.wrapped, &[key_package.wrapped.clone()])
            .expect("Could not add members.");
        
        (PyMlsMessageOut{wrapped:mls_message_out},
         PyMlsMessageOut{wrapped:welcome_out},
         PyOptionalGroupInfo{wrapped:optional_group_info},
        )
        }

    pub fn merge_pending_commit(
        &mut self,
        provider: &PyOpenMlsRustCrypto,
    ) {
        self.wrapped.merge_pending_commit(&provider.wrapped).expect("error merging pending commit");
    }
}


#[derive(Debug)]
#[pyclass(name="MlsMessageOut")]
pub struct PyMlsMessageOut {
    pub wrapped : MlsMessageOut,
}

#[pymethods]
impl PyMlsMessageOut {
    pub fn tls_serialize_detached(&self) -> Vec<u8> {
        self.wrapped.tls_serialize_detached().expect("MlsMessageout should be serializable")
    }
}
#[derive(Debug)]
#[pyclass(name="OptionalGroupInfo")]
pub struct PyOptionalGroupInfo {
    pub wrapped : Option<GroupInfo>,
}

