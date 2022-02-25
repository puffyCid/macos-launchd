//! Parse macOS launchd data (Daemons and Agents)
//!
//! Provides a library to parse macOS launchd data (Daemons and Agents).

use std::{fs::read_dir, path::Path};

use crate::error::LaunchdError;
use log::warn;
use plist::Dictionary;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LaunchdPlist {
    #[serde(flatten)]
    pub launchd_data: Dictionary,
    pub plist_path: String,
}

impl LaunchdPlist {
    /// Get and parse System and User launchd Daemons
    pub fn get_launchd_daemons() -> Result<Vec<LaunchdPlist>, LaunchdError> {
        let mut plist_files: Vec<String> = Vec::new();
        let user_launchd = LaunchdPlist::user_launchd_daemons();
        match user_launchd {
            Ok(mut launchd_data) => plist_files.append(&mut launchd_data),
            Err(err) => warn!("Failed to get user launchd daemon plist files: {:?}", err),
        }
        let system_launchd = LaunchdPlist::system_launchd_daemons();
        match system_launchd {
            Ok(mut launchd_data) => plist_files.append(&mut launchd_data),
            Err(err) => warn!("Failed to get system launchd daemon plist files: {:?}", err),
        }
        if plist_files.is_empty() {
            return Err(LaunchdError::PathError);
        }

        let mut launchd_plist_vec: Vec<LaunchdPlist> = Vec::new();
        for data in plist_files {
            if !data.ends_with("plist") {
                continue;
            }
            let launchd_results: Result<Dictionary, plist::Error> = plist::from_file(&data);
            match launchd_results {
                Ok(launchd_data_dictionary) => {
                    let launchd_data = LaunchdPlist {
                        launchd_data: launchd_data_dictionary,
                        plist_path: data,
                    };
                    launchd_plist_vec.push(launchd_data)
                }
                Err(err) => warn!("Failed to parse plist file {:?}: {:?}", &data, err),
            }
        }
        if launchd_plist_vec.is_empty() {
            return Err(LaunchdError::PlistParseError);
        }
        Ok(launchd_plist_vec)
    }

    /// Get and parse System and User launchd Agents
    pub fn get_launchd_agents() -> Result<Vec<LaunchdPlist>, LaunchdError> {
        let mut plist_files: Vec<String> = Vec::new();
        let user_launchd = LaunchdPlist::user_launchd_agents();
        match user_launchd {
            Ok(mut launchd_data) => plist_files.append(&mut launchd_data),
            Err(err) => warn!("Failed to get user launchd agent plist files: {:?}", err),
        }

        let system_launchd = LaunchdPlist::system_launchd_agents();
        match system_launchd {
            Ok(mut launchd_data) => plist_files.append(&mut launchd_data),
            Err(err) => warn!("Failed to get system launchd agent plist files: {:?}", err),
        }
        if plist_files.is_empty() {
            return Err(LaunchdError::PathError);
        }

        let mut launchd_plist_vec: Vec<LaunchdPlist> = Vec::new();
        for data in plist_files {
            if !data.ends_with("plist") {
                continue;
            }

            let launchd_results: Result<Dictionary, plist::Error> = plist::from_file(&data);
            match launchd_results {
                Ok(launchd_data_dictionary) => {
                    let launchd_data = LaunchdPlist {
                        launchd_data: launchd_data_dictionary,
                        plist_path: data,
                    };
                    launchd_plist_vec.push(launchd_data)
                }
                Err(err) => warn!("Failed to parse plist file {:?}: {:?}", &data, err),
            }
        }
        if launchd_plist_vec.is_empty() {
            return Err(LaunchdError::PlistParseError);
        }
        Ok(launchd_plist_vec)
    }

    /// Get User Launchd daemons
    fn user_launchd_daemons() -> Result<Vec<String>, std::io::Error> {
        const USER_LAUNCHD_DAEMONS: &str = "/Library/launchdaemons/";
        LaunchdPlist::launchd_data(USER_LAUNCHD_DAEMONS)
    }

