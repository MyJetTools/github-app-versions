use std::process::Command;

fn main() {
    // panic!("{:?}", std::env::current_dir());

    let mut command = Command::new("tsc");
    let result = command.output().expect("Failed to execute command");

    if result.status.success() {
        println!("tsc executed successfully");
    } else {
        //let out = std::str::from_utf8(&result.stdout.as_slice()).unwrap();
        //let err = std::str::from_utf8(result.stderr.as_slice()).unwrap();
        panic!("tsc failed to execute. Out: {:?}", result);
    }

    ci_utils::js::merge_js_files(
        &[
            //"utils.js",
            "html.js",
            "dialog.js",
            "app.js",
            //    "envs.js",
            //    "apps.js",
        ],
        "wwwroot/js/app.js",
    );
}
