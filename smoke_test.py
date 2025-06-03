from pyopenmls import (BasicCredential,
                       CredentialWithKey,
                       CredentialType,
                       Ciphersuite,
                       KeyPackage,
                       KeyPackageBundle,
                       OpenMlsRustCrypto,
                       SignatureKeyPair,
                       SignatureScheme,
                       MlsGroup,
                       MlsGroupCreateConfig,
                       MlsGroupJoinConfig,
                       MlsMessageIn,
                       StagedWelcome,
                      )

import quick_start_python

cred = BasicCredential(b'Secret_ID_bytes')
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

provider = OpenMlsRustCrypto()
print(f'{provider=}')


signature_key_pair_ED25519 = SignatureKeyPair(SignatureScheme.ED25519)
print(f'{signature_key_pair_ED25519=}')

signature_key_pair_ciphersuite = SignatureKeyPair(cipher_suite.signature_algorithm())
print(f'{signature_key_pair_ciphersuite=}')


ciphersuite = Ciphersuite.MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519
signature_algorithm = ciphersuite.signature_algorithm()
signature_key_pair = SignatureKeyPair(signature_algorithm)

print(f'{signature_key_pair=}')

print(f'{provider.storage_values=}')
signature_key_pair.store_in_provider(provider)
print(f'{provider.storage_values=}')

public_key = signature_key_pair.public()
print(f'{public_key.hex()=}')
credential_with_key = CredentialWithKey(cred, public_key)

print(f'{credential_with_key=}')

builder = KeyPackage.builder()
print(f'{builder=}')

key_package_bundle = builder.build(
                ciphersuite,
                provider,
                signature_key_pair,
                credential_with_key,
                )
print(f'{key_package_bundle=}')

create_default_config = MlsGroupCreateConfig()
print(f'{create_default_config=}')

group = MlsGroup(
    provider,
    signature_key_pair,
    create_default_config,
    credential_with_key,
)
print(f'{group=}')

maxim_credential_with_key, maxim_signer = quick_start_python.generate_credential_with_key(
    b"Maxim",
    CredentialType.Basic,
    signature_algorithm,
    provider,
)

maxim_key_package_bundle = quick_start_python.generate_key_package(
    ciphersuite,
    provider,
    maxim_signer,
    maxim_credential_with_key,
)
print(f'{maxim_key_package_bundle=}')

maxim_key_package=maxim_key_package_bundle.key_package()
print(f'{maxim_key_package=}')

mls_message_out, welcome_out, group_info = (group
    .add_member(provider, signature_key_pair, maxim_key_package)
)

print(f'{mls_message_out=}')
print(f'{welcome_out=}')
print(f'{group_info=}')

retval = group.merge_pending_commit(provider)
print(f'{retval=}')

serialized_welcome: bytes = welcome_out.tls_serialize_detached()
print(f'{serialized_welcome.hex()=}')

mls_message_in = MlsMessageIn.tls_deserialize(serialized_welcome)
print(f'{mls_message_in=}')

message_body_in = mls_message_in.extract()
print(f'{message_body_in=}')


welcome = quick_start_python.match_message_body_in_variant(message_body_in)


# welcome = mls_message_in.extract_welcome()
print(f'{welcome=}')

# welcome = match mls_message_in.extract() {
#    MlsMessageBodyIn::Welcome(welcome) => welcome,
#    # We know it's a welcome message, so we ignore all other cases.
#    _ => unreachable!("Unexpected message type."),
# };
group_join_default_config = MlsGroupJoinConfig()
print(f'{group_join_default_config=}')

exported_ratchet_tree = group.export_ratchet_tree()
print(f'{exported_ratchet_tree=}')

maxim_staged_join = StagedWelcome.new_from_welcome(
    provider,
    group_join_default_config,
    welcome,
    # The public tree is need and transferred out of band.
    # It is also possible to use the [`RatchetTreeExtension`]
    exported_ratchet_tree,
)
# .expect("Error creating a staged join from Welcome");
print(f'{maxim_staged_join=}')

maxim_group = maxim_staged_join.into_group(provider)
print(f'{maxim_group=}')

# Test senign application messages!
hello_maxim_message = group.create_message(provider, signature_key_pair, b"Hello Maxim from anon group creator.")

print(f'{hello_maxim_message=}')
hello_maxim_message_serialized = hello_maxim_message.tls_serialize_detached()
print(f'{hello_maxim_message_serialized=}')

hello_maxim_message_in = MlsMessageIn.tls_deserialize(hello_maxim_message_serialized)
print(f'{hello_maxim_message_in=}')

hello_maxim_message_body_in = quick_start_python.match_message_body_in_variant(hello_maxim_message_in.extract())

print(f'{hello_maxim_message_body_in=}')

plain_text: bytes = maxim_group.read_private_message(hello_maxim_message_body_in, provider)
print(f'{plain_text=}')
print('\n\n')

maxim_reply = maxim_group.create_message(provider, maxim_signer, b"Hello group members.")
print(f'{maxim_reply=}')

maxim_reply_message_serialized = maxim_reply.tls_serialize_detached()
print(f'{maxim_reply_message_serialized.hex()=}')

maxim_reply_message_in = MlsMessageIn.tls_deserialize(maxim_reply_message_serialized)
print(f'{maxim_reply_message_in=}')

maxim_reply_message_body_in = quick_start_python.match_message_body_in_variant(maxim_reply_message_in.extract())

print(f'{maxim_reply_message_body_in=}')

plain_text: bytes = group.read_private_message(maxim_reply_message_body_in, provider)
print(f'{plain_text=}')
print('\n\n')