# monitord-exporter

Tell prometheus how happy your systemd is ! 😊

A prometheus exporter using [monitord](https://github.com/cooperlees/monitord-exporter) to export statistic to prometheus collectors.

## Install

Install via cargo.

- `cargo install monitord-exporter`
- `monitord-exporter --help`

```console
$ monitord-exporter --help
prometheus exporter to share how happy your systemd is ! 😊

Usage: monitord-exporter [OPTIONS]

Options:
  -d, --dbus-address <DBUS_ADDRESS>
          dbus address

          [default: unix:path=/run/dbus/system_bus_socket]

  -l, --log-level <LOG_LEVEL>
          Adjust the console log-level

          [default: Info]
          [possible values: error, warn, info, debug, trace]

  -n, --networkd-state-file-path <NETWORKD_STATE_FILE_PATH>
          network netif dir

          [default: /run/systemd/netif/links]

  -p, --port <PORT>
          TCP Port to listen on

          [default: 1]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Example Output

```console
# TYPE monitord_networkd_address_state gauge
monitord_networkd_address_state{interface_name="eno4"} 3
monitord_networkd_address_state{interface_name="ens2f0"} 1
monitord_networkd_address_state{interface_name="spectrum"} 3
monitord_networkd_address_state{interface_name="vlan69"} 3
monitord_networkd_address_state{interface_name="vlan70"} 3
monitord_networkd_address_state{interface_name="wg0"} 3
# HELP monitord_networkd_admin_state Is the interface configured to be operational (Double check)
# TYPE monitord_networkd_admin_state gauge
monitord_networkd_admin_state{interface_name="eno4"} 4
monitord_networkd_admin_state{interface_name="ens2f0"} 3
monitord_networkd_admin_state{interface_name="spectrum"} 4
monitord_networkd_admin_state{interface_name="vlan69"} 4
monitord_networkd_admin_state{interface_name="vlan70"} 4
monitord_networkd_admin_state{interface_name="wg0"} 4
# HELP monitord_networkd_carrier_state Does the link have physical signal or not
# TYPE monitord_networkd_carrier_state gauge
monitord_networkd_carrier_state{interface_name="eno4"} 5
monitord_networkd_carrier_state{interface_name="ens2f0"} 1
monitord_networkd_carrier_state{interface_name="spectrum"} 5
monitord_networkd_carrier_state{interface_name="vlan69"} 5
monitord_networkd_carrier_state{interface_name="vlan70"} 5
monitord_networkd_carrier_state{interface_name="wg0"} 5
# HELP monitord_networkd_ipv4_address_state Deprecated IP on the interface operational state
# TYPE monitord_networkd_ipv4_address_state gauge
monitord_networkd_ipv4_address_state{interface_name="eno4"} 3
monitord_networkd_ipv4_address_state{interface_name="ens2f0"} 1
monitord_networkd_ipv4_address_state{interface_name="spectrum"} 3
monitord_networkd_ipv4_address_state{interface_name="vlan69"} 3
monitord_networkd_ipv4_address_state{interface_name="vlan70"} 1
monitord_networkd_ipv4_address_state{interface_name="wg0"} 3
# HELP monitord_networkd_ipv6_address_state IPv6 on the interface operational state
# TYPE monitord_networkd_ipv6_address_state gauge
monitord_networkd_ipv6_address_state{interface_name="eno4"} 2
monitord_networkd_ipv6_address_state{interface_name="ens2f0"} 1
monitord_networkd_ipv6_address_state{interface_name="spectrum"} 3
monitord_networkd_ipv6_address_state{interface_name="vlan69"} 3
monitord_networkd_ipv6_address_state{interface_name="vlan70"} 3
monitord_networkd_ipv6_address_state{interface_name="wg0"} 3
# HELP monitord_networkd_managed_interfaces Count of interfaces networkd manages
# TYPE monitord_networkd_managed_interfaces gauge
monitord_networkd_managed_interfaces 6
# HELP monitord_networkd_oper_state Interface overall operational state
# TYPE monitord_networkd_oper_state gauge
monitord_networkd_oper_state{interface_name="eno4"} 9
monitord_networkd_oper_state{interface_name="ens2f0"} 2
monitord_networkd_oper_state{interface_name="spectrum"} 9
monitord_networkd_oper_state{interface_name="vlan69"} 9
monitord_networkd_oper_state{interface_name="vlan70"} 9
monitord_networkd_oper_state{interface_name="wg0"} 9
# HELP monitord_networkd_required_for_online Bool state of systemd being configured to wait for this interface to come online before network online target.
# TYPE monitord_networkd_required_for_online gauge
monitord_networkd_required_for_online{interface_name="eno4"} 1
monitord_networkd_required_for_online{interface_name="ens2f0"} 0
monitord_networkd_required_for_online{interface_name="spectrum"} 1
monitord_networkd_required_for_online{interface_name="vlan69"} 1
monitord_networkd_required_for_online{interface_name="vlan70"} 1
monitord_networkd_required_for_online{interface_name="wg0"} 1
# HELP monitord_units_active_units Count of all active units
# TYPE monitord_units_active_units gauge
monitord_units_active_units 425
# HELP monitord_units_automount_units Count of all automount units
# TYPE monitord_units_automount_units gauge
monitord_units_automount_units 1
# HELP monitord_units_device_units Count of device units
# TYPE monitord_units_device_units gauge
# HELP monitord_units_failed_units Count of failed units - delete or fix
# TYPE monitord_units_failed_units gauge
monitord_units_failed_units 2
# HELP monitord_units_inactive_units Count of inactive units
# TYPE monitord_units_inactive_units gauge
monitord_units_inactive_units 157
# HELP monitord_units_jobs_queued systemd jobs queued - Add what a job is ...
# TYPE monitord_units_jobs_queued gauge
monitord_units_jobs_queued 0
# HELP monitord_units_loaded_units Count of loaded units
# TYPE monitord_units_loaded_units gauge
monitord_units_loaded_units 519
# HELP monitord_units_masked_units Count of masked units
# TYPE monitord_units_masked_units gauge
monitord_units_masked_units 25
# HELP monitord_units_mount_units Count of mount units
# TYPE monitord_units_mount_units gauge
monitord_units_mount_units 52
# HELP monitord_units_not_found_units Count of not found units
# TYPE monitord_units_not_found_units gauge
monitord_units_not_found_units 38
# HELP monitord_units_path_units Count of path units
# TYPE monitord_units_path_units gauge
monitord_units_path_units 4
# HELP monitord_units_scope_units Count of scope units
# TYPE monitord_units_scope_units gauge
monitord_units_scope_units 18
# HELP monitord_units_service_units Count of service units
# TYPE monitord_units_service_units gauge
monitord_units_service_units 200
# HELP monitord_units_slice_units Count of slice units
# TYPE monitord_units_slice_units gauge
monitord_units_slice_units 7
# HELP monitord_units_socket_units Count of socket units
# TYPE monitord_units_socket_units gauge
monitord_units_socket_units 28
# HELP monitord_units_target_units Count of target units
# TYPE monitord_units_target_units gauge
monitord_units_target_units 54
# HELP monitord_units_timer_units Count of timer units
# TYPE monitord_units_timer_units gauge
monitord_units_timer_units 20
# HELP monitord_units_total_units Count of total systemd units
# TYPE monitord_units_total_units gauge
monitord_units_total_units 584
```

## Development

To do test runs (requires `systemd` and optionally `systemd-networkd` *installed*)

- `cargo run -- -p 1234 -l debug`
  - `-l` for logging level. Recommend debug when developing
  - `-p` > 1024 to run as non root / with capabilities

Ensure the following pass before submitting a PR (CI checks):

- `cargo test`
- `cargo clippy`
- `cargo fmt`
