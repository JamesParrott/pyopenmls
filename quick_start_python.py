import copy

from pyopenmls import (BasicCredential,
                       CredentialType,
                       Ciphersuite,
                       OpenMlsRustCrypto,
                       SignatureKeyPair,
                       SignatureScheme,
                       CredentialWithKey,
                       KeyPackage,
                       KeyPackageBundle,
                       MlsGroup,
                       MlsGroupCreateConfig,
                      )
                      
print('Reproduce quickstart in Python')
ciphersuite = Ciphersuite.MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519
provider = OpenMlsRustCrypto()



def generate_credential_with_key(
    identity: bytes,
    credential_type: CredentialType = CredentialType.Basic,
    signature_algorithm: SignatureScheme | None = None,
    provider: OpenMlsRustCrypto = None,
    ciphersuite: Ciphersuite | None = None,
) -> tuple[CredentialWithKey, SignatureKeyPair]:
    basic_credential = BasicCredential(identity)

    ciphersuite = ciphersuite or Ciphersuite.MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519
    signature_algorithm = signature_algorithm or ciphersuite.signature_algorithm()
    signature_key_pair = SignatureKeyPair(signature_algorithm)

    provider = provider or OpenMlsRustCrypto()

    # This would need Python to borrow a ref 
    # to provider's key_store, even within a wrapper
    # which needs a little more care with PyO3.
    # key_store = provider.storage()
    # signature_key_pair.store(key_store)

    # print(f'{provider.storage_values=}')
    signature_key_pair.store_in_provider(provider)
    # print(f'{provider.storage_values=}')

    public_key = signature_key_pair.public()
    # print(f'{public_key=}')
    credential_with_key = CredentialWithKey(basic_credential, public_key)

    return (credential_with_key, signature_key_pair)

def generate_key_package(
    ciphersuite: Ciphersuite,
    provider: OpenMlsRustCrypto,
    signer: SignatureKeyPair,
    credential_with_key: CredentialWithKey,
) -> KeyPackageBundle:
    """ A helper to create key package bundles. """


    # Create the key package builder
    builder = KeyPackage.builder()
    print(f'{builder=}')
    
    # Create the key package bundle
    bundle = builder.build(
                ciphersuite,
                provider,
                signer,
                credential_with_key,
                )
    return bundle

cred_w_key, key_pair = generate_credential_with_key(
    identity = b'Super_secret_ID',
    credential_type = CredentialType.Basic,
    provider = provider,
    ciphersuite = ciphersuite,
    )

print(f'{cred_w_key=}')
print(f'{key_pair=}')

key_package_bundle = generate_key_package(ciphersuite, provider, key_pair, cred_w_key)
print(f'{key_package_bundle=}')


#First they need credentials to identify them
sasha_credential_with_key, sasha_signer = generate_credential_with_key(
    b"Sasha",
    CredentialType.Basic,
    ciphersuite.signature_algorithm(),
    provider,
)

# Don't do this in real code.  See:
# print(f'{provider.storage_values=}')
# Maxim now has access to Sasha's private key (even more easily than they already did,
# as they're both storing creds in the same Python process anyway). 
maxim_credential_with_key, maxim_signer = generate_credential_with_key(
    b"Maxim",
    CredentialType.Basic,
    ciphersuite.signature_algorithm(),
    provider,
)

# print(f'{sasha_credential_with_key=}, {sasha_signer=}')
# print(f'{maxim_credential_with_key=}, {maxim_signer=}')
# Then they generate key packages to facilitate the asynchronous handshakes
# in MLS

# Generate KeyPackages
maxim_key_package = generate_key_package(ciphersuite, provider, maxim_signer, maxim_credential_with_key)

# print(f'{maxim_key_package=}')

# Now Sasha starts a new group ...
sasha_group = MlsGroup(
    provider,
    sasha_signer,
    MlsGroupCreateConfig(),
    sasha_credential_with_key,
)
# print(f'{sasha_group=}')

maxim_k_p_only = maxim_key_package.key_package()
print(f'{maxim_k_p_only=}')

# ... and invites Maxim.
# The key package has to be retrieved from Maxim in some way. Most likely
# via a server storing key packages for users.
# mls_message_out, welcome_out, group_info = (sasha_group
#     .add_members(provider, sasha_signer, [maxim_key_package.key_package()])
# )
