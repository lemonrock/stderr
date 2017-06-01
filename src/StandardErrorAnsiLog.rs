// This file is part of stderr-logging. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/stderr-logging/master/COPYRIGHT. No part of stderr-logging, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of stderr-logging. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/stderr-logging/master/COPYRIGHT.


pub struct StandardErrorAnsiLog
{
	logLevel: LogLevel,
	standardErrorSupportsAnAnsiTerminal: bool,
}

impl Log for StandardErrorAnsiLog
{
	#[inline(always)]
	fn enabled(&self, metadata: &LogMetadata) -> bool
	{
		metadata.level() <= self.logLevel
	}

	#[inline(always)]
	fn log(&self, record: &LogRecord)
	{
		if unlikely(self.enabled(record.metadata()))
		{
			if unlikely(self.standardErrorSupportsAnAnsiTerminal)
			{
				let style = Self::style(record);
				let rawMessage = format!("{}", record.args());
				stderrln!("{}", style.paint(rawMessage));
			}
			else
			{
				stderrln!("{}:{}:{}", time::now_utc().rfc3339(), record.level(), record.args())
			}
		}
	}
}

impl StandardErrorAnsiLog
{
	#[inline(always)]
	fn style(record: &LogRecord) -> Style
	{
		let colour = match record.level()
		{
			LogLevel::Trace => Colour::Cyan,
			LogLevel::Debug => Colour::White,
			LogLevel::Info => Colour::Green,
			LogLevel::Warn => Colour::Yellow,
			LogLevel::Error => Colour::Red,
		};
		colour.normal()
	}
	
	#[allow(dead_code)]
	pub fn initialise(isQuiet: bool, verbosity: usize) -> Result<(), SetLoggerError>
	{
		let actualVerbosity = if isQuiet
		{
			0
		}
		else
		{
			verbosity + 1
		};
		
		let logLevel = match actualVerbosity
		{
			0 => LogLevel::Error,
			1 => LogLevel::Warn,
			2 => LogLevel::Info,
			3 => LogLevel::Debug,
			_ => LogLevel::Trace,
		};
		
		log::set_logger(|max_log_level|
		{
			max_log_level.set(logLevel.to_log_level_filter());
			Box::new
			(
				StandardErrorAnsiLog
				{
					logLevel: logLevel,
					standardErrorSupportsAnAnsiTerminal: *StandardErrorSupportsAnAnsiTerminal,
				}
			)
		})
	}
}
