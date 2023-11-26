# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]

- Sort questions by:
    - likes dislikes ratio.

- Scroll bar visible list

- Take input directly from the user lc session

- Select multiple tags.

- Summary of the question
    - View more question details

- Invalidate questions cache through `userSessionProgress`


## [0.4.0] - 2023-11-25

### Added

- Lock symbol in front of the question when it is marked premium
- New Key Bindings
- New Layout
- Show help for Keybindings using ? key
- Following symbols for status of the question:
    - Locked problems: 🔐
    - Already Solved: 👑
    - Attempted: 🏃

### Fixed

- Error Serialization dserialization failed key=`memory missing` for lc_1143
- Fix re-request when there's network error in fetching question.

### Removed

- Question jump feature (123G)

## [0.3.0] - 2023-08-10

### Added

- Neetcode 75 question list.
- Search feature on keypress `/`

### Changed

- Not null constraints on the fields that are never null from the server.
- `QuestionModelContainer { question: RefCell<QuestionModel> }` changed to `Rc<RefCell<QuestionModel>>`
    - As prior implemented hash. Hashables should not be mutable.
- Colorscheme as per tokyonight style.

### Fixed

- Some questions did not appear in "All" question list because they were not attached to any topic.
    - To resolve Unknown topic tag is added to the questions which do not have any topic tag.
- App now successfully restores the terminal state. No residual prints on closing the app.
- High CPU usage due to 100ms tick interval. Now tick interval changed to 5 seconds.

## [0.2.0] - 2023-07-30

### Added

- Read question view is scrollable using up and down keys.
- Question line is colored by easy => green, medium => yellow, hard => red.
- Show helps at the bottom/top.
- Open file in the editor to solve by pressing the key e.
- Create solution file in the preferred language
- Run/test the solution against test cases
    - show test case submission stats in the popup
- Submit solution file
- Update table question if solution accepted
- Loading spinner at the top.
- Fix config directories setup for windows
- Submission stats upon successful submit
- Added gif demo using [vhs tape](https://github.com/charmbracelet/vhs)
- Vim like keybinding to jump to a problem (number followed by G (123G) in topic tag "all" questions)

### Fixed

- Failing build windows

## [0.1.0] - 2023-07-19

### Added

- List all tags
    - Array
    - Hash Table
    - Linked List
    - Math
    - Recursion
    - Etc
- List questions related to the tag.

- Stats of the selected tag.
    - Total Attempted, Solved (Easy, Medium, Hard) by tag.

- Scrollable View of questions corresponding to the tags.

- Read question in the popup using `Enter` key on the selected question.
