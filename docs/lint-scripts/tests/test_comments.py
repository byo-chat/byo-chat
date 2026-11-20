"""tally.py comments: the comment lines of each language, what is not a site, the ratchet,
and the comment lines a change adds."""

from __future__ import annotations

import json
import unittest

from harness import ScriptCase

TS = """// Copyright 2026 Example. MIT License.

import { x } from "./x"; // trailing
const url = "http://example.test"; const re = /\\/\\/(?:a|b)/g;
const t = `a ${ { k: "//" }.k } // not a comment`;
const half = total / 2; // after a division
/**
 * The doc of f.
 */
export function f(): void {}
// eslint-disable-next-line no-console -- the logging module
// a plain comment
/* two
   lines */
/* global window */
// global state lives in the store
"""

RUST = """//! The crate doc.
fn f<'a>(s: &'a str) -> char {
    let raw = r#"a // "quoted" text"#;
    let c = '"';
    /* outer /* inner */ still outer */
    // SAFETY: the index is checked above,
    // and the slice is never empty.
    unsafe { g() }
    /// a doc on an item
    c
}
"""

PY = '''#!/usr/bin/env python3
"""Module doc."""
import os  # noqa: F401  # the reason

x = "# not a comment"


def f():
    """Function doc."""
    """A bare string: prose posing as code."""
    y = (
        """an argument"""
    )
    # a comment
    return y
'''

GO = """// Package p does things.
package p

// F does one thing.
func F() {
	s := `// raw`
	r := '"'
	//nolint:errcheck // the reason
	_ = s // trailing
}
"""


class Comments(ScriptCase):
    with_fixtures = False

    def setUp(self) -> None:
        super().setUp()
        self.git("init", "-q", ".")
        for name, body in (("a.ts", TS), ("a.rs", RUST), ("a.py", PY), ("a.go", GO)):
            self.write(f"src/{name}", body)
        self.git("add", "src")
        self.git("commit", "-q", "-m", "base")

    def sites(self, ext: str) -> dict[str, list[str]]:
        ran = self.script("tally.py", "comments", "src", "--ext", ext, "--json", "out.json")
        self.assertEqual(ran.code, 0, ran.out + ran.err)
        found: dict[str, list[str]] = json.loads((self.folder / "out.json").read_text("utf-8"))
        return found

    def test_typescript(self) -> None:
        self.assertEqual(
            self.sites(".ts"),
            {
                "comments:doc": ["src/a.ts:7:1", "src/a.ts:8:1", "src/a.ts:9:1"],
                "comments:line": [
                    "src/a.ts:12:1",
                    "src/a.ts:13:1",
                    "src/a.ts:14:1",
                    "src/a.ts:16:1",
                ],
                "comments:trailing": ["src/a.ts:3:26", "src/a.ts:6:25"],
            },
        )

    def test_rust(self) -> None:
        self.assertEqual(
            self.sites(".rs"),
            {
                "comments:doc": ["src/a.rs:1:1", "src/a.rs:9:5"],
                "comments:line": ["src/a.rs:5:5"],
            },
        )

    def test_python(self) -> None:
        self.assertEqual(
            self.sites(".py"),
            {
                "comments:doc": ["src/a.py:2:1", "src/a.py:9:5"],
                "comments:line": ["src/a.py:10:5", "src/a.py:14:5"],
            },
        )

    def test_go(self) -> None:
        self.assertEqual(
            self.sites(".go"),
            {
                "comments:doc": ["src/a.go:1:1", "src/a.go:4:1"],
                "comments:trailing": ["src/a.go:9:8"],
            },
        )

    def test_the_count_is_a_ratchet(self) -> None:
        held = self.script("tally.py", "comments", "src", "--ext", ".ts", "--expect", "9")
        grown = self.script("tally.py", "comments", "src", "--ext", ".ts", "--expect", "8")
        self.assertEqual((held.code, grown.code), (0, 1), held.out)

    def test_an_unknown_extension_fails(self) -> None:
        ran = self.script("tally.py", "comments", "--ext", ".zz")
        self.assertEqual(ran.code, 1)
        self.assertIn("no comment syntax for .zz", ran.err)

    def base(self, *args: str) -> tuple[int, str]:
        ran = self.script("tally.py", "comments", "src", "--ext", ".py", "--base", "HEAD", *args)
        return ran.code, ran.out + ran.err

    def test_a_comment_the_change_adds_fails(self) -> None:
        self.write("src/a.py", PY + "\n\n# why this works\nz = 1  # and again\n")
        code, out = self.base()
        self.assertEqual(code, 1)
        self.assertIn("src/a.py:18:1: comments:line", out)
        self.assertIn("src/a.py:19:8: comments:trailing", out)
        self.assertIn("2 comment lines this change adds", out)

    def test_a_directive_or_a_code_line_the_change_adds_passes(self) -> None:
        self.write("src/a.py", PY + "\n\nz = 1  # noqa: E501  # the reason\nw = 2\n")
        self.assertEqual(self.base(), (0, "0 comment lines this change adds\n"))

    def test_a_comment_in_a_new_file_fails(self) -> None:
        self.write("src/new.py", "# new\n")
        code, out = self.base()
        self.assertEqual(code, 1)
        self.assertIn("src/new.py:1:1: comments:line", out)

    def test_a_moved_comment_is_not_new(self) -> None:
        self.write("src/a.py", PY.replace("    # a comment\n", ""))
        self.write("src/b.py", "# a comment\n")
        self.assertEqual(self.base()[0], 0)

    def test_ignore_lets_the_docs_through(self) -> None:
        self.write("src/a.py", PY + '\n\ndef g():\n    """The doc of g."""\n')
        self.assertEqual(self.base()[0], 1)
        self.assertEqual(self.base("--ignore", "comments:doc")[0], 0)


if __name__ == "__main__":
    unittest.main()
