# videos/tour.webp: the language tour as a notebook, scrolling by
# (just play; seeded so the dice match the images).
Output "target/screens/tour.mp4"
Set Shell bash
Set FontSize 16
Set FontFamily "JuliaMono"
Set Width 1100
Set Height 720
Set Padding 16
Set Framerate 12
Hide
Type "clear"
Enter
Show
Type "./target/debug/xetal run --echo --seed 1 --delay 350 demos/tour.xtl"
Enter
Sleep 60s
