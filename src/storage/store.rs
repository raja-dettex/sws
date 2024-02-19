pub trait Store<T> {
    fn getAll(&self) -> Vec<T>;
    fn get(&self, key : String) -> Option<T>;
    fn add(&mut self, key: String, val : T) -> Option<T>;
    fn update(&mut self, key: String, val : T) -> Option<T>;
    fn delete(&mut self, key: String) -> Option<T>;
}