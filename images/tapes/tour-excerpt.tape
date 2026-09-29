# README image: part of the language tour as a notebook (seeded).
Output "target/screens/tour-excerpt.gif"
Set Shell bash
Set FontSize 15
Set Width 1000
Set Height 1340
Set Padding 20
Hide
Type "clear"
Enter
Show
Type "./target/debug/xetal run --echo --seed 1 demos/tour.xtl | sed -n '74,130p'"
Enter
Sleep 2s
