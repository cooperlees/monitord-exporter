use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use configparser::ini::Ini;

/// Load a `monitord::config::Config` from an INI-format `monitord.conf` file.
pub fn load_from_file(path: &Path) -> Result<monitord::config::Config> {
    let mut ini = Ini::new();
    ini.load(path)
        .map_err(|e| anyhow::anyhow!("Failed to load config file {:?}: {}", path, e))?;
    monitord::config::Config::try_from(ini)
        .map_err(|e| anyhow::anyhow!("Failed to parse config file {:?}: {}", path, e))
}

/// Build a `monitord::config::Config` from the individual CLI arguments (no config file).
#[allow(clippy::too_many_arguments)]
pub fn build_from_cli(
    dbus_address: &str,
    no_networkd: bool,
    networkd_state_file_path: &Path,
    no_pid1: bool,
    no_system_state: bool,
    services: &[String],
    no_timers: bool,
    timers: &[String],
    no_dbus: bool,
    no_unit_states: bool,
    no_machines: bool,
    boot_blame: bool,
    boot_blame_count: u64,
    no_boot_cache: bool,
    verify: bool,
) -> monitord::config::Config {
    let mut config = monitord::config::Config::default();
    config.monitord.dbus_address = dbus_address.to_string();
    config.networkd.enabled = !no_networkd;
    config.networkd.link_state_dir = PathBuf::from(networkd_state_file_path);
    config.pid1.enabled = !no_pid1;
    config.system_state.enabled = !no_system_state;
    config.services.extend(services.iter().cloned());
    config.timers.enabled = !no_timers;
    config.timers.allowlist.extend(timers.iter().cloned());
    config.dbus_stats.enabled = !no_dbus;
    config.units.state_stats = !no_unit_states;
    config.machines.enabled = !no_machines;
    config.boot_blame.enabled = boot_blame;
    config.boot_blame.num_slowest_units = boot_blame_count;
    config.boot_blame.cache_enabled = !no_boot_cache;
    config.verify.enabled = verify;
    config
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    // Full monitord.conf covering every option that monitord-exporter cares about.
    const FULL_CONFIG: &str = r###"
[monitord]
dbus_address = unix:path=/run/dbus/system_bus_socket
dbus_timeout = 5
daemon = false
daemon_stats_refresh_secs = 60
key_prefix = myprefix
output_format = json

[networkd]
enabled = true
link_state_dir = /run/systemd/netif/links

[pid1]
enabled = true

[services]
ssh.service
docker.service

[system-state]
enabled = true

[timers]
enabled = true

[timers.allowlist]
logrotate.timer

[timers.blocklist]
apt-daily.timer

[units]
enabled = true
state_stats = true
state_stats_time_in_state = true

[units.state_stats.allowlist]
ssh.service

[units.state_stats.blocklist]
snapd.service

[machines]
enabled = false

[dbus]
enabled = true
user_stats = true
peer_stats = false
peer_well_known_names_only = false
cgroup_stats = false

[boot]
enabled = true
num_slowest_units = 10

[boot.allowlist]
network-wait-online.service

[verify]
enabled = true
"###;

    const MINIMAL_CONFIG: &str = r###"
[monitord]
output_format = json
"###;

    fn write_tmp_config(content: &str) -> NamedTempFile {
        let mut f = NamedTempFile::new().expect("Unable to create temp file");
        f.write_all(content.as_bytes())
            .expect("Unable to write temp config");
        f
    }

    // ── load_from_file tests ────────────────────────────────────────────────

    #[test]
    fn test_load_full_config() {
        let tmp = write_tmp_config(FULL_CONFIG);
        let config = load_from_file(tmp.path()).expect("Failed to load full config");

        // [monitord]
        assert_eq!(
            config.monitord.dbus_address,
            "unix:path=/run/dbus/system_bus_socket"
        );
        assert_eq!(config.monitord.dbus_timeout, 5);
        assert!(!config.monitord.daemon);
        assert_eq!(config.monitord.daemon_stats_refresh_secs, 60);
        assert_eq!(config.monitord.key_prefix, "myprefix");

        // [networkd]
        assert!(config.networkd.enabled);
        assert_eq!(
            config.networkd.link_state_dir,
            PathBuf::from("/run/systemd/netif/links")
        );

        // [pid1]
        assert!(config.pid1.enabled);

        // [services]
        assert_eq!(
            config.services,
            HashSet::from(["ssh.service".to_string(), "docker.service".to_string()])
        );

        // [system-state]
        assert!(config.system_state.enabled);

        // [timers]
        assert!(config.timers.enabled);
        assert!(config.timers.allowlist.contains("logrotate.timer"));
        assert!(config.timers.blocklist.contains("apt-daily.timer"));

        // [units]
        assert!(config.units.enabled);
        assert!(config.units.state_stats);
        assert!(config.units.state_stats_time_in_state);
        assert!(config.units.state_stats_allowlist.contains("ssh.service"));
        assert!(config.units.state_stats_blocklist.contains("snapd.service"));

        // [machines]
        assert!(!config.machines.enabled);

        // [dbus]
        assert!(config.dbus_stats.enabled);
        assert!(config.dbus_stats.user_stats);
        assert!(!config.dbus_stats.peer_stats);
        assert!(!config.dbus_stats.cgroup_stats);

        // [boot]
        assert!(config.boot_blame.enabled);
        assert_eq!(config.boot_blame.num_slowest_units, 10);
        assert!(config
            .boot_blame
            .allowlist
            .contains("network-wait-online.service"));

        // [verify]
        assert!(config.verify.enabled);
    }

    #[test]
    fn test_load_minimal_config() {
        let tmp = write_tmp_config(MINIMAL_CONFIG);
        let config = load_from_file(tmp.path()).expect("Failed to load minimal config");

        // Minimal config only has [monitord] section; all section-guarded bools
        // that read_config_bool handles return false when the section is absent.
        assert!(!config.networkd.enabled);
        assert!(config.services.is_empty());
        assert!(config.timers.allowlist.is_empty());
    }

    #[test]
    fn test_load_missing_file_returns_error() {
        let result = load_from_file(Path::new("/nonexistent/path/monitord.conf"));
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("Failed to load config file"),
            "unexpected error message: {msg}"
        );
    }

    #[test]
    fn test_load_invalid_config_returns_error() {
        let invalid = "[monitord]\ndaemon = notabool\noutput_format = json\n";
        let tmp = write_tmp_config(invalid);
        let result = load_from_file(tmp.path());
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("Failed to parse config file"),
            "unexpected error message: {msg}"
        );
    }

    // ── build_from_cli tests ────────────────────────────────────────────────

    #[test]
    fn test_build_from_cli_defaults() {
        let config = build_from_cli(
            "unix:path=/run/dbus/system_bus_socket",
            false, // no_networkd
            Path::new("/run/systemd/netif/links"),
            false, // no_pid1
            false, // no_system_state
            &[],
            false, // no_timers
            &[],
            false, // no_dbus
            false, // no_unit_states
            false, // no_machines
            false, // boot_blame
            5,
            false, // no_boot_cache
            false, // verify
        );

        assert_eq!(
            config.monitord.dbus_address,
            "unix:path=/run/dbus/system_bus_socket"
        );
        assert!(config.networkd.enabled);
        assert_eq!(
            config.networkd.link_state_dir,
            PathBuf::from("/run/systemd/netif/links")
        );
        assert!(config.pid1.enabled);
        assert!(config.system_state.enabled);
        assert!(config.services.is_empty());
        assert!(config.timers.enabled);
        assert!(config.timers.allowlist.is_empty());
        assert!(config.dbus_stats.enabled);
        assert!(config.units.state_stats);
        assert!(config.machines.enabled);
        assert!(!config.boot_blame.enabled);
        assert_eq!(config.boot_blame.num_slowest_units, 5);
        assert!(config.boot_blame.cache_enabled);
        assert!(!config.verify.enabled);
    }

    #[test]
    fn test_build_from_cli_with_options() {
        let config = build_from_cli(
            "unix:path=/custom/bus",
            true, // no_networkd → networkd disabled
            Path::new("/custom/netif"),
            true, // no_pid1
            true, // no_system_state
            &["ssh.service".to_string(), "docker.service".to_string()],
            true, // no_timers
            &["foo.timer".to_string()],
            true, // no_dbus
            true, // no_unit_states
            true, // no_machines
            true, // boot_blame
            15,
            true, // no_boot_cache
            true, // verify
        );

        assert_eq!(config.monitord.dbus_address, "unix:path=/custom/bus");
        assert!(!config.networkd.enabled);
        assert_eq!(
            config.networkd.link_state_dir,
            PathBuf::from("/custom/netif")
        );
        assert!(!config.pid1.enabled);
        assert!(!config.system_state.enabled);
        assert!(config.services.contains("ssh.service"));
        assert!(config.services.contains("docker.service"));
        assert!(!config.timers.enabled);
        assert!(config.timers.allowlist.contains("foo.timer"));
        assert!(!config.dbus_stats.enabled);
        assert!(!config.units.state_stats);
        assert!(!config.machines.enabled);
        assert!(config.boot_blame.enabled);
        assert_eq!(config.boot_blame.num_slowest_units, 15);
        assert!(!config.boot_blame.cache_enabled);
        assert!(config.verify.enabled);
    }
}
