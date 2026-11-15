# 04: Gamble options

**What to build:** An option of a question can be a gamble: a chance and two outcomes, each with its own mechanic. The option's consequence text says it is a gamble. The roll is drawn when the Run is planned, so it does not depend on when or what the player answers. The Debrief note says whether the gamble paid off. The events with a risky option ship (1, 7, 13, 32, 33, 35, 36, F1, F6; 12 waits for ticket 05).

**Blocked by:** 03

**Status:** resolved

- [x] An option can define a chance with a good and a bad outcome in the catalog file; catalog tests reject a chance outside 0 to 1 or a missing outcome
- [x] The same seed gives the same gamble outcome every time, whatever other answers were given
- [x] Days before the question are identical whatever its answer
- [x] Both outcomes of a gamble are covered through the Run interface with a test catalog (chance 0 and chance 1)
- [x] The option's consequence text states the risk in plain language
- [x] The Debrief note tells the outcome of the gamble
