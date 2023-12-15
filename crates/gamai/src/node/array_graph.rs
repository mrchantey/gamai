pub struct Tree<T> {
	pub value: T,
	pub children: Vec<Tree<T>>,
}


impl<T> Into<Tree<T>> for (T, Vec<Tree<T>>) {
	fn into(self) -> Tree<T> { Tree::<T>::new_with_children(self.0, self.1) }
}


impl<T> Tree<T> {
	pub fn new(value: T) -> Self {
		Self {
			value,
			children: Vec::new(),
		}
	}
	pub fn new_with_children(value: T, children: Vec<Tree<T>>) -> Self {
		Self { value, children }
	}
}

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
