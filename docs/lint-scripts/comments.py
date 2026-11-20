"""The comment lines of a file, for `tally.py comments`.

A comment is prose the checker never reads: what it says about the code can be
wrong, and nothing fails. Each line a comment occupies is one site, under one of
three rules:

  comments:line      a comment on lines of its own
  comments:trailing  a comment after code on the same line
  comments:doc       a doc comment: `///`, `//!`, `/** */`, `/*! */`, a Go comment
                     right above a top-level declaration, a Python docstring

Not sites, because a tool reads them or the gate asks for them:

  - a directive: a suppression with its reason, a tool's pragma (`eslint-`,
    `oxlint-`, `@ts-expect-error`, `noqa`, `type: ignore`, `nolint`, `//go:`,
    `ast-grep-ignore`, `jscpd:ignore`, `prettier-ignore`, ...), each in the exact
    form its tool reads, so prose that opens with the same word is a comment;
  - a block whose first line opens with the marker of an unchecked invariant
    (`SAFETY:` by default), the whole block;
  - the license header: the comments before the first line of code, when they
    name a copyright, a license or an SPDX identifier;
  - a shebang and a Python coding line.

A bare string statement in Python (a triple-quoted string that is not a
docstring) is prose posing as code: `comments:line`.

The scan is lexical: strings, template literals, raw strings, Rust char literals,
nested Rust block comments and JavaScript regex literals are skipped, so a `//`
inside a string is code. A quoted string of JavaScript, Go or Python ends at its
line, so an apostrophe in JSX text costs one line at most. It approximates the
doc comments of Go (a top-level declaration only) and of Python (the first
statement of a module, a class or a function).
"""

from __future__ import annotations

import bisect
import re
from typing import NamedTuple

LANGUAGES = {
    **dict.fromkeys((".ts", ".tsx", ".mts", ".cts", ".js", ".jsx", ".mjs", ".cjs"), "js"),
    ".rs": "rust",
    ".go": "go",
    **dict.fromkeys((".py", ".pyi"), "python"),
}

_DIRECTIVE = re.compile(
    r"""^(?:
        eslint- | oxlint- | biome-ignore | deno-lint-ignore | prettier-ignore
      | @ts-(?:check|nocheck|expect-error|ignore) | jscpd:ignore | ast-grep-ignore
      | (?:c8|istanbul|v8)\s+ignore | @vite-ignore | webpack[A-Z]\w*: | @jsx\w*\s
      | @refresh\s | @license | @preserve | [#@]__PURE__ | <reference\s
      | (?i:noqa) | type:\s*ignore | pyright:\s*\w+ | mypy:\s | pragma:\s | fmt:\s*(?:off|on|skip)
      | isort:\s*skip | ruff:\s*(?:noqa|isort) | pylint:\s*disable | nolint | rustfmt::skip
    )""",
    re.VERBOSE,
)
_RAW_DIRECTIVE = re.compile(
    r"^(?://(?:go:\w|line\s|export\s|cgo\s|nolint|\+build\s)|/\*\s*(?:globals?|exported|eslint)\s)"
)
_LICENSE = re.compile(r"copyright|licen[cs]e|spdx-license-identifier", re.IGNORECASE)
_OPENER = re.compile(r"^(?:/[/*]+!?|#+)")
_GO_DECL = re.compile(r"(?:func|type|var|const|package)\b")


class Comment(NamedTuple):
    """One comment: its first and last line (1-based), its column, its kind, its text."""

    line: int
    col: int
    end: int
    doc: bool
    trailing: bool
    text: str


class _Lines:
    """Line and column of an offset, and the code before it on its line."""

    def __init__(self, text: str) -> None:
        self.text = text
        self.starts = [0, *(i + 1 for i, c in enumerate(text) if c == "\n")]

    def line(self, offset: int) -> int:
        return bisect.bisect_right(self.starts, offset)

    def comment(self, start: int, stop: int, *, doc: bool) -> Comment:
        line = self.line(start)
        before = self.text[self.starts[line - 1] : start]
        return Comment(
            line,
            start - self.starts[line - 1] + 1,
            self.line(max(start, stop - 1)),
            doc,
            bool(before.strip()),
            self.text[start:stop],
        )


def _eol(text: str, i: int) -> int:
    stop = text.find("\n", i)
    return len(text) if stop < 0 else stop


def _string_end(text: str, i: int, quote: str, *, escapes: bool, oneline: bool) -> int:
    """The offset after the string that opens at i; a one-line string stops at its line."""
    j = i + 1
    while j < len(text) and text[j] != quote:
        if oneline and text[j] == "\n":
            return j
        j += 2 if escapes and text[j] == "\\" else 1
    return j + 1


