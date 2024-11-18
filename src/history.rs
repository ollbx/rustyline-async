use std::collections::VecDeque;

pub struct History {
	pub entries: VecDeque<String>,
	pub max_size: usize,
	current_position: Option<usize>,
}
impl Default for History {
	fn default() -> Self {
		Self {
			entries: Default::default(),
			max_size: 1000,
			current_position: Default::default(),
		}
	}
}

impl History {
	pub fn add_entry(&mut self, line: String) {
		// Reset offset to newest entry
		self.current_position = None;
		// Don't add entry if last entry was same, or line was empty.
		if self.entries.back() == Some(&line) || line.is_empty() {
			return;
		}
		// Add entry to front of history
		self.entries.push_back(line);
		// Check if already have enough entries
		if self.entries.len() > self.max_size {
			// Remove oldest entry
			self.entries.pop_front();
		}
	}

	pub fn get_entries(&self) -> &VecDeque<String> {
		&self.entries
	}

	pub fn set_entries(&mut self, entries: VecDeque<String>) {
		self.entries = entries;

		while self.entries.len() > self.max_size {
			self.entries.pop_front();
		}
	}

	// Sets the history position back to the start.
	pub fn reset_position(&mut self) {
		self.current_position = None;
	}

	// Find next history that matches a given string from an index
	pub fn search_next(&mut self, _current: &str) -> Option<&str> {
		if let Some(index) = &mut self.current_position {
			if *index > 0 {
				*index -= 1;
			}
			Some(&self.entries[*index])
		} else if let Some(last) = self.entries.back() {
			self.current_position = Some(self.entries.len() - 1);
			Some(last)
		} else {
			None
		}
	}
	// Find previous history item that matches a given string from an index
	pub fn search_previous(&mut self, _current: &str) -> Option<&str> {
		if let Some(index) = &mut self.current_position {
			if *index >= self.entries.len() - 1 {
				self.current_position = None;
				return Some("");
			}
			*index += 1;
			Some(&self.entries[*index])
		} else {
			None
		}
	}
}
