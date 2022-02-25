use macos_launchd::launchd::LaunchdPlist;

#[test]
fn launchd_test() {
    let daemon_results = LaunchdPlist::get_launchd_daemons().unwrap();
    assert!(daemon_results.len() > 5);
    let mut label_name = false;
    let mut cf_bundle_name = false;
    for data in daemon_results {
        for (key, value) in data.launchd_data {
            if key == "Label" && value.as_string().unwrap() == "com.apple.wifianalyticsd" {
                label_name = true;
            }
            if key == "CFBundleName" && value.as_string().unwrap() == "fairplayd" {
                cf_bundle_name = true;
            }
        }
    }
    assert!(label_name == true);
    assert!(cf_bundle_name == true);

    let agent_results = LaunchdPlist::get_launchd_agents().unwrap();
    assert!(agent_results.len() > 5);
    let mut no_sandbox = false;
    let mut auxillary_boostrapper = false;

    for data in agent_results {
        for (key, value) in data.launchd_data {
            if key == "com.apple.private.security.no-sandbox" && value.as_boolean().unwrap_or(false)
            {
                no_sandbox = true;
            }
            if key == "AuxilliaryBootstrapper" && value.as_boolean().unwrap_or(false) {
                auxillary_boostrapper = true;
            }
        }
    }

    assert!(no_sandbox == true);
    assert!(auxillary_boostrapper == true);
}
