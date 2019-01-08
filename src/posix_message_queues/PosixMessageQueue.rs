// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a message queue file descriptor for reading, writing or both.
pub trait PosixMessageQueue: AsRawFd + IntoRawFd + Sized
{
	/// Creates a new instance.
	fn new(name: &CStr, open_or_create: &OpenOrCreatePosixMessageQueue) -> Result<Self, CreationError>;

	/// Removes and destroys a queue.
	///
	/// The message queue name `name` is removed immediately.
	/// The queue itself is destroyed once any other processes that have the queue open close their descriptors referring to the queue.
	///
	/// Failure is caused by the queue not existing or by not having permission.
	#[inline(always)]
	fn unlink(name: &CStr) -> Result<(), PosixMessageQueueUnlinkError>
	{
		PosixMessageQueueFileDescriptor::guard_name(name);

		let result = unsafe { mq_unlink(name.as_ptr()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			use self::PosixMessageQueueUnlinkError::*;

			Err
			(
				match errno().0
				{
					EACCES => PermissionDenied,

					ENOENT => DoesNotExist,

					ENAMETOOLONG => panic!("`name` was too long"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!()
		}
	}

	/// The maximum number of enqueued messages.
	///
	/// Will never change; extremely efficient to use.
	fn maximum_number_of_enqueued_messages(&self) -> usize;

	/// The maximum message size in bytes.
	///
	/// Will never change; extremely efficient to use.
	fn maximum_message_size_in_bytes(&self) -> usize;

	/// The number of unread messages in the queue.
	///
	/// Requires a syscall into the kernel.
	fn queue_depth(&self) -> usize;

	/// Is the queue full?
	///
	/// Requires a syscall into the kernel.
	#[inline(always)]
	fn queue_is_full(&self) -> bool
	{
		self.queue_depth() == self.maximum_message_size_in_bytes()
	}

	/// Is the queue empty?
	///
	/// Requires a syscall into the kernel.
	#[inline(always)]
	fn queue_is_empty(&self) -> bool
	{
		self.queue_depth() == 0
	}

	/// How many messages can be enqueued before the queue is full?
	///
	/// Requires a syscall into the kernel.
	#[inline(always)]
	fn remaining_space(&self) -> usize
	{
		self.maximum_number_of_enqueued_messages() - self.queue_depth()
	}
}
