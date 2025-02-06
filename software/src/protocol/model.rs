pub mod request {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
    pub enum EStatus {
        Running,
        Success,
        Failure,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
    pub struct BuildAction {
        pub target: String,
        pub status: EStatus,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
    pub struct BuildConfig {
        pub songdir: String,
        pub bookdir: String,
        pub builddir: String,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
    pub enum EChoice {
        ItemBuild,
        ItemOMakeChildrenInfo,
        ItemOMakeKill,
        ItemCleanBuildTree,
        ItemHealthCheck,
        ItemSeeProgress,
        ItemSourceTree,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct Choice {
        pub choice: EChoice,
    }
}
// answers

pub mod answer {
    use serde::{Deserialize, Serialize};
    // use std::path::Path;

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
    pub struct ChildInfo {
        pub pid: u32,
        pub cwd: Option<String>,
        pub name: String, // command: String,
        pub run_time: u64,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
    pub struct TreeInfo {
        pub item: u32,
        pub cwd: Option<String>,
        pub name: String, // command: String,
        pub run_time: u64,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
    pub struct ProgressItem {
        pub path: String,
        pub status: bool,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
    pub struct Progress {
        pub progress: Vec<ProgressItem>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
    pub struct SourceTreeItem {
        pub title: String,
        pub author: String,
        pub texfiles: Vec<String>,
        pub lyricstexfiles: Vec<String>,
        pub lyfiles: Vec<String>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
    pub struct SourceTree {
        pub items: Vec<SourceTreeItem>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
    pub enum EChoice {
        ItemOmakeBuild(u32),
        ItemOMakeOmakeChildren(Vec<ChildInfo>),
        ItemOkMessage,
        ItemErrorMessage(String),
        ItemHealthOk,
        ItemSeeProgress(Progress),
        ItemSourceTree(SourceTree),
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct Choice {
        pub choice: EChoice,
    }
}
