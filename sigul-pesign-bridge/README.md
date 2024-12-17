# sigul-pesign-bridge
Bridge pesign-client requests to a Sigul server

# Integration test setup

This is a royal pain. Steps for now:

```bash
mkdir ~/.sigul
cp devel/sigul-client.conf ~/.sigul/client.conf

podman compose up -d
podman cp siguldry_sigul-bridge_1:/var/lib/sigul/ ~/.sigul
# now vi /etc/hosts to have sigul-bridge resolve to localhost
# then:
./devel/sigul_key_setup.sh
```