use mq::Error;

pub(crate) fn convert_surrealdb_error(err: surrealdb::Error) -> Error {
    Error::OtherError(Box::new(err))
}
