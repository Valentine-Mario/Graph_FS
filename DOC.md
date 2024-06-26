## Graph FS Documentation

GraphFS is a command-line interface (CLI) utility engineered to enable the rapid instantiation of a GraphQL-compliant server, specifically tailored for the secure interrogation of file system data. This utility boasts expansive configurability, accommodating various scenarios including but not limited to the deployment of Secure Socket Layer (SSL) protocols, integration with remote file storage systems, as well as local file system interfaces.

Moreover, GraphFS implements robust authentication mechanisms to ensure secure access, with provisions for delineating permissions such that user interactions can be constrained to read-only activities within the file system domain, thereby upholding data integrity and preventing unauthorized data manipulation.


### Graph FS Guide

#### Add Users
> This feature is to be used only when `auth` is enabled
> To use this feature, be sure to have SQLite enabled on your device or PostgreSQL DB setup

GraphFS integrates an advanced user management system which can be operated via the command-line interface, providing administrators with the capability to define and control user privileges with precision. This system includes commands for the creation and modification of user accounts, delineating access permissions, and setting the scope of privileges.

For example, to create a new user within GraphFS, an administrator would execute a command in the following structure:

```console
graph_fs --manage_users add_user --acc_name val --acc_email val@gmail.com --acc_password 123
```

> This command by default creates a  `graph_fs.db` SQLite file which tracks the user's details. 
> Alternatively, you can choose to use PostgreSQL with the following command.

```console
graph_fs --manage_users add_user --acc_name val --acc_email val@gmail.com --acc_password 123 --storage psql --db_path postgres://path_to_db
```

> Please not that for all commands that require PostgreSQL, you’ll have to add the flag
```console
--storage psql --db_path postgres://path_to_db
```

This command specifies 'val' as the username, aligns it with the email address 'val@gmail.com', sets the password to '123', and assigns the user a role that permits both reading from and writing to the file system


You can add as many users as you wish but with unique email values. You can also update the `username`, `password` or `access`
> Set emails are non editable

#### Edit And Delete Users

- To edit the user `val` to `valentine` execute the command  
```console
graph_fs --manage_users update_username --acc_name valentine --acc_email val@gmail.com
```

- To update the user's password  
```console
graph_fs --manage_users update_user_password --acc_email val@gmail.com --acc_password new_password123
```

- To update a user's priviledge  
```console
graph_fs --manage_users update_user_permission --acc_email val@gmail.com --acc_permission read
```

- To delete a user  
```console
graph_fs --manage_users delete_user --acc_email val@gmail.com
```


#### Server Configuration

GraphFS offers the flexibility to launch the GraphQL server with a suite of customizable options that cater to various security and performance needs. The server can be configured to utilize SSL for secure communication, enable authentication mechanisms to ensure controlled access, connect to a remote file system for data management, and allow specification of the number of worker processes to optimize concurrent handling of incoming requests.

Additionally, when configuring the server to utilize PostgreSQL for user management storage, it is crucial to include the appropriate flags to connect to your PostgreSQL database. The flags --storage and --db_path are employed for this purpose:

```console
--storage psql --db_path postgres://path_to_db
```

