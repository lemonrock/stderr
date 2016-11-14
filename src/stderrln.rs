// This file is part of stderr. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/stderr/master/COPYRIGHT. No part of stderr, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of stderr. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/stderr/master/COPYRIGHT.


#[macro_export]
macro_rules! stderrln
(
	($($arg:tt)*) =>
	{
		{
			let result = writeln!(&mut io::stderr(), $($arg)*);
			result.expect("Could not write line to stderr");
		}
	}
);
