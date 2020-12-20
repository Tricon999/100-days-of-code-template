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

- Support generating the source of `tags` provider using the Rust binary, remove the vista.vim dep from `tags` provider. #795
- Initial support of preview with context. #798

### Fixed

- Fix the `proj_tags` cmd list under Vim. #796
- No syntax highlight for `c` preview. #800

### Improved

- Better performance for the pre-built binary. #804

### Changed

- Set `g:clap_builtin_fuzzy_filter_threshold` to `0` to always use the async `on_typed` implementation which is full-featured using the Rust backend.

### Internal

- Update to clap v3.0. #794

## [0.32] 2022-01-21

### Improved

- Rework the truncation of long lines. #788
- Support searching the definition/declaration in the `tags` file using `readtags` for `dumb_jump` provider. #789

  Aside from the previous regex searching, the results from the tags searching will be displayed first. You can control the
  tags searching scheme by adding `*` in the end:

  - `hel`: match the tags that starts with `hel`.
  - `hel*`: match the tags that contain `hel`.

- Add `gtags` support for dumb_jump provider. #792
- Introduce debounce for user typed event. #793

### Fixed

- Fix the regression that `filer` provider is not properly initialized on the Rust backend. #790

## [0.31] 2021-12-12

### Improved

- Always update the preview for `registers` otherwise the preview content could be outdated and add a preview title.

### Fixed

- Fix static binary build on ubuntu after upgrading to edition 2021(#785)

## [0.30] 2021-10-19

### Improved

- Improve the overal performance by using rayon. #754
- Parallel recursive ctags creation, 30x faster on my machine. #755
- Support expanding `~` in file path when using `preview/file`.
- Add `'AUTO'` option for `g:clap_preview_direction`. #767 @goolord

### Fixed

- Error when using `clap#preview#file()` with `g:clap_preview_direction = 'UD'`. #756
- `maps` provider: missing keybindings for neovim. #762 @ray-x
- `.cc` file preview not highlighted. #736
- Invoke `on_move` as well when initializing the display window. #768

## [0.29] 2021-08-30

### Added

- Support fzf-like search syntax. #738

### Improved

- Make the preview on enter work for `recent_files` provider. #731
- Refresh the cache when it might be outdated, detected on processing the OnMove event. #740
- Add a bonus score for the entry of `recent_files` based on cwd. #742

### Fixed

- Fix the preview of `filer` provider. #731
- Fix the installer on FreeBSD/OpenBSD. #733 Thanks to @spamwax

## [0.28] 2021-08-06

### Improved

- Rewrite the cache system to be more efficien