use prometheus_exporter::{self, prometheus::register_gauge_vec, prometheus::GaugeVec};
use tracing::debug;

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
    total_units: GaugeVec,
}

#[derive(Debug)]
pub struct MonitordPromStats {
    networkd: NetworkdStats,
    pid1: Pid1Stats,
    services: ServiceStats,
    units: UnitStats,
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
            total_units: register_gauge_vec!(
                "monitord_units_total_units",
                "Count of total systemd units",
                no_labels,
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
            units: UnitStats::new(),
        }
    }

    /// Parse monitord structs and set prometheus objects
    pub fn populate(&mut self, monitord_stats: &monitord::MonitordStats) {
        debug!("Setting prometheus objects ...");
        let no_labels = &[];

        // networkd stats
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

        // Set pid1 stats
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
            .total_units
            .with_label_values(no_labels)
            .set(monitord_stats.units.total_units as f64);
    }
}

impl Default for MonitordPromStats {
    fn default() -> Self {
        Self::new()
    }
}
