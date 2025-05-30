from pyopenmls import (BasicCredential,
                       Ciphersuite,
                       OpenMlsRustCrypto,
                       SignatureKeyPair,
                       SignatureScheme,
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


print('\n\n' 'Reproduce quickstart in Python')
cipher_suite = Ciphersuite.MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519
provider = OpenMlsRustCrypto()



def generate_credential_with_key(
    identity: bytes,
    signature_algorithm: SignatureScheme | None = None,
    provider: OpenMlsRustCrypto = None,
    cipher_suite: Ciphersuite | None = None,
) -> tuple[None, SignatureKeyPair]:
# ) -> tuple[CredentialWithKey, SignatureKeyPair]:
    basic_credential = BasicCredential(identity)

    cipher_suite = cipher_suite or Ciphersuite.MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519
    signature_algorithm = signature_algorithm or cipher_suite.signature_algorithm()
    signature_key_pair = SignatureKeyPair(signature_algorithm)

    provider = provider or OpenMlsRustCrypto()
    # key_store = provider.storage()
    # signature_key_pair.store(key_store)

    signature_key_pair.store_in_provider(provider)

    # public_key = signature_key_pair.public_key()
    # credential_with_key = CredentialWithKey(basic_credential, public_key)

    credential_with_key = None

    return (credential_with_key, signature_key_pair)


cred_w_key, key_pair = generate_credential_with_key(
    identity = b'Super_secret_ID',
    provider = provider,
    cipher_suite = cipher_suite,
    )

print(f'{key_pair=}')