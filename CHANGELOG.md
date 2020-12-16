# CHANGELOG

## [unreleased]


## [0.41] 2023-02-10

## [0.40] 2023-01-27

## [0.39] 2023-01-13
## [0.38] 2023-01-08

### Added

- Build executables for more platforms. #901
- Rework the bridge between the Rust backend and Vim, some commonly used providers such as `grep`, `files`, `blines` are reimplemented to be significantly faster and reponsive without using any caching tricks. #872

## [0.37] 2022-10-16

### Changed

- Rename the provider `grep` and `grep2`. `:Clap grep` becomes `:Clap live_grep`, `:Clap grep2` becomes `:Clap grep`. If you have some grep variables like `g:clap_provider_grep_foo` before, now you need to rename them to `g:clap_provider_live_grep_foo`. #879

### Fixed

- Fix the filer preview on backend. #863
- Make the fallback smooth if there is an error occurred while loading the Python dynamic module. #865

## [0.36] 2022-08-06

### Improved

- Support filtering in parallel with the dynamic progress update. The filtering time is reduced about 50% from the sequential to the parallel version.
  The following providers will be benefited from the parallel filtering:
  - `source` of the provider returns a String command.
  - `blines`, `grep2`, `proj_tags`.
- Speed up the cache creation signigicantly. #858

### Changed

- The spinner will be hidden if it's idle.
- `dumb_jump`: ignore the results from the files not trcked by git if the project is a git repo.

### Fixed

- Maple self-upgrade is broken. #847, #848
- Fix the language bonus matching of `blines` provider.

## [0.35] 2022-06-12

### Changed

- The default value of `g:clap_enable_background_shadow` is now `v:false` by default due to the side effect like #836.

### Fixed

- Fix the incompatiblity issue between vim-signature and vim-clap popup window. #817
- Escape the file name for filer sink. #822
- Seperate the vista impl for tags provider completely. #827
- Fix the regression of command `ripgrep-forerunner`.

### Improved

- Strip the current working directory prefix from the entry of `recent_files` provider, all the entries which are not in cwd still shows the absolute path. #834

## [0.34] 2022-03-25

### Added

- Add `g:clap_cache_threshold`. #806
- Add `+ignorecase` to enable case-insensitive search, e.g., `:Clap files +ignorecase`. #814

### Improved

- Tweak the final matching score using the language keyword matching. #808

## [0.33] 2022-02-20

### Added

- Support generating the source of `tags` provider using the Rust binary, remove the vista.vim dep from `tags` 