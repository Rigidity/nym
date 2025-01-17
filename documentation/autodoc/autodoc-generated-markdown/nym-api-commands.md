# `nym-api` Binary Commands (Autogenerated)

These docs are autogenerated by the [`autodocs`](https://github.com/nymtech/nym/tree/max/new-docs-framework/documentation/autodoc) script.
```sh
[2m2024-10-29T09:51:44.008469Z[0m [32m INFO[0m [2mnym-api/src/main.rs[0m[2m:[0m[2m41[0m[2m:[0m Starting nym api...
Usage: nym-api [OPTIONS] <COMMAND>

Commands:
  init        Initialise a Nym Api instance with persistent config.toml file
  run         Run the Nym Api with provided configuration optionally overriding set parameters
  build-info  Show build information of this binary
  help        Print this message or the help of the given subcommand(s)

Options:
  -c, --config-env-file <CONFIG_ENV_FILE>
          Path pointing to an env file that configures the Nym API [env: NYMAPI_CONFIG_ENV_FILE_ARG=]
      --no-banner
          A no-op flag included for consistency with other binaries (and compatibility with nymvisor, oops) [env: NYMAPI_NO_BANNER_ARG=]
  -h, --help
          Print help
  -V, --version
          Print version
```

### `init`
```sh
[2m2024-10-29T09:51:44.013292Z[0m [32m INFO[0m [2mnym-api/src/main.rs[0m[2m:[0m[2m41[0m[2m:[0m Starting nym api...
Initialise a Nym Api instance with persistent config.toml file

Usage: nym-api init [OPTIONS]

Options:
      --id <ID>
          Id of the nym-api we want to initialise. if unspecified, a default value will be used. default: "default" [env: NYMAPI_ID_ARG=] [default: default]
  -m, --enable-monitor
          Specifies whether network monitoring is enabled on this API default: false [env: NYMAPI_ENABLE_MONITOR_ARG=]
  -r, --enable-rewarding
          Specifies whether network rewarding is enabled on this API default: false [env: NYMAPI_ENABLE_REWARDING_ARG=]
      --nyxd-validator <NYXD_VALIDATOR>
          Endpoint to nyxd instance used for contract information. default: http://localhost:26657 [env: NYMAPI_NYXD_VALIDATOR_ARG=]
      --mnemonic <MNEMONIC>
          Mnemonic of the network monitor used for sending rewarding and zk-nyms transactions default: None [env: NYMAPI_MNEMONIC_ARG=]
      --enable-zk-nym
          Flag to indicate whether credential signer authority is enabled on this API default: false [env: NYMAPI_ENABLE_ZK_NYM_ARG=]
      --announce-address <ANNOUNCE_ADDRESS>
          Announced address that is going to be put in the DKG contract where zk-nym clients will connect to obtain their credentials default: None [env: NYMAPI_ANNOUNCE_ADDRESS_NYM_ARG=]
      --monitor-credentials-mode
          Set this nym api to work in a enabled credentials that would attempt to use gateway with the bandwidth credential requirement [env: NYMAPI_MONITOR_CREDENTIALS_MODE_ARG=]
      --bind-address <BIND_ADDRESS>
          Socket address this api will use for binding its http API. default: `127.0.0.1:8080` in `debug` builds and `0.0.0.0:8080` in `release`
  -h, --help
          Print help
```

### `run`
```sh
[2m2024-10-29T09:51:44.032688Z[0m [32m INFO[0m [2mnym-api/src/main.rs[0m[2m:[0m[2m41[0m[2m:[0m Starting nym api...
Run the Nym Api with provided configuration optionally overriding set parameters

Usage: nym-api run [OPTIONS]

Options:
      --id <ID>
          Id of the nym-api we want to run.if unspecified, a default value will be used. default: "default" [env: NYMAPI_ID_ARG=] [default: default]
  -m, --enable-monitor <ENABLE_MONITOR>
          Specifies whether network monitoring is enabled on this API default: None - config value will be used instead [env: NYMAPI_ENABLE_MONITOR_ARG=] [possible values: true, false]
  -r, --enable-rewarding <ENABLE_REWARDING>
          Specifies whether network rewarding is enabled on this API default: None - config value will be used instead [env: NYMAPI_ENABLE_REWARDING_ARG=] [possible values: true, false]
      --nyxd-validator <NYXD_VALIDATOR>
          Endpoint to nyxd instance used for contract information. default: None - config value will be used instead [env: NYMAPI_NYXD_VALIDATOR_ARG=]
      --mnemonic <MNEMONIC>
          Mnemonic of the network monitor used for sending rewarding and zk-nyms transactions default: None - config value will be used instead [env: NYMAPI_MNEMONIC_ARG=]
      --enable-zk-nym <ENABLE_ZK_NYM>
          Flag to indicate whether coconut signer authority is enabled on this API default: None - config value will be used instead [env: NYMAPI_ENABLE_ZK_NYM_ARG=] [possible values: true, false]
      --announce-address <ANNOUNCE_ADDRESS>
          Announced address that is going to be put in the DKG contract where zk-nym clients will connect to obtain their credentials default: None - config value will be used instead [env: NYMAPI_ANNOUNCE_ADDRESS_ARG=]
      --monitor-credentials-mode <MONITOR_CREDENTIALS_MODE>
          Set this nym api to work in a enabled credentials that would attempt to use gateway with the bandwidth credential requirement default: None - config value will be used instead [env: NYMAPI_MONITOR_CREDENTIALS_MODE_ARG=] [possible values: true, false]
      --bind-address <BIND_ADDRESS>
          Socket address this api will use for binding its http API. default: `127.0.0.1:8080` in `debug` builds and `0.0.0.0:8080` in `release`
  -h, --help
          Print help
```

### `build-info`
```sh
[2m2024-10-29T09:51:44.037915Z[0m [32m INFO[0m [2mnym-api/src/main.rs[0m[2m:[0m[2m41[0m[2m:[0m Starting nym api...
Show build information of this binary

Usage: nym-api build-info [OPTIONS]

Options:
  -o, --output <OUTPUT>  [default: text] [possible values: text, json]
  -h, --help             Print help
```
