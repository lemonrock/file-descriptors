// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


cfg_if!
{
	if #[cfg(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc64")))]
	{
		#[allow(dead_code)]
		pub(crate) const SO_DEBUG: c_int = 1;

		pub(crate) const SO_REUSEADDR: c_int = 0x0004;

		pub(crate) const SO_KEEPALIVE: c_int = 0x0008;

		#[allow(dead_code)]
		pub(crate) const SO_DONTROUTE: c_int = 0x0010;

		pub(crate) const SO_BROADCAST: c_int = 0x0020;

		pub(crate) const SO_LINGER: c_int = 0x0080;

		pub(crate) const SO_OOBINLINE: c_int = 0x0100;

		pub(crate) const SO_REUSEPORT: c_int = 0x0200;

		pub(crate) const SO_SNDBUF: c_int = 0x1001;

		pub(crate) const SO_RCVBUF: c_int = 0x1002;

		#[allow(dead_code)]
		pub(crate) const SO_SNDLOWAT: c_int = 0x1003;

		#[allow(dead_code)]
		pub(crate) const SO_RCVLOWAT: c_int = 0x1004;

		#[allow(dead_code)]
		pub(crate) const SO_RCVTIMEO: c_int = 0x1006;

		#[allow(dead_code)]
		pub(crate) const SO_SNDTIMEO: c_int = 0x1005;

		#[allow(dead_code)]
		pub(crate) const SO_ERROR: c_int = 0x1007;

		#[allow(dead_code)]
		pub(crate) const SO_TYPE: c_int = 0x1008;

		#[allow(dead_code)]
		pub(crate) const SO_ACCEPTCONN: c_int = 0x1009;

		#[allow(dead_code)]
		pub(crate) const SO_PROTOCOL: c_int = 0x1028;

		#[allow(dead_code)]
		pub(crate) const SO_DOMAIN: c_int = 0x1029;

		#[allow(dead_code)]
		pub(crate) const SO_NO_CHECK: c_int = 11;

		#[allow(dead_code)]
		pub(crate) const SO_PRIORITY: c_int = 12;

		#[allow(dead_code)]
		pub(crate) const SO_BSDCOMPAT: c_int = 14;

		#[allow(dead_code)]
		pub(crate) const SO_PASSCRED: c_int = 17;

		#[allow(dead_code)]
		pub(crate) const SO_PEERCRED: c_int = 18;

		#[allow(dead_code)]
		pub(crate) const SO_PEERSEC: c_int = 30;

		#[allow(dead_code)]
		pub(crate) const SO_SNDBUFFORCE: c_int = 31;

		#[allow(dead_code)]
		pub(crate) const SO_RCVBUFFORCE: c_int = 33;
	}
	else if #[cfg(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "powerpc", target_arch = "powerpc64")))]
	{
		#[allow(dead_code)]
		pub(crate) const SO_DEBUG: c_int = 1;

		pub(crate) const SO_REUSEADDR: c_int = 2;

		pub(crate) const SO_TYPE: c_int = 3;

		pub(crate) const SO_ERROR: c_int = 4;

		#[allow(dead_code)]
		pub(crate) const SO_DONTROUTE: c_int = 5;

		pub(crate) const SO_BROADCAST: c_int = 6;

		pub(crate) const SO_SNDBUF: c_int = 7;

		pub(crate) const SO_RCVBUF: c_int = 8;

		pub(crate) const SO_KEEPALIVE: c_int = 9;

		pub(crate) const SO_OOBINLINE: c_int = 10;

		#[allow(dead_code)]
		pub(crate) const SO_NO_CHECK: c_int = 11;

		#[allow(dead_code)]
		pub(crate) const SO_PRIORITY: c_int = 12;

		pub(crate) const SO_LINGER: c_int = 13;

		#[allow(dead_code)]
		pub(crate) const SO_BSDCOMPAT: c_int = 14;

		pub(crate) const SO_REUSEPORT: c_int = 15;


		// For some odd reason, these values differ for the PowerPC architecture from the generic values (below).

		#[allow(dead_code)]
		pub(crate) const SO_RCVLOWAT: c_int = 16;

		#[allow(dead_code)]
		pub(crate) const SO_SNDLOWAT: c_int = 17;

		#[allow(dead_code)]
		pub(crate) const SO_RCVTIMEO: c_int = 18;

		#[allow(dead_code)]
		pub(crate) const SO_SNDTIMEO: c_int = 19;

		#[allow(dead_code)]
		pub(crate) const SO_PASSCRED: c_int = 20;

		#[allow(dead_code)]
		pub(crate) const SO_PEERCRED: c_int = 21;


		#[allow(dead_code)]
		pub(crate) const SO_ACCEPTCONN: c_int = 30;

		#[allow(dead_code)]
		pub(crate) const SO_PEERSEC: c_int = 31;

		#[allow(dead_code)]
		pub(crate) const SO_SNDBUFFORCE: c_int = 32;

		#[allow(dead_code)]
		pub(crate) const SO_RCVBUFFORCE: c_int = 33;

		#[allow(dead_code)]
		pub(crate) const SO_PROTOCOL: c_int = 38;

		#[allow(dead_code)]
		pub(crate) const SO_DOMAIN: c_int = 39;
	}
	else
	{
		#[allow(dead_code)]
		pub(crate) const SO_DEBUG: c_int = 1;

		pub(crate) const SO_REUSEADDR: c_int = 2;

		#[allow(dead_code)]
		pub(crate) const SO_TYPE: c_int = 3;

		#[allow(dead_code)]
		pub(crate) const SO_ERROR: c_int = 4;

		#[allow(dead_code)]
		pub(crate) const SO_DONTROUTE: c_int = 5;

		pub(crate) const SO_BROADCAST: c_int = 6;

		pub(crate) const SO_SNDBUF: c_int = 7;

		pub(crate) const SO_RCVBUF: c_int = 8;

		pub(crate) const SO_KEEPALIVE: c_int = 9;

		pub(crate) const SO_OOBINLINE: c_int = 10;

		#[allow(dead_code)]
		pub(crate) const SO_NO_CHECK: c_int = 11;

		#[allow(dead_code)]
		pub(crate) const SO_PRIORITY: c_int = 12;

		pub(crate) const SO_LINGER: c_int = 13;

		#[allow(dead_code)]
		pub(crate) const SO_BSDCOMPAT: c_int = 14;

		pub(crate) const SO_REUSEPORT: c_int = 15;


		#[allow(dead_code)]
		pub(crate) const SO_PASSCRED: c_int = 16;

		#[allow(dead_code)]
		pub(crate) const SO_PEERCRED: c_int = 17;

		#[allow(dead_code)]
		pub(crate) const SO_RCVLOWAT: c_int = 18;

		#[allow(dead_code)]
		pub(crate) const SO_SNDLOWAT: c_int = 19;

		#[allow(dead_code)]
		pub(crate) const SO_RCVTIMEO: c_int = 20;

		#[allow(dead_code)]
		pub(crate) const SO_SNDTIMEO: c_int = 21;


		#[allow(dead_code)]
		pub(crate) const SO_ACCEPTCONN: c_int = 30;

		#[allow(dead_code)]
		pub(crate) const SO_PEERSEC: c_int = 31;

		#[allow(dead_code)]
		pub(crate) const SO_SNDBUFFORCE: c_int = 32;

		#[allow(dead_code)]
		pub(crate) const SO_RCVBUFFORCE: c_int = 33;

		#[allow(dead_code)]
		pub(crate) const SO_PROTOCOL: c_int = 38;

		#[allow(dead_code)]
		pub(crate) const SO_DOMAIN: c_int = 39;
	}
}

