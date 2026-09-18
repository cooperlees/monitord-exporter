# monitord-exporter

Tell Prometheus how happy your systemd is! 😊

A Prometheus exporter using [monitord](https://github.com/cooperlees/monitord) to export systemd health metrics — networkd interfaces, PID 1 stats, per-service stats, system state, unit counts, unit file stats, timers, D-Bus stats, boot blame, unit verification, and machine/container stats.

Stats are collected **on-demand** per Prometheus scrape (not polled on an interval), keeping resource usage minimal.

## How It Works

monitord-exporter wraps the [monitord](https://github.com/cooperlees/monitord) Rust library as a Prometheus exporter. On each scrape request, it calls `monitord::stat_collector()` which queries systemd over D-Bus in real-time, maps the results into Prometheus gauge and counter metrics, and serves them over HTTP.

The binary is optimized for small size (LTO + `opt-level = "z"`), making it suitable for embedded systems.

```
Prometheus scrape → HTTP endpoint ([::]:port) → monitord::stat_collector() → D-Bus → systemd
                                                       ↓
                                             Prometheus metrics served
```

## Requirements

- Linux with systemd
- D-Bus system bus access
- Optional: `systemd-networkd` for networkd metrics
- Optional: `systemd-analyze` for `--boot-blame` and `--verify` features
- Root or appropriate capabilities for PID 1 procfs stats

## Install

```bash
cargo install monitord-exporter
```

## Usage Examples

```bash
# Monitor ssh and docker services on port 9090
monitord-exporter -p 9090 -s ssh.service -s docker.service

# Minimal: just unit counts and system state, no networkd/pid1
monitord-exporter -p 9090 --no-networkd --no-pid1

# Full monitoring with boot blame and verification
monitord-exporter -p 9090 -s ssh.service --boot-blame --boot-blame-count 10 --verify

# Prefer systemd's varlink APIs, falling back to D-Bus per collector (>= 0.27.0)
monitord-exporter -p 9090 --varlink

# Load settings from a monitord.conf file (>= 0.19.0)
monitord-exporter -p 9090 -c /etc/monitord.conf

# Debug logging
monitord-exporter -p 9090 -l debug
```

## CLI Reference

### Config file (>= 0.19.0)

| Flag | Description |
|------|-------------|
| `-c, --config` | Path to a `monitord.conf` config file. Mutually exclusive with all other config arguments. |

When `-c` is supplied the exporter reads all monitord settings (services, timers, feature toggles, D-Bus address, etc.) from the provided INI-format config file — the same file format used by the `monitord` daemon. Passing any other config flag together with `-c` is an error. The exporter-only flags `-p/--port` and `-l/--log-level` may still be used alongside `-c`.

### Core options

| Flag | Default | Description |
|------|---------|-------------|
| `-p, --port` | `1` | TCP port to listen on (use >1024 for non-root) |
| `-d, --dbus-address` | `unix:path=/run/dbus/system_bus_socket` | D-Bus address |
| `-l, --log-level` | `Info` | Log level: error, warn, info, debug, trace |

### Feature toggles (disable collectors)

| Flag | Description |
|------|-------------|
| `--no-networkd` | Disable networkd interface stats |
| `--no-pid1` | Disable PID 1 process stats |
| `--no-system-state` | Disable system state |
| `--no-timers` | Disable timer stats |
| `--no-dbus` | Disable D-Bus stats |
| `--no-unit-states` | Disable per-unit state tracking |
| `--no-machines` | Disable machine/container stats |

### Service & timer tracking

| Flag | Description |
|------|-------------|
| `-s, --services` | Services to track (repeatable, e.g. `-s ssh.service -s docker.service`) |
| `--timers` | Specific timers to track (repeatable) |

### Boot analysis

| Flag | Default | Description |
|------|---------|-------------|
| `--boot-blame` | disabled | Enable boot blame stats (requires `systemd-analyze`) |
| `--boot-blame-count` | `5` | Number of slowest boot units to report (requires `--boot-blame`) |
| `--no-boot-cache` | cache enabled | Disable boot blame result caching (requires `--boot-blame`) |

### Other

| Flag | Default | Description |
|------|---------|-------------|
| `--verify` | disabled | Enable unit verification via `systemd-analyze verify` |
| `--varlink` | disabled | Collect via systemd's varlink APIs where available, falling back to D-Bus per collector |
| `--networkd-state-file-path` | `/run/systemd/netif/links` | Path to networkd link state files |

`--varlink` is the master switch only. Each varlink-capable collector also has
its own `varlink = true|false` key in the `[networkd]`, `[system-state]`,
`[units]`, `[machines]`, `[boot]` and `[verify]` sections of a `monitord.conf`
(all default `true`), so keeping a single collector on D-Bus while the rest
move to varlink requires `-c`.

## Metrics Reference

All metrics use the `monitord_` prefix. The example output below shows a subset; see each category for the full list.

### Networkd metrics (`monitord_networkd_*`)

Per-interface gauges (labeled by `interface_name`): address_state, admin_state, carrier_state, ipv4_address_state, ipv6_address_state, oper_state, required_for_online. Plus a global `managed_interfaces` count.

### PID 1 metrics (`monitord_pid1_*`)

Process stats for PID 1 (systemd): cpu_time_kernel, cpu_user_kernel, fd_count, memory_usage_bytes, tasks.

### Service metrics (`monitord_service_*`)

Per-service gauges (labeled by `service_name`): active_enter_timestamp, active_exit_timestamp, cpuusage_nsec, inactive_exit_timestamp, ioread_bytes, ioread_operations, memory_available, memory_current, nrestarts, processes, restart_usec, state_change_timestamp, status_errno, tasks_current, timeout_clean_usec, watchdog_usec.

### System state (`monitord_system_state`)

Overall systemd system state as a numeric enum value.

### Unit counts (`monitord_units_*`)

Counts of systemd units by type: active, automount, device, failed, inactive, loaded, masked, mount, not_found, path, scope, service, slice, socket, target, timer, total. Plus `jobs_queued`.

### Unit files metrics (`monitord_unit_files_*`)

Unit file counts by scope and unit type from filesystem stats:

- `monitord_unit_files_generated{scope="root|user",unit_type="..."}`
- `monitord_unit_files_transient{scope="root|user",unit_type="..."}`

### Timer metrics (`monitord_timer_*`)

Per-timer gauges (labeled by `timer_name`): accuracy_usec, fixed_random_delay, last_trigger_usec, last_trigger_usec_monotonic, next_elapse_usec_monotonic, next_elapse_usec_realtime, persistent, randomized_delay_usec, remain_after_elapse, service_unit_last_state_change_usec, service_unit_last_state_change_usec_monotonic.

### Unit state metrics (`monitord_unit_*`)

Per-unit gauges (labeled by `unit_name`): active_state, load_state, unhealthy, time_in_state_usecs.

### D-Bus metrics (`monitord_dbus_*`)

System-wide D-Bus stats: serial, active_connections, incomplete_connections, bus_names, peak_bus_names, peak_bus_names_per_connection, match_rules, peak_match_rules, peak_match_rules_per_connection.

Per-UID stats (`monitord_dbus_user_*`): bytes_cur, bytes_max, fds_cur, fds_max, matches_cur, matches_max, objects_cur, objects_max.

Per-peer stats (`monitord_dbus_peer_*`): name_objects, matches, match_bytes, reply_objects, incoming_bytes, incoming_fds, outgoing_bytes, outgoing_fds.

### Collection stats (`monitord_stat_collection_run_time_ms`)

End-to-end duration of the last monitord stat collection run in milliseconds. Always exported; useful for tracking collection overhead.

### Collector timing metrics (`monitord_collector_*`, `monitord_units_collection_*`)

Per-collector wall-time breakdown of the last `stat_collector` cycle, plus the inner D-Bus phase breakdown of the units collector. Always exported.

Per-collector gauges (labeled by `collector`, e.g. `units`, `pid1`, `dbus_stats`, `boot_blame`, `verify`, `version`, `system_state`, `networkd`, `machines`):

- `monitord_collector_start_offset_ms` — ms from the top of the cycle to the collector future's first poll. Sub-ms when collectors run in parallel; a non-trivial value means the spawn loop or runtime is delaying first poll.
- `monitord_collector_elapsed_ms` — ms from first poll to completion.
- `monitord_collector_success` — `1` if the collector returned `Ok`, else `0`.

Units collector inner phases (no labels):

- `monitord_units_collection_list_units_ms` — time of the systemd `ListUnits` D-Bus call (one batched call returning all units).
- `monitord_units_collection_unit_files_ms` — time spent collecting filesystem unit file stats.
- `monitord_units_collection_per_unit_loop_ms` — time spent in the per-unit parse loop, including any per-unit D-Bus calls.
- `monitord_units_collection_timer_dbus_fetches` — count of timer D-Bus property fetches this run.
- `monitord_units_collection_state_dbus_fetches` — count of unit-state D-Bus fetches this run.
- `monitord_units_collection_service_dbus_fetches` — count of per-service D-Bus property fetches this run.

Comparing `sum(monitord_collector_elapsed_ms) / monitord_stat_collection_run_time_ms` gives an effective parallelism ratio (`≈ N` means N-way parallelism, `≈ 1` means effectively serial).

### Varlink usage metrics (`monitord_varlink_usage`)

Which transport served each collector on the last run, labeled by `collector`
(`version`, `system_state`, `units`, `networkd`, `machines`, `boot_blame`,
`verify`): `1` when the varlink attempt succeeded, `0` when the collector fell
back to D-Bus (or to file-based collection for networkd). Collectors with no
varlink path at all (`pid1`, `dbus_stats`) and disabled collectors have no
series, so the series that are present are exactly the enabled set and
`avg(monitord_varlink_usage)` is the share of collectors served by varlink.

These are always exported, not gated on `--varlink`: every enabled collector
records the transport that served it, so without the flag the series are still
present and read `0`. The flag is only what makes a `1` possible.

Collectors flip from `0` to `1` with no config change as the host's systemd
upgrades past each endpoint's minimum version (networkd v257+, system
state/version v258+, units v260+, unit details v261+), so graphing this over a
fleet shows varlink adoption climbing.

### Boot blame metrics (`monitord_boot_blame_*`)

Enabled with `--boot-blame`. Reports the N slowest units at boot: `activation_time_seconds` (labeled by `unit_name`).

### Verify metrics (`monitord_verify_*`)

Enabled with `--verify`. Reports unit verification failures: `failed_units_total` and `failed_units_by_type` (labeled by `unit_type`).

### systemd version (`monitord_systemd_version_*`)

Reports the running systemd version: `major` (numeric) and `info` (labeled with version string).

### Machine/container metrics (`monitord_machine_*`)

Mirrors host metrics per machine/container, labeled by `machine_name`:

- **system_state** — systemd system state (numeric)
- **units** — unit counts by type (activating, active, failed, etc.)
- **pid1** — PID 1 CPU, memory, FDs, tasks
- **service** — per-service stats (labeled by `machine_name` + `service_name`)
- **networkd** — managed interface count
- **timer** — per-timer stats (labeled by `machine_name` + `timer_name`; gated by `--no-timers`)
- **unit_state** — per-unit active/load state (labeled by `machine_name` + `unit_name`; gated by `--no-unit-states`)
- **unit_files** — generated/transient unit file counts (labeled by `machine_name` + `scope` + `unit_type`)
- **version** — systemd version inside the machine (`major` numeric, `info` labeled with version string)
- **boot_blame** — slowest boot units (gated by `--boot-blame`)
- **verify** — unit verification failures (gated by `--verify`)
- **units_collection** — per-machine units collector inner timings (`monitord_machine_units_collection_{list_units_ms,unit_files_ms,per_unit_loop_ms,timer_dbus_fetches,state_dbus_fetches,service_dbus_fetches}`)
- **varlink_usage** — transport that served each collector inside the machine (`monitord_machine_varlink_usage`, labeled by `machine_name` + `collector`). A container `units` value of `1` still involves some D-Bus underneath (the timer backfill and oneshot type override have no varlink equivalent there), so compare host and container values separately rather than aggregating them. The host-side `machines` collector covers enumeration only and stays `0` until machined grows a varlink List API.

## Prometheus Scrape Config

```yaml
scrape_configs:
  - job_name: 'monitord'
    static_configs:
      - targets: ['localhost:9090']
```

## Example Output

The following shows a subset of metrics from a typical scrape. See the [Metrics Reference](#metrics-reference) for the full list.

```console
# HELP monitord_networkd_address_state Protocol independent address states
# TYPE monitord_networkd_address_state gauge
monitord_networkd_address_state{interface_name="eth0"} 3
monitord_networkd_address_state{interface_name="wg0"} 3
# HELP monitord_networkd_admin_state Is the interface configured to be operational
# TYPE monitord_networkd_admin_state gauge
monitord_networkd_admin_state{interface_name="eth0"} 4
monitord_networkd_admin_state{interface_name="wg0"} 4
# HELP monitord_networkd_carrier_state Does the link have physical signal or not
# TYPE monitord_networkd_carrier_state gauge
monitord_networkd_carrier_state{interface_name="eth0"} 5
monitord_networkd_carrier_state{interface_name="wg0"} 5
# HELP monitord_networkd_managed_interfaces Count of interfaces networkd manages
# TYPE monitord_networkd_managed_interfaces gauge
monitord_networkd_managed_interfaces 2
# HELP monitord_networkd_oper_state Interface overall operational state
# TYPE monitord_networkd_oper_state gauge
monitord_networkd_oper_state{interface_name="eth0"} 9
monitord_networkd_oper_state{interface_name="wg0"} 9
# HELP monitord_pid1_cpu_time_kernel CPU time used by PID1
# TYPE monitord_pid1_cpu_time_kernel gauge
monitord_pid1_cpu_time_kernel 189
# HELP monitord_pid1_fd_count Open file descriptors for PID1
# TYPE monitord_pid1_fd_count gauge
monitord_pid1_fd_count 169
# HELP monitord_pid1_memory_usage_bytes Memory usage in bytes for PID1
# TYPE monitord_pid1_memory_usage_bytes gauge
monitord_pid1_memory_usage_bytes 10289152
# HELP monitord_pid1_tasks Processes / threads of PID1
# TYPE monitord_pid1_tasks gauge
monitord_pid1_tasks 1
# HELP monitord_service_active_enter_timestamp Active enter timestamp
# TYPE monitord_service_active_enter_timestamp gauge
monitord_service_active_enter_timestamp{service_name="ssh.service"} 1717037801690678
# HELP monitord_service_cpuuage_nsec CPU usage nano seconds
# TYPE monitord_service_cpuuage_nsec gauge
monitord_service_cpuuage_nsec{service_name="ssh.service"} 3257412922000
# HELP monitord_service_memory_current Memory currently in use
# TYPE monitord_service_memory_current gauge
monitord_service_memory_current{service_name="ssh.service"} 6971392
# HELP monitord_service_nrestarts Count of automatic restarts of the service
# TYPE monitord_service_nrestarts gauge
monitord_service_nrestarts{service_name="ssh.service"} 0
# HELP monitord_service_processes Count of processes
# TYPE monitord_service_processes gauge
monitord_service_processes{service_name="ssh.service"} 1
# HELP monitord_system_state systemd system state - Refer to monitord enum for meaning
# TYPE monitord_system_state gauge
monitord_system_state 3
# HELP monitord_units_active_units Count of all active units
# TYPE monitord_units_active_units gauge
monitord_units_active_units 306
# HELP monitord_units_failed_units Count of failed units
# TYPE monitord_units_failed_units gauge
monitord_units_failed_units 0
# HELP monitord_units_loaded_units Count of loaded units
# TYPE monitord_units_loaded_units gauge
monitord_units_loaded_units 431
# HELP monitord_units_total_units Count of total systemd units
# TYPE monitord_units_total_units gauge
monitord_units_total_units 475
```

### Varlink usage metrics

Always exported; the sample below is from a host run with `--varlink`. Without
that flag the same series are present and every value is `0`.

```console
# HELP monitord_varlink_usage 1 if varlink served this collector on the last run, 0 if it fell back to D-Bus (or files for networkd). Collectors with no varlink path (pid1, dbus) and disabled collectors have no series
# TYPE monitord_varlink_usage gauge
monitord_varlink_usage{collector="machines"} 0
monitord_varlink_usage{collector="networkd"} 1
monitord_varlink_usage{collector="system_state"} 1
monitord_varlink_usage{collector="units"} 0
monitord_varlink_usage{collector="version"} 1
```

### Boot blame metrics (enabled with `--boot-blame`)

```console
# HELP monitord_boot_blame_activation_time_seconds Boot blame activation time in seconds per unit
# TYPE monitord_boot_blame_activation_time_seconds gauge
monitord_boot_blame_activation_time_seconds{unit_name="NetworkManager-wait-online.service"} 8.342
monitord_boot_blame_activation_time_seconds{unit_name="systemd-journal-flush.service"} 3.105
```

### Verify metrics (enabled with `--verify`)

```console
# HELP monitord_verify_failed_units_by_type Count of units with verification failures by unit type
# TYPE monitord_verify_failed_units_by_type gauge
monitord_verify_failed_units_by_type{unit_type="service"} 1
monitord_verify_failed_units_by_type{unit_type="timer"} 1
# HELP monitord_verify_failed_units_total Total count of units with verification failures
# TYPE monitord_verify_failed_units_total gauge
monitord_verify_failed_units_total 2
```

## Development

To do test runs (requires `systemd` and optionally `systemd-networkd` _installed_):

```bash
cargo run -- -p 1234 -l debug         # Run locally with debug logging
cargo build --release --all-features   # Release build (optimized for size via LTO)
```

- Use `-p` > 1024 to run as non-root / without capabilities
- Root is required to read procfs stats of PID 1
- The exporter binds to `[::]` (IPv6)

Ensure the following pass before submitting a PR (CI checks):

- `cargo test`
- `cargo clippy`
- `cargo fmt`
