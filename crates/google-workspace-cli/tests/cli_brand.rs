// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.

use std::process::Command;

fn xgc() -> Command {
    Command::new(env!("CARGO_BIN_EXE_xgc"))
}

#[test]
fn version_identifies_xgc_distribution() {
    let output = xgc().arg("--version").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.starts_with("xgc "));
    assert!(stdout.contains("xliner’s GWS-CLI"));
}

#[test]
fn help_uses_xgc_and_documents_profiles() {
    let output = xgc().arg("--help").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("xgc — xliner’s GWS-CLI"));
    assert!(stdout.contains("--profile <PROFILE>"));
    assert!(stdout.contains("XGC_PROFILE"));
    assert!(!stdout.contains("Usage: gws"));
}

#[test]
fn auth_status_uses_named_profile_directory() {
    let config = tempfile::tempdir().unwrap();
    let output = xgc()
        .env("XGC_CONFIG_DIR", config.path())
        .args(["auth", "status", "--profile", "personal"])
        .output()
        .unwrap();
    assert!(output.status.success());

    let status: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(status["profile"], "personal");
    assert_eq!(
        status["profile_dir"],
        config
            .path()
            .join("profiles/personal")
            .display()
            .to_string()
    );
    assert_eq!(
        status["encrypted_credentials"],
        config
            .path()
            .join("profiles/personal/credentials.enc")
            .display()
            .to_string()
    );
    assert_eq!(
        status["token_cache"],
        config
            .path()
            .join("profiles/personal/token_cache.json")
            .display()
            .to_string()
    );
}

#[test]
fn auth_status_defaults_to_config_root() {
    let config = tempfile::tempdir().unwrap();
    let output = xgc()
        .env("XGC_CONFIG_DIR", config.path())
        .env_remove("XGC_PROFILE")
        .args(["auth", "status"])
        .output()
        .unwrap();
    assert!(output.status.success());

    let status: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(status["profile"], "default");
    assert_eq!(status["profile_dir"], config.path().display().to_string());
    assert_eq!(
        status["encrypted_credentials"],
        config.path().join("credentials.enc").display().to_string()
    );
    assert_eq!(
        status["token_cache"],
        config.path().join("token_cache.json").display().to_string()
    );
}

#[test]
fn invalid_profile_is_rejected_before_auth_access() {
    let output = xgc()
        .args(["auth", "status", "--profile", "../../escape"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Profile names"));
}
