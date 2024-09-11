XDG Base Directory Specification (XDG BDS) introduces a range of variables where user-specific files used by programs should be found.[14] Many tools and applications utilize these variables by default.[15]

User directories
Besides the variables mentioned below, XDG BDS also specifies that users' local binary files may be installed into $HOME/.local/bin. Systems compliant with the spec are expected to make this directory available in their CLI's PATH environment variable.[14]

XDG_DATA_HOME
For user application's own data files
Default to $HOME/.local/share
XDG_CONFIG_HOME
For user's app configuration files
Default to $HOME/.config
XDG_STATE_HOME
For user-specific app session data, which should be stored for future reuse
Default to $HOME/.local/state
May include logs, recently used files, application-specific information (e.g. window layout, views, opened files, undo history, etc.), akin to session data that should be stored by app by request of system session manager, like X session manager
XDG_CACHE_HOME
For user-specific apps cache files
Default to $HOME/.cache
XDG_RUNTIME_DIR
For user-specific app runtime files like sockets which must not survive reboot and full logout/login cycles
System directories
XDG_DATA_DIRS
Colon-separated list of preference-ordered paths to search for data files in
Default to /usr/local/share/:/usr/share/
XDG_CONFIG_DIRS
The same as above but for config files
Default to /etc/xdg/