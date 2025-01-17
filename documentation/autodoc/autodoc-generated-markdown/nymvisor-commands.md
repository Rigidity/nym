# `nymvisor` Binary Commands (Autogenerated)

These docs are autogenerated by the [`autodocs`](https://github.com/nymtech/nym/tree/max/new-docs-framework/documentation/autodoc) script.
```sh
Usage: nymvisor [OPTIONS] <COMMAND>

Commands:
  init               Initialise a nymvisor instance with persistent Config.toml file
  run                Run the associated daemon with the preconfigured settings
  build-info         Show build information of this binary
  daemon-build-info  Show build information of the associated daemon
  add-upgrade        Queues up another upgrade for the associated daemon
  config             Show configuration options being used by this instance of nymvisor
  help               Print this message or the help of the given subcommand(s)

Options:
  -c, --config-env-file <CONFIG_ENV_FILE>
          Path pointing to an env file that configures the nymvisor and overrides any preconfigured values
  -h, --help
          Print help
  -V, --version
          Print version
```

### `init`
```sh
Initialise a nymvisor instance with persistent Config.toml file

Usage: nymvisor init [OPTIONS] <DAEMON_BINARY>

Arguments:
  <DAEMON_BINARY>  Path to the daemon's executable

Options:
      --id <ID>
          ID specifies the human readable ID of this particular nymvisor instance. Can be overridden with $NYMVISOR_ID environmental variable
      --upstream-base-upgrade-url <UPSTREAM_BASE_UPGRADE_URL>
          Sets the base url of the upstream source for obtaining upgrade information for the deaemon. It will be used fo constructing the full url, i.e. $NYMVISOR_UPSTREAM_BASE_UPGRADE_URL/$DAEMON_NAME/upgrade-info.json Can be overridden with $NYMVISOR_UPSTREAM_BASE_UPGRADE_URL environmental variable
      --upstream-polling-rate <UPSTREAM_POLLING_RATE>
          Specifies the rate of polling the upstream url for upgrade information. default: 1h Can be overridden with $NYMVISOR_UPSTREAM_POLLING_RATE
      --disable-nymvisor-logs
          If enabled, this will disable `nymvisor` logs (but not the underlying process) Can be overridden with $NYMVISOR_DISABLE_LOGS environmental variable
      --upgrade-data-directory <UPGRADE_DATA_DIRECTORY>
          Set custom directory for upgrade data - binaries and upgrade plans. If not set, the global nymvisors' data directory will be used instead. Can be overridden with $NYMVISOR_UPGRADE_DATA_DIRECTORY environmental variable
      --daemon-home <DAEMON_HOME>
          The location where the `nymvisor/` directory is kept that contains the auxiliary files associated with the underlying daemon, such as any backups or current version information. (e.g. $HOME/.nym/nym-api/my-nym-api, $HOME/.nym/mixnodes/my-mixnode, etc.). Can be overridden with $DAEMON_HOME environmental variable
      --daemon-absolute-upstream-upgrade-url <DAEMON_ABSOLUTE_UPSTREAM_UPGRADE_URL>
          Override url to the upstream source for upgrade plans for this daeamon. The Url has to point to an endpoint containing a valid [`UpgradeInfo`] json. Note: if set this takes precedence over `upstream_base_upgrade_url` Can be overridden with $DAEMON_ABSOLUTE_UPSTREAM_UPGRADE_URL environmental variable
      --allow-download-upgrade-binaries <ALLOW_DOWNLOAD_UPGRADE_BINARIES>
          If set to true, this will enable auto-downloading of new binaries using the url provided in the `upgrade-info.json` Can be overridden with $DAEMON_ALLOW_BINARIES_DOWNLOAD environmental variable [possible values: true, false]
      --enforce-download-checksum <ENFORCE_DOWNLOAD_CHECKSUM>
          If enabled nymvisor will require that a checksum is provided in the upgrade plan for the binary to be downloaded. If disabled, nymvisor will not require a checksum to be provided, but still check the checksum if one is provided. Can be overridden with $DAEMON_ENFORCE_DOWNLOAD_CHECKSUM environmental variable [possible values: true, false]
      --restart-daemon-after-upgrade <RESTART_DAEMON_AFTER_UPGRADE>
          If enabled, nymvisor will restart the subprocess with the same command-line arguments and flags (but with the new binary) after a successful upgrade. Otherwise (if disabled), nymvisor will stop running after an upgrade and will require the system administrator to manually restart it. Note restart is only after the upgrade and does not auto-restart the subprocess after an error occurs. Can be overridden with $DAEMON_RESTART_AFTER_UPGRADE environmental variable [possible values: true, false]
      --restart-daemon-on-failure
          If enabled, nymvisor will restart the subprocess with the same command-line arguments and flags after it has crashed Can be overridden with $DAEMON_RESTART_ON_FAILURE environmental variable
      --on-failure-daemon-restart-delay <ON_FAILURE_DAEMON_RESTART_DELAY>
          If `restart_on_failure` is enabled, the following value defines the amount of time `nymvisor` shall wait before restarting the subprocess. Can be overridden with $DAEMON_FAILURE_RESTART_DELAY environmental variable
      --max-daemon-startup-failures <MAX_DAEMON_STARTUP_FAILURES>
          Defines the maximum number of startup failures the subprocess can experience in a quick succession before no further restarts will be attempted and `nymvisor` will exit. Can be overridden with $DAEMON_MAX_STARTUP_FAILURES environmental variable
      --startup-period-duration <STARTUP_PERIOD_DURATION>
          Defines the length of time during which the subprocess is still considered to be in the startup phase when its failures are going to be considered in `max_startup_failures`. Can be overridden with $DAEMON_STARTUP_PERIOD_DURATION environmental variable
      --daemon-shutdown-grace-period <DAEMON_SHUTDOWN_GRACE_PERIOD>
          Specifies the amount of time `nymvisor` is willing to wait for the subprocess to undergo graceful shutdown after receiving an interrupt (for either an upgrade or shutdown of the `nymvisor` itself) Once the time passes, a kill signal is going to be sent instead. Can be overridden with $DAEMON_SHUTDOWN_GRACE_PERIOD environmental variable
      --daemon-backup-data-directory <DAEMON_BACKUP_DATA_DIRECTORY>
          Set custom backup directory for daemon data. If not set, the daemon's home directory will be used instead. Can be overridden with $DAEMON_BACKUP_DATA_DIRECTORY environmental variable
      --unsafe-skip-backup
          If enabled, `nymvisor` will perform upgrades directly without performing any backups. default: false Can be overridden with $DAEMON_UNSAFE_SKIP_BACKUP environmental variable
  -o, --output <OUTPUT>
          [default: text] [possible values: text, json]
  -h, --help
          Print help
```

### `run`
```sh
Run the associated daemon with the preconfigured settings

Usage: nymvisor run [DAEMON_ARGS]...

Arguments:
  [DAEMON_ARGS]...  

Options:
  -h, --help  Print help
```

### `build-info`
```sh
Show build information of this binary

Usage: nymvisor build-info [OPTIONS]

Options:
  -o, --output <OUTPUT>  [default: text] [possible values: text, json]
  -h, --help             Print help
```
Example output:
```sh

Binary Name:        nymvisor
Build Timestamp:    2024-10-29T09:48:31.988049207Z
Build Version:      0.1.8
Commit SHA:         299552881810511273af13eb135297a4cf7a38de
Commit Date:        2024-10-29T10:48:07.000000000+01:00
Commit Branch:      max/new-docs-framework
rustc Version:      1.80.0
rustc Channel:      stable
cargo Profile:      release

```

### `daemon-build-info`
```sh
Show build information of the associated daemon

Usage: nymvisor daemon-build-info [OPTIONS]

Options:
  -o, --output <OUTPUT>  [default: text] [possible values: text, json]
  -h, --help             Print help
```

### `add-upgrade`
```sh
Queues up another upgrade for the associated daemon

Usage: nymvisor add-upgrade [OPTIONS] --upgrade-name <UPGRADE_NAME> <DAEMON_BINARY>

Arguments:
  <DAEMON_BINARY>  Path to the daemon's upgrade executable

Options:
      --upgrade-name <UPGRADE_NAME>    Name of this upgrade
      --force                          Overwrite existing upgrade binary / upgrade-info.json file
      --add-binary                     Indicate that this command should only add binary to an *existing* scheduled upgrade
      --now                            Force the upgrade to happen immediately
      --publish-date <PUBLISH_DATE>    Specifies the publish date metadata field of this upgrade. If unset, the current time will be used
      --upgrade-time <UPGRADE_TIME>    Specifies the time at which the provided upgrade will be performed (RFC3339 formatted). If left unset, the upgrade will be performed in 15min
      --upgrade-delay <UPGRADE_DELAY>  Specifies delay until the provided upgrade is going to get performed. If let unset, the upgrade will be performed in 15min
  -o, --output <OUTPUT>                [default: text] [possible values: text, json]
  -h, --help                           Print help
```

### `config`
```sh
Show configuration options being used by this instance of nymvisor

Usage: nymvisor config [OPTIONS]

Options:
  -o, --output <OUTPUT>  [default: text] [possible values: text, json]
  -h, --help             Print help
```
