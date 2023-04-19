#![allow(clippy::all, warnings)]
pub struct UserTimeLogs;
pub mod user_time_logs {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UserTimeLogs";
    pub const QUERY : & str = "query UserTimeLogs {\n  currentUser {\n    timelogs {\n      nodes {\n        id\n        spentAt\n        project {\n          name\n        }\n        issue {\n          id,\n          title\n        }\n        timeSpent\n      }\n    }\n  }\n}\n\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type Time = super::Time;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "currentUser")]
        pub current_user: Option<UserTimeLogsCurrentUser>,
    }
    #[derive(Deserialize)]
    pub struct UserTimeLogsCurrentUser {
        pub timelogs: Option<UserTimeLogsCurrentUserTimelogs>,
    }
    #[derive(Deserialize)]
    pub struct UserTimeLogsCurrentUserTimelogs {
        pub nodes: Option<Vec<Option<UserTimeLogsCurrentUserTimelogsNodes>>>,
    }
    #[derive(Deserialize)]
    pub struct UserTimeLogsCurrentUserTimelogsNodes {
        pub id: ID,
        #[serde(rename = "spentAt")]
        pub spent_at: Option<Time>,
        pub project: UserTimeLogsCurrentUserTimelogsNodesProject,
        pub issue: Option<UserTimeLogsCurrentUserTimelogsNodesIssue>,
        #[serde(rename = "timeSpent")]
        pub time_spent: Int,
    }
    #[derive(Deserialize)]
    pub struct UserTimeLogsCurrentUserTimelogsNodesProject {
        pub name: String,
    }
    #[derive(Deserialize)]
    pub struct UserTimeLogsCurrentUserTimelogsNodesIssue {
        pub id: ID,
        pub title: String,
    }
}
impl graphql_client::GraphQLQuery for UserTimeLogs {
    type Variables = user_time_logs::Variables;
    type ResponseData = user_time_logs::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: user_time_logs::QUERY,
            operation_name: user_time_logs::OPERATION_NAME,
        }
    }
}
