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
                       MlsGroupJoinConfig,
                       MlsMessageIn,
                       MlsMessageBodyIn,
                       StagedWelcome,
                      )
                      



def generate_credential_with_key(
    identity: bytes,
    credential_type: CredentialType = CredentialType.Basic,
    signature_algorithm: SignatureScheme | None = None,
    provider: OpenMlsRustCrypto = None,
    ciphersuite: Ciphersuite | None = None,
) -> tuple[CredentialWithKey, SignatureKeyPair]:

    """ A helper to create credentials and key pairs. """
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

    signature_key_pair.store_in_provider(provider)

    public_key = signature_key_pair.public()

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
    
    # Create the key package bundle
    bundle = builder.build(
                ciphersuite,
                provider,
                signer,
                credential_with_key,
                )
    return bundle


def match_message_body_in_variant(message_body_in):
    match message_body_in:
        case MlsMessageBodyIn.PublicMessage(message):
            return message
        case MlsMessageBodyIn.PrivateMessage(message):
            return message
        case MlsMessageBodyIn.Welcome(welcome):
            return welcome
        case MlsMessageBodyIn.GroupInfo(group_info):
            return group_info
        case MlsMessageBodyIn.KeyPackage(key_package):
            return key_package
        case _:
            raise ValueError(f"No supported message body in variant found for: {message_body_in=}")


def main():
    print('Reproduce quickstart in Python')
    ciphersuite = Ciphersuite.MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519
    provider = OpenMlsRustCrypto()


    #First they need credentials to identify them
    sasha_credential_with_key, sasha_signer = generate_credential_with_key(
        b"Sasha",
        CredentialType.Basic,
        ciphersuite.signature_algorithm(),
        provider,
    )

    # Don't do this in real code.  See:
    # print(f'{provider.storage_values=}')
    # Doing this grants Maxim access to Sasha's private key (even more easily than they already did,
    # as they're both storing creds in the same Python process anyway). 
    maxim_credential_with_key, maxim_signer = generate_credential_with_key(
        b"Maxim",
        CredentialType.Basic,
        ciphersuite.signature_algorithm(),
        provider,
    )

    # Then they generate key packages to facilitate the asynchronous handshakes
    # in MLS

    # Generate KeyPackages
    maxim_key_package = generate_key_package(ciphersuite, provider, maxim_signer, maxim_credential_with_key)



    # Now Sasha starts a new group ...
    sasha_group = MlsGroup(
        provider,
        sasha_signer,
        MlsGroupCreateConfig(),
        sasha_credential_with_key,
    )

    # ... and invites Maxim.
    # The key package has to be retrieved from Maxim in some way. Most likely
    # via a server storing key packages for users.
    mls_message_out, welcome_out, group_info = (sasha_group
        .add_member(provider, sasha_signer, maxim_key_package.key_package())
    )

    # Sasha merges the pending commit that adds Maxim.
    sasha_group.merge_pending_commit(provider)


    # Sascha serializes the [`MlsMessageOut`] containing the [`Welcome`].
    serialized_welcome: bytes = welcome_out.tls_serialize_detached()

    # Maxim can now de-serialize the message as an [`MlsMessageIn`] ...
    mls_message_in = MlsMessageIn.tls_deserialize(serialized_welcome)

    # ... and inspect the message.
    # welcome = mls_message_in.extract_welcome()
    # welcome = match mls_message_in.extract() {
    #    MlsMessageBodyIn::Welcome(welcome) => welcome,
    #    # We know it's a welcome message, so we ignore all other cases.
    #    _ => unreachable!("Unexpected message type."),
    # };
    welcome = match_message_body_in_variant(mls_message_in.extract())

    MlsGroupJoinConfig()
    # Now Maxim can build a staged join for the group in order to inspect the welcome
    maxim_staged_join = StagedWelcome.new_from_welcome(
        provider,
        MlsGroupJoinConfig(),
        welcome,
        # The public tree is need and transferred out of band.
        # It is also possible to use the [`RatchetTreeExtension`]
        sasha_group.export_ratchet_tree(),
    )

    # Finally, Maxim can create the group
    maxim_group = maxim_staged_join.into_group(provider)




if __name__ == '__main__':
    main()