    /// Get System Launchd daemons
    fn system_launchd_daemons() -> Result<Vec<String>, std::io::Error> {
        const SYSTEM_LAUNCHD_DAEMONS: [&str; 2] = [
            "/System/Library/launchdaemons/",
            "/Library/Apple/System/Library/launchdaemons/",
        ];
        let mut system_agents: Vec<String> = Vec::new();
        for paths in SYSTEM_LAUNCHD_DAEMONS {
            let mut results = LaunchdPlist::launchd_data(paths)?;
            system_agents.append(&mut results);
        }
        Ok(system_agents)
    }

    /// Get System Launchd Agents
    fn system_launchd_agents() -> Result<Vec<String>, std::io::Error> {
        const SYSTEM_LAUNCHD_AGENTS: [&str; 2] = [
            "/System/Library/LaunchAgents/",
            "/Library/Apple/System/Library/LaunchAgents/",
        ];
        let mut system_agents: Vec<String> = Vec::new();
        for paths in SYSTEM_LAUNCHD_AGENTS {
            let mut results = LaunchdPlist::launchd_data(paths)?;
            system_agents.append(&mut results);
        }
        Ok(system_agents)
    }

    /// Get User launchd Agents
    fn user_launchd_agents() -> Result<Vec<String>, std::io::Error> {
        const USER_LAUNCHD_AGENTS: &str = "/Library/LaunchAgents/";
        let base_directory = "/Users/";
        let agents_path = "/Library/LaunchAgents/";
        let mut agent_plist_files: Vec<String> = Vec::new();
        for dir in read_dir(base_directory)? {
            let entry = dir?;
            let path = format!("{}{}", entry.path().display(), agents_path);
            let full_path = Path::new(&path);
            if !full_path.is_dir() {
                continue;
            }
            let mut plist_files = LaunchdPlist::launchd_data(&full_path.display().to_string())?;
            agent_plist_files.append(&mut plist_files);
        }
        let mut results = LaunchdPlist::launchd_data(USER_LAUNCHD_AGENTS)?;
        agent_plist_files.append(&mut results);
        Ok(agent_plist_files)
    }

    /// Get PLIST files from directory
    fn launchd_data(path: &str) -> Result<Vec<String>, std::io::Error> {
        let dir = read_dir(path)?;
        let mut files: Vec<String> = Vec::new();

        for file_path in dir {
            let data = file_path?;

            files.push(data.path().display().to_string())
        }
        Ok(files)
    }
}

#[cfg(test)]
mod tests {

    use super::LaunchdPlist;

    #[test]
    fn test_get_launchd_daemons() {
        let results = LaunchdPlist::get_launchd_daemons().unwrap();
        assert!(results.len() > 5)
    }

    #[test]
    fn test_get_launchd_agents() {
        let results = LaunchdPlist::get_launchd_agents().unwrap();
        assert!(results.len() > 5)
    }

    #[test]
    #[ignore = "Gets user launchd daemons"]
    fn test_user_launchd_daemons() {
        let results = LaunchdPlist::user_launchd_daemons().unwrap();
        assert!(results.len() > 0)
    }

    #[test]
    fn test_system_launchd_daemons() {
        let results = LaunchdPlist::system_launchd_daemons().unwrap();
        assert!(results.len() > 5)
    }

    #[test]
    fn test_system_launchd_agents() {
        let results = LaunchdPlist::system_launchd_agents().unwrap();
        assert!(results.len() > 5)
    }

    #[test]
    #[ignore = "Gets user launchd agents"]
    fn test_user_launchd_agents() {
        let results = LaunchdPlist::user_launchd_agents().unwrap();
        assert!(results.len() > 0)
    }

    #[test]
    fn test_launchd_data() {
        const SYSTEM_LAUNCHD_AGENTS: &str = "/System/Library/LaunchAgents/";
        let results = LaunchdPlist::launchd_data(SYSTEM_LAUNCHD_AGENTS).unwrap();
        assert!(results.len() > 5)
    }
}
