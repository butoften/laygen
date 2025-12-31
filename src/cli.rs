use clap::Parser;
use anyhow::Result;
use std::path::PathBuf;
use dialoguer::Select;
use crate::i18n::{Language, I18n};
use crate::file_ops::{ensure_directories, navigate_directory};
use crate::generator::{generate_controller, generate_service};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Target directory for code generation
    pub target: PathBuf,

    /// Framework to use (default: axum)
    #[arg(short, long, default_value = "axum")]
    pub framework: String,

    /// Language (en or zh, default: en)
    #[arg(short, long, default_value = "en")]
    pub language: String,

    /// Controller directory name (default: controller)
    #[arg(long, default_value = "controller")]
    pub controller_dir: String,

    /// Service directory name (default: service)
    #[arg(long, default_value = "service")]
    pub service_dir: String,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        // Validate language
        let lang = match self.language.as_str() {
            "en" | "zh" => Language::from_str(&self.language),
            _ => {
                println!("{}", I18n::invalid_language(&self.language));
                let selection = Select::new()
                    .with_prompt(I18n::select_language_prompt())
                    .items(&["en (English)", "zh (中文)"])
                    .default(0)
                    .interact()?;
                
                match selection {
                    0 => Language::En,
                    1 => Language::Zh,
                    _ => Language::En,
                }
            }
        };

        let i18n = I18n::new(lang);

        // Ensure target directory exists
        if !self.target.exists() {
            anyhow::bail!("{}: {}", i18n.target_not_found(), self.target.display());
        }

        // Check and create controller and service directories
        let controller_path = self.target.join(&self.controller_dir);
        let service_path = self.target.join(&self.service_dir);

        let (controller_exists, service_exists) = (
            controller_path.exists(),
            service_path.exists(),
        );

        if !controller_exists || !service_exists {
            let mut missing = Vec::new();
            if !controller_exists {
                missing.push(&self.controller_dir);
            }
            if !service_exists {
                missing.push(&self.service_dir);
            }

            println!("{}", i18n.directories_not_found(&missing));
            
            let options = vec![i18n.create_option(), i18n.cancel_option()];
            let selection = Select::new()
                .with_prompt(i18n.select_action_prompt())
                .items(&options)
                .default(0)
                .interact()?;

            match selection {
                0 => {
                    // Create missing directories
                    ensure_directories(&controller_path, &service_path)?;
                    println!("{}", i18n.directories_created());
                }
                1 => {
                    println!("{}", i18n.task_cancelled(&missing));
                    
                    let final_options = vec![
                        i18n.create_directories_option(&missing),
                        i18n.cancel_and_exit_option(),
                    ];
                    let final_selection = Select::new()
                        .with_prompt(i18n.select_action_prompt())
                        .items(&final_options)
                        .default(0)
                        .interact()?;

                    match final_selection {
                        0 => {
                            ensure_directories(&controller_path, &service_path)?;
                            println!("{}", i18n.directories_created());
                        }
                        1 => {
                            println!("{}", i18n.task_ended());
                            return Ok(());
                        }
                        _ => return Ok(()),
                    }
                }
                _ => return Ok(()),
            }
        }

        // Navigate service directory and get file info first
        println!("\n{}", i18n.navigate_service());
        let service_subdir = navigate_directory(&service_path, &self.service_dir, &i18n)?;
        let service_file_name = self.get_file_name(&i18n, i18n.service_file_prompt())?;
        let service_method_name = self.get_method_name(&i18n, &service_file_name)?;

        // Ask about _service suffix
        let options = vec![
            i18n.add_service_suffix_option(),
            i18n.skip_option(),
        ];
        let selection = Select::new()
            .with_prompt(i18n.service_suffix_prompt())
            .items(&options)
            .default(0)
            .interact()?;

        let final_service_file_name = match selection {
            0 => format!("{}_service", service_file_name),
            _ => service_file_name.clone(),
        };

        // Navigate controller directory and get file info
        println!("\n{}", i18n.navigate_controller());
        let controller_subdir = navigate_directory(&controller_path, &self.controller_dir, &i18n)?;
        let controller_file_name = self.get_file_name(&i18n, i18n.controller_file_prompt())?;
        let controller_method_name = self.get_method_name(&i18n, &controller_file_name)?;

        // Generate controller with service file name
        let controller_full_path = controller_path.join(&controller_subdir);
        generate_controller(
            &controller_full_path,
            &controller_file_name,
            &controller_method_name,
            &final_service_file_name,
            &i18n,
        )?;

        // Generate service
        let service_full_path = service_path.join(&service_subdir);
        generate_service(
            &service_full_path,
            &final_service_file_name,
            &service_method_name,
            &i18n,
        )?;

        println!("\n{}", i18n.generation_complete());
        Ok(())
    }

    fn get_file_name(&self, _i18n: &I18n, prompt: String) -> Result<String> {
        use dialoguer::Input;
        
        let name: String = Input::new()
            .with_prompt(prompt)
            .interact_text()?;
        
        Ok(name.trim().to_string())
    }

    fn get_method_name(&self, i18n: &I18n, file_name: &str) -> Result<String> {
        use dialoguer::Input;
        
        let method_name: String = Input::new()
            .with_prompt(i18n.method_name_prompt())
            .allow_empty(true)
            .interact_text()?;
        
        let method_name = method_name.trim();
        if method_name.is_empty() {
            Ok(file_name.to_string())
        } else {
            Ok(method_name.to_string())
        }
    }
}
