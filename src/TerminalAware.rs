// This file is part of stderr. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/stderr/master/COPYRIGHT. No part of stderr, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of stderr. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/stderr/master/COPYRIGHT.


pub trait TerminalAware
{
	fn paintForStandardError<'a, I, S: 'a + ToOwned + ?Sized>(self, input: I) -> ANSIGenericString<'a, S>
	where I: Into<Cow<'a, S>>, <S as ToOwned>::Owned: Debug;
}

impl TerminalAware for Colour
{
	fn paintForStandardError<'a, I, S: 'a + ToOwned + ?Sized>(self, input: I) -> ANSIGenericString<'a, S>
	where I: Into<Cow<'a, S>>, <S as ToOwned>::Owned: Debug
	{
		let mut result = self.paint(input);
		if unlikely(*StandardErrorSupportsAnAnsiTerminal)
		{
			result
		}
		else
		{
			result.make_plain();
			result
		}
	}
}

impl TerminalAware for Style
{
	fn paintForStandardError<'a, I, S: 'a + ToOwned + ?Sized>(self, input: I) -> ANSIGenericString<'a, S>
	where I: Into<Cow<'a, S>>, <S as ToOwned>::Owned: Debug
	{
		let mut result = self.paint(input);
		if unlikely(*StandardErrorSupportsAnAnsiTerminal)
		{
			result
		}
		else
		{
			result.make_plain();
			result
		}
	}
}
