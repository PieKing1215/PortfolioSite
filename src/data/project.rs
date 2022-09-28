use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub icon: String,
}

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            id: "test1".into(),
            name: "Test 1".into(),
            desc: "Description for test 1\nhere".into(),
            icon: "test".into(),
        },
        Project {
            id: "test2".into(),
            name: "Test 2".into(),
            desc: "Description for test 2\nhere".into(),
            icon: "test2".into(),
        },
        Project {
            id: "test3".into(),
            name: "Test 3".into(),
            desc: "Description for test 3\nhere".into(),
            icon: "test".into(),
        },
        Project {
            id: "test4".into(),
            name: "Test 4".into(),
            desc: "Description for test 4\nhere".into(),
            icon: "test2".into(),
        },
    ]
}
