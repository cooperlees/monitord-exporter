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

  -s, --services <SERVICES>
          Services to get service stats for

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Example Output

```console
# HELP monitord_networkd_address_state Protocol independent address states (Need to find a better explanation)
# TYPE monitord_networkd_address_state gauge
monitord_networkd_address_state{interface_name="eno4"} 3
monitord_networkd_address_state{interface_name="spectrum"} 3
monitord_networkd_address_state{interface_name="vlan69"} 3
monitord_networkd_address_state{interface_name="vlan70"} 3
monitord_networkd_address_state{interface_name="wg0"} 3
# HELP monitord_networkd_admin_state Is the interface configured to be operational (Double check)
# TYPE monitord_networkd_admin_state gauge
monitord_networkd_admin_state{interface_name="eno4"} 4
monitord_networkd_admin_state{interface_name="spectrum"} 4
monitord_networkd_admin_state{interface_name="vlan69"} 4
monitord_networkd_admin_state{interface_name="vlan70"} 4
monitord_networkd_admin_state{interface_name="wg0"} 4
# HELP monitord_networkd_carrier_state Does the link have physical signal or not
# TYPE monitord_networkd_carrier_state gauge
monitord_networkd_carrier_state{interface_name="eno4"} 5
monitord_networkd_carrier_state{interface_name="spectrum"} 5
monitord_networkd_carrier_state{interface_name="vlan69"} 5
monitord_networkd_carrier_state{interface_name="vlan70"} 5
monitord_networkd_carrier_state{interface_name="wg0"} 5
# HELP monitord_networkd_ipv4_address_state Deprecated IP on the interface operational state
# TYPE monitord_networkd_ipv4_address_state gauge
monitord_networkd_ipv4_address_state{interface_name="eno4"} 3
monitord_networkd_ipv4_address_state{interface_name="spectrum"} 3
monitord_networkd_ipv4_address_state{interface_name="vlan69"} 3
monitord_networkd_ipv4_address_state{interface_name="vlan70"} 1
monitord_networkd_ipv4_address_state{interface_name="wg0"} 3
# HELP monitord_networkd_ipv6_address_state IPv6 on the interface operational state
# TYPE monitord_networkd_ipv6_address_state gauge
monitord_networkd_ipv6_address_state{interface_name="eno4"} 2
monitord_networkd_ipv6_address_state{interface_name="spectrum"} 3
monitord_networkd_ipv6_address_state{interface_name="vlan69"} 3
monitord_networkd_ipv6_address_state{interface_name="vlan70"} 3
monitord_networkd_ipv6_address_state{interface_name="wg0"} 3
# HELP monitord_networkd_managed_interfaces Count of interfaces networkd manages
# TYPE monitord_networkd_managed_interfaces gauge
monitord_networkd_managed_interfaces 5
# HELP monitord_networkd_oper_state Interface overall operational state
# TYPE monitord_networkd_oper_state gauge
monitord_networkd_oper_state{interface_name="eno4"} 9
monitord_networkd_oper_state{interface_name="spectrum"} 9
monitord_networkd_oper_state{interface_name="vlan69"} 9
monitord_networkd_oper_state{interface_name="vlan70"} 9
monitord_networkd_oper_state{interface_name="wg0"} 9
# HELP monitord_networkd_required_for_online Bool state of systemd being configured to wait for this interface to come online before network online target.
# TYPE monitord_networkd_required_for_online gauge
monitord_networkd_required_for_online{interface_name="eno4"} 1
monitord_networkd_required_for_online{interface_name="spectrum"} 1
monitord_networkd_required_for_online{interface_name="vlan69"} 1
monitord_networkd_required_for_online{interface_name="vlan70"} 1
monitord_networkd_required_for_online{interface_name="wg0"} 1
# HELP monitord_service_active_enter_timestamp Active enter timestamp
# TYPE monitord_service_active_enter_timestamp gauge
monitord_service_active_enter_timestamp{service_name="ssh.service"} 1704352626952323
# HELP monitord_service_active_exit_timestamp Active exti timestamp
# TYPE monitord_service_active_exit_timestamp gauge
monitord_service_active_exit_timestamp{service_name="ssh.service"} 0
# HELP monitord_service_cpuuage_nsec CPU usage nano seconds
# TYPE monitord_service_cpuuage_nsec gauge
monitord_service_cpuuage_nsec{service_name="ssh.service"} 224650667000
# HELP monitord_service_inactive_exit_timestamp Inactive exit timestamp
# TYPE monitord_service_inactive_exit_timestamp gauge
monitord_service_inactive_exit_timestamp{service_name="ssh.service"} 1704352626895924
# HELP monitord_service_ioread_bytes IO bytes read
# TYPE monitord_service_ioread_bytes gauge
monitord_service_ioread_bytes{service_name="ssh.service"} -1
# HELP monitord_service_ioread_operations IO Opertations
# TYPE monitord_service_ioread_operations gauge
monitord_service_ioread_operations{service_name="ssh.service"} -1
# HELP monitord_service_memory_available Memory available
# TYPE monitord_service_memory_available gauge
monitord_service_memory_available{service_name="ssh.service"} -1
# HELP monitord_service_memory_current Memory currently in use
# TYPE monitord_service_memory_current gauge
monitord_service_memory_current{service_name="ssh.service"} 8265728
# HELP monitord_service_nrestarts Count of automatic restarts of the service
# TYPE monitord_service_nrestarts gauge
monitord_service_nrestarts{service_name="ssh.service"} 0
# HELP monitord_service_processes Count of processes
# TYPE monitord_service_processes gauge
monitord_service_processes{service_name="ssh.service"} 1
# HELP monitord_service_restart_usec Restart time in usecs
# TYPE monitord_service_restart_usec gauge
monitord_service_restart_usec{service_name="ssh.service"} 100000
# HELP monitord_service_state_chage_timestamp Last unit state change timestamp
# TYPE monitord_service_state_chage_timestamp gauge
monitord_service_state_chage_timestamp{service_name="ssh.service"} 1704352626952323
# HELP monitord_service_status_errno Status error number
# TYPE monitord_service_status_errno gauge
monitord_service_status_errno{service_name="ssh.service"} 0
# HELP monitord_service_tasks_current Tasks current (processes + threads)
# TYPE monitord_service_tasks_current gauge
monitord_service_tasks_current{service_name="ssh.service"} 1
# HELP monitord_service_timeout_clean_usec Clean timeout usecs
# TYPE monitord_service_timeout_clean_usec gauge
monitord_service_timeout_clean_usec{service_name="ssh.service"} -1
# HELP monitord_service_watchdog_usec Watchdog runtime usecs
# TYPE monitord_service_watchdog_usec gauge
monitord_service_watchdog_usec{service_name="ssh.service"} 0
# HELP monitord_units_active_units Count of all active units
# TYPE monitord_units_active_units gauge
monitord_units_active_units 434
# HELP monitord_units_automount_units Count of all automount units
# TYPE monitord_units_automount_units gauge
monitord_units_automount_units 1
# HELP monitord_units_device_units Count of device units
# TYPE monitord_units_device_units gauge
monitord_units_device_units 170
# HELP monitord_units_failed_units Count of failed units - delete or fix
# TYPE monitord_units_failed_units gauge
monitord_units_failed_units 0
# HELP monitord_units_inactive_units Count of inactive units
# TYPE monitord_units_inactive_units gauge
monitord_units_inactive_units 188
# HELP monitord_units_jobs_queued systemd jobs queued - Add what a job is ...
# TYPE monitord_units_jobs_queued gauge
monitord_units_jobs_queued 0
# HELP monitord_units_loaded_units Count of loaded units
# TYPE monitord_units_loaded_units gauge
monitord_units_loaded_units 547
# HELP monitord_units_masked_units Count of masked units
# TYPE monitord_units_masked_units gauge
monitord_units_masked_units 33
# HELP monitord_units_mount_units Count of mount units
# TYPE monitord_units_mount_units gauge
monitord_units_mount_units 57
# HELP monitord_units_not_found_units Count of not found units
# TYPE monitord_units_not_found_units gauge
monitord_units_not_found_units 40
# HELP monitord_units_path_units Count of path units
# TYPE monitord_units_path_units gauge
monitord_units_path_units 4
# HELP monitord_units_scope_units Count of scope units
# TYPE monitord_units_scope_units gauge
monitord_units_scope_units 20
# HELP monitord_units_service_units Count of service units
# TYPE monitord_units_service_units gauge
monitord_units_service_units 208
# HELP monitord_units_slice_units Count of slice units
# TYPE monitord_units_slice_units gauge
monitord_units_slice_units 7
# HELP monitord_units_socket_units Count of socket units
# TYPE monitord_units_socket_units gauge
monitord_units_socket_units 35
# HELP monitord_units_target_units Count of target units
# TYPE monitord_units_target_units gauge
monitord_units_target_units 64
# HELP monitord_units_timer_units Count of timer units
# TYPE monitord_units_timer_units gauge
monitord_units_timer_units 23
# HELP monitord_units_total_units Count of total systemd units
# TYPE monitord_units_total_units gauge
monitord_units_total_units 622
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
