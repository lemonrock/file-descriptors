# file-descriptors

[file-descriptors] is a Rust crate wrapping the various kinds of file descriptors with safe abstractions, including IPv4 / IPv6 sockets, Unix domain sockets, epoll, timerfd, signalfd, eventfd, POSIX message queues, pipes, FIFOs, terminals (and serial ports), character devices, inotify, fanotify and Files.

Fully functional on Android and Linux.

Somewhat functional on Fuschia, Illumos (a Solaris fork), uclibc and emscripten.

Support for the BSDs is desired.


## Supported File Descriptors

* character devices.
* epoll.
* eventfd.
* fanotify.
* inotify.
* POSIX message queues (<(https://linux.die.net/man/7/mq_overview>).
* pipes_and_fifos (anonymous and named (FIFO)s), including support for splice, vmsplice and tee.
* signalfd.
* sockets (TCP, UDP and the equivalent over Unix Domain Sockets).
* terminals (serial ports and modems).
* timerfd.


## Unix Domain Sockets


### When using file paths

* Every effort is made to create the socket file path cleanly;
* To make sure all parent folders exist;
* To make sure parent folder permissions are correctly set;
* To remove any stale files;
* To remove socket file paths on drop (close).

The above features may not work correctly after the use of `seccomp` to lock down system calls (particularly the attempt to delete a socket file path on close).


## Pipes

* The use of `splice()`, `vmsplice()` and `tee()` are supported for all file descriptors where possible (including Rust's `::std::fs::File`).
* To be able to use epoll with standard in (`stdin`), use `pipes_and_fifos::ReceivePipeFileDescriptor::standard_in()`.
* To be able to use epoll with standard out (`stdout`), use `pipes_and_fifos::SendPipeFileDescriptor::standard_out()`.
* To be able to use epoll with standard error (`stderr`), use `pipes_and_fifos::SendPipeFileDescriptor::standard_error()`.


## Unsupported for now

* Linux zero copy send (`MSG_ZEROCOPY`) and receive (`SO_ZEROCOPY`), mostly because they have a horrible, hacky API.
* `SO_BUSY_POLL` and `SO_INCOMING_CPU`.
* Unix Domain Sockets using `autobind`; setting of the `SO_PASSCRED` socket option.
* Receiving credentials over Unix Domain Sockets using `recvmsg()`.
* `mkfifo()`.
* `mknod()`.
* infiniband sockets.
* canbus (SocketCAN sockets and can4linux <http://can-wiki.info/can4linux/man/can4linux_8h_source.html> character device drivers).


## Licensing

The license for this project is MIT.

[file-descriptors]: https://github.com/lemonrock/file-descriptors "file-descriptors GitHub page"
