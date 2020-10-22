use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Common {
    pub action: String,
    pub sender: Value,
    pub repository: Option<Value>,
    pub organization: Option<Value>,
    pub installation: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Payload {
    IssueEvent(IssueEvent),
    Common(Common),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueEvent {
    pub action: String,
    pub changes: Changes,
    pub comment: Comment,
    pub issue: Issue,
    pub repository: Repository,
    pub sender: Sender,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Changes {
    pub body: Body,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    pub from: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub author_association: String,
    pub body: String,
    pub created_at: String,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub node_id: String,
    pub performed_via_github_app: Option<serde_json::Value>,
    pub updated_at: String,
    pub url: String,
    pub user: Sender,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sender {
    pub avatar_url: String,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: String,
    pub html_url: String,
    pub id: i64,
    pub login: String,
    pub node_id: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub subscriptions_url: String,
    #[serde(rename = "type")]
    pub sender_type: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub active_lock_reason: Option<serde_json::Value>,
    pub assignee: Option<serde_json::Value>,
    pub assignees: Vec<Option<serde_json::Value>>,
    pub author_association: String,
    pub body: String,
    pub closed_at: Option<serde_json::Value>,
    pub comments: i64,
    pub comments_url: String,
    pub created_at: String,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    pub labels: Vec<Option<serde_json::Value>>,
    pub labels_url: String,
    pub locked: bool,
    pub milestone: Option<serde_json::Value>,
    pub node_id: String,
    pub number: i64,
    pub performed_via_github_app: Option<serde_json::Value>,
    pub repository_url: String,
    pub state: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Sender,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub archive_url: String,
    pub archived: bool,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub created_at: String,
    pub default_branch: String,
    pub deployments_url: String,
    pub description: String,
    pub disabled: bool,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks: i64,
    pub forks_count: i64,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_pages: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub homepage: Option<serde_json::Value>,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub language: String,
    pub languages_url: String,
    pub license: License,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<serde_json::Value>,
    pub name: String,
    pub node_id: String,
    pub notifications_url: String,
    pub open_issues: i64,
    pub open_issues_count: i64,
    pub owner: Sender,
    pub private: bool,
    pub pulls_url: String,
    pub pushed_at: String,
    pub releases_url: String,
    pub size: i64,
    pub ssh_url: String,
    pub stargazers_count: i64,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub updated_at: String,
    pub url: String,
    pub watchers: i64,
    pub watchers_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    pub key: String,
    pub name: String,
    pub node_id: String,
    pub spdx_id: String,
    pub url: String,
}
