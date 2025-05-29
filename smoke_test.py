import pyopenmls

cred = pyopenmls.BasicCredential(b'32rgrrhej68563ywe')
print(f'{cred.identity=}')


signature_scheme = pyopenmls.SignatureScheme.ECDSA_SECP384R1_SHA384
print(f'{signature_scheme=}')
print(f'{signature_scheme is pyopenmls.SignatureScheme.ECDSA_SECP384R1_SHA384=}')


cipher_suite = pyopenmls.Ciphersuite.MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519
print(f'{cipher_suite=}')
print(f'{cipher_suite is pyopenmls.Ciphersuite.MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519=}')
print(f'{cipher_suite.code()=}')
print(f'{cipher_suite.signature_algorithm()=}')