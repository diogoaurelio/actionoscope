use actionoscope::Step;

#[test]
fn test_get_name_or_id_with_name() {
    let step = Step {
        name: Some(String::from("Test Step")),
        id: None,
        uses: None,
        shell: None,
        working_directory: None,
        run: None,
    };
    assert_eq!(step.get_name_or_id(), "Test Step");
}

#[test]
fn test_get_name_or_id_with_id() {
    let step = Step {
        name: None,
        id: Some(String::from("test_step")),
        uses: None,
        shell: None,
        working_directory: None,
        run: None,
    };
    assert_eq!(step.get_name_or_id(), "test_step");
}

#[test]
fn test_get_name_or_id_with_name_and_id() {
    let step = Step {
        name: Some(String::from("Test Step")),
        id: Some(String::from("test_step")),
        uses: None,
        shell: None,
        working_directory: None,
        run: None,
    };
    assert_eq!(step.get_name_or_id(), "Test Step");
}

#[test]
fn test_get_name_or_id_with_none() {
    let step = Step {
        name: None,
        id: None,
        uses: None,
        shell: None,
        working_directory: None,
        run: None,
    };
    assert_eq!(step.get_name_or_id(), "unknown");
}

#[test]
fn test_run_cmd_with_valid_command() {
    let step = Step {
        name: Some(String::from("Test Step")),
        id: Some(String::from("test_step")),
        uses: None,
        shell: Some(String::from("echo")),
        working_directory: None,
        run: Some(String::from("Hello, world!")),
    };
    assert!(step.run_cmd(None, None).is_ok());
}

#[test]
fn test_run_cmd_with_invalid_command() {
    let step = Step {
        name: Some(String::from("Test Step")),
        id: Some(String::from("test_step")),
        uses: None,
        shell: Some(String::from("invalid_shell")),
        working_directory: None,
        run: Some(String::from("Hello, world!")),
    };
    assert!(step.run_cmd(None, None).is_err());
}

#[test]
fn test_run_cmd_with_no_run_command() {
    let step = Step {
        name: Some(String::from("Test Step")),
        id: Some(String::from("test_step")),
        uses: None,
        shell: Some(String::from("echo")),
        working_directory: None,
        run: None,
    };
    assert!(step.run_cmd(None, None).is_err());
}
