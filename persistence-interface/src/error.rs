pub enum DbError {
    IdNotFound(IdType),
}

pub enum IdType {
    U64(u64),
    String(String),
}