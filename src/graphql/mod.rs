use crate::database::DbConn;
use crate::domain::user::User;
use juniper::{Context, RootNode};

pub struct GQLContext {
    pub database: DbConn,
    pub user: Option<User>,
}
impl Context for GQLContext {}

pub struct Query;
pub struct Mutations;
pub type Schema = RootNode<'static, Query, Mutations>;

// list of resolvers
// pub mod post_resolver;
