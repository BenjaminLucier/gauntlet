# The README's screenshots, once per theme. record.sh fills in the @...@
# placeholders and runs this with a fresh demo history, up to Invert Binary
# Tree.

Output "@TMP@/screenshots-@THEME@.gif"
Set Shell "bash"
Set FontFamily "Iosevka"
Set FontSize 22
Set Width 1600
Set Height 1000
Set Padding 30
Set Theme "@VHS_THEME@"

Hide
Type "PS1=''; clear; gauntlet"
Enter
Sleep 2s
Show

# The statement of the next problem up, with its first hint.
Type "i"
Sleep 0.5s
Tab
Enter
Sleep 1s
Screenshot "@OUT@/@THEME@/info.png"
Sleep 0.5s

# The problem before, solved: opening it runs its tests.
Type "a"
Type "p"
Sleep 5s
Screenshot "@OUT@/@THEME@/arena.png"
Sleep 0.5s

# The statistics: the overview, runs and PBs, and the history of a problem
# that was redone and found easier.
Type "s"
Sleep 1s
Type "g"
Sleep 1s
Screenshot "@OUT@/@THEME@/stats.png"
Sleep 0.5s
Down
Sleep 1s
Screenshot "@OUT@/@THEME@/runs.png"
Sleep 0.5s
Down
Right
Down 6
Sleep 1s
Screenshot "@OUT@/@THEME@/problem.png"
Sleep 0.5s
Type "q"
Sleep 1s
