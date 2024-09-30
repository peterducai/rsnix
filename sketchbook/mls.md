The Multi-Level Security (MLS) technology classifies data in a hierarchical classification using information security levels, for example:

```
[lowest] Unclassified
[low] Confidential
[high] Secret
[highest] Top secret
```

By default, the MLS SELinux policy uses 16 sensitivity levels:

s0 is the least sensitive.
s15 is the most sensitive.
MLS uses specific terminology to address sensitivity levels:

Users and processes are called subjects, whose sensitivity level is called clearance.
Files, devices, and other passive components of the system are called objects, whose sensitivity level is called classification.
To implement MLS, SELinux uses the Bell-La Padula Model (BLP) model. This model specifies how information can flow within the system based on labels attached to each subject and object.

The basic principle of BLP is “No read up, no write down.” This means that users can only read files at their own sensitivity level and lower, and data can flow only from lower levels to higher levels, and never the reverse.

The MLS SELinux policy, which is the implementation of MLS on RHEL, applies a modified principle called Bell-La Padula with write equality. This means that users can read files at their own sensitivity level and lower, but can write only at exactly their own level. This prevents, for example, low-clearance users from writing content into top-secret files.

For example, by default, a user with clearance level s2:

Can read files with sensitivity levels s0, s1, and s2.
Cannot read files with sensitivity level s3 and higher.
Can modify files with sensitivity level of exactly s2.
Cannot modify files with sensitivity level other than s2.
Note
Security administrators can adjust this behavior by modifying the system’s SELinux policy. For example, they can allow users to modify files at lower levels, which increases the file’s sensitivity level to the user’s clearance level.

In practice, users are typically assigned to a range of clearance levels, for example s1-s2. A user can read files with sensitivity levels lower than the user’s maximum level, and write to any files within that range.

For example, by default, a user with a clearance range s1-s2:

Can read files with sensitivity levels s0 and s1.
Cannot read files with sensitivity level s2 and higher.
Can modify files with sensitivity level s1.
Cannot modify files with sensitivity level other than s1.
Can change own clearance level to s2.
The security context for a non-privileged user in an MLS environment is, for example:

user_u:user_r:user_t:s1
Where:

user_u
Is the SELinux user.
user_r
Is the SELinux role.
user_t
Is the SELinux type.
s1
Is the range of MLS sensitivity levels.
The system always combines MLS access rules with conventional file access permissions. For example, if a user with a security level of "Secret" uses Discretionary Access Control (DAC) to block access to a file by other users, even “Top Secret” users cannot access that file. A high security clearance does not automatically permit a user to browse the entire file system.

Users with top-level clearances do not automatically acquire administrative rights on multi-level systems. While they might have access to all sensitive information about the system, this is different from having administrative rights.

In addition, administrative rights do not provide access to sensitive information. For example, even when someone logs in as root, they still cannot read top-secret information.

You can further adjust access within an MLS system by using categories. With Multi-Category Security (MCS), you can define categories such as projects or departments, and users will only be allowed to access files in the categories to which they are assigned. For additional information, see Using Multi-Category Security (MCS) for data confidentiality .

6.2. SELinux roles in MLS
Copy link
The SELinux policy maps each Linux user to an SELinux user. This allows Linux users to inherit the restrictions of SELinux users.

Important
The MLS policy does not contain the unconfined module, including unconfined users, types, and roles. As a result, users that would be unconfined, including root, cannot access every object and perform every action they could in the targeted policy.

You can customize the permissions for confined users in your SELinux policy according to specific needs by adjusting the booleans in policy. You can determine the current state of these booleans by using the semanage boolean -l command. To list all SELinux users, their SELinux roles, and MLS/MCS levels and ranges, use the semanage user -l command as root.


Table 6.1. Roles of SELinux users in MLS
User	Default role	Additional roles
guest_u

guest_r


xguest_u

xguest_r


user_u

user_r


staff_u

staff_r

auditadm_r

secadm_r

sysadm_r

staff_r

sysadm_u

sysadm_r


root

staff_r

auditadm_r

secadm_r

sysadm_r

system_r

system_u

system_r


