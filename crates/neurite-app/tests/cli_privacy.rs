//! Verify that routine HQ previews never call Jev, and actual Jev-assisted
//! runs require an independent operator-provided routing summary.

use std::io::Write;
use std::process::{Command, Output, Stdio};

fn call_hq(option: &str, message: &str, extra_env: &[(&str, &str)]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_neurite-app"));
    command
        .arg(option)
        .env("HQ_PROVIDER", "ollama")
        .env("HQ_MODEL", "not-a-loaded-model")
        .env("HQ_BASE_URL", "http://127.0.0.1:11434")
        .env("HQ_NOTE_TEXT", "private context must stay local")
        .env_remove("HQ_JEV_KEY_FILE")
        .env_remove("HQ_JEV_TASK_SUMMARY")
        .env_remove("HQ_TEXT_FILE")
        .env_remove("HQ_SECONDARY_PROVIDER")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (key, value) in extra_env {
        command.env(key, value);
    }
    let mut process = command.spawn().unwrap();
    process
        .stdin
        .take()
        .unwrap()
        .write_all(message.as_bytes())
        .unwrap();
    process.wait_with_output().unwrap()
}

#[test]
fn preview_is_offline_even_when_jev_key_file_is_unavailable() {
    let output = call_hq(
        "--agent-preview",
        "Answer my actual confidential question",
        &[("HQ_JEV_KEY_FILE", "this-path-intentionally-does-not-exist")],
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("private context must stay local"));
    assert!(text.contains("Answer my actual confidential question"));
    assert!(text.contains("hq.provider.ollama.primary"));
    assert!(text.contains("Jev choice deferred"));
}

#[test]
fn jev_assisted_run_refuses_to_forward_user_prompt_as_summary() {
    let output = call_hq(
        "--agent-run",
        "CONFIDENTIAL FULL USER PROMPT",
        &[("HQ_JEV_KEY_FILE", "this-path-intentionally-does-not-exist")],
    );
    assert!(!output.status.success());
    let diagnostics = String::from_utf8(output.stderr).unwrap();
    assert!(diagnostics.contains("HQ_JEV_TASK_SUMMARY"));
    assert!(!diagnostics.contains("CONFIDENTIAL FULL USER PROMPT"));
}
