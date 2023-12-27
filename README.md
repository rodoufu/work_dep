# work_dep

Verifies the dependencies of member projects of a workspace to find shared ones to be added to the depencencies of the workspace.

One can use:
```sh
work_dep --project-path ~/git/my_project
futures-util could be in the workspace, it is used by
[
    (
        "my_project_core",
        Value(
            "0.3",
        ),
    ),
    (
        "my_project_server",
        Value(
            "0.3",
        ),
    ),
    (
        "my_project_client",
        Value(
            "0.3",
        ),
    ),
]
```