Note that system_u is a special user identity for system processes and objects, and system_r is the associated role. Administrators must never associate this system_u user and the system_r role to a Linux user. Also, unconfined_u and root are unconfined users. For these reasons, the roles associated to these SELinux users are not included in the following table Types and access of SELinux roles.

Each SELinux role corresponds to an SELinux type and provides specific access rights.


Table 6.2. Types and access of SELinux roles in MLS
Role	Type	Login using X Window System	su and sudo	Execute in home directory and /tmp (default)	Networking
guest_r

guest_t

no

no

yes

no

xguest_r

xguest_t

yes

no

yes

web browsers only (Firefox, GNOME Web)

user_r

user_t

yes

no

yes

yes

staff_r

staff_t

yes

only sudo

yes

yes

auditadm_r

auditadm_t


yes

yes

yes

secadm_r

secadm_t


yes

yes

yes

sysadm_r

sysadm_t

only when the xdm_sysadm_login boolean is on

yes

yes

yes

By default, the sysadm_r role has the rights of the secadm_r role, which means a user with the sysadm_r role can manage the security policy. If this does not correspond to your use case, you can separate the two roles by disabling the sysadm_secadm module in the policy. For additional information, see Separating system administration from security administration in MLS.
Non-login roles dbadm_r, logadm_r, and webadm_r can be used for a subset of administrative tasks. By default, these roles are not associated with any SELinux user.
6.3. Switching the SELinux policy to MLS
Copy link
Use the following steps to switch the SELinux policy from targeted to Multi-Level Security (MLS).

Important
Do not use the MLS policy on a system that is running the X Window System. Furthermore, when you relabel the file system with MLS labels, the system may prevent confined domains from access, which prevents your system from starting correctly. Therefore ensure that you switch SELinux to permissive mode before you relabel the files. On most systems, you see a lot of SELinux denials after switching to MLS, and many of them are not trivial to fix.

Procedure

Install the selinux-policy-mls package:

# yum install selinux-policy-mls
Open the /etc/selinux/config file in a text editor of your choice, for example:

# vi /etc/selinux/config
Change SELinux mode from enforcing to permissive and switch from the targeted policy to MLS:

SELINUX=permissive
SELINUXTYPE=mls
Save the changes, and quit the editor.

Before you enable the MLS policy, you must relabel each file on the file system with an MLS label:

# fixfiles -F onboot
System will relabel on next boot
Restart the system:

# reboot
Check for SELinux denials:

# ausearch -m AVC,USER_AVC,SELINUX_ERR,USER_SELINUX_ERR -ts recent -i
Because the previous command does not cover all scenarios, see Troubleshooting problems related to SELinux for guidance on identifying, analyzing, and fixing SELinux denials.

After you ensure that there are no problems related to SELinux on your system, switch SELinux back to enforcing mode by changing the corresponding option in /etc/selinux/config:

SELINUX=enforcing
Restart the system:

# reboot
Important
If your system does not start or you are not able to log in after you switch to MLS, add the enforcing=0 parameter to your kernel command line. See Changing SELinux modes at boot time for more information.

Also note that in MLS, SSH logins as the root user mapped to the sysadm_r SELinux role differ from logging in as root in staff_r. Before you start your system in MLS for the first time, consider allowing SSH logins as sysadm_r by setting the ssh_sysadm_login SELinux boolean to 1. To enable ssh_sysadm_login later, already in MLS, you must log in as root in staff_r, switch to root in sysadm_r using the newrole -r sysadm_r command, and then set the boolean to 1.

Verification

Verify that SELinux runs in enforcing mode:

# getenforce
Enforcing
Check that the status of SELinux returns the mls value:

# sestatus | grep mls
Loaded policy name:             mls
Additional resources

fixfiles(8), setsebool(8), and ssh_selinux(8) man pages on your system
6.4. Establishing user clearance in MLS
Copy link
After you switch SELinux policy to MLS, you must assign security clearance levels to users by mapping them to confined SELinux users. By default, a user with a given security clearance:

Cannot read objects that have a higher sensitivity level.
Cannot write to objects at a different sensitivity level.
Prerequisites

