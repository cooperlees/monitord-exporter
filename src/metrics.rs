use prometheus_exporter::{
    self,
    prometheus::{register_gauge_vec, GaugeVec},
};
use tracing::debug;
use tracing::error;

#[derive(Debug)]
struct NetworkdInterfaceStats {
    address_state: GaugeVec,
    admin_state: GaugeVec,
    carrier_state: GaugeVec,
    ipv4_address_state: GaugeVec,
    ipv6_address_state: GaugeVec,
    oper_state: GaugeVec,
    required_for_online: GaugeVec,
}

#[derive(Debug)]
struct NetworkdStats {
    interfaces: NetworkdInterfaceStats,
    managed_interfaces: GaugeVec,
}

#[derive(Debug)]
struct Pid1Stats {
    pid1_cpu_time_kernel: GaugeVec,
    pid1_cpu_user_kernel: GaugeVec,
    pid1_fd_count: GaugeVec,
    pid1_memeory_usage_bytes: GaugeVec,
    pid1_tasks: GaugeVec,
}

#[derive(Debug)]
struct ServiceStats {
    active_enter_timestamp: GaugeVec,
    active_exit_timestamp: GaugeVec,
    cpuuage_nsec: GaugeVec,
    inactive_exit_timestamp: GaugeVec,
    ioread_bytes: GaugeVec,
    ioread_operations: GaugeVec,
    memory_available: GaugeVec,
    memory_current: GaugeVec,
    nrestarts: GaugeVec,
    processes: GaugeVec,
    restart_usec: GaugeVec,
    state_change_timestamp: GaugeVec,
    status_errno: GaugeVec,
    tasks_current: GaugeVec,
    timeout_clean_usec: GaugeVec,
    watchdog_usec: GaugeVec,
}

#[derive(Debug)]
struct SystemStats {
    system_state: GaugeVec,
}

#[derive(Debug)]
struct UnitStats {
    active_units: GaugeVec,
    automount_units: GaugeVec,
    device_units: GaugeVec,
    failed_units: GaugeVec,
    inactive_units: GaugeVec,
    jobs_queued: GaugeVec,
    loaded_units: GaugeVec,
    masked_units: GaugeVec,
    mount_units: GaugeVec,
    not_found_units: GaugeVec,
    path_units: GaugeVec,
    scope_units: GaugeVec,
    service_units: GaugeVec,
    slice_units: GaugeVec,
    socket_units: GaugeVec,
    target_units: GaugeVec,
    timer_units: GaugeVec,
    timer_persistent_units: GaugeVec,
    timer_remain_after_elapse: GaugeVec,
    total_units: GaugeVec,
}

#[derive(Debug)]
struct TimerPromStats {
    accuracy_usec: GaugeVec,
    fixed_random_delay: GaugeVec,
    last_trigger_usec: GaugeVec,
    last_trigger_usec_monotonic: GaugeVec,
    next_elapse_usec_monotonic: GaugeVec,
    next_elapse_usec_realtime: GaugeVec,
    persistent: GaugeVec,
    randomized_delay_usec: GaugeVec,
    remain_after_elapse: GaugeVec,
    service_unit_last_state_change_usec: GaugeVec,
    service_unit_last_state_change_usec_monotonic: GaugeVec,
}

#[derive(Debug)]
struct UnitStatesPromStats {
    active_state: GaugeVec,
    load_state: GaugeVec,
    unhealthy: GaugeVec,
    time_in_state_usecs: GaugeVec,
}

#[derive(Debug)]
struct DBusPromStats {
    serial: GaugeVec,
    active_connections: GaugeVec,
    incomplete_connections: GaugeVec,
    bus_names: GaugeVec,
    peak_bus_names: GaugeVec,
    peak_bus_names_per_connection: GaugeVec,
    match_rules: GaugeVec,
    peak_match_rules: GaugeVec,
    peak_match_rules_per_connection: GaugeVec,
}

#[derive(Debug)]
struct DBusUserPromStats {
    bytes_cur: GaugeVec,
    bytes_max: GaugeVec,
    fds_cur: GaugeVec,
    fds_max: GaugeVec,
    matches_cur: GaugeVec,
    matches_max: GaugeVec,
    objects_cur: GaugeVec,
    objects_max: GaugeVec,
}

#[derive(Debug)]
struct DBusPeerPromStats {
    name_objects: GaugeVec,
    matches: GaugeVec,
    match_bytes: GaugeVec,
    reply_objects: GaugeVec,
    incoming_bytes: GaugeVec,
    incoming_fds: GaugeVec,
    outgoing_bytes: GaugeVec,
    outgoing_fds: GaugeVec,
}

#[derive(Debug)]
struct VersionPromStats {
    version_major: GaugeVec,
    version_info: GaugeVec,
}

#[derive(Debug)]
struct MachinePromStats {
    system_state: GaugeVec,
    active_units: GaugeVec,
    automount_units: GaugeVec,
    device_units: GaugeVec,
    failed_units: GaugeVec,
    inactive_units: GaugeVec,
    jobs_queued: GaugeVec,
    loaded_units: GaugeVec,
    masked_units: GaugeVec,
    mount_units: GaugeVec,
    not_found_units: GaugeVec,
    path_units: GaugeVec,
    scope_units: GaugeVec,
    service_units: GaugeVec,
    slice_units: GaugeVec,
    socket_units: GaugeVec,
    target_units: GaugeVec,
    timer_units: GaugeVec,
    timer_persistent_units: GaugeVec,
    timer_remain_after_elapse: GaugeVec,
    total_units: GaugeVec,
    pid1_cpu_time_kernel: GaugeVec,
    pid1_cpu_user_kernel: GaugeVec,
    pid1_fd_count: GaugeVec,
    pid1_memory_usage_bytes: GaugeVec,
    pid1_tasks: GaugeVec,
    networkd_managed_interfaces: GaugeVec,
    service_active_enter_timestamp: GaugeVec,
    service_active_exit_timestamp: GaugeVec,
    service_cpuusage_nsec: GaugeVec,
    service_inactive_exit_timestamp: GaugeVec,
    service_ioread_bytes: GaugeVec,
    service_ioread_operations: GaugeVec,
    service_memory_available: GaugeVec,
    service_memory_current: GaugeVec,
    service_nrestarts: GaugeVec,
    service_processes: GaugeVec,
    service_restart_usec: GaugeVec,
    service_state_change_timestamp: GaugeVec,
    service_status_errno: GaugeVec,
    service_tasks_current: GaugeVec,
    service_timeout_clean_usec: GaugeVec,
    service_watchdog_usec: GaugeVec,
}

