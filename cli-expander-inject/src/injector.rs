#[derive(Debug)]
pub enum InjectionMethod {
    Uinput,
    Clipboard,
    TmuxSendKeys,
    Stdout,
}

pub trait Injector: Send {
    fn inject(&self, text: &str) -> anyhow::Result<()>;
    fn method(&self) -> InjectionMethod;
}

pub struct UinputInjector;

impl Injector for UinputInjector {
    fn inject(&self, text: &str) -> anyhow::Result<()> {
        match std::env::var("TMUX") {
            Ok(_) => {
                let output = std::process::Command::new("tmux")
                    .args(["send-keys", "-l", text])
                    .output()
                    .map_err(|e| anyhow::anyhow!("tmux send-keys failed: {}", e))?;
                if !output.status.success() {
                    anyhow::bail!("tmux send-keys exited with error");
                }
                Ok(())
            }
            Err(_) => {
                // Fallback: use ydotool if available
                if which("ydotool") {
                    let output = std::process::Command::new("ydotool")
                        .args(["type", text])
                        .output()
                        .map_err(|e| anyhow::anyhow!("ydotool failed: {}", e))?;
                    if !output.status.success() {
                        anyhow::bail!("ydotool exited with error");
                    }
                    return Ok(());
                }
                anyhow::bail!("No injection method available. Install ydotool or run inside tmux.");
            }
        }
    }

    fn method(&self) -> InjectionMethod {
        InjectionMethod::Uinput
    }
}

pub struct TmuxInjector;

#[derive(Debug, Clone, Default)]
pub struct TmuxInjectOptions {
    pub target_pane: Option<String>,
}

impl TmuxInjector {
    pub fn inject_with_options(
        &self,
        text: &str,
        options: &TmuxInjectOptions,
    ) -> anyhow::Result<()> {
        let args = tmux_send_keys_args(text, options.target_pane.as_deref())?;
        let output = std::process::Command::new("tmux")
            .args(args)
            .output()
            .map_err(|e| anyhow::anyhow!("tmux send-keys failed: {}", e))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            if stderr.is_empty() {
                anyhow::bail!("tmux send-keys exited with error");
            }
            anyhow::bail!("tmux send-keys exited with error: {}", stderr);
        }
        Ok(())
    }
}

impl Injector for TmuxInjector {
    fn inject(&self, text: &str) -> anyhow::Result<()> {
        self.inject_with_options(text, &TmuxInjectOptions::default())
    }

    fn method(&self) -> InjectionMethod {
        InjectionMethod::TmuxSendKeys
    }
}

pub struct ClipboardInjector;

impl Injector for ClipboardInjector {
    fn inject(&self, text: &str) -> anyhow::Result<()> {
        let mut clipboard = arboard::Clipboard::new()
            .map_err(|e| anyhow::anyhow!("Failed to open clipboard: {}", e))?;
        clipboard
            .set_text(text)
            .map_err(|e| anyhow::anyhow!("Failed to set clipboard text: {}", e))?;
        Ok(())
    }

    fn method(&self) -> InjectionMethod {
        InjectionMethod::Clipboard
    }
}

fn which(name: &str) -> bool {
    std::process::Command::new("which")
        .arg(name)
        .output()
        .is_ok_and(|o| o.status.success())
}

pub fn tmux_send_keys_args(text: &str, target_pane: Option<&str>) -> anyhow::Result<Vec<String>> {
    let mut args = vec!["send-keys".to_string()];
    if let Some(target) = target_pane {
        if target.is_empty() {
            anyhow::bail!("tmux target pane cannot be empty");
        }
        args.push("-t".to_string());
        args.push(target.to_string());
    }
    args.push("-l".to_string());
    args.push(text.to_string());
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_inject() {
        let injector = ClipboardInjector;
        let result = injector.inject("test text");
        // May fail if no display server, but shouldn't crash
        if let Err(e) = result {
            assert!(e.to_string().contains("clipboard"));
        }
    }

    #[test]
    fn test_injection_method_display() {
        assert_eq!(format!("{:?}", InjectionMethod::Uinput), "Uinput");
        assert_eq!(format!("{:?}", InjectionMethod::Clipboard), "Clipboard");
    }

    #[test]
    fn test_tmux_send_keys_args_without_target() {
        let args = tmux_send_keys_args("hello", None).unwrap();
        assert_eq!(args, vec!["send-keys", "-l", "hello"]);
    }

    #[test]
    fn test_tmux_send_keys_args_with_target() {
        let args = tmux_send_keys_args("hello", Some("%1")).unwrap();
        assert_eq!(args, vec!["send-keys", "-t", "%1", "-l", "hello"]);
    }

    #[test]
    fn test_tmux_send_keys_args_rejects_empty_target() {
        let err = tmux_send_keys_args("hello", Some("")).unwrap_err();
        assert!(err.to_string().contains("target pane cannot be empty"));
    }
}
