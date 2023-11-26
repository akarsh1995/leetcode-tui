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
Type@300ms 'tttt'
Sleep 300ms

# # Up down on question pane
Type@300ms 'jjj'
Sleep 800ms
Type@300ms 'kkk'

# # Showpopup question detail
Enter
Sleep 1s
Down@200ms 2
Up@200ms 2
Sleep 1s
Escape
Sleep 1s

# # Edit question in python lang in neovim
Type 'e'
Sleep 3s
Down@200ms 3
Enter
Sleep 2s
Type 'vgegldi'
Type 'from collections import Counter'
Enter
Type 'class Solution:'
Enter
Type 'def lengthOfLongestSubstring(self, s: str) -> int:'
Enter
Type 'ans = 0'
Enter
Type 'count = Counter()'
Enter
Type 'l = 0'
Enter
Type 'for r, c in enumerate(s):'
Enter
Type 'count[c] += 1'
Enter
Type 'while count[c] > 1:'
Enter
Type 'count[s[l]] -= 1'
Enter
Type 'l += 1'
Enter
Backspace
Type 'ans = max(ans, r - l + 1)'
Enter
Backspace
Type 'return ans'
Enter
Escape
Escape

Set TypingSpeed 500ms
Type ':wq'
Enter
Sleep 1s

# Run the solution
Type 'r'
Sleep 2s
# language selection
Enter
Sleep 6s
Escape

# quit app
Sleep 2s
Type 'q'
