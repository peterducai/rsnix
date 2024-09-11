# VISION

## Bits

64bit or 128bit? let's step into the future with 128bit..

* Universally Unique Identifiers (UUID) consist of a 128-bit value.
* IPv6 routes computer network traffic amongst a 128-bit range of addresses.
* ZFS is a 128-bit file system.

## Ideas from other OS

### System

* Live update
* Atomic system update using A/B partitions (similar to Android or ChromeOS)
* Fallback system version available in case of unexpected failure or bug
* Supports updating both the system and other environments
* Live migration
* multi version modular pkgs (aka HURD and Bolton Fedora)

### Security

* Role-based access control (RBAC) with domain support for multi-tenant environments

### HA

* Built-in clustering capabilities to simplify high availability


## Other ideas

* event driven FS watch to notify apps (kind of dtrace for user)

### OpenBSD

* Address space randomization (aka ASLR) no fixed jump targets or gaps
* W^X memory can be writable XOR executable
* Guard pages 'fence-like' unreadable, unwritable page after malloc()ed chunks, detect overruns
* Privilege separation daemons run bulk of their code as different non-privileged users (most in chroot without shell) - sshd was the first, the rest followed
* chroot jail -- daemons run in restricted environment ($HOME /var/empty, no shell)
* ProPolice random stack gap inserted, fixed returns fail
* OpenBSD 5.9 introduced pledge(2) syscall to restrict program behavior to predeclared profile
* OpenBSD 6.2 introduced KARL (kernel address randomized link) - kernel relinked with randomized layout for each boot, see the undeadly.org article or the tech@ message (kernel object files grew base by ~300MB)
* OpenBSD 6.4 introduced unveil(2) syscall to restrict file system access to predeclared profile
* Privilege revocation privsep'd daemons drop privilege as soon as possible

### Linux

* Selinux

### Solaris

* RBAC
* Bootadm
* ZFS

**backwards ABI compatibility**

```
Specific Criteria for Source Compatibility:
1) General Source Code Guidelines for Eligible Applications:
• must be written to comply with the standards as listed in standards(5);
• must not be converting 32 bit applications to 64 bit applications or vice versa;
• must not include Makefiles for building object files from compiled applications to run on
specific platforms;
• must not include 3rd party software libraries (binaries) or other programming interfaces;
• must not include Assembler code in any form.
2) Compiler Guidelines for Eligible Applications:
• must be C and C++ applications;
• must have been compiled with the same compiler and OS version on both the Originating
Platform and the Target Platform;
• must have been compiled using a supported Oracle Solaris Studio compiler on a
supported OS release. 
```

### Package manager

should

* Download or processing can be set to run in parallel.
* Can use multiple versions of the same package
* Support for multiple environments
* Supports multi-user package management: multiple users can share a common Nix store securely, don’t need to have root privileges to install software, and can install and use different versions of a package.

* simple repo server (git-like
* versionning
* modular.. posibility to integrate other pkgs


### Pkg security
```
If a CRITICAL or IMPORTANT security issue is currently open against a package, or a security issue of lower severity has been open for at least 6 months, four weeks before the branch point a procedure similar to long-standing FTBFS will be triggered immediately, with 8 weeks of weekly notifications to maintainers and subsequent orphaning and then subsequent removal from distribution.
```
