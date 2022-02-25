# macos-launchd
A simple macOS launchd parser (and library) written in Rust!  
[launchd](https://en.wikipedia.org/wiki/Launchd) is a service management daemon for macOS.  
This library/parser focuses on parsing common persistence mechanisms using launchd. Specifically:  
* LaunchAgents
* LaunchDaemons

# Use Case
Parsing LaunchAgents and LaunchDaemons is mainly useful for forensic investigations.  You can parse both artifacts to identify possible persistence locations.

# LaunchAgents/LaunchDaemons Data
Both LaunchAgents/LaunchDaemons are stored in PLIST files in a variety locations such as:
* `/System/Library/LaunchDaemons/`
* `/Library/launchdaemons/`
* `/Library/Apple/System/Library/LaunchDaemons/`
* `/Users/<USER>/Library/LaunchAgents/`
* `/System/Library/LaunchAgents/`
* `/Library/Apple/System/Library/LaunchAgents/`

Both LaunchAgents/LaunchDaemons contain similar/same data. However, many features are optional. LaunchAgents/LaunchDaemons only have two required features. Some data includes:
* Label (Required according to Apple. However, not not all LaunchAgents/LaunchDaemons have a label)
* ProgramArguments (Required according to Apple. However, not not all LaunchAgents/LaunchDaemons have a label)  

Some potential optional features:
* Program Path
* EnvironmentVariables
* LaunchEvents
* EnableTransactions

# References
https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/CreatingLaunchdJobs.html  
http://technologeeks.com/docs/launchd.pdf  
https://www.sentinelone.com/blog/how-malware-persists-on-macos/  
man launchd.plist
