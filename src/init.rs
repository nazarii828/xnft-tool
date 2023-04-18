use clap::{Parser, ValueHint};
use std::{
    fs,
    os::unix::process::CommandExt,
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
    /// Initialize the project without installing the dependencies
    #[arg(short, long)]
    offline: bool,
    /// Initialize the project using Javascript instead of TypeScript
    #[arg(long)]
    js: bool,
    /// Do not print any message
    #[arg(long)]
    quiet: bool,
    /// Do not create an initial commit
    #[arg(long)]
    no_commit: bool,
}

impl Cmd for InitArgs {
    type Output = ();

    fn run(self) -> Result<Self::Output, ()> {
        let InitArgs {
            name,
            force,
            template,
            no_git,
            offline,
            js,
            quiet,
            no_commit,
        } = self;

        let project_dir = name.unwrap_or_else(|| std::env::current_dir().unwrap());

        // ? check if directory exists. if not initialize it
        if !project_dir.exists() {
            if !quiet {
                println!("initializing new directory {}", project_dir.display());
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
                    // fail
                    if !quiet {
                        println!(
                            "\n ❌ \x1B[31mFAIL!!!{}",
                            format!("\x1B[31m\x1B[0m")
                                + " Directory non-empty"
                                + "\nRun with --fail flag to initialize anyway\n"
                        );
                    }
                    return Ok(());
                }
            }

            // ? initialize project with javascript if specified
            if !js {
                initialize_ts_project_files(quiet)
            } else {
                initialize_js_project_files(quiet)
            }

            // ? generating git
            if !no_git {
                println!(" 🐙 generating git");
				init_git_repo(&project_dir, no_commit);
            } else {
                println!(" 🐙 generating without git");
            }

            // ?  installing dependencies
            if !offline {
                println!(" 📥 installing dependencies")
            } else {
                println!(" 📴 generating without dependencies")
            }
        }

        Ok(())
    }
}

fn initialize_js_project_files(quiet: bool) {
    if !quiet {
        println!(" 🎉 initializing javascript project")
    }
}

fn initialize_ts_project_files(quiet: bool) {
    if !quiet {
        println!(" 🎉 initializing typescript project")
    }
}

/// Initializes a git repo for the project directory
/// 
/// creates a `.gitignore if it does exist already`
/// 
fn init_git_repo(project_dir: &Path, no_commit: bool) {
    //check if we are in an existing git repo
    if !is_git(&project_dir).unwrap() {
        Command::new("git")
            .arg("init")
            .current_dir(project_dir)
            .exec();
    }

    //.gitignore
	let gitignore = project_dir.join(".gitignore");
	if !gitignore.exists() {
		fs::write(gitignore, include_str!("../assets/.gitignoreTemplate")).unwrap();
	}

    // commit everything
    if !no_commit {
        Command::new("git")
            .args(["add", "."])
            .current_dir(project_dir)
            .exec();
        Command::new("git")
            .args(["commit", "-m", "chore: xnft init"])
            .current_dir(project_dir)
            .exec();
    }
}

/// Returns `true` if `project_dir` is already in an existing git repository
fn is_git(project_dir: &Path) -> Result<bool, ()> {
    let is_git = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(project_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()
        .unwrap();

    Ok(is_git.success())
}
