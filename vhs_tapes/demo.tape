Output gif_demo/demo.gif

Require leetui

Set Width 1280
Set Height 720
Set Shell "fish"
Set FontSize 16
Set Padding 3
Type@200ms "leetui"
Enter
Sleep 2s

# Up down on topic pane
Down@800ms 4
Sleep 800ms
Up@800ms 4

# Switch to questions pane
Right 1

# Up down on question pane
Down@800ms 4
Sleep 2s
Up@800ms 3

# Showpopup question detail
Enter
Sleep 3s
Down@500ms 5
Up@500ms 5
Sleep 3s
Escape
Escape

# Edit question in python lang in neovim
Type 'e'
Sleep 3s
Down@500ms 3
Enter
Sleep 2s
Type 'G'
Sleep 2s
Type@500ms ':wq'
Sleep 1s
Enter
Sleep 1s

# Run the solution
Type 'r'
Sleep 3s
Enter
Sleep 8s
Enter
Sleep 3s
Escape

# Exit
Type 'q'
