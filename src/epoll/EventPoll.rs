// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Abstracts behaviour that a `Reactor` should be able to access on an Event Poll (epoll) instance.
pub trait EventPoll
{
	/// Adds a file descriptor to an Event Poll (epoll) instance.
	#[inline(always)]
	fn add(&self, fd: RawFd, flags: EPollAddFlags, token: u64) -> Result<(), EPollAddError>;

	/// Modifies a file descriptor in an Event Poll (epoll) instance.
	#[inline(always)]
	fn modify(&self, fd: RawFd, flags: EPollModifyFlags, token: u64) -> Result<(), EPollModifyError>;

	/// Deletes a file descriptor in an Event Poll (epoll) instance.
	#[inline(always)]
	fn delete(&self, fd: RawFd) -> Result<(), EPollDeleteError>;
}
