# GraphFS

GraphFS is a GraphQL interface for interfacing with a file system. It works with both remote (via SSH) or local file system.

## Local setup

To setup with a local filesystem,

- Run `cargo build`, followed by:

```sh
./target/debug/graph_fs -p <port> -h <host> --auth_path <authorized path>
```

or just run:

```sh
cargo run -- -p <port> -h <host> --auth_path <authorized path>
```

The authorized path is for security measures. Once it is specified (e.g. `home/Pictures`), no operation would be carried out outside this directory. This allows you to expose only a segment of your computer directory and not your entire file system.

To specify the number of workers to use, you can add the flag: `-w <number of workers>`. This is optional and would default to 2 workers.

To read files and upload files, we use the REST API endpoint (I decided to use API here for optimization reasons and limitations from Juniper GraphQL).

- Read endpoint: `/get_local_file?path=<path to read>`
- Upload endpoint: `/add_local_file?path=<directory to upload>`
- GraphQL endpoint (where to send the GraphQL queries to, from a client): `/graphql`
- GraphiQL playground (GraphQL doc exist here): `/graphiql`

### Remote setup

> **_NOTE:_** Please note that the remote filesystem option only works with a Unix-like filesystems

#### To setup with a remote filesystem

- Run `cargo build`, followed by:

##### To use the `user password` auth option to authorize access to remote file system

```sh
./target/debug/graph_fs -p <port> -h <host> --remote true --auth_option user_password --remote_host <remote fs host> --remote_port <remote fs port> --username <name> --password <pass> --auth_path <authorized path>
```

or just run:

```sh
cargo run -- -p <port> -h <host> --remote true --auth_option user_password --remote_host <remote fs host> --remote_port <remote fs port> --username <name> --password <pass> --auth_path <authorized path>
```

##### To use `username` auth option

```sh
./target/debug/graph_fs -p <port> -h <host> --remote true --auth_option user_agent --remote_host <remote fs host> --remote_port <remote fs port> --username <name> --auth_path <authorized path>
```

or just run:

```sh
cargo run -- -p <port> -h <host> --remote true --auth_option user_agent --remote_host <remote fs host> --remote_port <remote fs port> --username <name> --auth_path <authorized path>
```

##### To use `public key` auth option

```sh
./target/debug/graph_fs -p <port> -h <host> --remote true --auth_option user_pub_key --remote_host <remote fs host> --remote_port <remote fs port> --username <name> --public_key <public key path> --private_key <private key path> --passphrase <passphrase> --auth_path <authorized path>
```

or just run:

```sh
cargo run -- -p <port> -h <host> --remote true --auth_option user_pub_key --remote_host <remote fs host> --remote_port <remote fs port> --username <name> --public_key <public key path> --private_key <private key path> --passphrase <passphrase> --auth_path <authorized path>
```

To specify the number of workers to use, you can add the flag: `-w <number of workers>`. This is optional and would default to 2 workers.

#### You should be able to access the local filesystem API and GraphQL endpoints while in remote mode

- Read endpoint: `/get_remote_file?path=<path to read>`
- Upload endpoint: `/add_remote_file?path=<directory to upload>`
- GraphQL endpoint (where to send the GraphQL queries to, from a client): `/graphql`
- GraphiQL playground (GraphQL doc exist here): `/graphiql`
