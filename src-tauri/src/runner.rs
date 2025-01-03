use std::fs::{self, File};
use std::path::Path;
use std::process::Command;
use uuid::Uuid;

/**
 * This function will do the following
 * - Generate a UUID
 * - Create a new directory for executing the notebok
 * - Execute the notebook
 * - Save the output to file
 */
pub fn execute_notebook(
    executable_path: String,
    data_directory: String,
    notebook_path: String,
) -> Uuid {
    let id = Uuid::new_v4();
    let path = Path::new(&notebook_path);
    let filename: String = String::from(path.file_name().unwrap().to_str().unwrap());
    let execution_directory = format!("{}/{}-{}", data_directory, filename, id);

    tauri::async_runtime::spawn(async move {
        fs::create_dir_all(&execution_directory).expect("Unable to create directory");

        let stdoutfilepath = format!("{}/output.log", &execution_directory);
        let stdoutfile = File::create(stdoutfilepath).expect("failed to open log");

        let stderrfilepath = format!("{}/error.log", &execution_directory);
        let stderrfile = File::create(stderrfilepath).expect("failed to open log");

        let outputfile = format!("{}/{}", &execution_directory, &filename);

        let mut child = Command::new(executable_path)
            .args(["-m", "papermill", &notebook_path, &outputfile])
            .current_dir(data_directory)
            .stderr(stderrfile)
            .stdout(stdoutfile)
            .spawn()
            .expect("Failed to spawn executable");

        child.wait().expect("Failed ot execute command");
    });

    return id;
}
