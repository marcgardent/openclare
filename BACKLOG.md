# openclare — backlog of technical debt

The tickets live in the issues of the repository since 11 October 2026:
<https://github.com/marcgardent/openclare/issues>.

One ticket per shortcut, approximation, fallback or item of the specification
([ALGORITHM.md](ALGORITHM.md)) that was not done. A ticket that is settled is closed, not
deleted, and says what settled it; what is left of it stays written in it.

**Numbers.** The tickets this file held, DT-001 to DT-053, are the issues of the same numbers:
`DT-034` in a document, a comment of the code or a commit message is issue #34. A new ticket
takes the number its issue is given, and is cited the same way (`DT-054`). "DT" stands for
*dette technique*, technical debt.

**Severity** is a label of the issue: *high* limits a result the acceptance rule is about;
*medium* limits quality or robustness outside the corpus; *low* is tidiness, comfort,
theoretical guarantee.

**Labels in the tickets**: J*n* is milestone *n* (from the French *jalon*); Q*n* a quality
track; T*n* a tools track.

This is not the roadmap: the milestones are in ALGORITHM.md, section 4, and what is left to do
today in [ROADMAP.md](ROADMAP.md). What this file said until then is in the history of the
repository (`git log -- BACKLOG.md`).

```bash
gh issue list --label "severity: high"
```