The SELinux policy is set to mls.
The SELinux mode is set to enforcing.
The policycoreutils-python-utils package is installed.
A user assigned to an SELinux confined user:

For a non-privileged user, assigned to user_u (example_user in the following procedure).
For a privileged user, assigned to staff_u (staff in the following procedure) .
Important
Make sure that the users have been created when the MLS policy was active. Users created in other SELinux policies cannot be used in MLS.

Procedure

Optional: To prevent adding errors to your SELinux policy, switch to the permissive SELinux mode, which facilitates troubleshooting:

# setenforce 0
Note that in permissive mode, SELinux does not enforce the active policy but only logs Access Vector Cache (AVC) messages, which can be then used for troubleshooting and debugging.

Define a clearance range for the staff_u SELinux user. For example, this command sets the clearance range from s1 to s15 with s1 being the default clearance level:

# semanage user -m -L s1 -r s1-s15 staff_u
Generate SELinux file context configuration entries for user home directories:

# genhomedircon
Restore file security contexts to default:

# restorecon -R -F -v /home/
Relabeled /home/staff from staff_u:object_r:user_home_dir_t:s0 to staff_u:object_r:user_home_dir_t:s1
Relabeled /home/staff/.bash_logout from staff_u:object_r:user_home_t:s0 to staff_u:object_r:user_home_t:s1
Relabeled /home/staff/.bash_profile from staff_u:object_r:user_home_t:s0 to staff_u:object_r:user_home_t:s1
Relabeled /home/staff/.bashrc from staff_u:object_r:user_home_t:s0 to staff_u:object_r:user_home_t:s1
Assign a clearance level to the user:

# semanage login -m -r s1 example_user
Where s1 is the clearance level assigned to the user.

Relabel the user’s home directory to the user’s clearance level:

# chcon -R -l s1 /home/example_user
Optional: If you previously switched to the permissive SELinux mode, and after you verify that everything works as expected, switch back to the enforcing SELinux mode:

# setenforce 1
Verification

Verify that the user is mapped to the correct SELinux user and has the correct clearance level assigned:


# semanage login -l
Login Name      SELinux User         MLS/MCS Range        Service
__default__     user_u               s0-s0                *
example_user    user_u               s1                   *
…
Log in as the user within MLS.
Verify that the user’s security level works correctly:

Warning
The files you use for verification should not contain any sensitive information in case the configuration is incorrect and the user actually can access the files without authorization.

Verify that the user cannot read a file with a higher-level sensitivity.
Verify that the user can write to a file with the same sensitivity.
Verify that the user can read a file with a lower-level sensitivity.
Additional resources

Switching the SELinux policy to MLS
Adding a new user as an SELinux-confined user
Permanent changes in SELinux states and modes
Troubleshooting problems related to SELinux
Basic SELinux Troubleshooting in CLI Knowledgebase article
6.5. Changing a user’s clearance level within the defined security range in MLS
Copy link
As a user in Multi-Level Security (MLS), you can change your current clearance level within the range the administrator assigned to you. You can never exceed the upper limit of your range or reduce your level below the lower limit of your range. This allows you, for example, to modify lower-sensitivity files without increasing their sensitivity level to your highest clearance level.

For example, as a user assigned to range s1-s3:

You can switch to levels s1, s2, and s3.
You can switch to ranges s1-s2, and s2-s3.
You cannot switch to ranges s0-s3 or s1-s4.
Switching to a different level opens a new shell with the different clearance. This means you cannot return to your original clearance level in the same way as decreasing it. However, you can always return to the previous shell by entering exit.

Prerequisites

The SELinux policy is set to mls.
SELinux mode is set to enforcing.
You can log in as a user assigned to a range of MLS clearance levels.
Procedure

Log in as the user from a secure terminal.

Secure terminals are defined in the /etc/selinux/mls/contexts/securetty_types file. By default, the console is a secure terminal, but SSH is not.

Check the current user’s security context:

$ id -Z
user_u:user_r:user_t:s0-s2
In this example, the user is assigned to the user_u SELinux user, user_r role, user_t type, and the MLS security range s0-s2.

Check the current user’s security context:

$ id -Z
user_u:user_r:user_t:s1-s2
Switch to a different security clearance range within the user’s clearance range:

