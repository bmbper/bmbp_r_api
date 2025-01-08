pub trait OrmRecord{
    fn insert(&self)->usize;
    fn update(&self)->usize;
    fn delete(&self)->usize;
    fn select(&self)->Vec<Self>;
    fn info(&self,id:usize)->Self;
}

