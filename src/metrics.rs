use log::debug;
use prometheus_exporter::{self, prometheus::register_gauge_vec, prometheus::GaugeVec};

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
