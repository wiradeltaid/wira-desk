---
type: structure
scope: document
verified: '2026-11-16'
commit: '8195140'
---

# Document Structure

## Verified

2026-11-16, read from the tracked tree (`git ls-files`) at commit `8195140`. `.what-rendered/` and
`.how-rendered/` are git-ignored and left out.

## Top level

```text
.constitution/                # how we work — populated: method package plus this product's room
.control/                     # what currently holds — populated: registries, 47 decisions, questions, memlog
.what/                        # what was promised — populated: brief, PRD, business rules, two PCs
.how/                         # how it is built — populated: platform portrait, two PCs
_bmad-output/                 # work in progress, not curated — README.md only
.scratch/                     # active spec workspaces — spec-29 to spec-34
.archive/                     # closed spec workspaces — specs/
.work/                        # committed working area — skill runs and coding briefs
docs/                         # predates the method — configuration, threat model, packaging, research, engines config
```

## Per layer

### `.constitution/`

```text
.constitution/
├── method/                   # WDI Method package, overwritten by `wdi-method update`
│   ├── constitution.md, repo-guide.md, structure-guide.md, branch-guide.md, ci-guide.md,
│   │   language-guide.md, method-glossary.md, README.md
│   ├── document/             # artifact guides and the BMad skill register
│   │   └── templates/        # templates the skills write from
│   ├── scripts/              # ★ validate.py, inventory.py, lifecycle.py, timeline.py
│   └── why/                  # status: Reference, explains and never binds
└── project/                  # this product's room, never overwritten by update
    ├── constitution.md
    ├── codebase-stack-guide.md, codebase-conventions-guide.md, codebase-brownfield-guide.md   # status: Accepted
    └── inventory-readers.py  # written for this repo's stack
```

### `.control/`

```text
.control/
├── registry/                 # ★ components.yaml, index.yaml, specs.yaml, decisions.yaml, goals.yaml,
│                             #   usecases.yaml, requirements-wira-desk.yaml, risks.yaml, defects.yaml
├── decisions/                # DEC-001 to DEC-047: 43 applied, 4 accepted
├── questions/                # blocking.md, assumptions.md, external.md, answered.md
├── memlog/                   # one file per PC plus brief, prd, spine, ux; autopilot-DEC-<n>.md per mandate; dated session and upgrade logs
├── meetings/                 # empty
├── test-targets/             # desktop.md, mobile.md, web.md
├── generated/                # validate.py --generate output, never hand-written
├── product-glossary.md
├── project-non-technical-log.md
├── wdi-method.yaml
├── custom-dispatch.yaml.example
├── structure-codebase.md
└── structure-document.md
```

### `.what/` and `.how/`, product level

```text
.what/
├── _product-brief/           # ★ brief.md, addendum.md (G1)
├── _prd/wira-desk/           # ★ prd.md, addendum.md (G2)
└── business-rules.md         # cross-component rules (G3)

.how/
└── _platform/                # ★ ARCHITECTURE-SPINE.md, c4-l1-system-context.md, c4-l2-containers.md,
                              #   cross-cutting.md, design-system.md, inventory-db/api/screen.md
```

### `.scratch/` and `.archive/`

```text
.scratch/
└── spec-<n>-<slug>/          # SPEC.md, issues/<nn>-<slug>.md, smoke/ when a smoke test ran

.archive/
└── specs/<spec-id>-<slug>/   # the same shape, for closed specs (w1, w4, spec-1 to spec-28)
```

## Product Components

| Product Component | `.what/<pc>/` | `.how/<pc>/` | Slots split out |
| --- | --- | --- | --- |
| `window-management` | `SRS-window-management.md` | `SDD-window-management.md` | `.what`: `02-rules/`, `03-domain/`, `04-usecases/` (UC-1, 2, 3, 7, 9, 10, 12, 13), `05-scenarios/` (SCN-01 to 04). `.how`: `02-contracts/`, `04-components/`, `05-model/`, `06-flows/`; no `01-ux/` |
| `settings` | `SRS-settings.md` | `SDD-settings.md` | `.what`: `02-rules/`, `03-domain/`, `04-usecases/` (EXPERIENCE, UC-4, 5, 6, 8, 11, 14), `05-scenarios/` (SCN-01 to 03). `.how`: `01-ux/` (DESIGN.md), `02-contracts/`, `04-components/`, `05-model/`, `06-flows/` |

## Registries and generated

| File | State |
| --- | --- |
| `.control/registry/*.yaml` | all nine carry entries |
| `.control/generated/` | produced: components, dag, decisions, estimate, report, risks, rtm, status, timeline |

## Findings

- `core` is registered in `components.yaml` as a Product Component (`mode: deep`) and as a container,
  but has no folder in `.what/` or `.how/`, and its code left this repository for
  `wiradeltaid/wira-desk-core`.
- `.how/_platform/ARCHITECTURE-SPINE.md` AD-12 and its dependency diagram still describe a four-crate
  workspace with `core` inside it.
- `.constitution/method/scripts/__pycache__/validate.cpython-314.pyc` is tracked although
  `.gitignore` excludes `__pycache__/`.
- `docs/` predates the method and is inventory to sort; `docs/agents/` is the engines' config.

---

★ = key document: single-copy, referenced from elsewhere, or the first thing a reader must find.
