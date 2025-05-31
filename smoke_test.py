from pyopenmls import (BasicCredential,
                       CredentialType,
                       Ciphersuite,
                       OpenMlsRustCrypto,
                       SignatureKeyPair,
                       SignatureScheme,
                       CredentialWithKey,
                       KeyPackage,
                       KeyPackageBundle,
                      )

cred = BasicCredential(b'32rgrrhej68563ywe')
print(f'{cred.identity=}')

def print_deets_of_enum_member(enum_member) -> None:
    print(f'{enum_member=}')
    print(f'{enum_member.name()=}')
    print(f'{hex(enum_member.value())=}')

signature_scheme = SignatureScheme.ECDSA_SECP384R1_SHA384
print(f'{signature_scheme is SignatureScheme.ECDSA_SECP384R1_SHA384=}')
print_deets_of_enum_member(signature_scheme)

cipher_suite = Ciphersuite.MLS_256_XWING_CHACHA20POLY1305_SHA256_Ed25519
print_deets_of_enum_member(cipher_suite)
print(f'{cipher_suite is Ciphersuite.MLS_256_XWING_CHACHA20POLY1305_SHA256_Ed25519=}')
print(f'{cipher_suite.signature_algorithm()=}')

default_provider = OpenMlsRustCrypto()
print(f'{default_provider=}')

# signature_key_pair_default = SignatureKeyPair()
# print(f'{signature_key_pair_default=}')
# signature_key_pair = SignatureKeyPair(signature_scheme, provider)
signature_key_pair_ED25519 = SignatureKeyPair(SignatureScheme.ED25519)
print(f'{signature_key_pair_ED25519=}')

signature_key_pair_ciphersuite = SignatureKeyPair(cipher_suite.signature_algorithm())
print(f'{signature_key_pair_ciphersuite=}')


print('\n\n')