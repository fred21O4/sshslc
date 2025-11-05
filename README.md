# SSHSLC

### Description
sshslc (ssh short lived certificates) is a server for providing clients with short lived ssh certificates generated from their public keys.   

### Goals:
- simple to set up 
- simple certificate request (process just a single http request)
- no complicated admin accounts or infrastructure.
- stateless, does not require any request tracking or authentication flows.

### Non Goals
- Automate the deployment/use of the certificates  
  For user keys this is trival, just request a cert just before connecting.  
  host keys are a bit more nuanced and require checking for close to expired keys and renewing them as needed
- Authenticate requests for certificates  
  ssh certificates are not secrets, just signed public keys with some extra metadata.  
  Any client still requires the corisponding private key to make use of them.
- Any kind of admin interface/web ui  
All admin/configuration is done by editing files on disk  
This offloads permission management to just file permissions

### Configuration
All config is within the prefix set with the env var SSHSLC_PREFIX
Server config is in $SSHSLC_PREFIX/config.json
Key settings are in $SSHSLC_PREFIX/[client]/[key].json

see [EXAMPLE.md](./EXAMPLE.md) for an example setup.

### Motivation
I created this to manage ssh certificates within my intranet, and security was not a big goal,   
However as only public certificates are ever provided to clients, and clients don't send any private info to the server, it should not send any secrets over the wire, consider using a reverse proxy with some form of access control if you feel you need it.

### Things to keep in mind
- Previous certificates are not invalidated when new ones are requested.

- Anyone with access to $SSHSLC_PREFIX has the ability to change certificate settings and thus $SSHSLC_PREFIX should only be accessible to privledged users/administrators.

- Short lived host certificates are more complicated to use compared to user certs, as they cant be requested at use time, as the hosts have no control over when they are needed.  
Most ssh certificate setups seem to solve this by checking if the cert is close to expiring on a timer and requesting a new one as needed.  
Consider using a systemd timer or crontab for this. 