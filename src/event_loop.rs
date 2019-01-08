// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


// TODO Share a file descriptor across threads
// SO_REUSEPORT with SO_INCOMING_CPU
// EPOLLEXCLUSIVE
// setsockopt(http->fd, SOL_SOCKET, SO_REUSEPORT, &val, sizeof(val));

/*

TODO: ARENA(s) and an enum pattern.
	- enum pattern could include an 'arena number'?

Currently 10 (11 if include epoll) top-level variants of file descriptors
- add 1 for 2 pipe variants
- add several for socket variants

We can use bottom 2 bits (32-bit) or 3 bits (64-bit) - not enough variation


Use an arena.
Use relative addressing within the arena.
Use multiple arenas - much more memory efficient.

So an arena per


Data structure in arena should implement Reactor.



*/

// TODO: terminate.


pub fn event_loop(terminate: Terminate, time_out_milliseconds: u16) -> Result<(), EPollCreationError_or_SignalEPollRegistrationError>
{
	let epoll_file_descriptor = EPollFileDescriptor::new()?;

	let signal_reactor = AllSignalReactor::new();
	signal_reactor.register_with_epoll(&epoll_file_descriptor)?;

	let ready_event_handler = |epoll_file_descriptor, token, flags|
	{
		// TODO: Define signal_token; maybe have a scheme where there is a tag in token for each of the various fd kinds.
		if token == signal_token
		{
			signal_reactor.react(epoll_file_descriptor, token, flags)
		}
	};

	let mut events: [epoll_event; 1024] = unsafe { uninitialized() };
	let epoll_time_out = EPollTimeOut::in_n_milliseconds(time_out_milliseconds);
	while terminate.should_continue()
	{
		if let Err(error) = epoll_file_descriptor.wait_until_ready(&mut events, epoll_time_out, ready_event_handler)
		{
			debug_assert_eq!(error, EPollWaitError::Interrupted, "error other than interuppted")
		}
	}

	Ok(())
}