#[derive(Debug)]
pub struct MonitordPromStats {
    networkd: NetworkdStats,
    pid1: Pid1Stats,
    services: ServiceStats,
    system: SystemStats,
    units: UnitStats,
    timers: TimerPromStats,
    unit_states: UnitStatesPromStats,
    dbus: DBusPromStats,
    dbus_users: DBusUserPromStats,
    dbus_peers: DBusPeerPromStats,
    version: VersionPromStats,
    machines: MachinePromStats,
}

impl NetworkdInterfaceStats {
    pub fn new() -> NetworkdInterfaceStats {
        let labels = vec!["interface_name"];
        NetworkdInterfaceStats {
            address_state: register_gauge_vec!(
                "monitord_networkd_address_state",
                "Protocol independent address states (Need to find a better explanation)",
                &labels,
            ).unwrap(),
            admin_state: register_gauge_vec!(
                "monitord_networkd_admin_state",
                "Is the interface configured to be operational (Double check)",
                &labels,
            ).unwrap(),
            carrier_state: register_gauge_vec!(
                "monitord_networkd_carrier_state",
                "Does the link have physical signal or not",
                &labels,
            ).unwrap(),
            ipv4_address_state: register_gauge_vec!(
                "monitord_networkd_ipv4_address_state",
                "Deprecated IP on the interface operational state",
                &labels,
            ).unwrap(),
            ipv6_address_state: register_gauge_vec!(
                "monitord_networkd_ipv6_address_state",
                "IPv6 on the interface operational state",
                &labels,
            ).unwrap(),
            oper_state: register_gauge_vec!(
                "monitord_networkd_oper_state",
                "Interface overall operational state",
                &labels,
            ).unwrap(),
            required_for_online: register_gauge_vec!(
                "monitord_networkd_required_for_online",
                "Bool state of systemd being configured to wait for this interface to come online before network online target.",
                &labels,
            ).unwrap(),
        }
    }
}

impl NetworkdStats {
    pub fn new() -> NetworkdStats {
        NetworkdStats {
            interfaces: NetworkdInterfaceStats::new(),
            managed_interfaces: register_gauge_vec!(
                "monitord_networkd_managed_interfaces",
                "Count of interfaces networkd manages",
                &[],
            )
            .unwrap(),
        }
    }
}

impl Pid1Stats {
    pub fn new() -> Pid1Stats {
        Pid1Stats {
            pid1_cpu_time_kernel: register_gauge_vec!(
                "monitord_pid1_cpu_time_kernel",
                "CPU time used by PID1",
                &[],
            )
            .unwrap(),
            pid1_cpu_user_kernel: register_gauge_vec!(
                "monitord_pid1_cpu_user_kernel",
                "CPU user space time used by PID1",
                &[],
            )
            .unwrap(),
            pid1_fd_count: register_gauge_vec!(
                "monitord_pid1_fd_count",
                "Open file descriptors for PID1",
                &[],
            )
            .unwrap(),
            pid1_memeory_usage_bytes: register_gauge_vec!(
                "monitord_pid1_memory_usage_bytes",
                "Memory usage in bytes for PID1",
                &[],
            )
            .unwrap(),
            pid1_tasks: register_gauge_vec!(
                "monitord_pid1_tasks",
                "Processes / threads of PID1",
                &[],
            )
            .unwrap(),
        }
    }
}

impl ServiceStats {
    pub fn new() -> ServiceStats {
        let labels = &["service_name"];
        ServiceStats {
            active_enter_timestamp: register_gauge_vec!(
                "monitord_service_active_enter_timestamp",
                "Active enter timestamp",
                labels,
            )
            .unwrap(),
            active_exit_timestamp: register_gauge_vec!(
                "monitord_service_active_exit_timestamp",
                "Active exti timestamp",
                labels,
            )
            .unwrap(),
            cpuuage_nsec: register_gauge_vec!(
                "monitord_service_cpuuage_nsec",
                "CPU usage nano seconds",
                labels,
            )
            .unwrap(),
            inactive_exit_timestamp: register_gauge_vec!(
                "monitord_service_inactive_exit_timestamp",
                "Inactive exit timestamp",
                labels,
            )
            .unwrap(),
            ioread_bytes: register_gauge_vec!(
                "monitord_service_ioread_bytes",
                "IO bytes read",
                labels,
            )
            .unwrap(),
            ioread_operations: register_gauge_vec!(
                "monitord_service_ioread_operations",
                "IO Opertations",
                labels,
            )
            .unwrap(),
            memory_available: register_gauge_vec!(
                "monitord_service_memory_available",
                "Memory available",
                labels,
            )
            .unwrap(),
            memory_current: register_gauge_vec!(
                "monitord_service_memory_current",
                "Memory currently in use",
                labels,
            )
            .unwrap(),
            nrestarts: register_gauge_vec!(
                "monitord_service_nrestarts",
                "Count of automatic restarts of the service",
                labels,
            )
            .unwrap(),
            processes: register_gauge_vec!(
                "monitord_service_processes",
                "Count of processes",
                labels,
            )
            .unwrap(),
            restart_usec: register_gauge_vec!(
                "monitord_service_restart_usec",
                "Restart time in usecs",
                labels,
            )
            .unwrap(),
            state_change_timestamp: register_gauge_vec!(
                "monitord_service_state_chage_timestamp",
                "Last unit state change timestamp",
                labels,
            )
            .unwrap(),
            status_errno: register_gauge_vec!(
                "monitord_service_status_errno",
                "Status error number",
                labels,
            )
            .unwrap(),
            tasks_current: register_gauge_vec!(
                "monitord_service_tasks_current",
                "Tasks current (processes + threads)",
                labels,
            )
            .unwrap(),
            timeout_clean_usec: register_gauge_vec!(
                "monitord_service_timeout_clean_usec",
                "Clean timeout usecs",
                labels,
            )
            .unwrap(),
            watchdog_usec: register_gauge_vec!(
                "monitord_service_watchdog_usec",
                "Watchdog runtime usecs",
                labels,
            )
            .unwrap(),
        }
    }
}

impl SystemStats {
    pub fn new() -> SystemStats {
        let no_labels = &[];
        SystemStats {
            system_state: register_gauge_vec!(
                "monitord_system_state",
                "systemd system state - Refer to monitord enum for meaning",
                no_labels,
            )
            .unwrap(),
        }
    }
}

