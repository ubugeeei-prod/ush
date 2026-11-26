use std::collections::HashMap;

/// An immutable match result becomes visible after its entire initializer;
/// uses in the subject and arms still see the previous binding.
pub(super) fn after_initializer(
    start: usize,
    next_line: usize,
    names: &[&str],
    opens: &HashMap<usize, usize>,
    ends: &[usize],
) -> usize {
    if !matches!(names[start], "let" | "const") {
        return next_line;
    }
    let assignment = (start + 1..next_line).find(|&i| names[i] == "=");
    if !assignment.is_some_and(|i| names.get(i + 1) == Some(&"match")) {
        return next_line;
    }
    (start + 1..next_line)
        .rev()
        .find_map(|i| {
            opens
                .get(&i)
                .filter(|_| names[i] == "{")
                .map(|&scope| ends[scope] + 1)
        })
        .unwrap_or(next_line)
}
