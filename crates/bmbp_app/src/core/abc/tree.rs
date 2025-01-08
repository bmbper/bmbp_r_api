pub trait BmbpTree<T> {
    fn code(&self) -> String;
    fn parent_code(&self) -> String;
    fn children(&self) -> Option<&Vec<Self>>
    where
        Self: Sized;
    fn set_children(&mut self, children: Vec<Self>)
    where
        Self: Sized;
}

pub struct BmbpTreeUtil;
impl BmbpTreeUtil {
    pub fn build_tree<T>(list: Vec<T>) -> Vec<T> {
        list
    }
}
