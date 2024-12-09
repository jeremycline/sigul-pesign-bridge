#!/bin/bash

set -euo pipefail

# Default key type is gpg. Server also says it supports RSA, but crashes the server if used..
printf 'my-password\0my-password\0' | sigul --batch -v -v \
	new-key --key-admin=sigul-client --key-type ECC ca-key > ca-key.pem

# The sigul server crashes if you try to have it create the RSA key.
# Importing an RSA key works, fortunately, and sigul requires an RSA key with pesign.
openssl genrsa -aes128 -passout file:/srv/siguldry/nss_db_password -out signing-keypair.pem
printf 'my-password\0my-password\0my-password\0' | sigul --batch -v -v \
	import-key --key-admin=sigul-client --key-type=RSA signing-key signing-keypair.pem

printf 'my-password\0' | sigul --batch -v -v \
	sign-certificate ca-key ca-key \
		--subject-certificate-name root \
		--validity 1y \
		--certificate-type ca \
		--subject-common-name="Root CA" \
		--subject-state="MI" > ca-cert.pem

printf 'my-password\0' | sigul --batch -v -v \
	sign-certificate ca-key signing-key \
		--issuer-certificate-name root \
		--subject-certificate-name codesigning \
		--validity 1y \
		--certificate-type codesigning \
		--subject-common-name="Code Signing" \
		--subject-state="MI" > signing-cert.pem

printf 'my-password\0my-password\0' | sigul --batch -v -v \
    --config-file=devel/sigul-client.conf -v -v list-keys
