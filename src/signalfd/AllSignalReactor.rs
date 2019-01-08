// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// This object forces all signals to be handled using epoll.
#[derive(Debug)]
pub struct AllSignalReactor<SH: SignalHandler>
{
	signal_handler: SH,
	signal_file_descriptor: SignalFileDescriptor,
}

impl<SH: SignalHandler> Reactor for AllSignalReactor<SH>
{
	/// React to events becoming ready.
	///
	/// If an error is returned then all activity is cut short; any dequeued events not yet 'reacted' to are discarded.
	fn react(&mut self, _event_poll: &impl EventPoll, _token: u64, flags: EPollEventFlags) -> Result<(), ()>
	{
		debug_assert_eq!(flags, EPollEventFlags::Input, "flags contained a flag other than `Input`");

		use self::StructReadError::*;

		let mut signals: [signalfd_siginfo; 32] = unsafe { uninitialized() };

		match self.signal_file_descriptor.read(&mut signals)
		{
			Err(WouldBlock) => (),

			Err(Cancelled) => panic!("Signal file descriptor was interrupted"),

			Err(Interrupted) => panic!("EINTR should not occur for read() et al when using a signalfd and blocking all signals on a thread"),

			Ok(signals) => for signal in signals
			{
				signal.handle_signal(&self.signal_handler)?
			},
		}

		Ok(())
	}
}

impl<SH: SignalHandler> AllSignalReactor<SH>
{
	/// Register with epoll.
	///
	/// Starts blocking signals at this point.
	#[inline(always)]
	pub fn register_with_epoll(signal_handler: SH, epoll_file_descriptor: &EPollFileDescriptor, signal_token: u64) -> Result<Self, EPollRegistrationError>
	{
		let signal_mask = Self::new_filled_signal_mask();

		let signal_file_descriptor = SignalFileDescriptor::new(&signal_mask)?;

		Self::block_signals(&signal_mask);

		epoll_file_descriptor.add(signal_file_descriptor.as_raw_fd(), EPollAddFlags::EdgeTriggeredInput, signal_token)?;

		Ok
		(
			Self
			{
				signal_handler,
				signal_file_descriptor,
			}
		)
	}

	#[inline(always)]
	fn new_filled_signal_mask() -> sigset_t
	{
		let mut signal_mask = unsafe { uninitialized() };
		let result = unsafe {  sigfillset(&mut signal_mask) };
		if likely!(result == 0)
		{
			signal_mask
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => panic!("Invalid arguments"),
				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!();
		}
	}

	#[inline(always)]
	fn block_signals(signal_mask: &sigset_t)
	{
		let result = unsafe { pthread_sigmask(SIG_BLOCK, signal_mask, null_mut()) };
		if unlikely!(result != 0)
		{
			match result
			{
				EFAULT => panic!("The `set` or `oldset` argument points outside the process's allocated address space"),
				EINVAL => panic!("Either the value specified in `how` was invalid or the kernel does not support the size passed in `sigsetsize`"),
				_ => unreachable!(),
			}
		}
	}
}
