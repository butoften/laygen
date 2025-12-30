#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    En,
    Zh,
}

impl Language {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "zh" => Language::Zh,
            _ => Language::En,
        }
    }
}

pub struct I18n {
    lang: Language,
}

impl I18n {
    pub fn new(lang: Language) -> Self {
        Self { lang }
    }

    pub fn invalid_language(input: &str) -> String {
        format!("Invalid language: {}. Please choose 'en' or 'zh'.", input)
    }

    pub fn select_language_prompt() -> String {
        "Please select a language / 请选择语言".to_string()
    }

    pub fn target_not_found(&self) -> String {
        match self.lang {
            Language::En => "Target directory not found".to_string(),
            Language::Zh => "目标目录不存在".to_string(),
        }
    }

    pub fn directories_not_found(&self, dirs: &[&String]) -> String {
        let dir_list = dirs.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        match self.lang {
            Language::En => format!("Directory not found: {}", dir_list),
            Language::Zh => format!("未找到目录: {}", dir_list),
        }
    }

    pub fn create_option(&self) -> String {
        match self.lang {
            Language::En => "Create".to_string(),
            Language::Zh => "新建".to_string(),
        }
    }

    pub fn cancel_option(&self) -> String {
        match self.lang {
            Language::En => "Cancel".to_string(),
            Language::Zh => "放弃".to_string(),
        }
    }

    pub fn select_action_prompt(&self) -> String {
        match self.lang {
            Language::En => "Please select an action".to_string(),
            Language::Zh => "请选择操作".to_string(),
        }
    }

    pub fn directories_created(&self) -> String {
        match self.lang {
            Language::En => "Directories created successfully".to_string(),
            Language::Zh => "目录创建成功".to_string(),
        }
    }

    pub fn task_cancelled(&self, dirs: &[&String]) -> String {
        let dir_list = dirs.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        match self.lang {
            Language::En => format!("User cancelled creation of: {}", dir_list),
            Language::Zh => format!("用户放弃创建: {}", dir_list),
        }
    }

    pub fn create_directories_option(&self, dirs: &[&String]) -> String {
        let dir_list = dirs.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        match self.lang {
            Language::En => format!("Create {} directories", dir_list),
            Language::Zh => format!("创建 {} 目录", dir_list),
        }
    }

    pub fn cancel_and_exit_option(&self) -> String {
        match self.lang {
            Language::En => "Cancel and exit, task ended".to_string(),
            Language::Zh => "放弃创建，本次任务结束".to_string(),
        }
    }

    pub fn task_ended(&self) -> String {
        match self.lang {
            Language::En => "Task ended".to_string(),
            Language::Zh => "本次任务已经结束".to_string(),
        }
    }

    pub fn navigate_controller(&self) -> String {
        match self.lang {
            Language::En => "=== Navigate Controller Directory ===".to_string(),
            Language::Zh => "=== 导航控制器目录 ===".to_string(),
        }
    }

    pub fn navigate_service(&self) -> String {
        match self.lang {
            Language::En => "=== Navigate Service Directory ===".to_string(),
            Language::Zh => "=== 导航服务目录 ===".to_string(),
        }
    }

    pub fn current_path(&self) -> String {
        match self.lang {
            Language::En => "Current path".to_string(),
            Language::Zh => "当前路径".to_string(),
        }
    }

    pub fn done_option(&self) -> String {
        match self.lang {
            Language::En => "✓ Done (create file here)".to_string(),
            Language::Zh => "✓ 选好了（在此创建文件）".to_string(),
        }
    }

    pub fn select_directory_prompt(&self) -> String {
        match self.lang {
            Language::En => "Select a directory or choose 'Done'".to_string(),
            Language::Zh => "选择目录或选择'选好了'".to_string(),
        }
    }

    pub fn controller_file_prompt(&self) -> String {
        match self.lang {
            Language::En => "Enter controller file name (without .rs)".to_string(),
            Language::Zh => "请输入控制器文件名称（不含.rs）".to_string(),
        }
    }

    pub fn service_file_prompt(&self) -> String {
        match self.lang {
            Language::En => "Enter service file name (without .rs)".to_string(),
            Language::Zh => "请输入服务文件名称（不含.rs）".to_string(),
        }
    }

    pub fn method_name_prompt(&self) -> String {
        match self.lang {
            Language::En => "Enter method name (leave empty to use file name)".to_string(),
            Language::Zh => "请输入方法名（如果不填写，则与文件名相同）".to_string(),
        }
    }

    pub fn add_service_suffix_option(&self) -> String {
        match self.lang {
            Language::En => "Add '_service' suffix to file name".to_string(),
            Language::Zh => "自动给文件名加 _service 后缀".to_string(),
        }
    }

    pub fn skip_option(&self) -> String {
        match self.lang {
            Language::En => "Skip".to_string(),
            Language::Zh => "跳过".to_string(),
        }
    }

    pub fn service_suffix_prompt(&self) -> String {
        match self.lang {
            Language::En => "Add service suffix?".to_string(),
            Language::Zh => "是否添加服务后缀？".to_string(),
        }
    }

    pub fn generation_complete(&self) -> String {
        match self.lang {
            Language::En => "✓ Code generation completed successfully!".to_string(),
            Language::Zh => "✓ 代码生成完成！".to_string(),
        }
    }

    pub fn file_created(&self, path: &str) -> String {
        match self.lang {
            Language::En => format!("✓ Created: {}", path),
            Language::Zh => format!("✓ 已创建: {}", path),
        }
    }

    pub fn mod_updated(&self, path: &str) -> String {
        match self.lang {
            Language::En => format!("✓ Updated: {}", path),
            Language::Zh => format!("✓ 已更新: {}", path),
        }
    }

    pub fn no_subdirectories(&self) -> String {
        match self.lang {
            Language::En => "No subdirectories found, creating file in current directory".to_string(),
            Language::Zh => "未找到子目录，将在当前目录创建文件".to_string(),
        }
    }
}