impl UnitStats {
    pub fn new() -> UnitStats {
        let no_labels = &[];
        UnitStats {
            active_units: register_gauge_vec!(
                "monitord_units_active_units",
                "Count of all active units",
                no_labels,
            )
            .unwrap(),
            automount_units: register_gauge_vec!(
                "monitord_units_automount_units",
                "Count of all automount units",
                no_labels,
            )
            .unwrap(),
            device_units: register_gauge_vec!(
                "monitord_units_device_units",
                "Count of device units",
                no_labels,
            )
            .unwrap(),
            failed_units: register_gauge_vec!(
                "monitord_units_failed_units",
                "Count of failed units - delete or fix",
                no_labels,
            )
            .unwrap(),
            inactive_units: register_gauge_vec!(
                "monitord_units_inactive_units",
                "Count of inactive units",
                no_labels,
            )
            .unwrap(),
            jobs_queued: register_gauge_vec!(
                "monitord_units_jobs_queued",
                "systemd jobs queued - Add what a job is ...",
                no_labels,
            )
            .unwrap(),
            loaded_units: register_gauge_vec!(
                "monitord_units_loaded_units",
                "Count of loaded units",
                no_labels,
            )
            .unwrap(),
            masked_units: register_gauge_vec!(
                "monitord_units_masked_units",
                "Count of masked units",
                no_labels,
            )
            .unwrap(),
            mount_units: register_gauge_vec!(
                "monitord_units_mount_units",
                "Count of mount units",
                no_labels,
            )
            .unwrap(),
            not_found_units: register_gauge_vec!(
                "monitord_units_not_found_units",
                "Count of not found units",
                no_labels,
            )
            .unwrap(),
            path_units: register_gauge_vec!(
                "monitord_units_path_units",
                "Count of path units",
                no_labels,
            )
            .unwrap(),
            scope_units: register_gauge_vec!(
                "monitord_units_scope_units",
                "Count of scope units",
                no_labels,
            )
            .unwrap(),
            service_units: register_gauge_vec!(
                "monitord_units_service_units",
                "Count of service units",
                no_labels,
            )
            .unwrap(),
            slice_units: register_gauge_vec!(
                "monitord_units_slice_units",
                "Count of slice units",
                no_labels,
            )
            .unwrap(),
            socket_units: register_gauge_vec!(
                "monitord_units_socket_units",
                "Count of socket units",
                no_labels,
            )
            .unwrap(),
            target_units: register_gauge_vec!(
                "monitord_units_target_units",
                "Count of target units",
                no_labels,
            )
            .unwrap(),
            timer_units: register_gauge_vec!(
                "monitord_units_timer_units",
                "Count of timer units",
                no_labels,
            )
            .unwrap(),
            timer_persistent_units: register_gauge_vec!(
                "monitord_units_timer_persistent_units",
                "Count of timer units with persistent enabled",
                no_labels,
            )
            .unwrap(),
            timer_remain_after_elapse: register_gauge_vec!(
                "monitord_units_timer_remain_after_elapse",
                "Count of timer units with remain after elapse enabled",
                no_labels,
            )
            .unwrap(),
            total_units: register_gauge_vec!(
                "monitord_units_total_units",
                "Count of total systemd units",
                no_labels,
            )
            .unwrap(),
        }
    }
}

impl TimerPromStats {
    pub fn new() -> TimerPromStats {
        let labels = &["timer_name"];
        TimerPromStats {
            accuracy_usec: register_gauge_vec!(
                "monitord_timer_accuracy_usec",
                "Timer accuracy in microseconds",
                labels,
            )
            .unwrap(),
            fixed_random_delay: register_gauge_vec!(
                "monitord_timer_fixed_random_delay",
                "Whether timer has fixed random delay",
                labels,
            )
            .unwrap(),
            last_trigger_usec: register_gauge_vec!(
                "monitord_timer_last_trigger_usec",
                "Last trigger time in microseconds",
                labels,
            )
            .unwrap(),
            last_trigger_usec_monotonic: register_gauge_vec!(
                "monitord_timer_last_trigger_usec_monotonic",
                "Last trigger time in monotonic microseconds",
                labels,
            )
            .unwrap(),
            next_elapse_usec_monotonic: register_gauge_vec!(
                "monitord_timer_next_elapse_usec_monotonic",
                "Next elapse time in monotonic microseconds",
                labels,
            )
            .unwrap(),
            next_elapse_usec_realtime: register_gauge_vec!(
                "monitord_timer_next_elapse_usec_realtime",
                "Next elapse time in realtime microseconds",
                labels,
            )
            .unwrap(),
            persistent: register_gauge_vec!(
                "monitord_timer_persistent",
                "Whether timer is persistent",
                labels,
            )
            .unwrap(),
            randomized_delay_usec: register_gauge_vec!(
                "monitord_timer_randomized_delay_usec",
                "Randomized delay in microseconds",
                labels,
            )
            .unwrap(),
            remain_after_elapse: register_gauge_vec!(
                "monitord_timer_remain_after_elapse",
                "Whether timer remains after elapse",
                labels,
            )
            .unwrap(),
            service_unit_last_state_change_usec: register_gauge_vec!(
                "monitord_timer_service_unit_last_state_change_usec",
                "Service unit last state change in microseconds",
                labels,
            )
            .unwrap(),
            service_unit_last_state_change_usec_monotonic: register_gauge_vec!(
                "monitord_timer_service_unit_last_state_change_usec_monotonic",
                "Service unit last state change in monotonic microseconds",
                labels,
            )
            .unwrap(),
        }
    }
}

impl UnitStatesPromStats {
    pub fn new() -> UnitStatesPromStats {
        let labels = &["unit_name"];
        UnitStatesPromStats {
            active_state: register_gauge_vec!(
                "monitord_unit_active_state",
                "Unit active state as numeric systemd ActiveState value; see systemd.unit(5) for mapping",
                labels,
            )
            .unwrap(),
            load_state: register_gauge_vec!(
                "monitord_unit_load_state",
                "Unit load state as numeric systemd LoadState value; see systemd.unit(5) for mapping",
                labels,
            )
            .unwrap(),
            unhealthy: register_gauge_vec!(
                "monitord_unit_unhealthy",
                "Whether unit is unhealthy",
                labels,
            )
            .unwrap(),
            time_in_state_usecs: register_gauge_vec!(
                "monitord_unit_time_in_state_usecs",
                "Time in current state in microseconds",
                labels,
            )
            .unwrap(),
        }
    }
}

