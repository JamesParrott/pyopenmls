use pyo3::prelude::{*};
use openmls::{prelude::{*,  tls_codec::*}};

#[allow(non_camel_case_types)]
#[pyclass(eq, eq_int, name="SignatureScheme")]
#[derive(PartialEq)]
pub enum PySignatureScheme {
    ECDSA_SECP256R1_SHA256 = SignatureScheme::ECDSA_SECP256R1_SHA256 as isize,
    ECDSA_SECP384R1_SHA384 = SignatureScheme::ECDSA_SECP384R1_SHA384 as isize,
    ECDSA_SECP521R1_SHA512 = SignatureScheme::ECDSA_SECP521R1_SHA512 as isize,
    ED25519 = SignatureScheme::ED25519 as isize,
    ED448 = SignatureScheme::ED448 as isize,
}