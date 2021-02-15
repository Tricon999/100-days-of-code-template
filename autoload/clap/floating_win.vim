" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: Neovim floating_win UI.

scriptencoding utf-8

let s:save_cpo = &cpoptions
set cpoptions&vim

let g:clap#floating_win#input = {}
let g:clap#floating_win#display = {}
let g:clap#floating_win#spinner = {}
let g:clap#floating_win#preview = {}

let s:has_nvim_0_5 = has('nvim-0.5')

let s:shadow_bufnr = nvim_create_buf(v:false, v:true)

let s:spinner_bufnr = nvim_create_buf(v:false, v:true)
let g:clap.spinner.bufnr = s:spinner_bufnr

let s:input_bufnr = nvim_create_buf(v:false, v:true)
let g:clap.input.bufnr = s:input_bufnr

let s:display_bufnr = nvim_create_buf(v:false, v:true)
let g:clap.display.bufnr = s:display_bufnr

let s:symbol_left_bufnr = nvim_create_buf(v:false, v:true)
let s:symbol_right_bufnr = nvim_create_buf(v:false, v:true)

let s:preview_bufnr = nvim_create_buf(v:false, v:true)

let s:indicator_bufnr = nvim_create_buf(v:false, v:true)
let g:__clap_indicator_bufnr = s:indicator_bufnr

let s:exists_deoplete = exists('*deoplete#custom#buffer_option')

let s:symbol_left = g:__clap_search_box_border_symbol.left
let s:symbol_right = g:__clap_search_box_border_symbol.right
let s:symbol_width = strdisplaywidth(s:symbol_right)

let s:shadow_winhl = 'Normal:ClapShadow,NormalNC:ClapShadow,EndOfBuffer:ClapShadow'
let s:display_winhl = 'Normal:ClapDisplay,EndOfBuffer:ClapDisplayInvisibleEndOfBuffer,SignColumn:ClapDisplay,ColorColumn:ClapDisplay'
let s:preview_winhl = 'Normal:ClapPreview,EndOfBuffer:ClapPreviewInvisibleEndOfBuffer,SignColumn:ClapPreview,ColorColumn:ClapPreview'

" shadow
"  -----------------------------
" | spinner | input             |
" |-----------------------------|
" |          display            |
" |-----------------------------|
" |          preview            |
"  -----------------------------
function! g:clap#floating_win#display.open() abort
  if exists('s:display_winid') && nvim_win_is_valid(s:display_winid)
    return
  endif
  " Check if the buffer is still valid as when switching between the sessions, it could become invalid.
  if !nvim_buf_is_valid(s:display_bufnr)
    let s:display_bufnr = nvim_create_buf(v:false, v:true)
    let g:clap.display.bufnr = s:display_bufnr
  endif

  let s:display_opts = clap#layout#calc()
  silent let s:display_winid = nvim_open_win(s:display_bufnr, v:true, s:display_opts)

  call setwinvar(s:display_winid, '&winhl', s:display_winhl)
  call setwinvar(s:display_winid, '&spell', 0)
  call matchadd('ClapNoMatchesFound', g:__clap_no_matches_pattern, 10, -1, {'window': s:display_winid})
  " call setwinvar(s:display_winid, '&winblend', 15)

  let g:clap.display.winid = s:display_winid

  " call setwinvar(s:display_winid, '&listchars', 'extends:•')
  " \ '&listchars': 'extends:•'
  " listchars would cause some troubles in some files using tab.
  " Is there a better solution?

  call g:clap.display.setbufvar_batch({
        \ '&filetype': 'clap_display',
        \ 'autopairs_enabled': 0,
        \ 'ale_enabled': 0,
        \ })
endfunction

function! g:clap#floating_win#display.shrink_if_undersize() abort
  if !clap#preview#is_always_open()
    let opts = nvim_win_get_config(s:display_winid)
    if g:clap.display.line_count() < s:display_opts.height
      let opts.height = g:clap.display.line_count()
    else
      let opts.height = s:display_opts.height
    endif
    call nvim_win_set_config(s:display_winid, opts)
    call s:try_adjust_preview()
  endif
endfunction

function! g:clap#floating_win#display.shrink() abort
  if !clap#preview#is_always_open()
    let height = g:clap.display.line_count()
    let opts = nvim_win_get_config(s:display_winid)
    if opts.height != height
      let opts.height = height
      call nvim_win_set_config(s:display_winid, opts)
      call s:try_adjust_preview()
    endif
  endif
endfunction

function! s:set_minimal_buf_style(bufnr, filetype) abort
  call setbufvar(a:bufnr, '&filetype', a:filetype)
  call setbufvar(a:bufnr, '&signcolumn', 'no')
  call setbufvar(a:bufnr, '&foldcolumn', 0)
endfunction

function! s:get_config_border_left() abort
  let opts = nvim_win_get_config(s:display_winid)
  let opts.row -= 1
  let opts.width = s:symbol_width
  let opts.height = 1
  let opts.focusable = v:false
  if s:has_nvim_0_5
    let opts.zindex = 1000
  endif
  return opts
endfunction

function! s:open_win_border_left() abort
  if s:symbol_width > 0
    if !nvim_buf_is_valid(s:symbol_left_bufnr)
      let s:symbol_left_bufnr = nvim_create_buf(v:false, v:true)
    endif
    silent let s:symbol_left_winid = nvim_open_win(s:symbol_left_bufnr, v:false, s:get_config_border_left())
    call setwinvar(s:symbol_left_winid, '&winhl', 'Normal:ClapSymbol')
    call s:set_minimal_buf_style(s:symbol_left_bufnr, 'clap_spinner')
    call setbufline(s:symbol_left_bufnr, 1, s:symbol_left)
  endif
endfunction

function! s:get_config_spinner() abort
  let opts = nvim_win_get_config(s:display_winid)
  let opts.col += s:symbol_width
  let opts.row -= 1
  let opts.width = clap#spinner#width()
  let opts.height = 1
  let opts.focusable = v:false
  if s:has_nvim_0_5
    let opts.zindex = 1000
  endif
  return opts
endfunction

function! g:clap#floating_win#spinner.open() abort
  if exists('s:spinner_winid') && nvim_win_is_valid(s:spinner_winid)
    return
  endif
  if !nvim_buf_is_valid(s:spinner_bufnr)
    let s:spinner_bufnr = nvim_create_buf(v:false, v:true)
    let g:clap.spinner.bufnr = s:spinner_bufnr
  endif
  silent let s:spinner_winid = nvim_open_win(s:spinner_bufnr, v:false, s:get_config_spinner())

  call setwinvar(s:spinner_winid, '&winhl', 'Normal:ClapSpinner')
  call s:set_minimal_buf_style(s:spinner_bufnr, 'clap_s