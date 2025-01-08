use crate::OrmRecord;

pub trait OrmRawExecutor {}
pub trait OrmRecordExecutor {
    fn insert<T>(&self, record: T)-> i64
    where
        T: OrmRecord;
    fn update<T>(&self, record: T)-> i64
    where
        T: OrmRecord;
    fn delete<T>(&self, record: T)-> i64
    where
        T: OrmRecord;
    fn select<T>(&self, record: T)-> Vec<T>
    where
        T: OrmRecord;
    fn info<T>(&self, record: T)-> Option<T>
    where
        T: OrmRecord;
}
pub trait OrmWrapperExecutor {}
