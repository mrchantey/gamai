// use std::ops::Deref;
// use std::ops::DerefMut;

// /// Trait for structs that implement Deref and DerefMut, required for use as a prop.
// pub trait Prop<T>: Deref<Target = T> + DerefMut<Target = T> {}
// impl<T> Prop<T> for T where T: Deref<Target = T> + DerefMut<Target = T> {}
// /// Trait for structs that implement Deref and DerefMut, required for use as a prop.
pub trait Prop<T> {
	fn get(&self) -> &T;
	fn set(&mut self, value: T);
}
// impl<T> Prop<T> for T where T: Deref<Target = T> + DerefMut<Target = T> {}
