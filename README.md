## GraphFS

GraphFS is a GrapQL interface for interfacing with a file system.   It works with both remote or local file system

### Local setup
To setup with a local filesystem
* Run `cargo build` followed by :

```
./target/debug/graph_fs -p <port> -h <host> --auth_path <authorized path>
```

The authorized path is for security measures. Once it is specified eg `home/Pictures` no operation would be carried out outside this directory. Allowing you to expose just a segment of your computer directory and not your entire file system.

To define workers you can add the flag `-w <number of workers>` this is optional and would default to 2 workers

To read file and upload file, we use the REST API endpoint (I decided to use API here for optimization reasons and limitations from juniper GraphQL)

Read endpoint `/get_local_file?path=<path to read>`
Upload endpoint `/add_local_file?path=<directory to upload>`
Graphql playground: `/graphiql` (GraphQL doc exist here)


### Remote setup
To setup with a remote filesystem

* Run `cargo build` followed by :

To use user password auth option to authorize remote file system
```
./target/debug/graph_fs -p <port> -h <host> --remote true --auth_option user_password --remote_host <remote fs host> --remote_port <remote fs port> --username <name> --password <pass> --auth_path <authorized path>
```

To use username
```
./target/debug/graph_fs -p <port> -h <host> --remote true --auth_option user_agent --remote_host <remote fs host> --remote_port <remote fs port> --username <name> --auth_path <authorized path>
```

To use public key
```
./target/debug/graph_fs -p <port> -h <host> --remote true --auth_option user_pub_key --remote_host <remote fs host> --remote_port <remote fs port> --username <name> --public_key <public key path> --private_key <private key path> --passphrase <passphrase> --auth_path <authorized path>
```
To define workers you can add the flag `-w <number of workers>` this is optional and would default to 2 workers

#### You should be able to access the local fs api and graphql endpoints while in remote mode.

The remote endpoins are still in development