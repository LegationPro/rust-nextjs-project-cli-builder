use core::fmt;
use std::path::PathBuf;
use clap::Parser;
use colored::Colorize;
use inquire::Select;
mod os_methods;

#[derive(Parser)]
struct NextApp {
    dir:                    PathBuf,
    use_vscode:             bool,
    use_eslint:             bool,
    lang:                   String,
    app_name:               String,
    use_tailwind:           bool,
    use_current_dir:        bool,
    package_manager:        String,
    coding_langs:           Vec<String>,
    package_managers:       Vec<String>,
    use_tailwind_opts:      Vec<String>,
    use_vscode_dir_opts:    Vec<String>,
    use_eslint_dir_opts:    Vec<String>,
    use_current_dir_opts:   Vec<String>,
}

impl Default for NextApp {
    fn default() -> Self {
        return NextApp {
            dir:                    os_methods::get_current_working_dir().unwrap(),
            use_vscode:             false,
            use_eslint:             false, 
            lang:                   String::from("JS"),
            app_name:               "".to_string(),
            use_tailwind:           true,
            use_current_dir:        true,
            package_manager:        String::from("npm"),
            coding_langs:           vec!["JS".bright_yellow().to_string(), "TS".bright_blue().to_string()],
            package_managers:       vec!["NPM".red().to_string(), "Yarn".bright_yellow().to_string()],
            use_tailwind_opts:      vec!["Yes".bright_green().to_string(), "No".bright_red().to_string()],
            use_vscode_dir_opts:    vec!["Yes".bright_green().to_string(), "No".bright_red().to_string()],
            use_eslint_dir_opts:    vec!["Yes".bright_green().to_string(), "No".bright_red().to_string()],
            use_current_dir_opts:   vec!["Yes".bright_green().to_string(), "No".bright_red().to_string()],
        }
    }
}

impl NextApp {
    fn app_name_check(&mut self, app_name: String) -> Option<String> {
        if app_name == "\n" {
            let mut new_app_name = String::new();
            println!("{}", "Please name your Application:".bright_green());
            std::io::stdin().read_line(&mut new_app_name).unwrap();

            if new_app_name == "\n" {
                self.app_name_check(app_name.clone());
            } else {
                self.app_name = new_app_name;
            }
        }
        Some(app_name)
    }
    
    fn build_project_info(&mut self) {
        let s = format!("{} {}", "Choose your preferred".bright_white(), "Package Manager:".bright_yellow());
        self.package_manager = Select::new(s.as_str(), self.package_managers.clone())
            .prompt()
            .unwrap_or_else(|err| {
                panic!("Error: {}", err.to_string().red())
            }
        );

        let s = format!("{} {}", "Choose your preferred".bright_white(), "Programming Language:".bright_purple());
        self.lang = Select::new(s.as_str(), self.coding_langs.clone())
            .prompt()
            .unwrap_or_else(|err| {
                panic!("Error: {}", err.to_string().red())
            }
        );

        let s = format!("{} {}", "Do you wish to add".blue(), "TailwindCSS?".bright_blue());
        let should_use_tailwind = Select::new(s.as_str(), self.use_tailwind_opts.clone())
            .prompt()
            .unwrap_or_else(|err| {
                panic!("Error: {}", err.to_string().red())
            }
        );

        if should_use_tailwind == "Yes" {
            self.use_tailwind = true;
        } else if should_use_tailwind == "False" {
            self.use_tailwind = false;
        }

        let s = format!("{} {}", "Do you wish to use the current".bright_white(), "Working Directory?:".bright_magenta());
        let use_current_dir = Select::new(s.as_str(), self.use_current_dir_opts.clone())
            .prompt()
            .unwrap_or_else(|err| {
                panic!("Error: {}", err.to_string().red())
            }
        );

        let s = format!("{} {} {}", "Do you wish to use ".bright_white(), "VS Code".bright_cyan(), "as your Primary Code Editor?".bright_white());
        let using_vscode = Select::new(s.as_str(), self.use_vscode_dir_opts.clone())
            .prompt()
            .unwrap_or_else(|err| {
                panic!("Error: {}", err.to_string().red())
            }
        );
        
        if using_vscode == "Yes" {
            self.use_vscode = true;
        } else if using_vscode == "False" {
            self.use_vscode = false;
        }
        
        let s = format!("{} {}", "Do you wish to use ".bright_white(), "ESLint".bright_cyan());
        let using_eslint = Select::new(s.as_str(), self.use_eslint_dir_opts.clone())
            .prompt()
            .unwrap_or_else(|err| {
                panic!("Error: {}", err.to_string().red())
            }
        );

        if using_eslint == "Yes" {
            self.use_eslint = true;
        } else if using_eslint == "False" {
            self.use_eslint = false;
        }

        let mut new_app_name = String::new();
        println!("{}", "Application Name:".bright_green());
        std::io::stdin().read_line(&mut new_app_name).unwrap();

        self.app_name = new_app_name;
        self.app_name_check(self.app_name.clone()).unwrap();

        if use_current_dir == "Yes" {
            self.dir = std::env::current_dir().unwrap()
        } else if use_current_dir == "no" {
            let mut user_path_input = String::new();
            println!("Type in the {} you wish your app to use: \n", "directory".blue());
            std::io::stdin().read_line(&mut user_path_input).unwrap();
            self.dir = PathBuf::from(user_path_input);
        }

        let nodejs = os_methods::has_nodejs(); if !nodejs {
            return println!("Error: {}", "Please install the latest version of Nodejs. https://nodejs.org/en/".red());
        }

        self.create();
    }

