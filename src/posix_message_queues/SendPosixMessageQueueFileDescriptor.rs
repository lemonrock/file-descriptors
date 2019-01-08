// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a POSIX message queue instance for sending messages.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendPosixMessageQueueFileDescriptor
{
	message_queue_file_descriptor: PosixMessageQueueFileDescriptor,
	maximum_number_of_enqueued_messages: usize,
	maximum_message_size_in_bytes: usize,
}

impl AsRawFd for SendPosixMessageQueueFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.message_queue_file_descriptor.as_raw_fd()
	}
}

impl IntoRawFd for SendPosixMessageQueueFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.message_queue_file_descriptor.into_raw_fd()
	}
}

impl PosixMessageQueue for SendPosixMessageQueueFileDescriptor
{
	#[inline(always)]
	fn new(name: &CStr, open_or_create: &OpenOrCreatePosixMessageQueue) -> Result<Self, CreationError>
	{
		PosixMessageQueueFileDescriptor::new(name, PosixMessageQueueCreateSendOrReceive::Send, open_or_create).map(|(message_queue_file_descriptor, maximum_number_of_enqueued_messages, maximum_message_size_in_bytes)| Self { message_queue_file_descriptor, maximum_number_of_enqueued_messages, maximum_message_size_in_bytes })
	}

	#[inline(always)]
	fn maximum_number_of_enqueued_messages(&self) -> usize
	{
		self.maximum_number_of_enqueued_messages
	}

	#[inline(always)]
	fn maximum_message_size_in_bytes(&self) -> usize
	{
		self.maximum_message_size_in_bytes
	}

	#[inline(always)]
	fn queue_depth(&self) -> usize
	{
		self.message_queue_file_descriptor.queue_depth()
	}
}

impl Send for SendPosixMessageQueueFileDescriptor
{
	#[inline(always)]
	fn send(&self, message_buffer: &[u8], message_priority: PosixMessagePriority) -> Result<(), StructWriteError>
	{
		debug_assert!(message_buffer.len() > self.maximum_message_size_in_bytes(), "message_buffer is too large to send a message");

		self.message_queue_file_descriptor.send(message_buffer, message_priority)
	}
}
