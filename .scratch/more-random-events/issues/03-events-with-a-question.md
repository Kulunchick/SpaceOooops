# 03: Events with a question

**What to build:** Some new Random Events ask the player a question before their day plays out, through the existing question UI and Decision flow. An event's question, its two options (label, consequence, mechanic and strength each) and its default come from the catalog file. An option the remaining Budget cannot pay for is shown as unavailable with the reason. A Run gets 3 or 4 interactive events among its new events when enough are eligible. The Debrief has a note on each question: what was chosen (or defaulted) and what it led to. The events whose options have certain outcomes ship (24, 25, 28, 34, 38, 39, F10, G9; 17 and 21 wait for ticket 05, 37 for ticket 06).

**Blocked by:** 01

**Status:** ready-for-agent

- [ ] An interactive event carries a prompt with a unique id, two options and a default, in the same shape as the flare and outage prompts
- [ ] The player's choice applies that option's mechanic; no answer or an invalid answer applies the default
- [ ] An option that costs more than the Budget left is unavailable with a plain-language reason, and a Decision naming it takes the default
- [ ] Days before a question are identical whatever its answer
- [ ] Across a sweep of seeds a Run has 3 or 4 interactive new events when at least that many are eligible, and never more than 4
- [ ] The Debrief lists a note per question in day order, together with the existing flare and outage notes, saying who chose (player or default) and the effect
- [ ] The question UI shows a new event's question and options with no change to team play
