### Example setup

- Create a prefix to store the config   
```bash
mkdir prefix
```

- Set SSHSLC_PREFIX to the created directory  
```bash 
export SSHSLC_PREFIX=$(realpath ./prefix)
```

- Create a config file for the server at $SSHSLC_PREFIX/config.json with the following content
```json
{
    "bind_addr": "0.0.0.0:3030",
    "user_key": "./user.key",
    "host_key": "./host.key"
}
```
"bind_addr" is the address and port to bind to, 0.0.0.0:3030 binds to all addresses on port 3030  
"user_key" is the path to the user ca private key
"host_key" is the path to the host ca private key

- Create a config folder for a client  
```mkdir $SSHSLC_PREFIX/example-client```

- Add a public key to the client at $SSHSLC_PREFIX/example-client/example-key.json with the following content  
```json
{
    "pub_key": "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAICTTgJET9S5pbd45tU49fJC9Kw07qRK5Fso5hatl+8de",
    "cert_type": "User",
    "key_id": "cert for example-client",
    "comment": "example-username@hostname",
    "valid_for": 60,
    "principles": [
        "example-username"
    ]
}
```
pub_key is set to an example public key.  
You can leave out the host from the pub_key which in this example is example@hostname  
comment is the value that will come after the cert  
When requested this would produce a cert like the following
```
ssh-ed25519-cert-v01@openssh.com AAAAIHNzaC1lZDI1NTE5LWNlcnQtdjAxQG9wZW5zc2guY29tAAAAEAYwAn6SFlnsLQA8E2US3zsAAAAgJNOAkRP1Lmlt3jm1Tj18kL0rDTupErkWyjmFq2X7x14AAAAAAAAAAAAAAAEAAAAXY2VydCBmb3IgZXhhbXBsZS1jbGllbnQAAAAUAAAAEGV4YW1wbGUtdXNlcm5hbWUAAAAAaQrOAgAAAABpCs4+AAAAAAAAAAAAAAAAAAAAMwAAAAtzc2gtZWQyNTUxOQAAACAvodfTi7hdNuGoBRHys3fJiKEj4xkUXSurt7GmJtZElAAAAFMAAAALc3NoLWVkMjU1MTkAAABAVW1cAzxiy/D5LZ3shX5VKzSYDGFeuarq+aYx4+JQ0RVEd/yl9rsVbSA0yzUvqhHtOtcteLw0Ha50XG1ACtJ7AQ== example-username@hostname
```
And when inspected with ```ssh-keygen -f ./example-cert.pub -L```
```
Type: ssh-ed25519-cert-v01@openssh.com user certificate
Public key: ED25519-CERT SHA256:ZRRa9CVUzKnPZwZEvposvXnmq6Kchgf/Y6wPy9VOr50
Signing CA: ED25519 SHA256:+ySwGDMXf5YrohfW1k9azLsewj25y5j8wVUsME3vVNA (using ssh-ed25519)
Key ID: "cert for example-client"
Serial: 0
Valid: from 2025-11-05T17:09:38 to 2025-11-05T17:10:38
Principals: 
        example-username
Critical Options: (none)
Extensions: (none)
```