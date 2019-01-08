// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can occur during registration with epoll.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EPollRegistrationError
{
	/// Error on creation.
	Creation(CreationError),

	/// Error during registration.
	Registration(EPollAddError),
}

impl Display for EPollRegistrationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<EPollRegistrationError as Debug>::fmt(self, f)
	}
}

impl error::Error for EPollRegistrationError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(error::Error + 'static)>
	{
		use self::EPollRegistrationError::*;

		match self
			{
				&Creation(ref error) => Some(error),
				&Registration(ref error) => Some(error),
			}
	}
}

impl From<CreationError> for EPollRegistrationError
{
	#[inline(always)]
	fn from(error: CreationError) -> Self
	{
		EPollRegistrationError::Creation(error)
	}
}

impl From<EPollAddError> for EPollRegistrationError
{
	#[inline(always)]
	fn from(error: EPollAddError) -> Self
	{
		EPollRegistrationError::Registration(error)
	}
}
