use pyo3::prelude::{*};
use openmls::{prelude::{*,  tls_codec::*}};
use signature_scheme::PySignatureScheme

#[allow(non_camel_case_types)]
#[pyclass(eq, eq_int, name="Ciphersuite")]
#[derive(PartialEq)]
pub enum PyCiphersuite {
    MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519 = Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519 as isize,
    MLS_128_DHKEMP256_AES128GCM_SHA256_P256 = Ciphersuite::MLS_128_DHKEMP256_AES128GCM_SHA256_P256 as isize,
    MLS_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519 = Ciphersuite::MLS_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519 as isize,
    MLS_256_DHKEMX448_AES256GCM_SHA512_Ed448 = Ciphersuite::MLS_256_DHKEMX448_AES256GCM_SHA512_Ed448 as isize,
    MLS_256_DHKEMP521_AES256GCM_SHA512_P521 = Ciphersuite::MLS_256_DHKEMP521_AES256GCM_SHA512_P521 as isize,
    MLS_256_DHKEMX448_CHACHA20POLY1305_SHA512_Ed448 = Ciphersuite::MLS_256_DHKEMX448_CHACHA20POLY1305_SHA512_Ed448 as isize,
    MLS_256_DHKEMP384_AES256GCM_SHA384_P384 = Ciphersuite::MLS_256_DHKEMP384_AES256GCM_SHA384_P384 as isize,
    MLS_256_XWING_CHACHA20POLY1305_SHA256_Ed25519 = Ciphersuite::MLS_256_XWING_CHACHA20POLY1305_SHA256_Ed25519 as isize,
}

#[pymethods]
impl PyCiphersuite {
    // #[new]
    // fn new(value: i32) -> Self {
    //     Number(value)
    // }
    pub const fn signature_algorithm(&self) -> PyResult<PySignatureScheme> {

    }
}