impl DBusPromStats {
    pub fn new() -> DBusPromStats {
        let no_labels = &[];
        DBusPromStats {
            serial: register_gauge_vec!("monitord_dbus_serial", "D-Bus serial number", no_labels,)
                .unwrap(),
            active_connections: register_gauge_vec!(
                "monitord_dbus_active_connections",
                "D-Bus active connections",
                no_labels,
            )
            .unwrap(),
            incomplete_connections: register_gauge_vec!(
                "monitord_dbus_incomplete_connections",
                "D-Bus incomplete connections",
                no_labels,
            )
            .unwrap(),
            bus_names:
                register_gauge_vec!("monitord_dbus_bus_names", "D-Bus bus names", no_labels,)
                    .unwrap(),
            peak_bus_names: register_gauge_vec!(
                "monitord_dbus_peak_bus_names",
                "D-Bus peak bus names",
                no_labels,
            )
            .unwrap(),
            peak_bus_names_per_connection: register_gauge_vec!(
                "monitord_dbus_peak_bus_names_per_connection",
                "D-Bus peak bus names per connection",
                no_labels,
            )
            .unwrap(),
            match_rules: register_gauge_vec!(
                "monitord_dbus_match_rules",
                "D-Bus match rules",
                no_labels,
            )
            .unwrap(),
            peak_match_rules: register_gauge_vec!(
                "monitord_dbus_peak_match_rules",
                "D-Bus peak match rules",
                no_labels,
            )
            .unwrap(),
            peak_match_rules_per_connection: register_gauge_vec!(
                "monitord_dbus_peak_match_rules_per_connection",
                "D-Bus peak match rules per connection",
                no_labels,
            )
            .unwrap(),
        }
    }
}

impl DBusUserPromStats {
    pub fn new() -> DBusUserPromStats {
        let labels = &["uid"];
        DBusUserPromStats {
            bytes_cur: register_gauge_vec!(
                "monitord_dbus_user_bytes_cur",
                "Current byte quota usage",
                labels,
            )
            .unwrap(),
            bytes_max: register_gauge_vec!(
                "monitord_dbus_user_bytes_max",
                "Max byte quota",
                labels,
            )
            .unwrap(),
            fds_cur: register_gauge_vec!(
                "monitord_dbus_user_fds_cur",
                "Current FD quota usage",
                labels,
            )
            .unwrap(),
            fds_max: register_gauge_vec!("monitord_dbus_user_fds_max", "Max FD quota", labels,)
                .unwrap(),
            matches_cur: register_gauge_vec!(
                "monitord_dbus_user_matches_cur",
                "Current match rule quota usage",
                labels,
            )
            .unwrap(),
            matches_max: register_gauge_vec!(
                "monitord_dbus_user_matches_max",
                "Max match rule quota",
                labels,
            )
            .unwrap(),
            objects_cur: register_gauge_vec!(
                "monitord_dbus_user_objects_cur",
                "Current object quota usage",
                labels,
            )
            .unwrap(),
            objects_max: register_gauge_vec!(
                "monitord_dbus_user_objects_max",
                "Max object quota",
                labels,
            )
            .unwrap(),
        }
    }
}

impl DBusPeerPromStats {
    pub fn new() -> DBusPeerPromStats {
        let labels = &["peer_id"];
        DBusPeerPromStats {
            name_objects: register_gauge_vec!(
                "monitord_dbus_peer_name_objects",
                "D-Bus peer name objects",
                labels,
            )
            .unwrap(),
            matches: register_gauge_vec!(
                "monitord_dbus_peer_matches",
                "D-Bus peer matches",
                labels,
            )
            .unwrap(),
            match_bytes: register_gauge_vec!(
                "monitord_dbus_peer_match_bytes",
                "D-Bus peer match bytes",
                labels,
            )
            .unwrap(),
            reply_objects: register_gauge_vec!(
                "monitord_dbus_peer_reply_objects",
                "D-Bus peer reply objects",
                labels,
            )
            .unwrap(),
            incoming_bytes: register_gauge_vec!(
                "monitord_dbus_peer_incoming_bytes",
                "D-Bus peer incoming bytes",
                labels,
            )
            .unwrap(),
            incoming_fds: register_gauge_vec!(
                "monitord_dbus_peer_incoming_fds",
                "D-Bus peer incoming file descriptors",
                labels,
            )
            .unwrap(),
            outgoing_bytes: register_gauge_vec!(
                "monitord_dbus_peer_outgoing_bytes",
                "D-Bus peer outgoing bytes",
                labels,
            )
            .unwrap(),
            outgoing_fds: register_gauge_vec!(
                "monitord_dbus_peer_outgoing_fds",
                "D-Bus peer outgoing file descriptors",
                labels,
            )
            .unwrap(),
        }
    }
}

impl VersionPromStats {
    pub fn new() -> VersionPromStats {
        VersionPromStats {
            version_major: register_gauge_vec!(
                "monitord_systemd_version_major",
                "Major systemd version number",
                &[],
            )
            .unwrap(),
            version_info: register_gauge_vec!(
                "monitord_systemd_version_info",
                "Systemd version info",
                &["version"],
            )
            .unwrap(),
        }
    }
}

