pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<(), std::io::Error> {
    // Function iterates through lines of file and prints all lines that contain the pattern.
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    
    Ok(())
}

// Unit-test to ensure find_matches was functioning as expected.
#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n")
}
