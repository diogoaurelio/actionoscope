use actionoscope::Step;

#[test]
fn test_get_name_or_id_with_name() {
    // Given: A step with a name and no ID
    let step = Step {
        name: Some(String::from("Test Step")),
        id: None,
        uses: None,
        shell: None,
        working_directory: None,
        run: None,
    };

    // When: The method get_name_or_id is called
    let result = step.get_name_or_id();

    // Then: The result should be the step's name
    assert_eq!(result, "Test Step");
}

#[test]
fn test_get_name_or_id_with_id() {
    // Given: A step with an ID and no name
    let step = Step {
        name: None,
        id: Some(String::from("test_step")),
        uses: None,
        shell: None,
        working_directory: None,
        run: None,
    };

    // When: The method get_name_or_id is called
    let result = step.get_name_or_id();

    // Then: The result should be the step's ID
    assert_eq!(result, "test_step");
}

#[test]
fn test_get_name_or_id_with_name_and_id() {
    // Given: A step with both a name and an ID
    let step = Step {
        name: Some(String::from("Test Step")),
        id: Some(String::from("test_step")),
        uses: None,
        shell: None,
        working_directory: None,
        run: None,
    };

    // When: The method get_name_or_id is called
    let result = step.get_name_or_id();

    // Then: The result should be the step's name
    assert_eq!(result, "Test Step");
}

#[test]
fn test_get_name_or_id_with_none() {
    // Given: A step with neither a name nor an ID
    let step = Step {
        name: None,
        id: None,
        uses: None,
        shell: None,
        working_directory: None,
        run: None,
    };

    // When: The method get_name_or_id is called
    let result = step.get_name_or_id();

    // Then: The result should be "unknown"
    assert_eq!(result, "unknown");
}

#[test]
fn test_run_cmd_with_valid_command() {
    // Given: A step with a valid shell and run command
    let step = Step {
        name: Some(String::from("Test Step")),
        id: Some(String::from("test_step")),
        uses: None,
        shell: Some(String::from("echo")),
        working_directory: None,
        run: Some(String::from("Hello, world!")),
    };

    // When: The method run_cmd is called
    let result = step.run_cmd(None, None);

    // Then: The result should indicate success
    assert!(result.is_ok());
}

#[test]
fn test_run_cmd_with_invalid_command() {
    // Given: A step with an invalid shell command
    let step = Step {
        name: Some(String::from("Test Step")),
        id: Some(String::from("test_step")),
        uses: None,
        shell: Some(String::from("invalid_shell")),
        working_directory: None,
        run: Some(String::from("Hello, world!")),
    };

    // When: The method run_cmd is called
    let result = step.run_cmd(None, None);

    // Then: The result should indicate an error
    assert!(result.is_err());
}

#[test]
fn test_run_cmd_with_no_run_command() {
    // Given: A step with a shell but no run command
    let step = Step {
        name: Some(String::from("Test Step")),
        id: Some(String::from("test_step")),
        uses: None,
        shell: Some(String::from("echo")),
        working_directory: None,
        run: None,
    };

    // When: The method run_cmd is called
    let result = step.run_cmd(None, None);

    // Then: The result should indicate an error
    assert!(result.is_err());
}
