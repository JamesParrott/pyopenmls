use pyo3::prelude::{*};
// use openmls::prelude::{CredentialType};
use strum_macros::{Display, FromRepr};


/// A wrapper for Credential Type
#[allow(non_camel_case_types)]
#[pyclass(eq, eq_int, name="CredentialType")]
#[derive(Display, FromRepr, PartialEq, Debug, Clone, Copy)]
#[repr(u16)]
pub enum PyCredentialType {


    // TODO:  Work out how to avoid duplicating the implementation in openmls
    // - can't use u16::from in a const
    // - can't use as u16 becuase this is a non-unit enum
    /// A [`BasicCredential`]
    Basic = 1,
    /// An X.509 [`Certificate`]
    X509 = 2,

    // TODO: Work around compiler error:
    // error: Unit variant `Basic` is not yet supported in a complex enum
    //     = help: change to an empty tuple variant instead: `Basic()`
    //     = note: the enum is complex because of non-unit variant `Other`
    // /// Another type of credential that is not in the MLS protocol spec.
    // Other(u16),
}

#[pymethods]
impl PyCredentialType {

    pub fn name(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    pub fn value(&self) -> PyResult<u16> {
        Ok(*self as u16)
    }

}