use anyhow::{Context as _, Result};

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Note {
	pub text: String,
	pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Note {
	pub fn new<T: Into<String>>(text: T) -> Self {
		Self { text: text.into(), created_at: chrono::Utc::now() }
	}

	pub fn get(index: usize) -> Result<Self> {
		let notes = Self::get_all().context("Failed to get all notes.")?;
		Ok(notes.get(index).unwrap().clone())
	}

	pub fn get_all() -> Result<Vec<Self>> {
		let content =
			std::fs::read_to_string(crate::consts::DATABASE_FILENAME)
				.context("Failed to read the database.")?;
		let notes = serde_json::from_str(&content)
			.context("Failed to convert database content to notes.")?;
		Ok(notes)
	}

	pub fn get_count() -> Result<usize> {
		let notes = Self::get_all().context("Failed to get all notes.")?;
		Ok(notes.len())
	}

	pub fn create<T: Into<String>>(text: T) -> Result<()> {
		let mut notes = Self::get_all().context("Failed to get all notes.")?;
		notes.push(Self::new(text));
		std::fs::write(
			crate::consts::DATABASE_FILENAME,
			serde_json::to_vec(&notes)
				.context("Failed to convert notes to JSON.")?,
		)?;
		Ok(())
	}

	pub fn update<T: Into<String>>(index: usize, new_text: T) -> Result<()> {
		let mut notes = Self::get_all().context("Failed to get all notes.")?;
		notes[index].text = new_text.into();
		std::fs::write(
			crate::consts::DATABASE_FILENAME,
			serde_json::to_vec(&notes)
				.context("Failed to convert notes to JSON.")?,
		)?;
		Ok(())
	}

	pub fn delete(index: usize) -> Result<()> {
		let mut notes = Self::get_all().context("Failed to get all notes.")?;
		notes.remove(index);
		std::fs::write(
			crate::consts::DATABASE_FILENAME,
			serde_json::to_vec(&notes)
				.context("Failed to convert notes to JSON.")?,
		)?;
		Ok(())
	}
}
