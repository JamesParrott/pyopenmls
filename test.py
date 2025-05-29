import pyopenmls

cred = pyopenmls.BasicCredential(b'32rgrrhej68563ywe')
print(f'{cred.identity=}')

cipher_suite = pyopenmls.Ciphersuite.MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519
print(f'{cipher_suite=}')