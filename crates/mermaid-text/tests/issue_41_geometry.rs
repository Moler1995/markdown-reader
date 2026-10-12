use std::collections::VecDeque;

use mermaid_text::{render, render_ascii};

const STATE_NOTE: &str = "stateDiagram-v2\n[*] --> Idle\nnote left of Idle\n  draft\nend note";

const NESTED_SEQUENCE: &str = "sequenceDiagram\nAlice->>Bob: one\nactivate Bob\nBob->>Carol: two\nactivate Carol\nCarol-->>Bob: three\ndeactivate Carol\nBob-->>Alice: four\ndeactivate Bob";

const SAME_PARTICIPANT_NESTING: &str = "sequenceDiagram\nA->>B: outer call\nactivate B\nA->>B: inner call\nactivate B\nB->>A: inner reply\ndeactivate B\nB->>A: outer reply\ndeactivate B";

fn lines(rendered: &str) -> Vec<Vec<char>> {
    rendered
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

// Ports are N/E/S/W. A connector is continuous only when neighboring
// cells expose compatible ports, rather than merely touching each other.
fn ports(ch: char) -> u8 {
    match ch {
        '┆' | '┊' | '│' | '|' | ':' => 1 | 4,
        '┄' | '┈' | '─' | '-' | '.' => 2 | 8,
        '┌' | '╭' => 2 | 4,
        '┐' | '╮' => 4 | 8,
        '└' | '╰' => 1 | 2,
        '┘' | '╯' => 1 | 8,
        '+' => 1 | 2 | 4 | 8,
        _ => 0,
    }
}

fn assert_note_connector_is_continuous(rendered: &str) {
    let grid = lines(rendered);
    let note_row = grid
        .iter()
        .position(|row| row.iter().collect::<String>().contains("draft"))
        .unwrap();
    let note_right = grid[note_row]
        .iter()
        .rposition(|ch| matches!(ch, '│' | '|'))
        .unwrap();
    let state_row = grid
        .iter()
        .position(|row| row.iter().collect::<String>().contains("Idle"))
        .unwrap();
    let state_left = grid[state_row]
        .iter()
        .position(|ch| matches!(ch, '│' | '|'))
        .unwrap();
    let state_right = grid[state_row]
        .iter()
        .rposition(|ch| matches!(ch, '│' | '|'))
        .unwrap();
    let target = |r: usize, c: usize| {
        (r == state_row && (c == state_left || c == state_right))
            || ((r == state_row - 1 || r == state_row + 1)
                && (state_left..=state_right).contains(&c))
    };
    let start = (note_row, note_right + 1);
    assert_ne!(
        ports(grid[start.0][start.1]) & 8,
        0,
        "note must attach to first connector:\n{rendered}"
    );
    let mut queue = VecDeque::from([start]);
    let mut seen = std::collections::HashSet::from([start]);
    let mut reached = false;
    while let Some((r, c)) = queue.pop_front() {
        for (nr, nc, outgoing, incoming) in [
            (r.wrapping_sub(1), c, 1, 4),
            (r, c + 1, 2, 8),
            (r + 1, c, 4, 1),
            (r, c.wrapping_sub(1), 8, 2),
        ] {
            if nr >= grid.len() || nc >= grid[nr].len() || ports(grid[r][c]) & outgoing == 0 {
                continue;
            }
            if target(nr, nc) {
                reached = true;
                break;
            }
            if ports(grid[nr][nc]) & incoming != 0 && seen.insert((nr, nc)) {
                queue.push_back((nr, nc));
            }
        }
    }
    assert!(
        reached,
        "directionally disconnected note-to-state route:\n{rendered}"
    );
}

fn assert_labels_clear_of_activation_bars(rendered: &str, bar: char, labels: &[&str]) {
    let lines: Vec<&str> = rendered.lines().collect();
    for &label in labels {
        let row = lines
            .iter()
            .find(|line| line.contains(label))
            .unwrap_or_else(|| panic!("message label `{label}` missing:\n{rendered}"));
        let start = row.find(label).expect("label just found");
        let label_cells = &row[start..start + label.len()];
        assert!(
            !label_cells.contains(bar),
            "activation bar overlaps message label `{label}` in row `{row}`:\n{rendered}"
        );
        let message_prefix = &row[..start];
        assert!(
            !message_prefix.ends_with(bar),
            "activation bar touches message label `{label}` with no clearance in row `{row}`:\n{rendered}"
        );
        let after_label = &row[start + label.len()..];
        assert!(
            !after_label.starts_with(bar),
            "activation bar touches message label `{label}` with no clearance in row `{row}`:\n{rendered}"
        );
    }
}

#[test]
fn left_state_note_connector_is_continuous_in_unicode_and_ascii() {
    let unicode = render(STATE_NOTE).unwrap();
    assert_note_connector_is_continuous(&unicode);

    let ascii = render_ascii(STATE_NOTE).unwrap();
    assert_note_connector_is_continuous(&ascii);
}

#[test]
fn nested_sequence_activation_bars_clear_message_labels_in_unicode_and_ascii() {
    let unicode = render(NESTED_SEQUENCE).unwrap();
    assert_labels_clear_of_activation_bars(&unicode, '█', &["one", "two", "three", "four"]);

    let ascii = render_ascii(NESTED_SEQUENCE).unwrap();
    assert_labels_clear_of_activation_bars(&ascii, '#', &["one", "two", "three", "four"]);

    let same_participant_unicode = render(SAME_PARTICIPANT_NESTING).unwrap();
    assert_labels_clear_of_activation_bars(
        &same_participant_unicode,
        '█',
        &["outer call", "inner call", "inner reply", "outer reply"],
    );

    let same_participant_ascii = render_ascii(SAME_PARTICIPANT_NESTING).unwrap();
    assert_labels_clear_of_activation_bars(
        &same_participant_ascii,
        '#',
        &["outer call", "inner call", "inner reply", "outer reply"],
    );
}

// Each expected lane must consist of exactly two blocks on every row in its
// declared span, and contain no blocks outside it. Removing bars cannot pass.
fn assert_lanes(out: &str, bar: char, spans: &[(usize, usize, usize)]) {
    let grid = lines(out);
    let mut actual = std::collections::HashSet::new();
    for (row, cells) in grid.iter().enumerate() {
        for (col, &ch) in cells.iter().enumerate() {
            if ch == bar {
                actual.insert((row, col));
            }
        }
    }
    let expected: std::collections::HashSet<_> = spans
        .iter()
        .flat_map(|&(col, lo, hi)| (lo..=hi).flat_map(move |row| [(row, col), (row, col + 1)]))
        .collect();
    assert_eq!(actual, expected, "activation lanes drifted:\n{out}");
}

fn center(out: &str, name: &str) -> usize {
    let row = out.lines().nth(1).unwrap();
    let name_start = row
        .chars()
        .collect::<Vec<_>>()
        .windows(name.len())
        .position(|cs| cs.iter().collect::<String>() == name)
        .unwrap();
    let cells: Vec<_> = row.chars().collect();
    let left = (0..name_start)
        .rfind(|&i| matches!(cells[i], '│' | '|'))
        .unwrap();
    let right = (name_start..cells.len())
        .find(|&i| matches!(cells[i], '│' | '|'))
        .unwrap();
    left + (right - left).div_ceil(2)
}

#[test]
fn activation_spans_are_exact_continuous_and_distinct() {
    for (render, bar) in [
        (
            mermaid_text::render as fn(&str) -> Result<String, mermaid_text::Error>,
            '█',
        ),
        (
            mermaid_text::render_ascii as fn(&str) -> Result<String, mermaid_text::Error>,
            '#',
        ),
    ] {
        let out = render(NESTED_SEQUENCE).unwrap();
        let bob = center(&out, "Bob");
        let carol = center(&out, "Carol");
        assert_lanes(&out, bar, &[(bob, 5, 10), (carol, 7, 8)]);
        let out = render(SAME_PARTICIPANT_NESTING).unwrap();
        let b = center(&out, "B");
        assert_lanes(&out, bar, &[(b, 5, 10), (b + 3, 7, 8)]);
        let out = render("sequenceDiagram\nA->>B: call\nactivate B\nactivate B\nB->>B: self\ndeactivate B\ndeactivate B").unwrap();
        let b = center(&out, "B");
        assert_lanes(&out, bar, &[(b, 5, 8), (b + 3, 5, 8)]);
        assert_labels_clear_of_activation_bars(&out, bar, &["call", "self"]);
    }
}
