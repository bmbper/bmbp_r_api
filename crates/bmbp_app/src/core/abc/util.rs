use uuid::Uuid;

pub fn simple_id() -> String {
    Uuid::new_v4().to_string().replace("-", "")
}
pub fn now_date_time()->String{
    let now = chrono::Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}