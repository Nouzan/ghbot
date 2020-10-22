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
    pub issue: Issue,
    pub changes: Changes,
    pub repository: Repository,
    pub sender: Sender,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Changes {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub url: String,
    pub repository_url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    pub title: String,
    pub user: Sender,
    pub labels: Vec<Label>,
    pub state: String,
    pub locked: bool,
    pub assignee: Sender,
    pub assignees: Vec<Sender>,
    pub milestone: Milestone,
    pub comments: i64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<serde_json::Value>,
    pub author_association: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sender {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub sender_type: String,
    pub site_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub name: String,
    pub color: String,
    #[serde(rename = "default")]
    pub label_default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Milestone {
    pub url: String,
    pub html_url: String,
    pub labels_url: String,
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    pub title: String,
    pub description: String,
    pub creator: Sender,
    pub open_issues: i64,
    pub closed_issues: i64,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub due_on: String,
    pub closed_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub owner: Sender,
    pub html_url: String,
    pub description: Option<serde_json::Value>,
    pub fork: bool,
    pub url: String,
    pub forks_url: String,
    pub keys_url: String,
    pub collaborators_url: String,
    pub teams_url: String,
    pub hooks_url: String,
    pub issue_events_url: String,
    pub events_url: String,
    pub assignees_url: String,
    pub branches_url: String,
    pub tags_url: String,
    pub blobs_url: String,
    pub git_tags_url: String,
    pub git_refs_url: String,
    pub trees_url: String,
    pub statuses_url: String,
    pub languages_url: String,
    pub stargazers_url: String,
    pub contributors_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub commits_url: String,
    pub git_commits_url: String,
    pub comments_url: String,
    pub issue_comment_url: String,
    pub contents_url: String,
    pub compare_url: String,
    pub merges_url: String,
    pub archive_url: String,
    pub downloads_url: String,
    pub issues_url: String,
    pub pulls_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub labels_url: String,
    pub releases_url: String,
    pub deployments_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    pub homepage: Option<serde_json::Value>,
    pub size: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub language: Option<serde_json::Value>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: i64,
    pub mirror_url: Option<serde_json::Value>,
    pub archived: bool,
    pub disabled: bool,
    pub open_issues_count: i64,
    pub license: Option<serde_json::Value>,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub default_branch: String,
}