_RUST_CHAR = re.compile(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^\\'\n])'")
_RUST_RAW = re.compile(r"(?<![\w])[bc]?r(#*)\"")
_REGEX_AFTER = set("(,=:[!&|?{};+-*%<>~^")
_REGEX_WORDS = {"return", "typeof", "case", "do", "else", "in", "of", "new", "delete"}
_REGEX_WORDS |= {"void", "throw", "yield", "await"}


def _block_end(text: str, i: int, *, nests: bool) -> int:
    depth, j = 1, i + 2
    while j < len(text) and depth:
        pair = text[j : j + 2]
        if pair == "*/":
            depth, j = depth - 1, j + 2
        elif nests and pair == "/*":
            depth, j = depth + 1, j + 2
        else:
            j += 1
    return j


def _regex_can_start(text: str, i: int) -> bool:
    """A `/` opens a regex literal after an operator or a keyword, not after a value."""
    j = i - 1
    while j >= 0 and text[j] in " \t\r\n":
        j -= 1
    if j < 0 or text[j] in _REGEX_AFTER:
        return True
    k = j
    while k >= 0 and (text[k].isalnum() or text[k] in "_$"):
        k -= 1
    return text[k + 1 : j + 1] in _REGEX_WORDS


def _regex_end(text: str, i: int) -> int:
    """The offset after a regex literal; with no closing slash on its line, a division."""
    j, in_class = i + 1, False
    while j < len(text) and text[j] != "\n":
        c = text[j]
        if c == "\\":
            j += 1
        elif c in "[]":
            in_class = c == "["
        elif c == "/" and not in_class:
            return j + 1
        j += 1
    return i + 1


class _CScan:
    """JavaScript and TypeScript, Rust, Go."""

    def __init__(self, text: str, lang: str) -> None:
        self.text = text
        self.lang = lang
        self.lines = _Lines(text)
        self.found: list[Comment] = []

    def run(self) -> list[Comment]:
        self.code(0, stop_at_brace=False)
        return self.found

    def code(self, i: int, *, stop_at_brace: bool) -> int:
        """Scan code from i; inside a template's `${`, stop after its closing brace."""
        depth = 0
        while i < len(self.text):
            c = self.text[i]
            if stop_at_brace and c in "{}":
                depth += 1 if c == "{" else -1
                if depth < 0:
                    return i + 1
            i = self.step(i)
        return i

    def step(self, i: int) -> int:
        """The offset after the token that starts at i."""
        text, c = self.text, self.text[i]
        pair = text[i : i + 2]
        if pair == "//":
            return self.line_comment(i)
        if pair == "/*":
            return self.block_comment(i)
        if c in "\"'`":
            return self.quoted(i, c)
        if self.lang == "rust" and c in "bcr" and (m := _RUST_RAW.match(text, i)):
            close = text.find('"' + m.group(1), m.end())
            return len(text) if close < 0 else close + 1 + len(m.group(1))
        if self.lang == "js" and c == "/" and _regex_can_start(text, i):
            return _regex_end(text, i)
        return i + 1

    def quoted(self, i: int, quote: str) -> int:
        text = self.text
        if quote == "'" and self.lang == "rust":
            m = _RUST_CHAR.match(text, i)
            return m.end() if m else i + 1
        if quote == "`" and self.lang == "js":
            return self.template(i)
        if quote == "`":
            return _string_end(text, i, "`", escapes=False, oneline=False)
        return _string_end(text, i, quote, escapes=True, oneline=self.lang != "rust")

    def template(self, i: int) -> int:
        j = i + 1
        while j < len(self.text) and self.text[j] != "`":
            if self.text[j] == "\\":
                j += 2
            elif self.text[j : j + 2] == "${":
                j = self.code(j + 2, stop_at_brace=True)
            else:
                j += 1
        return j + 1

    def line_comment(self, i: int) -> int:
        stop = _eol(self.text, i)
        body = self.text[i:stop]
        doc = self.lang == "rust" and bool(re.match(r"//!|///(?!/)", body))
        self.found.append(self.lines.comment(i, stop, doc=doc))
        return stop

    def block_comment(self, i: int) -> int:
        stop = _block_end(self.text, i, nests=self.lang == "rust")
        body = self.text[i:stop]
        rust_inner = self.lang == "rust" and body.startswith("/*!")
        doc = bool(re.match(r"/\*\*(?![*/])", body)) or rust_inner
        self.found.append(self.lines.comment(i, stop, doc=doc))
        return stop


def _go_docs(comments: list[Comment], lines: list[str]) -> list[Comment]:
    """A Go comment block right above a top-level declaration is its doc comment."""
    out = list(comments)
    for k in range(len(out) - 1, -1, -1):
        c = out[k]
        below = lines[c.end] if c.end < len(lines) else ""
        follows = k + 1 < len(out) and out[k + 1].doc and out[k + 1].line == c.end + 1
        if not c.trailing and (_GO_DECL.match(below) or follows):
            out[k] = c._replace(doc=True)
    return out


_PY_STRING = re.compile(r"(?i)(?:[rbuf]|rb|br|fr|rf)?(\"\"\"|'''|\"|')")
_PY_HEADER = re.compile(r"(?:async\s+)?(?:def|class)\b")


class _PyScan:
    """Comments, and the string statements: docstrings and bare strings."""

    def __init__(self, text: str) -> None:
        self.text = text
        self.lines = _Lines(text)
        self.found: list[Comment] = []
        self.depth = 0
        self.logical = ""
        self.previous: str | None = None

    def run(self) -> list[Comment]:
        i = 0
        while i < len(self.text):
            i = self.step(i)
        return self.found

    def step(self, i: int) -> int:
        text, c = self.text, self.text[i]
        if c == "#":
            stop = _eol(text, i)
            self.found.append(self.lines.comment(i, stop, doc=False))
            return stop
        m = _PY_STRING.match(text, i) if c.isalpha() or c in "\"'" else None
        if m and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
            return self.string(i, m)
        self.code_char(c)
        return i + 1

    def code_char(self, c: str) -> None:
        """Track brackets and logical lines: `logical` is the code of the line being
        read, `previous` the last logical line with code."""
        if c in "([{":
            self.depth += 1
        elif c in ")]}":
            self.depth = max(0, self.depth - 1)
        continued = self.logical.rstrip(" \t\r").endswith("\\")
        if c == "\n" and self.depth == 0 and not continued:
            if self.logical.strip():
                self.previous = self.logical.strip()
            self.logical = ""
        else:
            self.logical += c

    def string(self, i: int, m: re.Match[str]) -> int:
        quote = m.group(1)
        if len(quote) == 3:
            close = self.text.find(quote, m.end())
            stop = len(self.text) if close < 0 else close + 3
        else:
            stop = _string_end(self.text, m.end() - 1, quote, escapes=True, oneline=True)
        rest = self.text[stop : _eol(self.text, stop)].split("#")[0]
        if self.depth == 0 and not self.logical.strip() and not rest.strip():
            prev = self.previous
            doc = prev is None or (bool(_PY_HEADER.match(prev)) and prev.endswith(":"))
            self.found.append(self.lines.comment(i, stop, doc=doc))
        self.logical += "s"
        return stop


def _body(text: str) -> str:
    """The first line of a comment without its opener."""
    first = text.lstrip().split("\n", 1)[0]
    first = _OPENER.sub("", first).lstrip(" *")
    return re.sub(r"^[rbuf]*(\"\"\"|''')", "", first, flags=re.IGNORECASE).strip()


def _joins(last: Comment | None, c: Comment) -> bool:
    """Two line comments of one kind, each on lines of its own, on consecutive lines."""
    if last is None or c.trailing or last.trailing or c.doc != last.doc:
        return False
    line_form = all(x.text.startswith(("//", "#")) for x in (last, c))
    return line_form and c.line == last.end + 1


def _blocks(comments: list[Comment]) -> list[list[Comment]]:
    """Runs of line comments on consecutive lines: one block of prose each."""
    blocks: list[list[Comment]] = []
    for c in comments:
        if _joins(blocks[-1][-1] if blocks else None, c):
            blocks[-1].append(c)
        else:
            blocks.append([c])
    return blocks


def _first_code(text: str, comments: list[Comment]) -> int:
    covered = {n for c in comments if not c.trailing for n in range(c.line, c.end + 1)}
    rows = text.splitlines()
    code = (n for n, row in enumerate(rows, 1) if row.strip() and n not in covered)
    return next(code, len(rows) + 1)


def _header(comments: list[Comment], first_code: int) -> set[int]:
    """The first lines of the comments of the license header."""
    head = [c for c in comments if c.end < first_code]
    return {c.line for c in head} if any(_LICENSE.search(c.text) for c in head) else set()


def _sanctioned(c: Comment, header: set[int]) -> bool:
    body = _body(c.text)
    shebang = c.line == 1 and c.text.startswith("#!")
    directive = bool(_DIRECTIVE.match(body) or _RAW_DIRECTIVE.match(c.text))
    return c.line in header or shebang or directive or "-*- coding" in body


def scan(text: str, ext: str) -> list[Comment]:
    """Every comment of a file, sanctioned or not."""
    lang = LANGUAGES[ext]
    found = _PyScan(text).run() if lang == "python" else _CScan(text, lang).run()
    return _go_docs(found, text.splitlines()) if lang == "go" else found


def counted(text: str, ext: str, marker: str = "SAFETY") -> list[Comment]:
    """The comments of a file that are sites: directives, marker blocks and the
    license header left out."""
    found = scan(text, ext)
    header = _header(found, _first_code(text, found))
    return [
        c
        for block in _blocks(found)
        if not _body(block[0].text).startswith(f"{marker}:")
        for c in block
        if not _sanctioned(c, header)
    ]


def rule_of(c: Comment) -> str:
    if c.doc:
        return "comments:doc"
    return "comments:trailing" if c.trailing else "comments:line"
