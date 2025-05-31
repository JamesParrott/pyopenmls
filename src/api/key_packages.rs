use pyo3::prelude::{*};
use openmls::prelude::{KeyPackage, KeyPackageBuilder, KeyPackageBundle};
use super::cipher_suite::PyCiphersuite;
use super::credential_with_key::PyCredentialWithKey;
use super::openmls_rust_crypto_provider::PyOpenMlsRustCrypto;
use super::signature_key_pair::PySignatureKeyPair;


/// Wrapper for OpenMLS KeyPackage
#[pyclass(name = "KeyPackage")]
pub struct PyKeyPackage {
    pub wrapped: KeyPackage,
}

#[pymethods]
impl PyKeyPackage {
    #[staticmethod]
    pub fn builder() -> PyResult<PyKeyPackageBuilder> {
        Ok(PyKeyPackageBuilder::new())
    }

    // pub fn key_package() {

    // }
}



/// Wrapper for OpenMLS KeyPackageBuilder
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

    fn build(
        &self,
        ciphersuite: PyCiphersuite,
        provider: &PyOpenMlsRustCrypto,
        signer: &PySignatureKeyPair,
        credential_with_key: PyCredentialWithKey,
        ) -> PyResult<PyKeyPackageBundle> {
       
        let key_package_bundle = self.wrapped.clone().build(
            ciphersuite.get_wrapped_equiv(),
            &provider.wrapped,
            &signer.wrapped,
            credential_with_key.wrapped,
        );
        Ok(PyKeyPackageBundle{
            wrapped: key_package_bundle.unwrap(),
        })
    }
}


/// Wrapper for OpenMLS KeyPackageBundle
#[pyclass(name = "KeyPackageBundle")]
pub struct PyKeyPackageBundle {
    pub wrapped: KeyPackageBundle,
}

#[pymethods]
impl PyKeyPackageBundle {

}