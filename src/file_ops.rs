use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use dialoguer::Select;
use crate::i18n::I18n;

pub fn ensure_directories(controller_path: &Path, service_path: &Path) -> Result<()> {
    if !controller_path.exists() {
        fs::create_dir_all(controller_path)?;
    }
    if !service_path.exists() {
        fs::create_dir_all(service_path)?;
    }
    Ok(())
}

pub fn navigate_directory(base_path: &Path, base_dir_name: &str, i18n: &I18n) -> Result<PathBuf> {
    let mut current_path = base_path.to_path_buf();
    
    loop {
        // Get subdirectories
        let subdirs = get_subdirectories(&current_path)?;
        
        if subdirs.is_empty() {
            // Show current path even when no subdirectories
            let relative_path = current_path.strip_prefix(base_path)
                .unwrap_or(Path::new(""))
                .display()
                .to_string();
            let display_path = if relative_path.is_empty() {
                format!("{}/", base_dir_name)
            } else {
                format!("{}/{}/", base_dir_name, relative_path)
            };
            
            println!("\n{}: {}", i18n.current_path(), display_path);
            println!("{}", i18n.no_subdirectories());
            return Ok(current_path.strip_prefix(base_path)?.to_path_buf());
        }

        // Build options: "Done" + subdirectories
        let mut options = vec![i18n.done_option()];
        options.extend(subdirs.iter().map(|d| d.to_string_lossy().to_string()));

        // Show current path with base directory name
        let relative_path = current_path.strip_prefix(base_path)
            .unwrap_or(Path::new(""))
            .display()
            .to_string();
        let display_path = if relative_path.is_empty() {
            format!("{}/", base_dir_name)
        } else {
            format!("{}/{}/", base_dir_name, relative_path)
        };
        
        println!("\n{}: {}", i18n.current_path(), display_path);

        let selection = Select::new()
            .with_prompt(i18n.select_directory_prompt())
            .items(&options)
            .default(0)
            .interact()?;

        if selection == 0 {
            // User selected "Done"
            return Ok(current_path.strip_prefix(base_path)?.to_path_buf());
        } else {
            // Navigate into selected subdirectory
            current_path = current_path.join(&subdirs[selection - 1]);
        }
    }
}

fn get_subdirectories(path: &Path) -> Result<Vec<PathBuf>> {
    let mut subdirs = Vec::new();
    
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    subdirs.push(PathBuf::from(name));
                }
            }
        }
    }
    
    subdirs.sort();
    Ok(subdirs)
}

pub fn update_mod_file(dir_path: &Path, module_name: &str) -> Result<()> {
    let mod_file = dir_path.join("mod.rs");
    let mod_line = format!("pub mod {};\n", module_name);
    
    if mod_file.exists() {
        // Check if module already exists
        let content = fs::read_to_string(&mod_file)?;
        if !content.contains(&format!("pub mod {}", module_name)) {
            fs::write(&mod_file, format!("{}{}", content, mod_line))?;
        }
    } else {
        // Create new mod.rs
        fs::write(&mod_file, mod_line)?;
    }
    
    Ok(())
}
