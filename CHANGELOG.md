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

- Rewrite the cache system to be more efficient and convenient. See #726 for details.
- Less allocations. #728

## [0.27] 2021-07-22

### Added

- Add a new provider `recent_files` for recent files history, which is persistent and can keep up to 10,000 entries ordered by [Frecency](https://en.wikipedia.org/wiki/Frecency). #724

### Fixed

- Fix rg 13.0.0 does not work for neovim. #711
- Fix grep2 does not work on Windows. #533

### Improved

- Support passing the cursor position instead of full cursor line from Vim to Rust since the performance of Vim is pretty bad when the cursor line is extremely long. #719

## [0.26] 2021-06-15

### Added

- [neovim] Add zindex option to fix the tricky floating_win overlapping, and add border for the preview window, use `let g:clap_popup_border = 'nil'` to disable the order. #693
- Impl preview for `quickfix` provider. #691
- Impl `preview/file` for easier external async preview integration. #706

### Changed

- Now `g:clap_provider_grep_enable_icon` is initialized using `g:clap_enable_icon`. #701

### Fixed

- Handle the non-utf8 line of rg's output properly. #673
- [neovim] Fix the action dialog creation using floating_win. #688
- Fix the indicator winwidth is not flexible. #687
- Fix the icon offset when restoring the full display line for grep provider. #701
- Fix the Pyo3 compilation on M1. #707
- Add icon for `*.tex`. #709

### Perf

- Use faster simdutf8. #681

## [0.25] 2021-04-25

### Added

- Add `dumb_jump` provider, which will fall back to the normal grep way when the regexp approach fails. #659

### Internal change

- Move `stdio_server` crate into a module of `maple_cli` crate for reusing the utilities in `maple_cli` easily.

### Fixed

- Force using sync impl for the providers's `source_type` that is list type. #672

## [0.24] 2021-03-13

### Added

- Add user autocmd `ClapOnInitialize`, can be used to ignore some buffers when opening clap. #653
- Add `g:clap_provider_colors_ignore_default` to ignore the default colors in `VIMRUNTIME`. #632
- Support neovim floating_win based action menu. #655

### Improved

- Truncate the lines of `grep` provider. #650
- Support unordered substring query. #652
- Add `hi default link ClapIndicator ClapInput` for the default theme.

### Fixed

- Cannot open files with pipe in file path. #643
- Fix the grep preview when `g:clap_enable_icon` is enabled and `g:clap_provider_grep_enable_icon` is disabled. #648
- Reset the old selections when the input changes. #646
- Make customize the icon easier. #392

### Changed

- Remove some colors close the white color in the default value of `g:clap_fuzzy_match_hl_groups`.

## [0.23] 2021-02-16

### Added

- Add `g:clap_force_matchfuzzy` to use the builtin `matchfuzzy()` when filtering in sync way. #607
- Add `g:clap_force_python` to always use the Python sync filter as some improvements are only implemented on the Rust side and you need the Python dynamic module to use that. #614
- Support `+name-only` for Lua sync filter. #612
- Add `g:ClapProviderHistoryCustomFilter` for customizing the source of `history` provider. #615
- Add a bonus for the match in the filename when the source item is a path, but you can only have this when you are using Python dynamic module or the Rust backend. #614.
- Add a bonus for the files you opened since you enter vim. #622
- Add async preview support for `help_tags` provider, the Rust binary is required. #630
- ~~Add `g:clap_always_open_preview` to open the preview always if the provider impls `on_move_async`~~(See `g:clap_open_preview`), it's on by the default which changes the behavior before. #625
- Add `g:clap_preview_direction` for opening the preview window on the right of the display window, and the default behavior has been changed to `LR` if your screen's `columns` is less than 80. #634
- Add `g:clap_open_preview` to control the opening of preview window, you can set it to `never` to fully disable the preview feature. #636

### Fixed

- Add `--color=never` to the default grep option. #609
- Show create new file entry when in empty directory. #624

### Internal

- Introduce `MatchText` for passing more match context easier later. #626

## [0.22] 2021-01-01

### Added

- Add `g:clap_enable_background_shadow` to render a transparent shadow (neovim-only) #546, #550
- Add `g:clap_popup_move_manager` so that Vim users can override the default mappings easily. #536
- Allow user to always download the prebuilt binary. #531
- Support smartcase fitlering for fzy algo and it's the default behavior. #541 @romgrk
- Add initial support for fzy lua, neovim-nightly or vim compiled with lua is required. #599
- Add the providers defined via global variable into the `providers` provider, which means you can see the global variable type providers when you call `:Clap` now. But you have to define `description` explicitly otherwise they won't be found. #605
  ```vim
  let g:clap_provider_tasks = {
            \ 'source': function('TaskListSource'),
            \ 'sink': function('TaskListSink'),
            \ 'description': 'List various tasks',
            \ }
  ```

### Improved

- Command provider has a better rendering and let's the user add arguments #570
- Fix the sluggish of vim when the preview lines are awfully long. #543

### Fixed

- Fix the installer on Windows. #529 @Grueslayer
- Fix the condition of vim8 job exists or not. #566
- Add the missing `ClapOnExit` for `g:clap_open_action` operation. #576

### Improved

- Keybindings for `filer`: `<CR>` now expands directory instead of editing it
- Make the grep opts work as normal in the command line. #595

## [0.21] 2020-09-27

### Added

- New shortcut for `+no-cache`, `:Clap files!` is equivalent to `:Clap!! files` and `:Clap files +no-cache`. ([#509](https://github.com/liuchengxu/vim-clap/pull/509))
- Add `g:clap_enable_debug`, useful when you find vim-clap is problematic and want to debug vim-clap.

### Improved

- The open action `ctrl-t`, `ctrl-v`, `ctrl-t` now supports the multiple files. ([#496](https://github.com/liuchengxu/vim-clap/issues/496))
- Check if the ctags has the JSON output feature. ([#491](https://github.com/liuchengxu/vim-clap/issues/491))

### Fixed

- Fix `:Clap install-binary` does not work correctly on Windows. ([#494](https://github.com/liuchengxu/vim-clap/pull/494)) @Bakudankun
- Fix [#306](https://github.com/liuchengxu/vim-clap/issues/306), note the signature of `bs_action` are different between vim and neovim now. ([#503](https://github.com/liuchengxu/vim-clap/pull/503))
- Fix filer issue on Windows [#370](https://github.com/liuchengxu/vim-clap/issues/370). @Grueslayer
- Handle the maple error in the filer provider, fix [#500](https://github.com/liuchengxu/vim-clap/issues/500), [#501](https://github.com/liuchengxu/vim-clap/issues/501).
- Fix regression #513
- Fix #515
- Fix #517
- Fix #526

## [0.20] 2020-08-06

### Added

- Python dynamic module now can be compiled using stable Rust. ([#471](https://github.com/liuchengxu/vim-clap/pull/471))
- Add `windows` preview support. ([#473](https://github.com/liuchengxu/vim-clap/pull/473))
- Impl `commits` and `bcommits` provider. ([#477](https://github.com/liuchengxu/vim-clap/pull/477)) @ray-