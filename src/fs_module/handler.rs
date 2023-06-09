use crate::schema::{Context, MutationRoot, QueryRoot};

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    fn local_fs(&self) -> super::local_fs::LocalFsQuery {
        super::local_fs::LocalFsQuery
    }

    fn remote_fs(&self) -> super::remote_fs::RemoteFsQuery {
        super::remote_fs::RemoteFsQuery
    }
}

#[juniper::graphql_object(context = Context)]
impl MutationRoot {
    fn local_fs(&self) -> super::local_fs::LocalFsMutation {
        super::local_fs::LocalFsMutation
    }

    fn remote_fs(&self) -> super::remote_fs::RemoteFsMutation {
        super::remote_fs::RemoteFsMutation
    }
}
