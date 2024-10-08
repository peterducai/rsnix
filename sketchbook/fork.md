GNU Hurd/ hurd/ ng/ trivialconfinementvsconstructorvsfork
Recent Changes Preferences
 Edit History Source ?Discussion
Welcome to...... the GNU Hurd!

Home
Community
Contact Us
Donate
Contributing
Public Hurd Boxen
QEMU Images
Getting Help
Project Ideas
Open Issues
Documentation
FAQ
Hurd
Documentation
Running
Mach
Documentation
GNU Mach
MIG
Documentation
GNU MIG
Debian GNU/Hurd
GNU System
Hurd NG
About this page
This page compares trivial confinement, the constructor mechanism, and POSIX fork(). First there is a short description of the process, then there is a discussion about the differences.

This comparison is about a simple situation: there is a parent process P, which wants to spawn a child process C. We assume that P holds capabilities A and B. The end result should be that C holds a copy of A, but not of B.

Trivial Confinement
For trivial confinement, there is a system call to create a process from some memory pages. P performs the following steps:

Allocate some memory and put the code image of the child into that memory. This can be done by P, or for example by the file system which then gives the resulting memory (space bank) to P.
Perform the system call on that memory. The result is a capability to C.
Send A to C using the returned capability.
Note that it is up to the implementation of the system what happens with P's access to the memory which holds the child. For example, it is probably a good idea if it is at least unmapped, so it cannot accidentily write things in it. It could even be revoked, so that it can't write things in it, even if it wants to.

Constructor
With the constructor mechanism, there are programs running on the system for the purpose of starting new programs. This means that the process is split in two steps. They need not be performed by the same party, in fact they often aren't. So there are two parents, P1 and P2. The first step:

P1 sends a message to the meta-constructor M (a constructor to create new constructors) sending it the code image for the child (and due to the limitation of this discussion, no initial capabilities).
In respose, M creates a C-constructor and passes a capability to it to P1.
The second step:

P2 somehow receives a copy of the capability to the C-constructor.
P2 uses the capability to tell the C-constructor it should build a new process. It provides memory and processor time for the purpose.
The C-constructor uses a method similar to Trivial Confinement to actually create C. It passes the capability C back to P2 as a reply to the capability invocation.
P2 passes A throught the returned capability to C.
This mechanism is targeted at a specific use pattern, namely that a process is created once, and then spawned many times.

POSIX Fork
POSIX fork, or rather fork+exec, is how things are done on many current systems. It may be insightful to see it included in the comparison, especially for people who are new to the subject. There are two system calls, fork and exec. Fork will create a clone of the current process, including all the capabilities (that is, file descriptors) of the parent (except the ones which have explicitly been excluded). Exec is a system call which really goes to the filesystem, not the kernel (although on systems which use it, the filesystem usually resides in the kernel), and asks it to spawn a new process from the contents of a certain path in place of the caller. This passes all capabilities to the new process. The procedure is:

P calls fork(), creating P'.
P' drops B.
P' calls exec(), turning P' into C.
Fork vs the others
Fork is bad. The following text should convince the reader that we do not want to use fork. If it fails to do so, please write your reply to l4-hurd@gnuNOSPAM.org. We can then improve this text, either by using better arguments, or by saying that fork is acceptable after all. :-)

First of all, it must be clear that we are using a capability-based system. This offers a lot of security that POSIX doesn't have. The actual solution presented below doesn really work on POSIX, because C can simply reopen all the files. After all, it is running as the same user, with all the same rights, as P. This is not the case in a capability based system. Every process needs a capability to do things. The closest to the POSIX concept of a "user" is a bunch of capabilities to all objects that the user is allowed to handle. However, even if P has all those capabilities, that doesn't mean C does as well. So for example, if P doesn't give a capability to the user's home directory (and it probably will not do that indeed, but it'll give a private part instead which C can use for files), then C cannot get access to the files in there. And so in particular, if P has a capability to ~/.ssh/id_dsa, and it doesn't give it to C, then C cannot simply call open and get the capability from there, because the file isn't in its file system.

The big difference between fork and the other options is capability B. B is a private capability of P. P does not want it to be passed anywhere. In all cases this is achieved. However, fork needs to be explicit about this. If P (or actually P') forgets to drop B, everything will still work (C didn't use B anyway). However, if C contains a security bug and is taken over by a cracker, then that cracker has access to B. This means that due to a simple mistake, the concequences of a compromised C are bigger than they need to be. This problem is of course even bigger if C is untrusted code in the first place, because it doesn't even need to be "taken over" then, it may simply be malicious.

In contrast, the other two options don't pass anything by default. If there is a similar mistake there, they would forget to pass A to C. That will soon be noticed, because C actually needs A (otherwise it shouldn't receive it). So C will fail to work. This will quickly be fixed, resulting in a better program.