$ newrole -l s1
You can switch to any range whose maximum is lower or equal to your assigned range. Entering a single-level range changes the lower limit of the assigned range. For example, entering newrole -l s1 as a user with a s0-s2 range is equivalent to entering newrole -l s1-s2.

Verification

Display the current user’s security context:

$ id -Z
user_u:user_r:user_t:s1-s2
Return to the previous shell with the original range by terminating the current shell:

$ exit
Additional resources

Establishing user clearance in MLS
newrole(1) and securetty_types(5) man pages on your system
6.6. Increasing file sensitivity levels in MLS
Copy link
By default, Multi-Level Security (MLS) users cannot increase file sensitivity levels. However, the security administrator (secadm_r) can change this default behavior to allow users to increase the sensitivity of files by adding the local module mlsfilewrite to the system’s SELinux policy. Then, users assigned to the SELinux type defined in the policy module can increase file classification levels by modifying the file. Any time a user modifies a file, the file’s sensitivity level increases to the lower value of the user’s current security range.

The security administrator, when logged in as a user assigned to the secadm_r role, can change the security levels of files by using the chcon -l s0 /path/to/file command. For more information, see Changing file sensitivity in MLS.

Prerequisites

The SELinux policy is set to mls.
SELinux mode is set to enforcing.
The policycoreutils-python-utils package is installed.
The mlsfilewrite local module is installed in the SELinux MLS policy.
You are logged in as a user in MLS which is:

Assigned to a defined security range. This example shows a user with a security range s0-s2.
Assigned to the same SELinux type defined in the mlsfilewrite module. This example requires the (typeattributeset mlsfilewrite (user_t)) module.
Procedure

Optional: Display the security context of the current user:

$ id -Z
user_u:user_r:user_t:s0-s2
Change the lower level of the user’s MLS clearance range to the level which you want to assign to the file:

$ newrole -l s1-s2
Optional: Display the security context of the current user:

$ id -Z
user_u:user_r:user_t:s1-s2
Optional: Display the security context of the file:

$ ls -Z /path/to/file
user_u:object_r:user_home_t:s0 /path/to/file
Change the file’s sensitivity level to the lower level of the user’s clearance range by modifying the file:

$ touch /path/to/file
Important
The classification level reverts to the default value if the restorecon command is used on the system.

Optional: Exit the shell to return to the user’s previous security range:

$ exit
Verification

Display the security context of the file:

$ ls -Z /path/to/file
user_u:object_r:user_home_t:s1 /path/to/file
Additional resources

Allowing MLS users to edit files on lower levels.
6.7. Changing file sensitivity in MLS
Copy link
In the MLS SELinux policy, users can only modify files at their own sensitivity level. This is intended to prevent any highly sensitive information to be exposed to users at lower clearance levels, and also prevent low-clearance users creating high-sensitivity documents. Administrators, however, can manually increase a file’s classification, for example for the file to be processed at the higher level.

Prerequisites

SELinux policy is set to mls.
SELinux mode is set to enforcing.
You have security administration rights, which means that you are assigned to either:

The secadm_r role.
If the sysadm_secadm module is enabled, to the sysadm_r role. The sysadm_secadm module is enabled by default.
The policycoreutils-python-utils package is installed.
A user assigned to any clearance level. For additional information, see Establishing user clearance levels in MLS .

In this example, User1 has clearance level s1.

A file with a classification level assigned and to which you have access.

In this example, /path/to/file has classification level s1.

Procedure

Check the file’s classification level:

# ls -lZ /path/to/file
-rw-r-----. 1 User1 User1 user_u:object_r:user_home_t:s1 0 12. Feb 10:43 /path/to/file
Change the file’s default classification level:

# semanage fcontext -a -r s2 /path/to/file
Force the relabeling of the file’s SELinux context:

# restorecon -F -v /path/to/file
Relabeled /path/to/file from user_u:object_r:user_home_t:s1 to user_u:object_r:user_home_t:s2
Verification

Check the file’s classification level:

