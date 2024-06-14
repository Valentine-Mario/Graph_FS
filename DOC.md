### Graph FS Documentation


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
| secret   | `JWT` secret, must be provided when in auth mode | None |
| jwt_duration   | `JWT` secret, duration, to be supplied in days | 30 days |
| storage   | `DB` storage for manging users for now it only accepts `psql` leave empty to use default option | sqlite |
| db_path   | `DB` connection string | None |
| allow_origin   | `CORS` origin to allow | None |