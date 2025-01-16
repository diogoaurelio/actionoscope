use actionoscope::Workflow;

#[test]
fn test_workflow_from_yaml() {
    let yaml_data = r#"
    name: Test Workflow
    on:
      push:
        branches:
          - main
    jobs:
      test_job:
        runs-on: ubuntu-latest
        steps:
          - name: Test Step
            run: echo "Hello, world!"
    "#;

    let workflow = Workflow::from_yaml(yaml_data).expect("Failed to parse YAML");
    assert_eq!(workflow.name, "Test Workflow");
    assert!(workflow.get_job("test_job").is_some());
}

#[test]
fn test_get_job() {
    let yaml_data = r#"
    name: Test Workflow
    on:
      push:
        branches:
          - main
    jobs:
      test_job:
        runs-on: ubuntu-latest
        steps:
          - name: Test Step
            run: echo "Hello, world!"
    "#;

    let workflow = Workflow::from_yaml(yaml_data).expect("Failed to parse YAML");
    let job = workflow.get_job("test_job").expect("Job not found");
    assert_eq!(job.runs_on, "ubuntu-latest");
}

#[test]
fn test_get_step() {
    let yaml_data = r#"
    name: Test Workflow
    on:
      push:
        branches:
          - main
    jobs:
      test_job:
        runs-on: ubuntu-latest
        steps:
          - name: Test Step
            id: test_step
            run: echo "Hello, world!"
    "#;

    let workflow = Workflow::from_yaml(yaml_data).expect("Failed to parse YAML");
    let job = workflow.get_job("test_job").expect("Job not found");
    let step = job.get_step("test_step").expect("Step not found");
    assert_eq!(step.get_name_or_id(), "Test Step");
}

#[test]
fn test_get_all_steps_since() {
    let yaml_data = r#"
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
    "#;

    let workflow = Workflow::from_yaml(yaml_data).expect("Failed to parse YAML");
    let job = workflow.get_job("test_job").expect("Job not found");
    let steps = job.get_all_steps_since("step2");
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0].get_name_or_id(), "Step 2");
    assert_eq!(steps[1].get_name_or_id(), "Step 3");
}
