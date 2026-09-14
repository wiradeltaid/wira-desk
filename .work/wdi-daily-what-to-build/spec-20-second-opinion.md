# Second Opinion Review Request: SPEC-20 / DEF-22

Please review the drafted specification `SPEC-20` and defect `DEF-22`.

## 1. Path to Drafted Spec and Ticket
- Spec file: `.scratch/spec-20-static-crt-and-software-renderer/SPEC.md`
- Ticket file: `.scratch/spec-20-static-crt-and-software-renderer/issues/01-defect-def-22-static-crt-settings-renderer.md`
- Defect entry: `.control/registry/defects.yaml` (entry `DEF-22`)
- Specs registry entry: `.control/registry/specs.yaml` (entry `SPEC-20`)

## 2. Original Raw Notes from User (Verbatim, Unedited)
```
Mendaftarkan defect DEF-22 dan membuka paket spesifikasi SPEC-20 untuk mulai mengeksekusi Opsi A.
```

## 3. Standing Mandate
If this draft touches the architecture spine, an SRS, an SDD, or a SPEC, you're authorized to run `wdi-review` on it yourself and edit the document directly to apply its stamp — no need to ask first, that permission is already given for this dispatch.

Note: As you do not have a native Skill tool, open and follow `.claude/skills/wdi-review/SKILL.md` as plain instructions.

Perform a rigorous review across [structure, prose, edge-case-hunter] lenses.
Evaluate:
1. Are the acceptance criteria in `SPEC-20-01` complete, unambiguous, and directly testable?
2. Does the spec avoid inventing unnecessary requirements or touching out-of-scope files?
3. Are all failure modes and validation steps properly accounted for (clean Windows VM, zero dynamic CRT imports, CI guard without bypass flags)?
4. If improvements are needed, edit the files directly or output the exact revisions to fold back in.
