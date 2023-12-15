


pub struct ArrayGraph<T> {
	pub items: Vec<T>,
	pub children: Vec<ArrayGraph<T>>,
}

impl<T> ArrayGraph<T> {
	pub fn new() -> Self {
		Self {
			items: Vec::new(),
			children: Vec::new(),
		}
	}

	pub fn flatten(self) -> Vec<T> {
		let min_items = self.items.len() + self.children.len();
		let mut items = Vec::with_capacity(min_items);

		for item in self.items.into_iter() {
			items.push(item);
		}
		for child in self.children.into_iter() {
			items.extend(child.flatten());
		}
		items
	}
}
