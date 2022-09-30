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
            desc: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".into(),
            icon: "test".into(),
        },
        Project {
            id: "test2".into(),
            name: "Test 2".into(),
            desc: "Description for test 2<br>here".into(),
            icon: "test2".into(),
        },
        Project {
            id: "test3".into(),
            name: "Test 3".into(),
            desc: "Description for test 3<br>here".into(),
            icon: "test".into(),
        },
        Project {
            id: "test4".into(),
            name: "Test 4".into(),
            desc: "Description for test 4<br>here".into(),
            icon: "test2".into(),
        },
        Project {
            id: "test4".into(),
            name: "Test 4".into(),
            desc: "Description for test 4<br>here".into(),
            icon: "test2".into(),
        },
        Project {
            id: "test4".into(),
            name: "Test 4".into(),
            desc: "Description for test 4<br>here".into(),
            icon: "test2".into(),
        },
        Project {
            id: "test4".into(),
            name: "Test 4".into(),
            desc: "Description for test 4<br>here".into(),
            icon: "test2".into(),
        },
        Project {
            id: "test4".into(),
            name: "Test 4".into(),
            desc: "Description for test 4<br>here".into(),
            icon: "test2".into(),
        },
        Project {
            id: "test4".into(),
            name: "Test 4".into(),
            desc: "Description for test 4<br>here".into(),
            icon: "test2".into(),
        },
        Project {
            id: "test4".into(),
            name: "Test 4".into(),
            desc: "Description for test 4<br>here".into(),
            icon: "test2".into(),
        },
        Project {
            id: "test4".into(),
            name: "Test 4".into(),
            desc: "Description for test 4<br>here".into(),
            icon: "test2".into(),
        },
    ]
}
