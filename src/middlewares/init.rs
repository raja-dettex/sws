pub trait Initializer<T> {
    fn init(item : &T) -> Self; 
}