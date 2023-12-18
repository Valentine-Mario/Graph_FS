use crate::{
    auth::check_access::check_write_access,
    schema::{Context, MutationRoot, QueryRoot},
};

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

pub fn graphql_write_access(context: &Context) -> bool {
    if context.args.use_auth.is_some() && context.args.use_auth.unwrap() {
        let token = context.clone().auth_token.unwrap_or("".to_string());
        return check_write_access(context.args.clone(), &token);
    }
    return true;
}