impl MachinePromStats {
    pub fn new() -> MachinePromStats {
        let labels = &["machine_name"];
        let svc_labels = &["machine_name", "service_name"];
        MachinePromStats {
            // This gauge stores the systemd Manager SystemState as a numeric value.
            // See the org.freedesktop.systemd1.Manager.SystemState documentation for
            // the list of possible states and their meanings.
            system_state: register_gauge_vec!(
                "monitord_machine_system_state",
                "Machine systemd system state (numeric; see systemd Manager.SystemState for values)",
                labels,
            )
            .unwrap(),
            active_units: register_gauge_vec!(
                "monitord_machine_units_active_units",
                "Machine count of active units",
                labels,
            )
            .unwrap(),
            automount_units: register_gauge_vec!(
                "monitord_machine_units_automount_units",
                "Machine count of automount units",
                labels,
            )
            .unwrap(),
            device_units: register_gauge_vec!(
                "monitord_machine_units_device_units",
                "Machine count of device units",
                labels,
            )
            .unwrap(),
            failed_units: register_gauge_vec!(
                "monitord_machine_units_failed_units",
                "Machine count of failed units",
                labels,
            )
            .unwrap(),
            inactive_units: register_gauge_vec!(
                "monitord_machine_units_inactive_units",
                "Machine count of inactive units",
                labels,
            )
            .unwrap(),
            jobs_queued: register_gauge_vec!(
                "monitord_machine_units_jobs_queued",
                "Machine systemd jobs queued",
                labels,
            )
            .unwrap(),
            loaded_units: register_gauge_vec!(
                "monitord_machine_units_loaded_units",
                "Machine count of loaded units",
                labels,
            )
            .unwrap(),
            masked_units: register_gauge_vec!(
                "monitord_machine_units_masked_units",
                "Machine count of masked units",
                labels,
            )
            .unwrap(),
            mount_units: register_gauge_vec!(
                "monitord_machine_units_mount_units",
                "Machine count of mount units",
                labels,
            )
            .unwrap(),
            not_found_units: register_gauge_vec!(
                "monitord_machine_units_not_found_units",
                "Machine count of not found units",
                labels,
            )
            .unwrap(),
            path_units: register_gauge_vec!(
                "monitord_machine_units_path_units",
                "Machine count of path units",
                labels,
            )
            .unwrap(),
            scope_units: register_gauge_vec!(
                "monitord_machine_units_scope_units",
                "Machine count of scope units",
                labels,
            )
            .unwrap(),
            service_units: register_gauge_vec!(
                "monitord_machine_units_service_units",
                "Machine count of service units",
                labels,
            )
            .unwrap(),
            slice_units: register_gauge_vec!(
                "monitord_machine_units_slice_units",
                "Machine count of slice units",
                labels,
            )
            .unwrap(),
            socket_units: register_gauge_vec!(
                "monitord_machine_units_socket_units",
                "Machine count of socket units",
                labels,
            )
            .unwrap(),
            target_units: register_gauge_vec!(
                "monitord_machine_units_target_units",
                "Machine count of target units",
                labels,
            )
            .unwrap(),
            timer_units: register_gauge_vec!(
                "monitord_machine_units_timer_units",
                "Machine count of timer units",
                labels,
            )
            .unwrap(),
            timer_persistent_units: register_gauge_vec!(
                "monitord_machine_units_timer_persistent_units",
                "Machine count of persistent timer units",
                labels,
            )
            .unwrap(),
            timer_remain_after_elapse: register_gauge_vec!(
                "monitord_machine_units_timer_remain_after_elapse",
                "Machine count of remain after elapse timer units",
                labels,
            )
            .unwrap(),
            total_units: register_gauge_vec!(
                "monitord_machine_units_total_units",
                "Machine count of total units",
                labels,
            )
            .unwrap(),
            pid1_cpu_time_kernel: register_gauge_vec!(
                "monitord_machine_pid1_cpu_time_kernel",
                "Machine PID1 CPU kernel time",
                labels,
            )
            .unwrap(),
            pid1_cpu_user_kernel: register_gauge_vec!(
                "monitord_machine_pid1_cpu_user_kernel",
                "Machine PID1 CPU user time",
                labels,
            )
            .unwrap(),
            pid1_fd_count: register_gauge_vec!(
                "monitord_machine_pid1_fd_count",
                "Machine PID1 open file descriptors",
                labels,
            )
            .unwrap(),
            pid1_memory_usage_bytes: register_gauge_vec!(
                "monitord_machine_pid1_memory_usage_bytes",
                "Machine PID1 memory usage in bytes",
                labels,
            )
            .unwrap(),
            pid1_tasks: register_gauge_vec!(
                "monitord_machine_pid1_tasks",
                "Machine PID1 tasks",
                labels,
            )
            .unwrap(),
            networkd_managed_interfaces: register_gauge_vec!(
                "monitord_machine_networkd_managed_interfaces",
                "Machine networkd managed interfaces count",
                labels,
            )
            .unwrap(),
            service_active_enter_timestamp: register_gauge_vec!(
                "monitord_machine_service_active_enter_timestamp",
                "Machine service active enter timestamp",
                svc_labels,
            )
            .unwrap(),
            service_active_exit_timestamp: register_gauge_vec!(
                "monitord_machine_service_active_exit_timestamp",
                "Machine service active exit timestamp",
                svc_labels,
            )
            .unwrap(),
            service_cpuusage_nsec: register_gauge_vec!(
                "monitord_machine_service_cpuusage_nsec",
                "Machine service CPU usage in nanoseconds",
                svc_labels,
            )
            .unwrap(),
            service_inactive_exit_timestamp: register_gauge_vec!(
                "monitord_machine_service_inactive_exit_timestamp",
                "Machine service inactive exit timestamp",
                svc_labels,
            )
            .unwrap(),
            service_ioread_bytes: register_gauge_vec!(
                "monitord_machine_service_ioread_bytes",
                "Machine service IO bytes read",
                svc_labels,
            )
            .unwrap(),
            service_ioread_operations: register_gauge_vec!(
                "monitord_machine_service_ioread_operations",
                "Machine service IO operations",
                svc_labels,
            )
            .unwrap(),
            service_memory_available: register_gauge_vec!(
                "monitord_machine_service_memory_available",
                "Machine service memory available",
                svc_labels,
            )
            .unwrap(),
            service_memory_current: register_gauge_vec!(
                "monitord_machine_service_memory_current",
                "Machine service memory current",
                svc_labels,
            )
            .unwrap(),
            service_nrestarts: register_gauge_vec!(
                "monitord_machine_service_nrestarts",
                "Machine service restart count",
                svc_labels,
            )
            .unwrap(),
            service_processes: register_gauge_vec!(
                "monitord_machine_service_processes",
                "Machine service process count",
                svc_labels,
            )
            .unwrap(),
            service_restart_usec: register_gauge_vec!(
                "monitord_machine_service_restart_usec",
                "Machine service restart time in usecs",
                svc_labels,
            )
            .unwrap(),
            service_state_change_timestamp: register_gauge_vec!(
                "monitord_machine_service_state_change_timestamp",
                "Machine service last state change timestamp",
                svc_labels,
            )
            .unwrap(),
            service_status_errno: register_gauge_vec!(
                "monitord_machine_service_status_errno",
                "Machine service status error number",
                svc_labels,
            )
            .unwrap(),
            service_tasks_current: register_gauge_vec!(
                "monitord_machine_service_tasks_current",
                "Machine service tasks current",
                svc_labels,
            )
            .unwrap(),
            service_timeout_clean_usec: register_gauge_vec!(
                "monitord_machine_service_timeout_clean_usec",
                "Machine service clean timeout usecs",
                svc_labels,
            )
            .unwrap(),
            service_watchdog_usec: register_gauge_vec!(
                "monitord_machine_service_watchdog_usec",
                "Machine service watchdog usecs",
                svc_labels,
            )
            .unwrap(),
        }
    }
}

impl MonitordPromStats {
    pub fn new() -> MonitordPromStats {
        MonitordPromStats {
            networkd: NetworkdStats::new(),
            pid1: Pid1Stats::new(),
            services: ServiceStats::new(),
            system: SystemStats::new(),
            units: UnitStats::new(),
            timers: TimerPromStats::new(),
            unit_states: UnitStatesPromStats::new(),
            dbus: DBusPromStats::new(),
            dbus_users: DBusUserPromStats::new(),
            dbus_peers: DBusPeerPromStats::new(),
            version: VersionPromStats::new(),
            machines: MachinePromStats::new(),
        }
    }

