// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use super::*;
use self::syscall::*;
use ::libc::c_uint;
use ::libc::c_ulong;
use ::libc::ENAMETOOLONG;
use ::libc::ENXIO;
use ::libc::EOVERFLOW;
use ::libc::EROFS;
use ::libc::ESPIPE;
use ::libc::ETXTBSY;
use ::libc::iovec;
use ::std::ffi::CString;
use ::std::fs::File;
use ::std::io;
use ::std::io::ErrorKind;
use ::std::io::Initializer;
use ::std::io::Read;
use ::std::io::Write;


pub(crate) mod syscall;


include!("ReceivePipeFileDescriptor.rs");
include!("SendPipeFileDescriptor.rs");
include!("SpliceRecipient.rs");
include!("SpliceSender.rs");
