#!/usr/bin/env python3
"""Lightweight CI validator for bundled Codex/Claude Code skills."""

from __future__ import annotations

import re
import sys
from pathlib import Path


def fail(message: str) -> int:
    print(f"error: {message}", file=sys.stderr)
    return 1


def main() -> int:
    if len(sys.argv) != 2:
        return fail("usage: validate_skill.py <skill-dir>")

    skill_dir = Path(sys.argv[1])
    skill_md = skill_dir / "SKILL.md"
    openai_yaml = skill_dir / "agents" / "openai.yaml"

    if not skill_md.is_file():
        return fail(f"missing {skill_md}")
    if not openai_yaml.is_file():
        return fail(f"missing {openai_yaml}")

    text = skill_md.read_text(encoding="utf-8")
    match = re.match(r"^---\n(.*?)\n---\n", text, re.S)
    if not match:
        return fail("SKILL.md missing YAML frontmatter")

    frontmatter = match.group(1)
    if not re.search(r"^name:\s*embedded-debugger\s*$", frontmatter, re.M):
        return fail("SKILL.md frontmatter must set name: embedded-debugger")
    if not re.search(r"^description:\s*.+", frontmatter, re.M):
        return fail("SKILL.md frontmatter must include description")
    placeholder_marker = "TO" + "DO"
    if f"[{placeholder_marker}" in text or f"{placeholder_marker}:" in text:
        return fail("SKILL.md still contains placeholder markers")

    metadata = openai_yaml.read_text(encoding="utf-8")
    if "Use $embedded-debugger" not in metadata:
        return fail("agents/openai.yaml default prompt must mention $embedded-debugger")

    print(f"validated {skill_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
