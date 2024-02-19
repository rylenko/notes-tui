use anyhow::{Context as _, Result};

#[derive(Clone, Copy)]
pub enum InputMode {
	Null,
	Creating,
	Updating,
}

#[must_use]
pub fn beautify_datetime(datetime: chrono::DateTime<chrono::Utc>) -> String {
	use chrono::{Datelike as _, Timelike as _};
	format!(
		"{}.{}.{} at {}:{}",
		datetime.day(),
		datetime.month(),
		datetime.year(),
		datetime.hour(),
		datetime.minute()
	)
}

pub fn create_database_if_not_exists() -> Result<()> {
	if !std::path::Path::new(crate::consts::DATABASE_FILENAME).exists() {
		std::fs::write(crate::consts::DATABASE_FILENAME, "[]")
			.context("Failed to initialize the database.")?;
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_beautify_datetime() {
		let result = beautify_datetime(chrono::MIN_DATETIME);
		assert_eq!(result, "1.1.-262144 at 0:0");
	}
}