# ls -lZ /path/to/file
-rw-r-----. 1 User1 User1 user_u:object_r:user_home_t:s2 0 12. Feb 10:53 /path/to/file
Optional: Verify that the lower-clearance user cannot read the file:

$ cat /path/to/file
cat: file: Permission denied
Additional resources

Establishing user clearance levels in MLS .
6.8. Separating system administration from security administration in MLS
Copy link
By default, the sysadm_r role has the rights of the secadm_r role, which means a user with the sysadm_r role can manage the security policy. If you need more control over security authorizations, you can separate system administration from security administration by assigning a Linux user to the secadm_r role and disabling the sysadm_secadm module in the SELinux policy.

Prerequisites

The SELinux policy is set to mls.
The SELinux mode is set to enforcing.
The policycoreutils-python-utils package is installed.
A Linux user which will be assigned to the secadm_r role:

The user is assigned to the staff_u SELinux user
A password for this user has been defined.
Warning
Make sure you can log in as the user which will be assigned to the secadm role. If not, you can prevent any future modifications of the system’s SELinux policy.

Procedure

Create a new sudoers file in the /etc/sudoers.d directory for the user:

# visudo -f /etc/sudoers.d/<sec_adm_user>
To keep the sudoers files organized, replace <sec_adm_user> with the Linux user which will be assigned to the secadm role.

Add the following content into the /etc/sudoers.d/<sec_adm_user> file:

<sec_adm_user> ALL=(ALL) TYPE=secadm_t ROLE=secadm_r ALL
This line authorizes <secadmuser> on all hosts to perform all commands, and maps the user to the secadm SELinux type and role by default.

Log in as the <sec_adm_user> user.

To make sure that the SELinux context (which consists of SELinux user, role, and type) is changed, log in using ssh, the console, or xdm. Other ways, such as su and sudo, cannot change the entire SELinux context.

Verify the user’s security context:

$ id
uid=1000(<sec_adm_user>) gid=1000(<sec_adm_user>) groups=1000(<sec_adm_user>) context=staff_u:staff_r:staff_t:s0-s15:c0.c1023
Run the interactive shell for the root user:

$ sudo -i
[sudo] password for <sec_adm_user>:
Verify the current user’s security context:

# id
uid=0(root) gid=0(root) groups=0(root) context=staff_u:secadm_r:secadm_t:s0-s15:c0.c1023
Disable the sysadm_secadm module from the policy:

# semodule -d sysadm_secadm
Important
Use the semodule -d command instead of removing the system policy module by using the semodule -r command. The semodule -r command deletes the module from your system’s storage, which means it cannot be loaded again without reinstalling the selinux-policy-mls package.

Verification

As the user assigned to the secadm role, and in the interactive shell for the root user, verify that you can access the security policy data:

# seinfo -xt secadm_t

Types: 1
type secadm_t, can_relabelto_shadow_passwords, (…) userdomain;
Log out from the root shell:

# logout
Log out from the <sec_adm_user> user:

$ logout
Connection to localhost closed.
Display the current security context:

# id
uid=0(root) gid=0(root) groups=0(root) context=root:sysadm_r:sysadm_t:s0-s15:c0.c1023
Attempt to enable the sysadm_secadm module. The command should fail:

# semodule -e sysadm_secadm
SELinux:  Could not load policy file /etc/selinux/mls/policy/policy.31:  Permission denied
/sbin/load_policy:  Can't load policy:  Permission denied
libsemanage.semanage_reload_policy: load_policy returned error code 2. (No such file or directory).
SELinux:  Could not load policy file /etc/selinux/mls/policy/policy.31:  Permission denied
/sbin/load_policy:  Can't load policy:  Permission denied
libsemanage.semanage_reload_policy: load_policy returned error code 2. (No such file or directory).
semodule:  Failed!
Attempt to display the details about the sysadm_t SELinux type. The command should fail:

# seinfo -xt sysadm_t
[Errno 13] Permission denied: '/sys/fs/selinux/policy'
6.9. Defining a secure terminal in MLS
Copy link
The SELinux policy checks the type of the terminal from which a user is connected, and allows running of certain SELinux applications, for example newrole, only from secure terminals. Attempting this from a non-secure terminal produces an error: Error: you are not allowed to change levels on a non secure terminal;.

