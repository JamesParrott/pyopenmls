use pyo3::prelude::{*};
use pyo3::exceptions::PyRuntimeError;
use openmls::prelude::{*};
use super::openmls_rust_crypto_provider::PyOpenMlsRustCrypto;
use super::messages::PyWelcome;
use super::mls_group::{PyMlsGroup,PyMlsGroupJoinConfig};
use super::ratchet_tree_in::PyRatchetTreeIn;

/// Wrapper for OpenMLS StagedWelcome
#[pyclass(name = "StagedWelcome")]
pub struct PyStagedWelcome {
    pub wrapped: Option<StagedWelcome>,
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

    ) -> Self {
        Self { wrapped: Some(StagedWelcome::new_from_welcome(
                    &py_provider.wrapped,
                    &py_config.wrapped,
                    py_welcome.wrapped,
                    // The public tree is need and transferred out of band.
                    // It is also possible to use the [`RatchetTreeExtension`]
                    Some(py_ratchet_tree_in.wrapped),
                ).expect("Error creating a staged join from Welcome")),
        }    
    }

    // Try `&self`, `&mut self, `slf: PyRef<'_, Self>` or `slf: PyRefMut<'_, Self>`.
    pub fn into_group(&mut self, py_provider: &PyOpenMlsRustCrypto) -> PyResult<PyMlsGroup> {
        let staged_welcome = self.wrapped.take().take().ok_or_else(|| {
            PyRuntimeError::new_err("This Outer has already been consumed")
        }).expect("Not called with Option<StagedWelcome> == None");
        let mls_group = staged_welcome.into_group(&py_provider.wrapped).expect("Error creating the group from the staged join");
        Ok(PyMlsGroup{
            wrapped: mls_group,
        })
    }
}
