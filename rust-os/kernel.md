On most systems, the kernel is one of the first programs loaded on startup (after the bootloader).

The kernel's interface is a low-level abstraction layer. When a process requests a service from the kernel, it must invoke a system call, usually through a wrapper function.

![Types of kernels](resources/kernels.png "Types of kernels")

### Ram 
Random-access memory (RAM) is used to store both program instructions and data. The kernel is responsible for deciding which memory each process can use, and determining what to do when insufficient memory is available.

### I/O
I/O devices include, but are not limited to, peripherals such as keyboards, mice, display devices, etc. The kernel provides convenient methods for applications to use these devices, which are typically abstracted by the kernel so that applications do not need to know their implementation details.

### Resources
Defining the execution domain (address space) and the protection mechanism used to mediate access to the resources within a domain. Kernels also provide methods for synchronization and inter-process communication (IPC). The kernel is also responsible for context switching between processes or threads.

### Memory
Virtual addressing allows the kernel to make a given physical address appear to be another address, the virtual address.

The memory that one process accesses at a particular (virtual) address may be different memory from what another process accesses at the same address. This allows every program to behave as if it is the only one (apart from the kernel) running and thus prevents applications from crashing each other.

Virtual addressing also allows creation of virtual partitions of memory in two disjoint areas, one being reserved for the kernel (kernel space) and the other for the applications (user space).

### Device
A device driver is a computer program encapsulating, monitoring and controlling a hardware device on behalf of the OS. It provides the operating system with an API, procedures and information about how to control and communicate with a certain piece of hardware. The design goal of a driver is abstraction; the function of the driver is to translate the OS-mandated abstract function calls (programming calls) into device-specific calls.
