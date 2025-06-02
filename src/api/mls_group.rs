use pyo3::prelude::{*};
use pyo3::exceptions::PyValueError;
use openmls::prelude::tls_codec::{Serialize,Deserialize};
use openmls::prelude::{*,group_info::{GroupInfo},};

use super::openmls_rust_crypto_provider::PyOpenMlsRustCrypto;
use super::signature_key_pair::PySignatureKeyPair;
use super::credential_with_key::PyCredentialWithKey;
use super::key_packages::PyKeyPackage;
use super::ratchet_tree_in::PyRatchetTreeIn;
use super::messages::{PyWelcome,PyPublicMessageIn,PyPrivateMessageIn,PyVerifiableGroupInfo,PyKeyPackageMsgIn};


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
#[pyclass(name="MlsGroupJoinConfig")]
pub struct PyMlsGroupJoinConfig {
    pub wrapped : MlsGroupJoinConfig,
}

#[pymethods]
impl PyMlsGroupJoinConfig {
    #[new]
    pub fn new() -> Self {
        Self {
            wrapped: MlsGroupJoinConfig::default(),
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

    pub fn export_ratchet_tree(&self) -> PyResult<PyRatchetTreeIn>{
        Ok(PyRatchetTreeIn{wrapped:self.wrapped.export_ratchet_tree().into()})
    }

    pub fn create_message(
        &mut self,
        provider: &PyOpenMlsRustCrypto,
        signer: &PySignatureKeyPair,
        message: &[u8],
    ) -> PyResult<PyMlsMessageOut> {
        if let Ok(message_out) = self.wrapped.create_message(&provider.wrapped, &signer.wrapped, message) {
            Ok(PyMlsMessageOut{wrapped: message_out})
        } else {
            Err(PyValueError::new_err(format!("Could not create message: {:#?} from signer: {:#?}",message,signer)))
        }
    }

    pub fn read_private_message(
        &mut self,
        py_message: &PyPrivateMessageIn,
        py_provider: &PyOpenMlsRustCrypto,
    ) -> PyResult<Vec<u8>> {
        let processed_message: ProcessedMessage = self.wrapped.process_message(&py_provider.wrapped, py_message.wrapped.clone())
                                            .expect("Could not process message.");
        if let ProcessedMessageContent::ApplicationMessage(application_message) = processed_message.into_content() {
            Ok(application_message.into_bytes())
        } else {
            Err(PyValueError::new_err("Could not read private message"))

        }
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
        self.wrapped.tls_serialize_detached().expect("MlsMessageOut should be serializable")
    }
    pub fn __repr__(&self) -> String {
        if let MlsMessageBodyOut::PrivateMessage(ciphertext) = self.wrapped.body() {
            format!("MlsMessageOut< Ciphertext: {:#?} >",ciphertext)
        } else {
            "Could not unwrap message".to_string()
        }
    }
}

#[derive(Debug)]
#[pyclass(name="MlsMessageIn")]
pub struct PyMlsMessageIn {
    pub wrapped : MlsMessageIn,
}

#[pymethods]
impl PyMlsMessageIn {
    #[staticmethod]
    pub fn tls_deserialize(serialized_bytes: Vec<u8>) -> PyResult<PyMlsMessageIn> {   
        let result = MlsMessageIn::tls_deserialize(& mut serialized_bytes.as_slice());
        if let Ok(mls_message_in) = result {
            Ok(PyMlsMessageIn{wrapped: mls_message_in})
        } else {
            Err(PyValueError::new_err("Could not deserialize data to MlsMessageIn. "))
        }
    }

    pub fn extract_welcome(&self) -> PyResult<PyWelcome>{
        match self.wrapped.clone().extract() {
            MlsMessageBodyIn::Welcome(welcome) => Ok(PyWelcome{wrapped:welcome}),
            _ => Err(PyValueError::new_err("MlsMessageIn did not match a welcome message.")),
        }
    }
    pub fn extract(&self) -> PyResult<PyMlsMessageBodyIn>{
        PyMlsMessageBodyIn::from_MlsMessageBodyIn(self.wrapped.clone())
        // match self.wrapped.clone().extract() {
        //     MlsMessageBodyIn::Welcome(welcome) => Ok(PyWelcome{wrapped:welcome}),
        //     _ => Err(PyValueError::new_err("MlsMessageIn did not match a welcome message.")),
        // }
    }
}

#[derive(Debug)]
#[pyclass(name="MlsMessageBodyIn")]
pub enum PyMlsMessageBodyIn {
    PublicMessage(PyPublicMessageIn),
    PrivateMessage(PyPrivateMessageIn),
    Welcome(PyWelcome),
    GroupInfo(PyVerifiableGroupInfo),
    KeyPackage(PyKeyPackageMsgIn),
}

impl PyMlsMessageBodyIn {                
    #[allow(non_snake_case)]
    pub fn from_MlsMessageBodyIn(message_in: MlsMessageIn) -> PyResult<Self> {
        match message_in.extract() {
            MlsMessageBodyIn::Welcome(welcome) => Ok(PyMlsMessageBodyIn::Welcome(PyWelcome{wrapped:welcome})),
            MlsMessageBodyIn::PrivateMessage(message) => Ok(PyMlsMessageBodyIn::PrivateMessage(PyPrivateMessageIn{wrapped:message})),
            MlsMessageBodyIn::PublicMessage(message) => Ok(PyMlsMessageBodyIn::PublicMessage(PyPublicMessageIn{wrapped:message})),
            MlsMessageBodyIn::GroupInfo(group_info) => Ok(PyMlsMessageBodyIn::GroupInfo(PyVerifiableGroupInfo{wrapped:group_info})),
            MlsMessageBodyIn::KeyPackage(key_package) => Ok(PyMlsMessageBodyIn::KeyPackage(PyKeyPackageMsgIn{wrapped:key_package})),
        }

    }

}


#[derive(Debug)]
#[pyclass(name="OptionalGroupInfo")]
pub struct PyOptionalGroupInfo {
    pub wrapped : Option<GroupInfo>,
}


