use pyo3::prelude::{*};
use openmls::prelude::{*};
use super::openmls_rust_crypto_provider::PyOpenMlsRustCrypto;
use super::welcome::PyWelcome;
use super::mls_group::PyMlsGroupJoinConfig;
use super::ratchet_tree_in::PyRatchetTreeIn;

/// Wrapper for OpenMLS StagedWelcome
#[pyclass(name = "StagedWelcome")]
pub struct PyStagedWelcome {
    pub wrapped: StagedWelcome,
}

#[pymethods]
impl PyStagedWelcome {
    // #[new]
    // pub fn new(identity: Vec<u8>) -> Self {
    //     Self {
    //         wrapped: StagedWelcome::new(identity),
    //     }
    // }

    // #[getter]
    // fn identity(&self) -> PyResult<&[u8]> {
    //     Ok(self.wrapped.identity())
    // }

    #[staticmethod]
    pub fn new_from_welcome(
        py_provider: &PyOpenMlsRustCrypto,
        py_config: &PyMlsGroupJoinConfig,
        py_welcome: PyWelcome,
        py_ratchet_tree_in: PyRatchetTreeIn,

    ) {
        StagedWelcome::new_from_welcome(
            &py_provider.wrapped,
            &py_config.wrapped,
            py_welcome.wrapped,
            // The public tree is need and transferred out of band.
            // It is also possible to use the [`RatchetTreeExtension`]
            Some(py_ratchet_tree_in.wrapped),
        ).expect("Error creating a staged join from Welcome");    
    }
}