The /etc/selinux/mls/contexts/securetty_types file defines secure terminals for the Multi-Level Security (MLS) policy.

Default contents of the file:

console_device_t
sysadm_tty_device_t
user_tty_device_t
staff_tty_device_t
auditadm_tty_device_t
secureadm_tty_device_t
Warning
Adding terminal types to the list of secure terminals can expose your system to security risks.

Prerequisites

SELinux policy is set to mls.
You are connected from an already secure terminal, or SELinux is in permissive mode.
You have security administration rights, which means that you are assigned to either:

The secadm_r role.
If the sysadm_secadm module is enabled, to the sysadm_r role. The sysadm_secadm module is enabled by default.
The policycoreutils-python-utils package is installed.
Procedure

Determine the current terminal type:

# ls -Z `tty`
root:object_r:user_devpts_t:s0 /dev/pts/0
In this example output, user_devpts_t is the current terminal type.

Add the relevant SELinux type on a new line in the /etc/selinux/mls/contexts/securetty_types file.
Optional: Switch SELinux to enforcing mode:

# setenforce 1
Verification

Log in from the previously insecure terminal you have added to the /etc/selinux/mls/contexts/securetty_types file.
Additional resources

securetty_types(5) man page
6.10. Allowing MLS users to edit files on lower levels
Copy link
By default, MLS users cannot write to files which have a sensitivity level below the lower value of the clearance range. If your scenario requires allowing users to edit files on lower levels, you can do so by creating a local SELinux module. However, writing to a file will increase its sensitivity level to the lower value of the user’s current range.

Prerequisites

The SELinux policy is set to mls.
The SELinux mode is set to enforcing.
The policycoreutils-python-utils package is installed.
The setools-console and audit packages for verification.
Procedure

Optional: Switch to permissive mode for easier troubleshooting.

# setenforce 0
Open a new .cil file with a text editor, for example ~/local_mlsfilewrite.cil, and insert the following custom rule:

(typeattributeset mlsfilewrite (_staff_t_))
You can replace staff_t with a different SELinux type. By specifying SELinux type here, you can control which SELinux roles can edit lower-level files.

To keep your local modules better organized, use the local_ prefix in the names of local SELinux policy modules.

Install the policy module:

# semodule -i ~/local_mlsfilewrite.cil
Note
To remove the local policy module, use semodule -r ~/local_mlsfilewrite. Note that you must refer to the module name without the .cil suffix.

Optional: If you previously switched back to permissive mode, return to enforcing mode:

# setenforce 1
Verification

Find the local module in the list of installed SELinux modules:

# semodule -lfull | grep "local_mls"
400 local_mlsfilewrite  cil
Because local modules have priority 400, you can list them also by using the semodule -lfull | grep -v ^100 command.

Log in as a user assigned to the type defined in the custom rule, for example, staff_t.
Attempt to write to a file with a lower sensitivity level. This increases the file’s classification level to the user’s clearance level.

Important
The files you use for verification should not contain any sensitive information in case the configuration is incorrect and the user actually can access the files without authorization.

Previous
Next
Select page format

Multi-page

Back to top
Red Hat logo
Learn
Developer resources
Cloud learning hub
Interactive labs
Training and certification
Customer support
See all documentation
Try, buy, & sell
Product trial center
Red Hat Marketplace
Red Hat Ecosystem Catalog
Red Hat Store
Communities
Customer Portal Community
Events
How we contribute
About Red Hat Documentation
We help Red Hat users innovate and achieve their goals with our products and services with content they can trust.

Making open source more inclusive
Red Hat is committed to replacing problematic language in our code, documentation, and web properties. For more details, see the Red Hat Blog.

About Red Hat
We deliver hardened solutions that make it easier for enterprises to work across platforms and environments, from the core datacenter to the network edge.

About Red Hat
Jobs
Events
Locations
Contact Red Hat
Red Hat Blog
Diversity, equity, and inclusion
Cool Stuff Store
Red Hat Summit
All systems operational
© 2024 Red Hat, Inc.
Privacy statement
Terms of use
All policies and guidelines
Digital accessibility
Cookie preferences
