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
    if g:cl