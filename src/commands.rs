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

// use the templates
pub struct Template {
    //empty
}

impl Create {
    pub fn new_rn(name: &String) -> io::Result<()> {
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

    pub fn new_native(name: &String) -> io::Result<()> {
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

        //package.json
        create_and_write_file(
            format!("{}/package.json", name),
            NativeTemplate::package_json(),
        )?;

        //gitignore
        create_and_write_file(format!("{}/.gitignore", name), NativeTemplate::gitignore())?;

        //create the src directory
        let src_dir_path = format!("{name}/src");
        new_folder(&src_dir_path)?;

        //index.tsx
        create_and_write_file(
            format!("{src_dir_path}/index.tsx"),
            NativeTemplate::index_tsx(),
        )?;

        //app.tsx
        create_and_write_file(format!("{src_dir_path}/App.tsx"), NativeTemplate::app_tsx())?;

        // create the components views, folders and utils folders
        let components_path = format!("{src_dir_path}/components");
        let views_path = format!("{src_dir_path}/views");
        let utils_path = format!("{src_dir_path}/utils");
        new_folder(&components_path)?;
        new_folder(&views_path)?;
        new_folder(&utils_path)?;

        //components
        create_and_write_file(
            format!("{components_path}/Header.tsx"),
            NativeTemplate::header_tsx(),
        )?;
        create_and_write_file(
            format!("{components_path}/Screen.tsx"),
            NativeTemplate::screen_tsx(),
        )?;
        create_and_write_file(
            format!("{components_path}/TabNavigator.tsx"),
            NativeTemplate::tab_navigator(),
        )?;

        //utils
        create_and_write_file(
            format!("{utils_path}/icons.tsx"),
            NativeTemplate::icons_tsx(),
        )?;

        //views
        create_and_write_file(
            format!("{views_path}/About.tsx"),
            NativeTemplate::about_tsx(),
        )?;

        create_and_write_file(format!("{views_path}/Home.tsx"), NativeTemplate::home_tsx())?;

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

impl Template {
    pub fn print_available_templates() {
        println!("\n\t\t\t*** Available Templates ***\n");
        println!(" * xnft-quickstart");
    }

    pub fn default_template() {
        println!("\nCloning into default quickstart repo\n");
        clone_git_repo("https://github.com/coral-xyz/xnft-quickstart.git").unwrap();
    }

    pub fn get_template(template_name: &str) {
        if template_name == "xnft-quickstart" {
            Self::default_template()
        }
        println!("{} not found among the listed templates", template_name);
    }
}

pub fn new_folder(name: &String) -> std::io::Result<()> {
    fs::create_dir(name)?;
    Ok(())
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

pub fn clone_git_repo(repo_name: &str) -> Result<process::Output, ()> {
    Command::new("git")
        .arg("clone")
        .arg(repo_name)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| println!("error initialize git -> {}", e))
}
