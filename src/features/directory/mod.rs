pub trait Directory<T> {
    fn cache(id: &str) -> T;
    fn find(id: &str) -> T;
    fn fetch(id: &str) -> T;
}
