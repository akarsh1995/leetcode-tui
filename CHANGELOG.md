# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

- Sort questions by:
    - likes dislikes ratio.

- Filter

- Search feature.

- Scroll bar visible list

- Take input directly from the user lc session

- Select multiple tags.

- Summary of the question
    - View more question details

- Invalidate questions cache through `userSessionProgress`

## [0.2.1] - [Unreleased]

### Added

- Neetcode 75 question list.

### Changed

- Not null constraints on the fields that are never null from the server.
- `QuestionModelContainer { question: RefCell<QuestionModel> }` changed to `Rc<RefCell<QuestionModel>>`
    - As prior implemented hash. Hashables should not be mutable.
- Colorscheme as per tokyonight style.

### Fixed

- Some questions did not appear in "All" question list because they were not attached to any topic.
    - To resolve Unknown topic tag is added to the questions which do not have any topic tag.

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
