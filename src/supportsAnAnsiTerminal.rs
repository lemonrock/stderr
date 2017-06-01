// This file is part of stderr-logging. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/stderr-logging/master/COPYRIGHT. No part of stderr-logging, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of stderr-logging. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/stderr-logging/master/COPYRIGHT.


lazy_static!
{
	#[derive(Debug)]
	pub static ref StandardOutSupportsAnAnsiTerminal: bool =
	{
		supportsAnAnsiTerminal(1)
	};
	
	#[derive(Debug)]
	pub static ref StandardErrorSupportsAnAnsiTerminal: bool =
	{
		supportsAnAnsiTerminal(2)
	};
}

// TODO: Can be supported, but can we be bothered?
#[cfg(windows)]
pub fn supportsAnAnsiTerminal(fileDescriptor: i32) -> bool
{
	// as well as checking isatty(), need to call SetConsoleMode with the ENABLE_VIRTUAL_TERMINAL_PROCESSING flag
	// See also https://msdn.microsoft.com/en-us/library/windows/desktop/mt638032(v=vs.85).aspx
	false
}

#[cfg(unix)]
pub fn supportsAnAnsiTerminal(fileDescriptor: i32) -> bool
{
	match unsafe { isatty(fileDescriptor) }
	{
		1 =>
		{
			if let Ok(terminal) = var("TERM")
			{
				// This list is based on Alpine Linux ncursesw6 /etc/terminfo, which contains the most common terminals
				match &terminal[..]
				{
					"dumb" => false,
			
					"ansi" => true,
					"linux" => true,
					"rxvt" => true,
					"screen" => true,
			
					_ =>
					{
						if terminal.starts_with("vt")
						{
							true
						}
						// Some Mac OS X users might still be using nsterm*, but most now use xterm-256color
						else if terminal.starts_with("xterm") || terminal.starts_with("nsterm")
						{
							true
						}
						else
						{
							false
						}
					}
				}
		
			}
			else
			{
				false
			}
		},

		// Technically, we can check errno but there's really no advantage
		0 => false,
		
		illegal @ _ => panic!("Not a valid result from isatty '{}'", illegal),
	}
}
