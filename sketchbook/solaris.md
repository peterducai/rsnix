Solaris development continued aggressively throughout the 1990s. Several key features distinguish Solaris from earlier UNIX implementations.
• Symmetric multiprocessing — Solaris is implemented on systems ranging from single-processor systems to 64-processor symmetric multiprocessor
servers. Solaris provides linear scalability up to the currently supported maximum of 64 processors.
• 64-bit kernel and process address space — A 64-bit kernel for 64-bit
platforms provides an LP64 execution environment. (LP64 refers to the data
model: long and pointer data types are 64 bits wide.) A 32-bit application
environment is also provided so that 32-bit binaries execute on a 64-bit
Solaris kernel alongside 64-bit applications.
• Multiple platform support — Solaris supports a wide range of SPARC and
Intel x86 microprocessor-based architectures. A layered architecture means
that over 90 percent of the Solaris source is platform independent.
• Modular binary kernel — The Solaris kernel uses dynamic linking and
dynamic modules to divide the kernel into modular binaries. A core kernel
binary contains central facilities; device drivers, file systems, schedulers, and
some system calls are implemented as dynamically loadable modules. Consequently, the Solaris kernel is delivered as a binary rather than source and
object, and kernel compiles are not required upon a change of parameters or
addition of new functionality.
• Multithreaded process execution — A process can have more than one
thread of execution, and these threads can run concurrently on one or more
processors. Thus, a single process can use multiple processors for concurrent
thread execution, thereby using multiprocessor platforms more efficiently.
• Multithreaded kernel — The Solaris kernel uses threads as the entity for
scheduling and execution: The kernel schedules interrupts and kernel services as regular kernel threads. This key feature provides interrupt scalability and low-latency interrupt response.
Previous UNIX implementations manipulated processor priority levels to
ensure exclusive access to critical interrupt data structures. As a result, the
inability of interrupt code to block led to poor scalability. Solaris provides
greater parallelism by scheduling interrupts as threads, which can then use
regular kernel locks to ensure exclusive access to data structures.
• Fully preemptable kernel — The Solaris kernel is fully preemptable and
does not require manipulation of hardware interrupt levels to protect critical
data—locks synchronize access to kernel data. This means threads that need
to run can interrupt another, lower-priority thread; hence, low latency scheduling and low latency interrupt dispatch become possible. For example, a process waking up after sleeping for a disk I/O can be scheduled immediately,
chpt_intro.fm Page 8 Friday, August 25, 2000 1:23 PM
Key Differentiators 9
rather than waiting until the scheduler runs. Additionally, by not raising priority levels and blocking interrupts, the system need not periodically suspend activity during interrupt handling, so system resources are used more
efficiently.
• Support for multiple schedulers — Solaris provides a configurable scheduler environment. Multiple schedulers can operate concurrently, each with its
own scheduling algorithms and priority levels. Schedulers are supplied as
kernel modules and are dynamically loaded into the operating system. Solaris
offers a table-driven, usage-decayed, timesharing user scheduler (TS); a window system optimized timeshare scheduler (IA); and a real-time fixed priority scheduler (RT). An optional fair-share scheduler class can be loaded with
the Solaris Resource Manager package.
• Support for multiple file systems — Solaris provides a virtual file system
(VFS) framework that allows multiple file systems to be configured into the
system. The framework implements several disk-based file systems (UNIX
File System, MS-DOS file system, CD-ROM file system, etc.) and the network file system (NFS V2 and V3). The virtual file system framework also
implements pseudo file systems, including the process file system, procfs, a
file system that abstracts processes as files. The virtual file system framework is integrated with the virtual memory system to provide dynamic file
system caching that uses available free memory as a file system cache.
• Processor partitioning and binding — Special facilities allow fine-grained
processor control, including binding processes to processors. Processors can be
configured into scheduling groups to partition system resources.
• Demand-paged virtual memory system — This feature allows systems to
load applications on demand, rather than loading whole executables or
library images into memory. Demand-paging speeds up application startup
and potentially reduces memory footprint.
• Modular virtual memory system — The virtual memory system separates
virtual memory functions into distinct layers; the address space layer, segment drivers, and hardware-specific components are consolidated into a hardware address translation (HAT) layer. Segment drivers can abstract memory
as files, and files can be memory-mapped into an address space. Segment
drivers enable different abstractions, including physical memory and devices,
to appear in an address space.
• Modular device I/O system — Dynamically loadable device and bus drivers allow a hierarchy of buses and devices to be installed and configured. A
device driver interface (DDI) shields device drivers from platform-specific
infrastructure, thus maximizing portability of device drivers.
• Integrated networking — With the data link provider interface (DLPI),
multiple concurrent network interfaces can be configured, and a variety of
different protocols—including Ethernet, X.25, SDLC, ISDN, FDDI, token bus,
bi-sync, and other datalink-level protocols—can be configured upon them.
chpt_intro.fm Page 9 Friday, August 25, 2000 1:23 PM
10 An Introduction to Solaris
• Integrated Internet protocol — Solaris implements TCP/IP by use of the
DLPI interfaces.
• Real-time architecture — The Solaris kernel was designed and implemented to provide real-time capabilities. The combination of the preemptive
kernel, kernel interrupts as threads, fixed priority scheduling, high-resolution timers, and fine-grained processor control makes Solaris an ideal environment for real-time applications.
The differentiators listed above represent many innovative features integrated in
the Solaris kernel. In the remaining chapters, we closely examine the core modules and major subsystems of the kernel.