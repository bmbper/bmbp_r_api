pub trait OrmIdent {
    fn name(&self) -> String;
}
impl<T> OrmIdent for T
where
    T: ToString,
{
    fn name(&self) -> String {
        self.to_string()
    }
}
