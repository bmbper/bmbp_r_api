use crate::OrmTableTrait;

pub trait OrmCurdTrait<T>
where
    T: OrmTableTrait,
{
    fn insert(&self) -> usize;
    fn update(&self) -> usize;
    fn delete(&self) -> usize;
    fn select_one(&self) -> Option<T>;
    fn select_list(&self) -> Option<Vec<T>>;
    fn select_page(&self) -> Option<T>;
    fn insert_with_null(&self) -> usize;
    fn update_with_null(&self) -> usize;

    fn enable(&self)->usize;

    fn disable(&self)->usize;

    fn select_list_() -> Option<Vec<T>>;
    fn select_page_() -> Option<Vec<T>>;
    fn select_one_() -> Option<Vec<T>>;
    fn select_one_by_id() -> Option<T>;
    fn enable_by_id() -> usize;
    fn disable_by_id() -> usize;
    fn delete_by_id() -> usize;
}
