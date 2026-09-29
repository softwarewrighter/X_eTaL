# docs image: the whole language tour as a notebook (seeded).
Output "target/screens/tour-full.gif"
Set Shell bash
Set FontSize 14
Set Width 1000
Set Height 4800
Set Padding 20
Hide
Type "clear"
Enter
Show
Type "./target/debug/xetal run --echo --seed 1 demos/tour.xtl"
Enter
Sleep 3s