    fn should_lint(&self) -> &str {
        if self.use_eslint {
            return "--eslint"
        } else {
            return "--no-eslint"
        }
    }

    fn build_with_ts(&self) {
        let lint = self.should_lint();
        let use_pkg = format!("--use-{}", self.package_manager);
        let next_app_args = format!("npx create-next-app@latest {} {} {} {}", self.app_name, use_pkg, lint, String::from("--ts"));

        let output = os_methods::native_cmd_executor()
            .arg("-c")
            .arg(next_app_args).output().unwrap();

        let build_success = os_methods::output_status(&output); if !build_success {
            println!("{}", "Error: Something went wrong".red());
        }
    }

    fn build_with_js(&self) {
        let lint = self.should_lint();
        let use_pkg = format!("--use-{}", self.package_manager);
        let next_app_args = format!("npx create-next-app@latest {} {} {} {}", self.app_name.trim(), use_pkg, lint, String::from("--js"));

        let output = os_methods::native_cmd_executor()
            .arg("-c")
            .arg(next_app_args).spawn().unwrap();

        let res = output.wait_with_output().unwrap();
        os_methods::output_status(&res);
    }

    fn open_with_vscode(&self) {
        if self.use_vscode {
            let vs_success = os_methods::open_with_vs_code();

            if !vs_success {
                println!("{}", "Could not find a copy of Visual Studio Code. Please, install it.\n");
                println!("{}", "https://code.visualstudio.com/".bright_cyan());
                return
            }
        }
    }

    fn create(&self) {
        if self.package_manager == "yarn" {
            os_methods::handle_yarn_installation();
            println!("{}", "Building NextJs project...".blue());
        }

        if self.lang == "ts".to_string() {
            self.build_with_ts();
        } else {
            self.build_with_js();
        }
    }
}

#[derive(Parser)]
struct Cli {
    project: String,
    dir: PathBuf,
}

impl fmt::Display for Cli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "project: {}, dir: {}", self.project, std::path::PathBuf::from(&self.dir).display())
    }
}

fn main() {
    let args = Cli::parse();

    let project = match args.project.as_str() {
        "next_app" => Some(NextApp::default()),
        _ => None,
    };

    if project.is_none() {
        println!("{} Project {} doesn't exist.", "ERROR:".red(), args.project.blue());
        return
    }

    let mut project = project.unwrap();
    project.build_project_info();
}