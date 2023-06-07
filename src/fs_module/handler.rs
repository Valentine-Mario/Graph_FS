use crate::schema::{Context, QueryRoot};

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    fn local_fs(&self) -> super::local_fs::LocalFsQuery {
        super::local_fs::LocalFsQuery
    }

    fn remote_fs(&self) -> super::remote_fs::RemoteFsQuery {
        super::remote_fs::RemoteFsQuery
    }
}
