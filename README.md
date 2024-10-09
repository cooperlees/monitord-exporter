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

      --no-networkd
          networkd stats disable

      --no-pid1
          pid1 stats disable

      --no-system-state
          system state stats disable

      --networkd-state-file-path <NETWORKD_STATE_FILE_PATH>
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
monitord_networkd_address_state{interface_name="eth0"} 3
monitord_networkd_address_state{interface_name="wg0"} 3
# HELP monitord_networkd_admin_state Is the interface configured to be operational (Double check)
# TYPE monitord_networkd_admin_state gauge
monitord_networkd_admin_state{interface_name="eth0"} 4
monitord_networkd_admin_state{interface_name="wg0"} 4
# HELP monitord_networkd_carrier_state Does the link have physical signal or not
# TYPE monitord_networkd_carrier_state gauge
monitord_networkd_carrier_state{interface_name="eth0"} 5
monitord_networkd_carrier_state{interface_name="wg0"} 5
# HELP monitord_networkd_ipv4_address_state Deprecated IP on the interface operational state
# TYPE monitord_networkd_ipv4_address_state gauge
monitord_networkd_ipv4_address_state{interface_name="eth0"} 3
monitord_networkd_ipv4_address_state{interface_name="wg0"} 3
# HELP monitord_networkd_ipv6_address_state IPv6 on the interface operational state
# TYPE monitord_networkd_ipv6_address_state gauge
monitord_networkd_ipv6_address_state{interface_name="eth0"} 3
monitord_networkd_ipv6_address_state{interface_name="wg0"} 3
# HELP monitord_networkd_managed_interfaces Count of interfaces networkd manages
# TYPE monitord_networkd_managed_interfaces gauge
monitord_networkd_managed_interfaces 2
# HELP monitord_networkd_oper_state Interface overall operational state
# TYPE monitord_networkd_oper_state gauge
monitord_networkd_oper_state{interface_name="eth0"} 9
monitord_networkd_oper_state{interface_name="wg0"} 9
# HELP monitord_networkd_required_for_online Bool state of systemd being configured to wait for this interface to come online before network online target.
# TYPE monitord_networkd_required_for_online gauge
monitord_networkd_required_for_online{interface_name="eth0"} 1
monitord_networkd_required_for_online{interface_name="wg0"} 1
# HELP monitord_pid1_cpu_time_kernel CPU time used by PID1
# TYPE monitord_pid1_cpu_time_kernel gauge
monitord_pid1_cpu_time_kernel 189
# HELP monitord_pid1_cpu_user_kernel CPU user space time used by PID1
# TYPE monitord_pid1_cpu_user_kernel gauge
monitord_pid1_cpu_user_kernel 268
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
# HELP monitord_service_active_exit_timestamp Active exti timestamp
# TYPE monitord_service_active_exit_timestamp gauge
monitord_service_active_exit_timestamp{service_name="ssh.service"} 0
# HELP monitord_service_cpuuage_nsec CPU usage nano seconds
# TYPE monitord_service_cpuuage_nsec gauge
monitord_service_cpuuage_nsec{service_name="ssh.service"} 3257412922000
# HELP monitord_service_inactive_exit_timestamp Inactive exit timestamp
# TYPE monitord_service_inactive_exit_timestamp gauge
monitord_service_inactive_exit_timestamp{service_name="ssh.service"} 1717037801646233
# HELP monitord_service_ioread_bytes IO bytes read
# TYPE monitord_service_ioread_bytes gauge
monitord_service_ioread_bytes{service_name="ssh.service"} -1
# HELP monitord_service_ioread_operations IO Opertations
# TYPE monitord_service_ioread_operations gauge
monitord_service_ioread_operations{service_name="ssh.service"} -1
# HELP monitord_service_memory_available Memory available
# TYPE monitord_service_memory_available gauge
monitord_service_memory_available{service_name="ssh.service"} 771878912
# HELP monitord_service_memory_current Memory currently in use
# TYPE monitord_service_memory_current gauge
monitord_service_memory_current{service_name="ssh.service"} 6971392
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
monitord_service_state_chage_timestamp{service_name="ssh.service"} 1717037801690678
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
# HELP monitord_system_state systemd system state - Refer to monitord enum for meaning
# TYPE monitord_system_state gauge
monitord_system_state 3
# HELP monitord_units_active_units Count of all active units
# TYPE monitord_units_active_units gauge
monitord_units_active_units 306
# HELP monitord_units_automount_units Count of all automount units
# TYPE monitord_units_automount_units gauge
monitord_units_automount_units 2
# HELP monitord_units_device_units Count of device units
# TYPE monitord_units_device_units gauge
monitord_units_device_units 79
# HELP monitord_units_failed_units Count of failed units - delete or fix
# TYPE monitord_units_failed_units gauge
monitord_units_failed_units 0
# HELP monitord_units_inactive_units Count of inactive units
# TYPE monitord_units_inactive_units gauge
monitord_units_inactive_units 169
# HELP monitord_units_jobs_queued systemd jobs queued - Add what a job is ...
# TYPE monitord_units_jobs_queued gauge
monitord_units_jobs_queued 0
# HELP monitord_units_loaded_units Count of loaded units
# TYPE monitord_units_loaded_units gauge
monitord_units_loaded_units 431
# HELP monitord_units_masked_units Count of masked units
# TYPE monitord_units_masked_units gauge
monitord_units_masked_units 3
# HELP monitord_units_mount_units Count of mount units
# TYPE monitord_units_mount_units gauge
monitord_units_mount_units 40
# HELP monitord_units_not_found_units Count of not found units
# TYPE monitord_units_not_found_units gauge
monitord_units_not_found_units 41
# HELP monitord_units_path_units Count of path units
# TYPE monitord_units_path_units gauge
monitord_units_path_units 6
# HELP monitord_units_scope_units Count of scope units
# TYPE monitord_units_scope_units gauge
monitord_units_scope_units 17
# HELP monitord_units_service_units Count of service units
# TYPE monitord_units_service_units gauge
monitord_units_service_units 179
# HELP monitord_units_slice_units Count of slice units
# TYPE monitord_units_slice_units gauge
monitord_units_slice_units 8
# HELP monitord_units_socket_units Count of socket units
# TYPE monitord_units_socket_units gauge
monitord_units_socket_units 24
# HELP monitord_units_target_units Count of target units
# TYPE monitord_units_target_units gauge
monitord_units_target_units 49
# HELP monitord_units_timer_units Count of timer units
# TYPE monitord_units_timer_units gauge
monitord_units_timer_units 17
# HELP monitord_units_total_units Count of total systemd units
# TYPE monitord_units_total_units gauge
monitord_units_total_units 475
# HELP prometheus_exporter_request_duration_seconds The HTTP request latencies in seconds.
# TYPE prometheus_exporter_request_duration_seconds histogram
prometheus_exporter_request_duration_seconds_bucket{le="0.005"} 0
prometheus_exporter_request_duration_seconds_bucket{le="0.01"} 0
prometheus_exporter_request_duration_seconds_bucket{le="0.025"} 3
prometheus_exporter_request_duration_seconds_bucket{le="0.05"} 3
prometheus_exporter_request_duration_seconds_bucket{le="0.1"} 3
prometheus_exporter_request_duration_seconds_bucket{le="0.25"} 3
prometheus_exporter_request_duration_seconds_bucket{le="0.5"} 3
prometheus_exporter_request_duration_seconds_bucket{le="1"} 3
prometheus_exporter_request_duration_seconds_bucket{le="2.5"} 3
prometheus_exporter_request_duration_seconds_bucket{le="5"} 3
prometheus_exporter_request_duration_seconds_bucket{le="10"} 3
prometheus_exporter_request_duration_seconds_bucket{le="+Inf"} 3
prometheus_exporter_request_duration_seconds_sum 0.044447549
prometheus_exporter_request_duration_seconds_count 3
# HELP prometheus_exporter_requests_total Number of HTTP requests received.
# TYPE prometheus_exporter_requests_total counter
prometheus_exporter_requests_total 3
# HELP prometheus_exporter_response_size_bytes The HTTP response sizes in bytes.
# TYPE prometheus_exporter_response_size_bytes gauge
prometheus_exporter_response_size_bytes 9439
```

## Development

To do test runs (requires `systemd` and optionally `systemd-networkd` _installed_)

- `cargo run -- -p 1234 -l debug`
  - `-l` for logging level. Recommend debug when developing
  - `-p` > 1024 to run as non root / with capabilities

Ensure the following pass before submitting a PR (CI checks):

- `cargo test`
- `cargo clippy`
- `cargo fmt`