    /// Parse monitord structs and set prometheus objects
    pub fn populate(
        &mut self,
        config: &monitord::config::Config,
        monitord_stats: &monitord::MonitordStats,
    ) {
        debug!("Setting prometheus objects ...");
        let no_labels = &[];

        // networkd stats
        if config.networkd.enabled {
            self.networkd
                .managed_interfaces
                .with_label_values(no_labels)
                .set(monitord_stats.networkd.managed_interfaces as f64);

            // networkd stats - set interface stats
            for interface in &monitord_stats.networkd.interfaces_state {
                let labels = &[interface.name.as_str()];
                self.networkd
                    .interfaces
                    .address_state
                    .with_label_values(labels)
                    .set((interface.address_state as i64) as f64);
                self.networkd
                    .interfaces
                    .admin_state
                    .with_label_values(labels)
                    .set((interface.admin_state as i64) as f64);
                self.networkd
                    .interfaces
                    .carrier_state
                    .with_label_values(labels)
                    .set((interface.carrier_state as i64) as f64);
                self.networkd
                    .interfaces
                    .ipv4_address_state
                    .with_label_values(labels)
                    .set((interface.ipv4_address_state as i64) as f64);
                self.networkd
                    .interfaces
                    .ipv6_address_state
                    .with_label_values(labels)
                    .set((interface.ipv6_address_state as i64) as f64);
                self.networkd
                    .interfaces
                    .oper_state
                    .with_label_values(labels)
                    .set((interface.oper_state as i64) as f64);
                self.networkd
                    .interfaces
                    .required_for_online
                    .with_label_values(labels)
                    .set((interface.required_for_online as i64) as f64);
            }
        }

        // Set pid1 stats
        if config.pid1.enabled {
            if let Some(p1s) = &monitord_stats.pid1 {
                self.pid1
                    .pid1_cpu_time_kernel
                    .with_label_values(no_labels)
                    .set(p1s.cpu_time_kernel as f64);
                self.pid1
                    .pid1_cpu_user_kernel
                    .with_label_values(no_labels)
                    .set(p1s.cpu_time_user as f64);
                self.pid1
                    .pid1_fd_count
                    .with_label_values(no_labels)
                    .set(p1s.fd_count as f64);
                self.pid1
                    .pid1_memeory_usage_bytes
                    .with_label_values(no_labels)
                    .set(p1s.memory_usage_bytes as f64);
                self.pid1
                    .pid1_tasks
                    .with_label_values(no_labels)
                    .set(p1s.tasks as f64);
            } else {
                error!("PID 1 stats are enabled but we don't have any set")
            }
        }

        // Set services stats
        for (service_name, service_stats) in monitord_stats.units.service_stats.iter() {
            let service_labels = &[service_name.as_str()];
            self.services
                .active_enter_timestamp
                .with_label_values(service_labels)
                .set((service_stats.active_enter_timestamp as i64) as f64);
            self.services
                .active_exit_timestamp
                .with_label_values(service_labels)
                .set((service_stats.active_exit_timestamp as i64) as f64);
            self.services
                .cpuuage_nsec
                .with_label_values(service_labels)
                .set((service_stats.cpuusage_nsec as i64) as f64);
            self.services
                .inactive_exit_timestamp
                .with_label_values(service_labels)
                .set((service_stats.inactive_exit_timestamp as i64) as f64);
            self.services
                .ioread_bytes
                .with_label_values(service_labels)
                .set((service_stats.ioread_bytes as i64) as f64);
            self.services
                .ioread_operations
                .with_label_values(service_labels)
                .set((service_stats.ioread_operations as i64) as f64);
            self.services
                .memory_available
                .with_label_values(service_labels)
                .set((service_stats.memory_available as i64) as f64);
            self.services
                .memory_current
                .with_label_values(service_labels)
                .set((service_stats.memory_current as i64) as f64);
            self.services
                .nrestarts
                .with_label_values(service_labels)
                .set((service_stats.nrestarts as i64) as f64);
            self.services
                .processes
                .with_label_values(service_labels)
                .set((service_stats.processes as i64) as f64);
            self.services
                .restart_usec
                .with_label_values(service_labels)
                .set((service_stats.restart_usec as i64) as f64);
            self.services
                .state_change_timestamp
                .with_label_values(service_labels)
                .set((service_stats.state_change_timestamp as i64) as f64);
            self.services
                .status_errno
                .with_label_values(service_labels)
                .set((service_stats.status_errno as i64) as f64);
            self.services
                .tasks_current
                .with_label_values(service_labels)
                .set((service_stats.tasks_current as i64) as f64);
            self.services
                .timeout_clean_usec
                .with_label_values(service_labels)
                .set((service_stats.timeout_clean_usec as i64) as f64);
            self.services
                .watchdog_usec
                .with_label_values(service_labels)
                .set((service_stats.watchdog_usec as i64) as f64);
        }

        // Set the system state
        if config.system_state.enabled {
            self.system
                .system_state
                .with_label_values(no_labels)
                .set((monitord_stats.system_state as u64) as f64);
        }

        // Set all the unit stats
        self.units
            .active_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.active_units as f64);
        self.units
            .automount_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.automount_units as f64);
        self.units
            .device_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.device_units as f64);
        self.units
            .failed_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.failed_units as f64);
        self.units
            .inactive_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.inactive_units as f64);
        self.units
            .jobs_queued
            .with_label_values(no_labels)
            .set(monitord_stats.units.jobs_queued as f64);
        self.units
            .loaded_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.loaded_units as f64);
        self.units
            .masked_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.masked_units as f64);
        self.units
            .mount_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.mount_units as f64);
        self.units
            .not_found_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.not_found_units as f64);
        self.units
            .path_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.path_units as f64);
        self.units
            .scope_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.scope_units as f64);
        self.units
            .service_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.service_units as f64);
        self.units
            .slice_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.slice_units as f64);
        self.units
            .socket_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.socket_units as f64);
        self.units
            .target_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.target_units as f64);
        self.units
            .timer_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.timer_units as f64);
        self.units
            .timer_persistent_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.timer_persistent_units as f64);
        self.units
            .timer_remain_after_elapse
            .with_label_values(no_labels)
            .set(monitord_stats.units.timer_remain_after_elapse as f64);
        self.units
            .total_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.total_units as f64);

        // Set timer stats
        if config.timers.enabled {
            for (timer_name, timer_stats) in monitord_stats.units.timer_stats.iter() {
                let labels = &[timer_name.as_str()];
                self.timers
                    .accuracy_usec
                    .with_label_values(labels)
                    .set(timer_stats.accuracy_usec as f64);
                self.timers
                    .fixed_random_delay
                    .with_label_values(labels)
                    .set(timer_stats.fixed_random_delay as u64 as f64);
                self.timers
                    .last_trigger_usec
                    .with_label_values(labels)
                    .set(timer_stats.last_trigger_usec as f64);
                self.timers
                    .last_trigger_usec_monotonic
                    .with_label_values(labels)
                    .set(timer_stats.last_trigger_usec_monotonic as f64);
                self.timers
                    .next_elapse_usec_monotonic
                    .with_label_values(labels)
                    .set(timer_stats.next_elapse_usec_monotonic as f64);
                self.timers
                    .next_elapse_usec_realtime
                    .with_label_values(labels)
                    .set(timer_stats.next_elapse_usec_realtime as f64);
                self.timers
                    .persistent
                    .with_label_values(labels)
                    .set(timer_stats.persistent as u64 as f64);
                self.timers
                    .randomized_delay_usec
                    .with_label_values(labels)
                    .set(timer_stats.randomized_delay_usec as f64);
                self.timers
                    .remain_after_elapse
                    .with_label_values(labels)
                    .set(timer_stats.remain_after_elapse as u64 as f64);
                self.timers
                    .service_unit_last_state_change_usec
                    .with_label_values(labels)
                    .set(timer_stats.service_unit_last_state_change_usec as f64);
                self.timers
                    .service_unit_last_state_change_usec_monotonic
                    .with_label_values(labels)
                    .set(timer_stats.service_unit_last_state_change_usec_monotonic as f64);
            }
        }

        // Set unit states
        if config.units.state_stats {
            for (unit_name, unit_state) in monitord_stats.units.unit_states.iter() {
                let labels = &[unit_name.as_str()];
                self.unit_states
                    .active_state
                    .with_label_values(labels)
                    .set((unit_state.active_state as i64) as f64);
                self.unit_states
                    .load_state
                    .with_label_values(labels)
                    .set((unit_state.load_state as i64) as f64);
                self.unit_states
                    .unhealthy
                    .with_label_values(labels)
                    .set(unit_state.unhealthy as u64 as f64);
                self.unit_states
                    .time_in_state_usecs
                    .with_label_values(labels)
                    .set(unit_state.time_in_state_usecs.unwrap_or(0) as f64);
            }
        }

        // Set D-Bus stats
        if config.dbus_stats.enabled {
            if let Some(dbus) = &monitord_stats.dbus_stats {
                if let Some(v) = dbus.serial {
                    self.dbus.serial.with_label_values(no_labels).set(v as f64);
                }
                if let Some(v) = dbus.active_connections {
                    self.dbus
                        .active_connections
                        .with_label_values(no_labels)
                        .set(v as f64);
                }
                if let Some(v) = dbus.incomplete_connections {
                    self.dbus
                        .incomplete_connections
                        .with_label_values(no_labels)
                        .set(v as f64);
                }
                if let Some(v) = dbus.bus_names {
                    self.dbus
                        .bus_names
                        .with_label_values(no_labels)
                        .set(v as f64);
                }
                if let Some(v) = dbus.peak_bus_names {
                    self.dbus
                        .peak_bus_names
                        .with_label_values(no_labels)
                        .set(v as f64);
                }
                if let Some(v) = dbus.peak_bus_names_per_connection {
                    self.dbus
                        .peak_bus_names_per_connection
                        .with_label_values(no_labels)
                        .set(v as f64);
                }
                if let Some(v) = dbus.match_rules {
                    self.dbus
                        .match_rules
                        .with_label_values(no_labels)
                        .set(v as f64);
                }
                if let Some(v) = dbus.peak_match_rules {
                    self.dbus
                        .peak_match_rules
                        .with_label_values(no_labels)
                        .set(v as f64);
                }
                if let Some(v) = dbus.peak_match_rules_per_connection {
                    self.dbus
                        .peak_match_rules_per_connection
                        .with_label_values(no_labels)
                        .set(v as f64);
                }

                // Set D-Bus user accounting stats
                if let Some(user_accounting) = &dbus.dbus_broker_user_accounting {
                    for (uid, user_stats) in user_accounting.iter() {
                        let uid_str = uid.to_string();
                        let labels = &[uid_str.as_str()];
                        if let Some(bytes) = &user_stats.bytes {
                            self.dbus_users
                                .bytes_cur
                                .with_label_values(labels)
                                .set(bytes.cur as f64);
                            self.dbus_users
                                .bytes_max
                                .with_label_values(labels)
                                .set(bytes.max as f64);
                        }
                        if let Some(fds) = &user_stats.fds {
                            self.dbus_users
                                .fds_cur
                                .with_label_values(labels)
                                .set(fds.cur as f64);
                            self.dbus_users
                                .fds_max
                                .with_label_values(labels)
                                .set(fds.max as f64);
                        }
                        if let Some(matches) = &user_stats.matches {
                            self.dbus_users
                                .matches_cur
                                .with_label_values(labels)
                                .set(matches.cur as f64);
                            self.dbus_users
                                .matches_max
                                .with_label_values(labels)
                                .set(matches.max as f64);
                        }
                        if let Some(objects) = &user_stats.objects {
                            self.dbus_users
                                .objects_cur
                                .with_label_values(labels)
                                .set(objects.cur as f64);
                            self.dbus_users
                                .objects_max
                                .with_label_values(labels)
                                .set(objects.max as f64);
                        }
                    }
                }

                // Set D-Bus peer accounting stats
                if let Some(peer_accounting) = &dbus.dbus_broker_peer_accounting {
                    for (peer_id, peer_stats) in peer_accounting.iter() {
                        let labels = &[peer_id.as_str()];
                        if let Some(v) = peer_stats.name_objects {
                            self.dbus_peers
                                .name_objects
                                .with_label_values(labels)
                                .set(v as f64);
                        }
                        if let Some(v) = peer_stats.matches {
                            self.dbus_peers
                                .matches
                                .with_label_values(labels)
                                .set(v as f64);
                        }
                        if let Some(v) = peer_stats.match_bytes {
                            self.dbus_peers
                                .match_bytes
                                .with_label_values(labels)
                                .set(v as f64);
                        }
                        if let Some(v) = peer_stats.reply_objects {
                            self.dbus_peers
                                .reply_objects
                                .with_label_values(labels)
                                .set(v as f64);
                        }
                        if let Some(v) = peer_stats.incoming_bytes {
                            self.dbus_peers
                                .incoming_bytes
                                .with_label_values(labels)
                                .set(v as f64);
                        }
                        if let Some(v) = peer_stats.incoming_fds {
                            self.dbus_peers
                                .incoming_fds
                                .with_label_values(labels)
                                .set(v as f64);
                        }
                        if let Some(v) = peer_stats.outgoing_bytes {
                            self.dbus_peers
                                .outgoing_bytes
                                .with_label_values(labels)
                                .set(v as f64);
                        }
                        if let Some(v) = peer_stats.outgoing_fds {
                            self.dbus_peers
                                .outgoing_fds
                                .with_label_values(labels)
                                .set(v as f64);
                        }
                    }
                }
            }
        }

        // Set version stats - fields are private, parse from Display
        let version_str = monitord_stats.version.to_string();
        let major: f64 = version_str
            .split('.')
            .next()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0) as f64;
        self.version
            .version_major
            .with_label_values(no_labels)
            .set(major);
        self.version
            .version_info
            .with_label_values(&[&version_str])
            .set(1.0);

        // Set machine stats
        if config.machines.enabled {
            for (machine_name, machine_stats) in monitord_stats.machines.iter() {
                let labels = &[machine_name.as_str()];
                self.machines
                    .system_state
                    .with_label_values(labels)
                    .set((machine_stats.system_state as u64) as f64);
                self.machines
                    .active_units
                    .with_label_values(labels)
                    .set(machine_stats.units.active_units as f64);
                self.machines
                    .automount_units
                    .with_label_values(labels)
                    .set(machine_stats.units.automount_units as f64);
                self.machines
                    .device_units
                    .with_label_values(labels)
                    .set(machine_stats.units.device_units as f64);
                self.machines
                    .failed_units
                    .with_label_values(labels)
                    .set(machine_stats.units.failed_units as f64);
                self.machines
                    .inactive_units
                    .with_label_values(labels)
                    .set(machine_stats.units.inactive_units as f64);
                self.machines
                    .jobs_queued
                    .with_label_values(labels)
                    .set(machine_stats.units.jobs_queued as f64);
                self.machines
                    .loaded_units
                    .with_label_values(labels)
                    .set(machine_stats.units.loaded_units as f64);
                self.machines
                    .masked_units
                    .with_label_values(labels)
                    .set(machine_stats.units.masked_units as f64);
                self.machines
                    .mount_units
                    .with_label_values(labels)
                    .set(machine_stats.units.mount_units as f64);
                self.machines
                    .not_found_units
                    .with_label_values(labels)
                    .set(machine_stats.units.not_found_units as f64);
                self.machines
                    .path_units
                    .with_label_values(labels)
                    .set(machine_stats.units.path_units as f64);
                self.machines
                    .scope_units
                    .with_label_values(labels)
                    .set(machine_stats.units.scope_units as f64);
                self.machines
                    .service_units
                    .with_label_values(labels)
                    .set(machine_stats.units.service_units as f64);
                self.machines
                    .slice_units
                    .with_label_values(labels)
                    .set(machine_stats.units.slice_units as f64);
                self.machines
                    .socket_units
                    .with_label_values(labels)
                    .set(machine_stats.units.socket_units as f64);
                self.machines
                    .target_units
                    .with_label_values(labels)
                    .set(machine_stats.units.target_units as f64);
                self.machines
                    .timer_units
                    .with_label_values(labels)
                    .set(machine_stats.units.timer_units as f64);
                self.machines
                    .timer_persistent_units
                    .with_label_values(labels)
                    .set(machine_stats.units.timer_persistent_units as f64);
                self.machines
                    .timer_remain_after_elapse
                    .with_label_values(labels)
                    .set(machine_stats.units.timer_remain_after_elapse as f64);
                self.machines
                    .total_units
                    .with_label_values(labels)
                    .set(machine_stats.units.total_units as f64);
                self.machines
                    .networkd_managed_interfaces
                    .with_label_values(labels)
                    .set(machine_stats.networkd.managed_interfaces as f64);

                // Machine PID1 stats
                if let Some(p1s) = &machine_stats.pid1 {
                    self.machines
                        .pid1_cpu_time_kernel
                        .with_label_values(labels)
                        .set(p1s.cpu_time_kernel as f64);
                    self.machines
                        .pid1_cpu_user_kernel
                        .with_label_values(labels)
                        .set(p1s.cpu_time_user as f64);
                    self.machines
                        .pid1_fd_count
                        .with_label_values(labels)
                        .set(p1s.fd_count as f64);
                    self.machines
                        .pid1_memory_usage_bytes
                        .with_label_values(labels)
                        .set(p1s.memory_usage_bytes as f64);
                    self.machines
                        .pid1_tasks
                        .with_label_values(labels)
                        .set(p1s.tasks as f64);
                }

                // Machine service stats
                for (svc_name, svc_stats) in machine_stats.units.service_stats.iter() {
                    let svc_labels = &[machine_name.as_str(), svc_name.as_str()];
                    self.machines
                        .service_active_enter_timestamp
                        .with_label_values(svc_labels)
                        .set((svc_stats.active_enter_timestamp as i64) as f64);
                    self.machines
                        .service_active_exit_timestamp
                        .with_label_values(svc_labels)
                        .set((svc_stats.active_exit_timestamp as i64) as f64);
                    self.machines
                        .service_cpuusage_nsec
                        .with_label_values(svc_labels)
                        .set((svc_stats.cpuusage_nsec as i64) as f64);
                    self.machines
                        .service_inactive_exit_timestamp
                        .with_label_values(svc_labels)
                        .set((svc_stats.inactive_exit_timestamp as i64) as f64);
                    self.machines
                        .service_ioread_bytes
                        .with_label_values(svc_labels)
                        .set((svc_stats.ioread_bytes as i64) as f64);
                    self.machines
                        .service_ioread_operations
                        .with_label_values(svc_labels)
                        .set((svc_stats.ioread_operations as i64) as f64);
                    self.machines
                        .service_memory_available
                        .with_label_values(svc_labels)
                        .set((svc_stats.memory_available as i64) as f64);
                    self.machines
                        .service_memory_current
                        .with_label_values(svc_labels)
                        .set((svc_stats.memory_current as i64) as f64);
                    self.machines
                        .service_nrestarts
                        .with_label_values(svc_labels)
                        .set((svc_stats.nrestarts as i64) as f64);
                    self.machines
                        .service_processes
                        .with_label_values(svc_labels)
                        .set((svc_stats.processes as i64) as f64);
                    self.machines
                        .service_restart_usec
                        .with_label_values(svc_labels)
                        .set((svc_stats.restart_usec as i64) as f64);
                    self.machines
                        .service_state_change_timestamp
                        .with_label_values(svc_labels)
                        .set((svc_stats.state_change_timestamp as i64) as f64);
                    self.machines
                        .service_status_errno
                        .with_label_values(svc_labels)
                        .set((svc_stats.status_errno as i64) as f64);
                    self.machines
                        .service_tasks_current
                        .with_label_values(svc_labels)
                        .set((svc_stats.tasks_current as i64) as f64);
                    self.machines
                        .service_timeout_clean_usec
                        .with_label_values(svc_labels)
                        .set((svc_stats.timeout_clean_usec as i64) as f64);
                    self.machines
                        .service_watchdog_usec
                        .with_label_values(svc_labels)
                        .set((svc_stats.watchdog_usec as i64) as f64);
                }
            }
        }
    }
}

impl Default for MonitordPromStats {
    fn default() -> Self {
        Self::new()
    }
}
