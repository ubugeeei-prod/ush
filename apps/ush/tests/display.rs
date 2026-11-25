use std::{fs, process::Command};

use tempfile::tempdir;

fn ush() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ush"))
}

#[test]
fn ush_scripts_support_display_trait_and_methods() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("display.ush");
    fs::write(
        &script,
        r#"
        type User {
          name: String,
          age: Int,
        }
        impl Display for User {
          fn fmt(self) -> String {
            self.name + ":" + self.age
          }
        }
        impl User {
          fn prefixed(self, prefix: String) -> String {
            prefix + ":" + self.name
          }
        }
        let user = User { name: "ush", age: 7 }
        print user
        print format $user
        print user.prefixed("id")
        "#,
    )
    .expect("write script");

    let output = ush().arg(&script).output().expect("run ush");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "ush:7\nush:7\nid:ush\n"
    );
}
