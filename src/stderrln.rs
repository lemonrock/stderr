// This file is part of stderr-logging. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/stderr-logging/master/COPYRIGHT. No part of stderr-logging, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of stderr-logging. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/stderr-logging/master/COPYRIGHT.


#[macro_export]
macro_rules! stderrln
(
	($($arg:tt)*) =>
	{
		{
			use ::std::io::Write;
			let result = writeln!(&mut ::std::io::stderr(), $($arg)*);
			result.expect("Could not write line to stderr");
		}
	}
);
