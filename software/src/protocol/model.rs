pub mod request {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
    pub enum EStatus {
        Running,
        Success,
        Failure,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
    pub struct BuildAction {
        pub target: String,
        pub status: EStatus,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
    pub struct BuildConfig {
        pub songdir: String,
        pub bookdir: String,
        pub builddir: String,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
    pub struct InfoSaveFile {
        pub path: String,
        pub content: String,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
    pub enum EChoice {
        ItemBuild,
        ItemOMakeChildrenInfo,
        ItemOMakeKill,
        ItemCleanBuildTree,
        ItemHealthCheck,
        ItemSeeProgress,
        ItemSourceTree,
        ItemSaveFile(InfoSaveFile),
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    pub struct Choice {
        pub choice: EChoice,
    }
}
// answers

pub mod answer {
    use serde::{Deserialize, Serialize};
    // use std::path::Path;

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
    pub struct ChildInfo {
        pub pid: u32,
        pub cwd: Option<String>,
        pub name: String, // command: String,
        pub run_time: u64,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
    pub struct TreeInfo {
        pub item: u32,
        pub cwd: Option<String>,
        pub name: String, // command: String,
        pub run_time: u64,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
    pub struct ProgressItem {
        pub path: String,
        pub status: bool,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
    pub struct Progress {
        pub progress: Vec<ProgressItem>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
    pub struct SourceTreeItem {
        pub title: String,
        pub author: String,
        pub masterjsonfile: String,
        pub texfiles: Vec<String>,
        pub lyricstexfiles: Vec<String>,
        pub lyfiles: Vec<String>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
    pub struct SourceTree {
        pub items: Vec<SourceTreeItem>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
    pub enum EChoice {
        ItemOmakeBuild(u32),
        ItemOMakeOmakeChildren(Vec<ChildInfo>),
        ItemOkMessage,
        ItemErrorMessage(String),
        ItemHealthOk,
        ItemSeeProgress(Progress),
        ItemSourceTree(SourceTree),
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    pub struct Choice {
        pub choice: EChoice,
    }
}
