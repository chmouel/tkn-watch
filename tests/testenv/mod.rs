use std::{env, path::PathBuf, process};

/// Find the *tkn-watch* executable
pub fn find_tknwatch() -> PathBuf {
    let root = env::current_exe()
        .expect("tests executable")
        .parent()
        .expect("tests executable directory")
        .parent()
        .expect("tknwatch executable directory")
        .to_path_buf();
    root.join("tkn-watch")
}

/// Format an error message for when *tkn-watch* did not exit successfully.
pub fn format_exit_error(args: &[&str], output: &process::Output) -> String {
    format!(
        "`tkn-watch {}` did not exit successfully.\nstdout:\n---\n{}---\nstderr:\n---\n{}---",
        args.join(" "),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}
#[macro_export]
macro_rules! golden {
    ($fun: ident, $args:tt) => {
        #[test]
        fn $fun() {
            // i have no idea how to fake time yet :((
            let fixregexp = regex::Regex::new(r"(started|failed)\s*\d+.*").unwrap();
            let target = stringify!($fun);
            let output_file = format!("tests/testdata/{}.output", target);
            let json_file = format!("tests/testdata/{}.json", target);
            let env = testenv::TestEnv::new();

            let mut args = vec!["-f", &json_file];
            // need to figure out how to make macros  works with empty args
            // so the type system recognizes the args when empty
            if !$args.is_empty() && $args.get(0).unwrap() != &"" {
                args.extend($args);
            }

            let output = env.assert_success_and_get_output(&args);
            let output = String::from_utf8_lossy(&output.stdout);
            let output = fixregexp.replace_all(&output, "$1").to_string();

            // if we have the env GOLDEN_UPDATE then update the golden instead of testing
            if env::var("GOLDEN_UPDATE").is_ok() {
                let mut golden_file = File::create(output_file).unwrap();
                golden_file.write_all(output.as_bytes()).unwrap();
            } else {
                // open finished output file and read it
                let mut file = File::open(output_file).unwrap();
                let mut golden_output = String::new();
                file.read_to_string(&mut golden_output).unwrap();

                // replace a regexp in golden_output because we are too dumb to know how to fake time yet
                let golden_output = fixregexp.replace_all(&golden_output, "$1");

                // compare it to the output
                assert_eq!(golden_output, output);
            }
        }
    };
}
pub struct TestEnv {
    pub tknwatch_exe: PathBuf,
}

impl TestEnv {
    pub fn new() -> Self {
        Self {
            tknwatch_exe: find_tknwatch(),
        }
    }

    pub fn assert_success_and_get_output(&self, args: &[&str]) -> process::Output {
        let mut cmd = process::Command::new(&self.tknwatch_exe);
        cmd.args(args);
        // Run *tknn-watch*.
        let output = cmd.output().expect("tkn-watch output");

        // Check for exit status.
        if !output.status.success() {
            panic!("{}", format_exit_error(args, &output));
        }

        output
    }
}
