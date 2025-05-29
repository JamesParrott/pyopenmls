import pyopenmls

cred = pyopenmls.BasicCredential(b'32rgrrhej68563ywe')
print(f'{cred.identity=}')

def print_deets_of_enum_member(enum_member) -> None:
    print(f'{enum_member=}')
    print(f'{enum_member.name()=}')
    print(f'{hex(enum_member.value())=}')

signature_scheme = pyopenmls.SignatureScheme.ECDSA_SECP384R1_SHA384
print(f'{signature_scheme is pyopenmls.SignatureScheme.ECDSA_SECP384R1_SHA384=}')
print_deets_of_enum_member(signature_scheme)

cipher_suite = pyopenmls.Ciphersuite.MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519
print_deets_of_enum_member(cipher_suite)
print(f'{cipher_suite is pyopenmls.Ciphersuite.MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519=}')
print(f'{cipher_suite.signature_algorithm()=}')

provider = pyopenmls.OpenMlsRustCrypto()
print(f'{provider=}')