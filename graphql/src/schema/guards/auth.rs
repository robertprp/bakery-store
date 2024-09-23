// use async_graphql::{async_trait::async_trait, Context, Guard, Result};
//
// use crate::schema::GQLJWTData;
//
// pub struct AuthGuard {}
//
// impl AuthGuard {
//     pub fn new() -> Self {
//         AuthGuard {}
//     }
// }
//
// #[async_trait]
// impl Guard for AuthGuard {
//     async fn check<'a>(&'a self, ctx: &'a Context<'a>) -> Result<()> {
//         let claims = ctx
//             .data_opt::<GQLJWTData>()
//             .and_then(|rd| rd.claims.as_ref());
//
//         if claims.is_none() {
//             return Err("Unauthorized request".into());
//         }
//
//         Ok(())
//     }
// }
