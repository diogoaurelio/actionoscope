use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_run_single_step() {
    let mut cmd = Command::cargo_bin("actionoscope").unwrap();
    cmd.arg("run")
        .arg("--workflow-file")
        .arg("test_workflow.yml")
        .arg("--job")
        .arg("test_job")
        .arg("--step")
        .arg("step1");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Running step name/id 'Step 1'"));
}

#[test]
fn test_run_single_step_invalid_job() {
    let mut cmd = Command::cargo_bin("actionoscope").unwrap();
    cmd.arg("run")
        .arg("--workflow-file")
        .arg("test_workflow.yml")
        .arg("--job")
        .arg("invalid_job")
        .arg("--step")
        .arg("step1");

    cmd.assert().failure();
}

#[test]
fn test_run_single_step_invalid_step() {
    let mut cmd = Command::cargo_bin("actionoscope").unwrap();
    cmd.arg("run")
        .arg("--workflow-file")
        .arg("test_workflow.yml")
        .arg("--job")
        .arg("test_job")
        .arg("--step")
        .arg("invalid_step");

    cmd.assert().failure();
}

#[test]
fn test_run_all_steps_since_invalid_step() {
    let mut cmd = Command::cargo_bin("actionoscope").unwrap();
    cmd.arg("run")
        .arg("--workflow-file")
        .arg("test_workflow.yml")
        .arg("--job")
        .arg("test_job")
        .arg("--from-step")
        .arg("invalid_step");

    cmd.assert().failure();
}

#[test]
fn test_run_all_steps_since() {
    let mut cmd = Command::cargo_bin("actionoscope").unwrap();
    cmd.arg("run")
        .arg("--workflow-file")
        .arg("test_workflow.yml")
        .arg("--job")
        .arg("test_job")
        .arg("--from-step")
        .arg("step2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Running step name/id 'Step 2'"))
        .stdout(predicate::str::contains("Running step name/id 'Step 3'"));
}

fn setup_test_workflow() {
    let workflow_content = r#"
    name: Test Workflow
    on:
      push:
        branches:
          - main
    jobs:
      test_job:
        runs-on: ubuntu-latest
        steps:
          - name: Step 1
            id: step1
            run: echo "Step 1"
          - name: Step 2
            id: step2
            run: echo "Step 2"
          - name: Step 3
            id: step3
            run: echo "Step 3"
          - name: Step 4
            id: step4
            run: echo "Step 4"
    "#;

    fs::write("test_workflow.yml", workflow_content).unwrap();
}

#[ctor::ctor]
fn init() {
    setup_test_workflow();
}
