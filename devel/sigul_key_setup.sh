#!/bin/bash
#
# Assumes you've set up sigul configuration by copying sigul-client.conf to
# ~/.sigul/client.conf.
#
# The configuration, in turn, assumes you have the sigul client keypair which you
# can get from the running container (assuming you ran `podman compose up -d`) with
#
# $ podman cp siguldry_sigul-bridge_1:/var/lib/sigul/ ~/.sigul
#
# You'll need to re-copy the keys if you build/pull a new container.
set -xeuo pipefail

mkdir -p keys/
pushd keys

echo "my-password" > ./nss_db_password

# Default key type is gpg. Server also says it supports RSA, but crashes the server if used..
printf 'my-password\0my-password\0' | sigul --batch -v -v \
	new-key --key-admin=sigul-client --key-type ECC ca-key > ca-key.pem

# The sigul server crashes if you try to have it create the RSA key.
# Importing an RSA key works, fortunately, and sigul requires an RSA key with pesign.
openssl genrsa -aes128 -passout file:./nss_db_password -out signing-keypair.pem
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
	list-keys

popd