Solving the problem
The problem of fork+exec can be solved. It is if the default would be to not pass capabilities to the new process, but specify a list of capabilities that it should keep, or (like in the other cases) pass them over a new channel which is implicitly created during the fork. However, in that case the only difference with trivial confinement is that P' dies in the process (and thus must be created to prevent P from dying). Almost any use of exec is in practice preceded by a fork for this purpose. It would be easier to make trivial confinement the default operation and let P die directly after it in the rare case that it should.

The only reason for continuing to use fork+exec would be that it is what existing programs do. However, they break anyway if they need to specify which file descriptors to pass. So they need to be adapted. Therefore, it's better to make the usual spawning method the primitive one, and emulate the other.

Trivial Confinement vs Constructor
Note: the following has not been extensively discussed on the mailing list, and no consensus has been reached AFAIK. This is the personal opinion of Bas Wijnen.

The difference between trivial confinement and the constructor is one of control. With trivial confinement, P is in full control of the process (and since P is under full control of its own parent, that parent also fully controls C, and the parent's parent as well, etc. Note that the chain of parents is usually short). For example, if P is a debugger, it could choose to put some breakpoints into C before starting it. With the constructor, this control lies with P1. However, P2 is likely the one who will want to use the debugger. The constructor is explicitly designed to allow this type of control by the programmer (or system administrator) over the user.

In the Hurd we want to enable the user to do these sort of things. We specifically don't want the administrator to use such control. So we do not need to provide the means for it in our system. (Note that not using a constructor doesn't actually guarantee that this kind of control is impossible.)

Except for the control, there is really only one other difference, and that's address space separation. The constructor puts the code for process spawning into its own address space. This means that it cannot be corrupted by broken programs. Extending this principle would mean that every library call should be turned into a server which performs the operation for you. This is however also what trivial confinement does to a large extent anyway.

What it doesn't do is protect the code image against bugs in P. In the constructor the trusted and well-tested constructor code is handling the image, for trivial confinement the (very possibly) buggy program P. In particular, when starting a program from a file system, with trivial confinement the operation is:

Ask the file system for the code, receive a capability to a space bank with a copy (on write) of it.
Make the system call to turn it into a program.
Now this isn't much more complicated than the constructor which does:

Ask the filesystem (which the constructor is part of) to spawn a new process.
Therefore I am not so convinced that we want a constructor. It gets in the way of debugging, for example, and it doesn't really give any gain.

-- ?BasWijnen - 13 Jun 2006

Links: glibc/fork ng
Copyright © 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011, 2012, 2013, 2014, 2015, 2016, 2017, 2018 The Contributing Authors	License: GFDL 1.2+	Last edited 2010-11-30 09:39:28 UTC











------------------------------------------------------

Canonical Process Destruction

Process destruction can be done either cooperatively, or forcibly.
The difference corresponds approximately to the difference between
SIGTERM and SIGKILL in Unix.  To destroy a process cooperatively, a
request message is sent to a special capability implemented by the
child process.  The child can then begin to tear down the program, and
at some time send a request back to the parent process to ask for
forced process destruction.

Forced process destruction can be done by the parent process without
any cooperation by the child process.  The parent process simply
destroys the primary container of the child (this means that the
parent process should retain the primary container capability).

Because container destruction works recursively, forced process
destruction works recursively as well.