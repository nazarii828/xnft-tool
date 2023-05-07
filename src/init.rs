// ? Initialize a new xnft project 
use anyhow::{self, Ok};
use clap::{Parser, ValueHint};
use serde_json::{json, Value};
use std::{
    fs, io,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::utils::Cmd;

/// CLI arguments for xnft-tool init
#[derive(Debug, Clone, Parser)]
pub struct InitArgs {
    /// directory to initialize the new project. Defaults to the current working directory
    #[arg(value_hint = ValueHint::DirPath)]
    name: Option<PathBuf>,
    /// Create the project in specified directory even if not empty
    #[arg(long, default_value_t = false)]
    force: bool,
    /// Initialize the project from an existing template
    #[arg(long)]
    template: Option<String>,
    /// Initialize the project without git
    #[arg(long, default_value_t = false)]
    no_git: bool,
    /// Initialize project without installing the dependencies `default`
    #[arg(short, long, default_value_t = true, visible_alias = "no-deps")]
    offline: bool,

    /// Initialize the project using Javascript instead of TypeScript
    // #[arg(long, disabled = true)]
    // js: bool,

    /// Do not print any message
    #[arg(long)]
    quiet: bool,
    /// Do not create an initial commit
    #[arg(long)]
    no_commit: bool,
    /// Assume 'yes' as the answer to all prompts
    #[arg(short, long)]
    yes: bool,
}

impl Cmd for InitArgs {
    type Output = ();

    fn run(self) -> anyhow::Result<Self::Output> {
        let InitArgs {
            name,
            force,
            template,
            no_git,
            offline,
            quiet,
            no_commit,
            yes,
        } = self;

        let project_dir = name.unwrap_or_else(|| std::env::current_dir().unwrap());
        let project_name = project_dir
            .components()
            .last()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap();

        // ? check if directory exists. if not initialize it
        if !project_dir.exists() {
            if !quiet {
                println!(" initializing {} project", project_dir.display());
            }
            fs::create_dir_all(&project_dir).unwrap();
        }

        // clone a template if specified and init it as the projects own
        if let Some(template) = template {
            // ! check if the git repo project has an xnft.json
            println!("template supplied {template}");
        } else {
            // ? check if cwd is empty
            if project_dir.read_dir().unwrap().next().is_some() {
                if !force {
                    // todo create macro for this
                    if !quiet {
                        println!(
                            "\n ❌ \x1B[31mFAIL!!!{}",
                            format!("\x1B[31m\x1B[0m")
                                + " Directory non-empty"
                                + "\n Run with --force flag to initialize anyway\n"
                        );
                    }
                    return Ok(());
                }
            }

            // ? generating git
            if !no_git {
                //todo -> create macro for this
                if !quiet {
                    println!(" 🐙 generating git");
                }
                init_git_repo(&project_dir, no_commit).unwrap();
            } else {
                //todo -> create macro for this
                if !quiet {
                    println!(" 🐙 generating without git");
                }
            }

            // ? config files
            generate_root_files(&project_dir, project_name, yes)?;

            // ?  installing dependencies
            if !offline {
                //todo -> create macro for this
                if !quiet {
                    println!(" 📥 installing dependencies");
                }
                let yarn_success = install_node_modules("yarn", &project_dir)?;
                //if not successful try installation with npm
                if !yarn_success {
                    println!("Failed yarn install will attempt to npm install");
                    install_node_modules("npm", &project_dir).unwrap();
                }
            } else {
                //todo -> create macro for this
                if !quiet {
                    println!(" 📴 generating without dependencies");
                }
            }

            //crate the directories
            let src = project_dir.join("src"); //pathbuf
            fs::create_dir_all(&src).unwrap();

            let assets = project_dir.join("assets");
            fs::create_dir_all(&assets).unwrap();

            //write the main.tsx file
            let src_file_path = src.join("main.tsx");
            fs::write(src_file_path, include_str!("../assets/main.tsx"))?;

            //copy
            let asset_file_path = assets.join("icon.png");
            fs::copy("./assets/icon.png", asset_file_path)?;
        }

        //todo -> create macro for this
        if !quiet {
            println!(" 🎉 successfully initialized {project_name}")
        }
        Ok(())
    }
}

/// Initializes a git repo for the project directory
///
/// creates a `.gitignore if it does exist already`
///
fn init_git_repo(project_dir: &Path, no_commit: bool) -> anyhow::Result<()> {
    //check if we are in an existing git repo
    if !is_git(&project_dir).unwrap() {
        Command::new("git")
            .arg("init")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .current_dir(project_dir)
            .output()?;
    }

    //.gitignore
    let gitignore = project_dir.join(".gitignore");
    if !gitignore.exists() {
        fs::write(gitignore, include_str!("../assets/.gitignore")).unwrap();
    }

    // commit everything
    if !no_commit {
        Command::new("git")
            .current_dir(project_dir)
            .args(["add", "."])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;

        Command::new("git")
            .args(["commit", "-m", "chore: xnft init"])
            .current_dir(project_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;
    }

    Ok(())
}

/// Returns `true` if `project_dir` is already in an existing git repository
fn is_git(project_dir: &Path) -> anyhow::Result<bool> {
    let is_git = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(project_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()
        .unwrap();

    Ok(is_git.success())
}

/// install node modules with the initialization
fn install_node_modules(package_manager: &str, project_dir: &Path) -> anyhow::Result<bool> {
    let is_installed = Command::new(package_manager)
        .arg("install")
        .current_dir(project_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();

    Ok(is_installed.success())
}

/// generates the config and root files for the project
///
pub fn generate_root_files(
    project_dir: &Path,
    project_name: &str,
    yes: bool,
) -> anyhow::Result<()> {
    //babel.config.js
    let babel_config = project_dir.join("babel.config.js");
    fs::write(babel_config, include_str!("../assets/babel.config.js"))?;

    //tsconfig.json
    let ts_config = project_dir.join("tsconfig.json");
    fs::write(ts_config, include_str!("../assets/tsconfig.json"))?;

    //readme
    let readme_md = project_dir.join("README.md");
    fs::write(readme_md, include_str!("../assets/readme.md"))?;

    //package.json
    let mut package_json_file = fs::File::open("./assets/package.json")?;
    let mut package_json_str = String::new();
    package_json_file
        .read_to_string(&mut package_json_str)
        .unwrap();
    let mut package_json: Value = serde_json::from_str(package_json_str.as_str())?;
    package_json["name"] = json!(project_name);
    let package_json_path = project_dir.join("package.json");
    let package_json_file = fs::File::create(package_json_path)?;

    let buffered_writer = io::BufWriter::new(package_json_file);
    serde_json::to_writer_pretty(buffered_writer, &package_json)?;

    //app.json
    let mut app_json_file = fs::File::open("./assets/app.json")?;
    let mut app_json_str = String::new();
    app_json_file.read_to_string(&mut app_json_str).unwrap();
    let mut app_json: Value = serde_json::from_str(&app_json_str.as_str())?;
    app_json["expo"]["name"] = json!(project_name);
    app_json["expo"]["slug"] = json!(project_name);

    let app_json_path = project_dir.join("app.json");
    let app_json_file = fs::File::create(app_json_path)?;

    let buffered_writer = io::BufWriter::new(app_json_file);
    serde_json::to_writer_pretty(buffered_writer, &app_json)?;

    //xnft.json
    match yes {
        true => {
            let xnft_json = project_dir.join("xnft.json");
            let xnft_json_file = fs::File::create(xnft_json)?;

            let buffered_writer = io::BufWriter::new(xnft_json_file);
            serde_json::to_writer_pretty(buffered_writer, include_str!("../assets/xnft.json"))?;
        }
        false => {
            let description = prompt_user(" 📝 describe your app: ")?;
            let website = prompt_user(" 🌐 enter app website: ")?;
            let contact = prompt_user(" 📞 enter contact details: ")?;

            let mut xnft_json_file = fs::File::open("./assets/xnft.json")?;
            let mut xnft_json_str = String::new();
            xnft_json_file.read_to_string(&mut xnft_json_str)?;
            let mut xnft_json: Value = serde_json::from_str(&xnft_json_str.as_str())?;
            xnft_json["description"] = json!(description);
            xnft_json["name"] = json!(project_name);
            xnft_json["website"] = json!(website);
            xnft_json["contact"] = json!(contact);

            let xnft_json_path = project_dir.join("xnft.json");
            let xnft_json_file = fs::File::create(xnft_json_path)?;

            let buffered_writer = io::BufWriter::new(xnft_json_file);
            serde_json::to_writer_pretty(buffered_writer, &xnft_json)?;
        }
    }

    Ok(())
}

/// prompts for app details
fn prompt_user(prompt: &str) -> anyhow::Result<String> {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut output = String::new();
    io::stdin().read_line(&mut output)?;

    Ok(output.trim_end_matches("\n").to_string())
}
