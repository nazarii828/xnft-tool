// ! contains functions to call once a command is matched

use crate::template::{NativeTemplate, RNTemplate};

use std::{
    env, fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
use std::{io, process};

//creates the xnft project
pub struct Create {}

impl Create {
    pub fn newRN(name: &String) -> io::Result<()> {
        let cur_dir = env::current_dir();
        println!(
            "\ncreating a new project `{}` in cur_dir -> {:?}\n",
            name,
            cur_dir.as_ref().unwrap()
        );

        //check if project already exists
        if Path::new(name).is_dir() {
            println!("directory already exists, Run command with new name");
            process::exit(1);
        }

        //create project folder
        new_folder(name)?;

        // ? create and write to the config files

        //webpack
        create_and_write_file(
            format!("{}/webpack.config.js", name),
            RNTemplate::webpack_config_js(),
        )?;

        // tsconfig
        create_and_write_file(format!("{}/tsconfig.json", name), RNTemplate::ts_config())?;

        // RNTemplate.html
        create_and_write_file(
            format!("{}/RNTemplate.html", name),
            RNTemplate::template_html(),
        )?;

        // readme
        create_and_write_file(format!("{}/README.md", name), RNTemplate::readme_md())?;

        //package.json
        create_and_write_file(format!("{}/package.json", name), RNTemplate::package_json())?;

        //babel config
        create_and_write_file(
            format!("{name}/babel.config.js"),
            RNTemplate::babel_config_json(),
        )?;

        //app.json
        create_and_write_file(format!("{name}/app.json"), RNTemplate::app_json())?;

        //.nvmrc
        create_and_write_file(format!("{name}/.nvmrc"), RNTemplate::nvmrc())?;

        //gitignore
        create_and_write_file(format!("{name}/.gitignore"), RNTemplate::gitignore())?;

        //create the src directory
        let src_dir_path = format!("{name}/src");
        new_folder(&src_dir_path)?;

        create_and_write_file(format!("{src_dir_path}/App.tsx"), RNTemplate::app_tsx())?;

        // create the components and screens folders
        let components_path = format!("{src_dir_path}/components");
        let screens_path = format!("{src_dir_path}/screens");
        new_folder(&components_path)?;
        new_folder(&screens_path)?;

        create_and_write_file(
            format!("{components_path}/Screen.tsx"),
            RNTemplate::screen_tsx(),
        )?;
        create_and_write_file(
            format!("{components_path}/Section.tsx"),
            RNTemplate::section_tsx(),
        )?;
        create_and_write_file(
            format!("{components_path}/TabNavigator.tsx"),
            RNTemplate::tab_navigator_tsx(),
        )?;

        create_and_write_file(
            format!("{screens_path}/ExampleScreen.tsx"),
            RNTemplate::example_screen_tsx(),
        )?;
        create_and_write_file(
            format!("{screens_path}/HomeScreen.tsx"),
            RNTemplate::home_screen_tsx(),
        )?;

        // set the current directory
        env::set_current_dir(name).unwrap();

        //init git
        let git_result = initialize_git().unwrap();
        if !git_result.status.success() {
            eprintln!("Failed to automatically initialize a new git repository");
        }

        // install node modules
        let yarn_result = install_node_modules("yarn").unwrap();
        if !yarn_result.status.success() {
            println!("Failed yarn install will attempt to npm install");
            install_node_modules("npm").unwrap();
        }

        Ok(())
    }
}

pub fn new_folder(name: &String) -> std::io::Result<()> {
    fs::create_dir(name)?;
    Ok(())
}

pub fn black_backpack_image() {
    //
}

pub fn create_and_write_file(file_name: String, content_to_write: &str) -> std::io::Result<()> {
    let mut created_file = fs::File::create(file_name)?;

    created_file.write_all(content_to_write.as_bytes())?;

    Ok(())
}

fn install_node_modules(cmd: &str) -> Result<process::Output, ()> {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg(format!("/C {} install", cmd))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .map_err(|e| println!("{} install failed: {}", cmd, e.to_string()))
    } else {
        Command::new(cmd)
            .arg("install")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .map_err(|e| println!("{} install failed: {}", cmd, e.to_string()))
    }
}

pub fn initialize_git() -> Result<process::Output, ()> {
    Command::new("git")
        .arg("init")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| println!("error initialize git -> {}", e))
}