#[allow(dead_code)]
pub(crate) const SO_SECURITY_AUTHENTICATION: c_int = 22;

#[allow(dead_code)]
pub(crate) const SO_SECURITY_ENCRYPTION_TRANSPORT: c_int = 23;

#[allow(dead_code)]
pub(crate) const SO_SECURITY_ENCRYPTION_NETWORK: c_int = 24;

pub(crate) const SO_BINDTODEVICE: c_int = 25;

#[allow(dead_code)]
pub(crate) const SO_ATTACH_FILTER: c_int = 26;

#[allow(dead_code)]
pub(crate) const SO_DETACH_FILTER: c_int = 27;

#[allow(dead_code)]
pub(crate) const SO_GET_FILTER: c_int = SO_ATTACH_FILTER;

#[allow(dead_code)]
pub(crate) const SO_PEERNAME: c_int = 28;

#[allow(dead_code)]
pub(crate) const SO_TIMESTAMP: c_int = 29;

#[allow(dead_code)]
pub(crate) const SO_PASSSEC: c_int = 34;

#[allow(dead_code)]
pub(crate) const SO_TIMESTAMPNS: c_int = 35;

#[allow(dead_code)]
pub(crate) const SO_MARK: c_int = 36;

#[allow(dead_code)]
pub(crate) const SO_TIMESTAMPING: c_int = 37;

#[allow(dead_code)]
pub(crate) const SO_RXQ_OVFL: c_int = 40;

#[allow(dead_code)]
pub(crate) const SO_WIFI_STATUS: c_int = 41;

#[allow(dead_code)]
pub(crate) const SO_PEEK_OFF: c_int = 42;

#[allow(dead_code)]
pub(crate) const SO_NOFCS: c_int = 43;

#[allow(dead_code)]
pub(crate) const SO_LOCK_FILTER: c_int = 44;

#[allow(dead_code)]
pub(crate) const SO_SELECT_ERR_QUEUE: c_int = 45;

#[allow(dead_code)]
pub(crate) const SO_BUSY_POLL: c_int = 46;

#[allow(dead_code)]
pub(crate) const SO_MAX_PACING_RATE: c_int = 47;

#[allow(dead_code)]
pub(crate) const SO_BPF_EXTENSIONS: c_int = 48;

pub(crate) const SO_INCOMING_CPU: c_int = 49;

#[allow(dead_code)]
pub(crate) const SO_ATTACH_BPF: c_int = 50;

#[allow(dead_code)]
pub(crate) const SO_DETACH_BPF: c_int = SO_DETACH_FILTER;

#[allow(dead_code)]
pub(crate) const SO_ATTACH_REUSEPORT_CBPF: c_int = 51;

#[allow(dead_code)]
pub(crate) const SO_ATTACH_REUSEPORT_EBPF: c_int = 52;

#[allow(dead_code)]
pub(crate) const SO_CNX_ADVICE: c_int = 53;

#[allow(dead_code)]
pub(crate) const SO_MEMINFO: c_int = 55;

#[allow(dead_code)]
pub(crate) const SO_INCOMING_NAPI_ID: c_int = 56;

#[allow(dead_code)]
pub(crate) const SO_COOKIE: c_int = 57;

#[allow(dead_code)]
pub(crate) const SO_PEERGROUPS: c_int = 59;

#[allow(dead_code)]
pub(crate) const SO_ZEROCOPY: c_int = 60;