The --storage psql flag explicitly signals that PostgreSQL is the storage mechanism for user data, while --db_path is accompanied by your database connection string (e.g., postgres://username:password@hostname:port/database_name). Be sure to replace postgres://path_to_db with your actual PostgreSQL database connection string.
If you wish to use the default `Sqlite` you can omit the flag

- To start the server with no auth enabled using the local file system, run the command  
```console
graph_fs -p 8000 -h 127.0.0.1 --auth_path /home/acc/Pictures -w 3
```

**Breaking down the arguments:**

- **-p** 8000 configures the server to listen on port 8000.
- **-h** 127.0.0.1 sets the server host to the local interface, 127.0.0.1, which is typically associated with 'localhost'.
- **-w** 3 instructs the server to spawn 3 worker processes for handling incoming queries in order to improve performance and concurrency.
- **--auth_path** is a mandatory argument that securely confines the scope of directory queries to the specified path, preventing users from accessing files outside this designated directory hierarchy.
The --auth_path parameter implements an additional layer of access control by explicitly defining the root folder that can be queried by GraphFS. In the provided command, `/home/acc/Pictures` is the stipulated directory, ensuring that all user file system interactions are limited to this path.

Furthermore, `--auth_path` parameter can also accept relative paths. For example:

```console
graph_fs -p 8000 -h 127.0.0.1 --auth_path ~/Pictures -w 3
```

In this case, `~/Pictures` would resolve to the equivalent absolute path `/home/acc/Pictures`, assuming 'acc' is the current user's home directory.

Additionally, you can base the relative path on the current working directory from which you execute the graphfs_bin command. If your current directory is `/home/acc`, and you have a folder named 'Documents' within it, you could use:

```console
graph_fs -p 8000 -h 127.0.0.1 --auth_path Documents -w 3
```

Here, `Documents` would resolve to `/home/acc/Documents`, which would become the root for file system queries by GraphFS.

This flexibility allows for a simple and dynamic way to set up the permissible query directory based on either an absolute path, a home directory-relative path, or a relative path to the current directory, catering to the desired level of access control in a variety of deployment environments.

- To start the server with auth enabled, using the local file system, run the command  
```console
graph_fs -p 8000 -h 127.0.0.1 --auth_path /home/acc/Pictures --use_auth true --secret my_jwt_secret --jwt_duration 5
```


Here's a detailed explanation of the additional parameters:

- **--use_auth** true activates the authentication system, necessitating that users present a valid token to gain access to the server's endpoints.
- **--secret** my_jwt_secret establishes the secret key used for signing JSON Web Tokens (JWTs), which are a method for securely transmitting information between parties as a JSON object. This secret key should be kept confidential and is vital for the integrity and security of the tokens.
- **--jwt_duration** 5 specifies the lifespan of the issued JWT, in this context, it is set for 5 days. If the --jwt_duration is omitted, the default token validity period will be assumed to be 30 days.

Once authentication is enabled with the `--use_auth` flag, users will need to authenticate themselves to the server. They will do so by logging in, a process during which they will provide their credentials, and upon successful authentication, they will receive a JWT. This token must be included in the headers of subsequent requests to the server to access protected endpoints within the server's GraphQL API.


- To start the server with a remote file system, you can use auth enabled or disabled just as above but you need to add the following additional parameters


```console
-p 8000 -h 127.0.0.1 --remote true --auth_option user_password --remote_host 127.0.0.1 --remote_port 22 --username <name> --password <pass>
```

Let's dissect the provided command parameters:

- **-p** 8000 sets the server to listen on port 8000.
- **-h** 127.0.0.1 designates the local host IP address for server binding.
- **--remote** true enables the usage of a remote file system over SSH.
- **--auth_option** user_password specifies that user-password authentication is required for remote SSH access.
- **--remote_host** 127.0.0.1 determines the IP address of the remote file system host.
- **--remote_port** 22 defines the port number on which to connect to the remote SSH service; port 22 is the default SSH port.
- **--username** <name> and **--password** <pass> are placeholders for the SSH username and password that will be used to authenticate to the remote system.

For alternative SSH remote authentication methods, the provided examples display the diverse authentication configurations that GraphFS can facilitate:


Using an SSH agent, which avoids storing credentials in the configuration:  

```console
--auth_option user_agent --username <name>
```

In this case, user_agent indicates that the server should utilize an SSH agent for authentication, and `--username` <name> should be replaced with the actual SSH username.

Using public and private key authentication, which is a more secure method compared to passwords:  

```console
--auth_option user_pub_key --username <name> --public_key <path_to_pub_key> --private_key <path_to_private_key> --passphrase <passphrase>
```


Here, user_pub_key signals the usage of public key authentication. `--username` <name> is once again the SSH username. The `--public_key` and `--private_key` parameters accept paths to the user's public and private key files, respectively. If the private key is protected by a passphrase, it must be supplied using the `--passphrase` option.

Remember to replace <name>, <path_to_pub_key>, <path_to_private_key>, and <passphrase> with the actual values relevant to your SSH configuration. Each authentication option is designed to match your security practices and preferences for remote file system access using GraphFS.

- To start the server with SSL just add the following parameters

```console
--key_path <path to key file> --cert_path <path to certificate file>
```


Graph FS would immediately detect this and start the server with SSL mode


#### Endpoints
- To log in, you would need to use the REST endpoint `/login` with the payload

```js
{
"email": "<email of user created>",
"password": "<corresponding password for this user>"
}
```

This would return a JWT token which can then be used when auth is enabled. It should be passed into the header as thus: 

```js
{
    "authorization": "token"
}
```

- GraphQL endpoint is `/graphql` to see the playground, access the endpoint is `/graphiql` this has all the documentation of mutation and queries that exist (move a file, move folder, get the list of files, delete a file, etc).


The instructions provided outline the specific HTTP requests necessary for interacting with both local and remote file systems using the GraphFS server. Here's how to use the GraphFS API to manage files:

**Uploading Files:**

- For Local File System (FS):

To upload a file, execute a multipart POST request to the endpoint `/add_local_file`, supplying the `?path=<path to upload>` query parameter where <path to upload> is the desired destination path within the local file system. The path specified must also be a sub-path of the directory defined by `auth_path` to ensure it is within the authorized range.
Example:

`POST /add_local_file?path=/authorized/subfolder/filename.ext`


- For Remote File System (FS):

To upload a file to a remote file system, send a similar multipart POST request to `/add_remote_file` with the query parameter `?path=<path to upload>` indicating the target path on the remote file system.
Example:

`POST /add_remote_file?path=/remote/target/folder/filename.ext`

Reading Files:

- For Local File System (FS):

To read (or download) a file from the local FS, perform a GET request to `/get_local_file` including the `?path=<path_to_file>` query parameter, where <path_to_file> is the path to the desired file within the local FS. Again, this path must be within the `auth_path` directory scope.
Example:

`GET /get_local_file?path=/authorized/subfolder/filename.ext`


- For Remote File System (FS):

To read (or download) a file from the remote FS, issue a GET request to `/get_remote_file` with the `?path=<path to file>` query parameter for specifying the file to be accessed on the remote system.
Example:

`GET /get_remote_file?path=/remote/target/folder/filename.ext`

**Note:**

When the server is set up to interact with a remote FS, the feature set is not limited exclusively to remote operations; you retain the ability to utilize the local FS functionality as well. This dual capability allows for a versatile file management system that can serve both local and remote files transparently to the user.
To effectively use these endpoints, ensure that the paths provided match the actual directory structures of the local and remote file systems. Also, proper authentication is required when accessing these protected endpoints if the server has authentication mechanisms enabled.


### Graph FS Payload 
| Value      |  Description |Default value     |
| :---        |    :----  |    :---: |
| host (-h)  |  Define the host that your application would run on.  | None |
| port (-p)  |  Define the host that your application would run on.| None |
| auth_path   |  This is the only part of your file system that Graph FS is authorized to access| None |
| worker (-w)   |  This allows you to specify how many workers you want your application to eun on| 2 |
| remote   |  Specify to `true` if you wish to use a remote file system | false |
| remote_host   |  The remote host for the file system you wish to use. Would only be used if `remote` is set to true | None |
| remote_port   |  The remote port for the file system you wish to use. Would only be used if `remote` is set to true | None |
| auth_option   |  This allows you to specify the authentication option you desire for your file system which includes with `user password`, `user agent` or `user public key` | None |
| username   |  The `username` for your `auth_option` | None |
| password   |  The `password` for your `auth_option` | None |
| public_key   |  The `public_key` for your `auth_option` | None |
| private_key   |  The `private_key` for your `auth_option` | None |
| passphrase   |  The `passphrase` for your `auth_option` | None |
| cert_path   | The path to the `cert file`. Would automatically set `ssl` if this and `key_path` are provided | None |
| key_path   | The path to the `key file`. Would automatically set `ssl` if this and `cert_path` are provided | None |
| use_auth   | Set value to `true` if you wish to enable authentication for your GraphFS server | false |
| manage_users   | This allows you mange user configurations like `add_user`, `update_username`, `delete_user`, `update_user_password`, and `update_user_permission` | None |
| acc_name   | name for new users or to update exiting users | None |
| acc_email   | email for new users (unique) and immutable | None |
| acc_password   | password for new users or to update existing users password  | None |
| acc_permission   | permission for new users or to update existing users permission  | None |
| secret   | `JWT` secret, must be provided when in auth mode | "default" |
| jwt_duration   | `JWT` secret, duration, to be supplied in days | 30 days |
| storage   | `DB` storage for manging users for now it only accepts `psql` leave empty to use default option | sqlite |
| db_path   | `DB` connection string | None |
| allow_origin   | `CORS` origin to allow | * |