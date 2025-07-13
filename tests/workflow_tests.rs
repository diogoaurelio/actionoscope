use actionoscope::Workflow;

#[test]
fn test_workflow_from_yaml() {
    // Given: A YAML string representing a workflow
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

    // When: The workflow is parsed from the YAML data
    let workflow = Workflow::from_yaml(yaml_data).expect("Failed to parse YAML");

    // Then: The workflow name and job should be correctly parsed
    assert_eq!(workflow.name, "Test Workflow");
    assert!(workflow.get_job("test_job").is_some());
}

#[test]
fn test_get_job() {
    // Given: A YAML string representing a workflow with a job
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

    // When: The workflow is parsed and a job is retrieved
    let workflow = Workflow::from_yaml(yaml_data).expect("Failed to parse YAML");
    let job = workflow.get_job("test_job").expect("Job not found");

    // Then: The job's runs-on field should match the expected value
    assert_eq!(job.runs_on, "ubuntu-latest");
}

#[test]
fn test_get_step() {
    // Given: A YAML string representing a workflow with a job and a step
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

    // When: The workflow is parsed, a job is retrieved, and a step is retrieved
    let workflow = Workflow::from_yaml(yaml_data).expect("Failed to parse YAML");
    let job = workflow.get_job("test_job").expect("Job not found");
    let step = job.get_step("test_step").expect("Step not found");

    // Then: The step's name or ID should match the expected value
    assert_eq!(step.get_name_or_id(), "Test Step");
}

#[test]
fn test_get_all_steps_since() {
    // Given: A YAML string representing a workflow with multiple steps
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

    // When: The workflow is parsed, a job is retrieved, and all steps since a specific step are retrieved
    let workflow = Workflow::from_yaml(yaml_data).expect("Failed to parse YAML");
    let job = workflow.get_job("test_job").expect("Job not found");
    let steps = job.get_all_steps_since(Some("step2"), None);

    // Then: The retrieved steps should match the expected steps
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0].get_name_or_id(), "Step 2");
    assert_eq!(steps[1].get_name_or_id(), "Step 3");
}

#[test]
fn test_get_all_steps_since_and_until() {
    // Given: A YAML string representing a workflow with multiple steps
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
          - name: Step 4
            id: step4
            run: echo "Step 4"
    "#;

    // When: The workflow is parsed, a job is retrieved, and all steps between two specific steps are retrieved
    let workflow = Workflow::from_yaml(yaml_data).expect("Failed to parse YAML");
    let job = workflow.get_job("test_job").expect("Job not found");
    let steps = job.get_all_steps_since(Some("step2"), Some("step3"));

    // Then: The retrieved steps should match the expected steps
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0].get_name_or_id(), "Step 2");
    assert_eq!(steps[1].get_name_or_id(), "Step 3");
}
