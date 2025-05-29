#[allow(unused_imports)]

use pyo3::prelude::{*};
// use openmls::{prelude::{*,  tls_codec::*}};
use openmls_rust_crypto::{OpenMlsRustCrypto};
use openmls_basic_credential::SignatureKeyPair;

mod api;

// Define ciphersuite ...
// const ciphersuite = Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;
// // ... and the crypto provider to use.
// const provider = &OpenMlsRustCrypto::default();





/// A Python module implemented in Rust.
#[pymodule]
fn pyopenmls(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(api::sum_as_string::sum_as_string, m)?)?;
    m.add_class::<api::basic_credential::PyBasicCredential>()?;
    m.add_class::<api::cipher_suite::PyCiphersuite>()?;
    Ok(())
}
