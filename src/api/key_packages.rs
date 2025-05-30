use pyo3::prelude::{*};
use openmls::prelude::{KeyPackage, KeyPackageBuilder, KeyPackageBundle};

/// Wrapper for OpenMLS KeyPackage
#[pyclass(name = "KeyPackage")]
pub struct PyKeyPackage {
    pub wrapped: KeyPackage,
}

#[pymethods]
impl PyKeyPackage {
    pub fn builder(&self) -> PyKeyPackageBuilder {
        PyKeyPackageBuilder::new()
    }
}



/// Wrapper for OpenMLS KeyPackageBuilder
#[derive(Debug, PartialEq, Clone)]
#[pyclass(name = "KeyPackageBuilder")]
pub struct PyKeyPackageBuilder {
    pub wrapped: KeyPackageBuilder,
}

#[pymethods]
impl PyKeyPackageBuilder {
    #[new]
    pub fn new() -> Self {
        Self {
            wrapped: KeyPackageBuilder::new(),
        }
    }

    fn build(&self) -> PyKeyPackageBundle {
        Ok(self.wrapped.identity())
    }
}


/// Wrapper for OpenMLS KeyPackageBundle
#[derive(Debug, PartialEq, Clone)]
#[pyclass(name = "KeyPackageBundle")]
pub struct PyKeyPackageBundle {
    pub wrapped: KeyPackageBundle,
}

#[pymethods]
impl PyKeyPackageBundle {

}