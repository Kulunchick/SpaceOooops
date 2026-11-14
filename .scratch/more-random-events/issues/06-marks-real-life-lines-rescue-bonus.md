# 06: Marks, real-life lines and the rescue bonus

**What to build:** The event log marks a fantasy Random Event as "Unexplained" and shows a lucky one distinctly from bad news. The Debrief shows, for every real event that happened, one short line on how it happens in real life (no agency names); fantasy events have none. A Score-bonus mechanic exists, and event 37 (rescuers ask for flood images) ships with it: helping costs two days of planned science and adds its own Score line, without touching Science Value. The glossary is updated.

**Blocked by:** 03

**Status:** ready-for-agent

- [ ] Fantasy events show an "Unexplained" mark in the event log; lucky events are visually distinct; both are readable without colour alone
- [ ] Every real event in the catalog file has a real-life line, and catalog tests fail when one is missing or a fantasy event has one
- [ ] The Debrief lists the real-life line of each real event that happened in the Run, once
- [ ] Helping the rescuers adds a separate Score line and leaves Science Value and Success unaffected by the bonus
- [ ] GLOSSARY: the Random Event entry covers the new groups, and "Unexplained" is defined as the mark of a fantasy Random Event
