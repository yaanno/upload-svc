use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type GithubActions = Vec<GithubAction>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubAction {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub actor: Actor,
    pub repo: Repo,
    pub payload: Option<Payload>,
    pub public: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub org: Option<Org>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    pub id: i64,
    pub login: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub id: i64,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    #[serde(rename = "ref")]
    pub ref_field: Option<String>,
    #[serde(rename = "ref_type")]
    pub ref_type: Option<String>,
    #[serde(rename = "master_branch")]
    pub master_branch: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "pusher_type")]
    pub pusher_type: Option<String>,
    #[serde(rename = "push_id")]
    pub push_id: Option<i64>,
    pub size: Option<i64>,
    #[serde(rename = "distinct_size")]
    pub distinct_size: Option<i64>,
    pub head: Option<String>,
    pub before: Option<String>,
    #[serde(default)]
    pub commits: Vec<Commit>,
    pub action: Option<String>,
    pub release: Option<Release>,
    pub number: Option<i64>,
    #[serde(rename = "pull_request")]
    pub pull_request: Option<PullRequest>,
    pub issue: Option<Issue2>,
    pub forkee: Option<Forkee>,
    #[serde(default)]
    pub pages: Vec<Page>,
    pub comment: Option<Comment>,
    pub member: Option<User>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commit {
    pub sha: String,
    pub author: Author,
    pub message: String,
    pub distinct: bool,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub email: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub url: String,
    #[serde(rename = "assets_url")]
    pub assets_url: String,
    #[serde(rename = "upload_url")]
    pub upload_url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub id: i64,
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    #[serde(rename = "target_commitish")]
    pub target_commitish: String,
    pub name: Option<String>,
    pub draft: bool,
    pub author: User,
    pub prerelease: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "published_at")]
    pub published_at: String,
    pub assets: Vec<Asset>,
    #[serde(rename = "tarball_url")]
    pub tarball_url: String,
    #[serde(rename = "zipball_url")]
    pub zipball_url: String,
    pub body: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub url: String,
    pub id: i64,
    pub name: String,
    pub label: Option<String>,
    pub uploader: User,
    #[serde(rename = "content_type")]
    pub content_type: String,
    pub state: String,
    pub size: i64,
    #[serde(rename = "download_count")]
    pub download_count: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "browser_download_url")]
    pub browser_download_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    pub url: String,
    pub id: i64,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "diff_url")]
    pub diff_url: String,
    #[serde(rename = "patch_url")]
    pub patch_url: String,
    #[serde(rename = "issue_url")]
    pub issue_url: String,
    pub number: i64,
    pub state: String,
    pub locked: bool,
    pub title: String,
    pub user: User,
    pub body: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "closed_at")]
    pub closed_at: Option<String>,
    #[serde(rename = "merged_at")]
    pub merged_at: Option<String>,
    #[serde(rename = "merge_commit_sha")]
    pub merge_commit_sha: Option<String>,
    pub assignee: Option<User>,
    pub milestone: Option<Milestone>,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "review_comments_url")]
    pub review_comments_url: String,
    #[serde(rename = "review_comment_url")]
    pub review_comment_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    pub head: Head,
    pub base: Base,
    #[serde(rename = "_links")]
    pub links: Links,
    pub merged: Option<bool>,
    pub mergeable: Option<bool>,
    #[serde(rename = "mergeable_state")]
    pub mergeable_state: Option<String>,
    #[serde(rename = "merged_by")]
    pub merged_by: Option<User>,
    pub comments: Option<i64>,
    #[serde(rename = "review_comments")]
    pub review_comments: Option<i64>,
    pub commits: Option<i64>,
    pub additions: Option<i64>,
    pub deletions: Option<i64>,
    #[serde(rename = "changed_files")]
    pub changed_files: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub login: String,
    pub id: i64,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Milestone {
    pub url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    pub id: i64,
    pub number: i64,
    pub title: String,
    pub description: Option<String>,
    pub creator: User,
    #[serde(rename = "open_issues")]
    pub open_issues: i64,
    #[serde(rename = "closed_issues")]
    pub closed_issues: i64,
    pub state: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "due_on")]
    pub due_on: Option<String>,
    #[serde(rename = "closed_at")]
    pub closed_at: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Head {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub sha: String,
    pub user: User,
    pub repo: Option<Repo2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo2 {
    pub id: i64,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub owner: User,
    pub private: bool,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    #[serde(rename = "keys_url")]
    pub keys_url: String,
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: String,
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "assignees_url")]
    pub assignees_url: String,
    #[serde(rename = "branches_url")]
    pub branches_url: String,
    #[serde(rename = "tags_url")]
    pub tags_url: String,
    #[serde(rename = "blobs_url")]
    pub blobs_url: String,
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: String,
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: String,
    #[serde(rename = "trees_url")]
    pub trees_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: String,
    #[serde(rename = "contributors_url")]
    pub contributors_url: String,
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: String,
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "compare_url")]
    pub compare_url: String,
    #[serde(rename = "merges_url")]
    pub merges_url: String,
    #[serde(rename = "archive_url")]
    pub archive_url: String,
    #[serde(rename = "downloads_url")]
    pub downloads_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "pulls_url")]
    pub pulls_url: String,
    #[serde(rename = "milestones_url")]
    pub milestones_url: String,
    #[serde(rename = "notifications_url")]
    pub notifications_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "releases_url")]
    pub releases_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "pushed_at")]
    pub pushed_at: String,
    #[serde(rename = "git_url")]
    pub git_url: String,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "clone_url")]
    pub clone_url: String,
    #[serde(rename = "svn_url")]
    pub svn_url: String,
    pub homepage: Option<String>,
    pub size: i64,
    #[serde(rename = "stargazers_count")]
    pub stargazers_count: i64,
    #[serde(rename = "watchers_count")]
    pub watchers_count: i64,
    pub language: Option<String>,
    #[serde(rename = "has_issues")]
    pub has_issues: bool,
    #[serde(rename = "has_downloads")]
    pub has_downloads: bool,
    #[serde(rename = "has_wiki")]
    pub has_wiki: bool,
    #[serde(rename = "has_pages")]
    pub has_pages: bool,
    #[serde(rename = "forks_count")]
    pub forks_count: i64,
    #[serde(rename = "mirror_url")]
    pub mirror_url: Value,
    #[serde(rename = "open_issues_count")]
    pub open_issues_count: i64,
    pub forks: i64,
    #[serde(rename = "open_issues")]
    pub open_issues: i64,
    pub watchers: i64,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Base {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub sha: String,
    pub user: User,
    pub repo: Repo3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo3 {
    pub id: i64,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub owner: User,
    pub private: bool,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    #[serde(rename = "keys_url")]
    pub keys_url: String,
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: String,
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "assignees_url")]
    pub assignees_url: String,
    #[serde(rename = "branches_url")]
    pub branches_url: String,
    #[serde(rename = "tags_url")]
    pub tags_url: String,
    #[serde(rename = "blobs_url")]
    pub blobs_url: String,
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: String,
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: String,
    #[serde(rename = "trees_url")]
    pub trees_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: String,
    #[serde(rename = "contributors_url")]
    pub contributors_url: String,
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: String,
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "compare_url")]
    pub compare_url: String,
    #[serde(rename = "merges_url")]
    pub merges_url: String,
    #[serde(rename = "archive_url")]
    pub archive_url: String,
    #[serde(rename = "downloads_url")]
    pub downloads_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "pulls_url")]
    pub pulls_url: String,
    #[serde(rename = "milestones_url")]
    pub milestones_url: String,
    #[serde(rename = "notifications_url")]
    pub notifications_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "releases_url")]
    pub releases_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "pushed_at")]
    pub pushed_at: String,
    #[serde(rename = "git_url")]
    pub git_url: String,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "clone_url")]
    pub clone_url: String,
    #[serde(rename = "svn_url")]
    pub svn_url: String,
    pub homepage: Option<String>,
    pub size: i64,
    #[serde(rename = "stargazers_count")]
    pub stargazers_count: i64,
    #[serde(rename = "watchers_count")]
    pub watchers_count: i64,
    pub language: Option<String>,
    #[serde(rename = "has_issues")]
    pub has_issues: bool,
    #[serde(rename = "has_downloads")]
    pub has_downloads: bool,
    #[serde(rename = "has_wiki")]
    pub has_wiki: bool,
    #[serde(rename = "has_pages")]
    pub has_pages: bool,
    #[serde(rename = "forks_count")]
    pub forks_count: i64,
    #[serde(rename = "mirror_url")]
    pub mirror_url: Option<String>,
    #[serde(rename = "open_issues_count")]
    pub open_issues_count: i64,
    pub forks: i64,
    #[serde(rename = "open_issues")]
    pub open_issues: i64,
    pub watchers: i64,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: SelfField,
    pub html: Html,
    pub issue: Issue,
    pub comments: Comments,
    #[serde(rename = "review_comments")]
    pub review_comments: ReviewComments,
    #[serde(rename = "review_comment")]
    pub review_comment: ReviewComment,
    pub commits: Commits,
    pub statuses: Statuses,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfField {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Html {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comments {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewComments {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewComment {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commits {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statuses {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue2 {
    pub url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub id: i64,
    pub number: i64,
    pub title: String,
    pub user: User,
    pub labels: Vec<Label>,
    pub state: String,
    pub locked: bool,
    pub assignee: Option<User>,
    pub milestone: Option<Milestone2>,
    pub comments: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "closed_at")]
    pub closed_at: Option<String>,
    pub body: Option<String>,
    #[serde(rename = "pull_request")]
    pub pull_request: Option<PullRequest2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub url: String,
    pub name: String,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Milestone2 {
    pub url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    pub id: i64,
    pub number: i64,
    pub title: String,
    pub description: Option<String>,
    pub creator: User,
    #[serde(rename = "open_issues")]
    pub open_issues: i64,
    #[serde(rename = "closed_issues")]
    pub closed_issues: i64,
    pub state: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "due_on")]
    pub due_on: Option<String>,
    #[serde(rename = "closed_at")]
    pub closed_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest2 {
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "diff_url")]
    pub diff_url: String,
    #[serde(rename = "patch_url")]
    pub patch_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forkee {
    pub id: i64,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub owner: User,
    pub private: bool,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    #[serde(rename = "keys_url")]
    pub keys_url: String,
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: String,
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "assignees_url")]
    pub assignees_url: String,
    #[serde(rename = "branches_url")]
    pub branches_url: String,
    #[serde(rename = "tags_url")]
    pub tags_url: String,
    #[serde(rename = "blobs_url")]
    pub blobs_url: String,
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: String,
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: String,
    #[serde(rename = "trees_url")]
    pub trees_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: String,
    #[serde(rename = "contributors_url")]
    pub contributors_url: String,
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: String,
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "compare_url")]
    pub compare_url: String,
    #[serde(rename = "merges_url")]
    pub merges_url: String,
    #[serde(rename = "archive_url")]
    pub archive_url: String,
    #[serde(rename = "downloads_url")]
    pub downloads_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "pulls_url")]
    pub pulls_url: String,
    #[serde(rename = "milestones_url")]
    pub milestones_url: String,
    #[serde(rename = "notifications_url")]
    pub notifications_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "releases_url")]
    pub releases_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "pushed_at")]
    pub pushed_at: String,
    #[serde(rename = "git_url")]
    pub git_url: String,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "clone_url")]
    pub clone_url: String,
    #[serde(rename = "svn_url")]
    pub svn_url: String,
    pub homepage: Option<String>,
    pub size: i64,
    #[serde(rename = "stargazers_count")]
    pub stargazers_count: i64,
    #[serde(rename = "watchers_count")]
    pub watchers_count: i64,
    pub language: Option<String>,
    #[serde(rename = "has_issues")]
    pub has_issues: bool,
    #[serde(rename = "has_downloads")]
    pub has_downloads: bool,
    #[serde(rename = "has_wiki")]
    pub has_wiki: bool,
    #[serde(rename = "has_pages")]
    pub has_pages: bool,
    #[serde(rename = "forks_count")]
    pub forks_count: i64,
    #[serde(rename = "mirror_url")]
    pub mirror_url: Value,
    #[serde(rename = "open_issues_count")]
    pub open_issues_count: i64,
    pub forks: i64,
    #[serde(rename = "open_issues")]
    pub open_issues: i64,
    pub watchers: i64,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
    pub public: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    #[serde(rename = "page_name")]
    pub page_name: String,
    pub title: String,
    pub summary: Value,
    pub action: String,
    pub sha: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "issue_url")]
    pub issue_url: Option<String>,
    pub id: i64,
    pub user: User,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub body: String,
    pub position: Option<i64>,
    pub line: Option<i64>,
    pub path: Option<String>,
    #[serde(rename = "commit_id")]
    pub commit_id: Option<String>,
    #[serde(rename = "diff_hunk")]
    pub diff_hunk: Option<String>,
    #[serde(rename = "original_position")]
    pub original_position: Option<i64>,
    #[serde(rename = "original_commit_id")]
    pub original_commit_id: Option<String>,
    #[serde(rename = "pull_request_url")]
    pub pull_request_url: Option<String>,
    #[serde(rename = "_links")]
    pub links: Option<Links2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links2 {
    #[serde(rename = "self")]
    pub self_field: SelfField2,
    pub html: Html2,
    #[serde(rename = "pull_request")]
    pub pull_request: PullRequest3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfField2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Html2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest3 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Org {
    pub id: i64,
    pub login: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
}
