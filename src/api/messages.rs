use pyo3::prelude::{*};
use openmls::prelude::{*,group_info::VerifiableGroupInfo};
// use super::signature_scheme::PySignatureScheme;


/// Wrapper for OpenMLS Welcome
#[derive(Debug,Clone)]
#[pyclass(name = "Welcome")]
pub struct PyWelcome {
    pub wrapped: Welcome,
}

/// Wrapper for OpenMLS PublicMessageIn
#[derive(Debug,Clone)]
#[pyclass(name = "PublicMessageIn")]
pub struct PyPublicMessageIn {
    pub wrapped: PublicMessageIn,
}


/// Wrapper for OpenMLS PrivateMessageIn
#[derive(Debug,Clone)]
#[pyclass(name = "PrivateMessageIn")]
pub struct PyPrivateMessageIn {
    pub wrapped: PrivateMessageIn,
}


/// Wrapper for OpenMLS VerifiableGroupInfo
#[derive(Debug,Clone)]
#[pyclass(name = "VerifiableGroupInfo")]
pub struct PyVerifiableGroupInfo {
    pub wrapped: VerifiableGroupInfo,
}

/// Wrapper for OpenMLS KeyPackageIn
#[derive(Debug,Clone)]
#[pyclass(name = "KeyPackageMsgIn")]
pub struct PyKeyPackageMsgIn {
    pub wrapped: KeyPackageIn,
}
