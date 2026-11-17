# The README's animated demo. record.sh fills in the @...@ placeholders
# and runs this with a fresh demo history, up to Invert Binary Tree.

Output "@OUT@/demo.gif"
Set Shell "bash"
Set FontFamily "Iosevka"
Set FontSize 22
Set Width 1600
Set Height 1000
Set Padding 30
Set Theme "@VHS_THEME@"

Hide
# solve.sh stands in for an editor: it saves a buggy solution 9 seconds
# from now, and the right one 6 seconds after that.
Type "PS1=''; clear; $DEMO_DIR/solve.sh 9 6 & clear; gauntlet"
Enter
Sleep 1.5s
Show

# The next problem up, and its statement.
Sleep 2s
Type "i"
Sleep 2.5s
Down 6
Sleep 1.5s
Type "a"
# The buggy save, then the right one, which passes; rate it.
Sleep 11s
Type "4"
Sleep 4s

# The statistics: the problem just solved, the overview, then the runs
# and personal bests.
Type "s"
Sleep 3s
Type "g"
Sleep 3.5s
Down
Sleep 3.5s
Hide
Type "q"
