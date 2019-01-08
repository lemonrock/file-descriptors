// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// A reactor 'reacts' to events becoming ready from an epoll instance.
pub trait Reactor
{
	/// React to events becoming ready.
	///
	/// If an error is returned then all activity is cut short; any dequeued events not yet 'reacted' to are discarded.
	fn react(&mut self, event_poll: &impl EventPoll, token: u64, flags: EPollEventFlags) -> Result<(), ()>;
}
