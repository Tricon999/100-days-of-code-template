" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: Initialize the clap theme.

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:is_nvim = has('nvim')

let s:input_default_hi_group = 'Visual'
let s:display_default_hi_group = 'ClapDefaultPreview'
let s:preview_default_hi_group = 'PmenuSel'

function! s:extract(group, what, gui_or_cterm) abort
  return synIDattr(synIDtrans(hlID(a:group)), a:what, a:gui_or_cterm)
endfunction

function! s:extract_or(group, what, gui_or_cterm, default) abort
  let v = s:extract(a:group, a:what, a:gui_or_cterm)
  return empty(v) ? a:default : v
endfunction

" Try to sync the spinner bg with input window.
function! s:hi_spinner() abort
  let vis_ctermbg = s:extract_or(s:input_default_hi_group, 'bg', 'cterm', '60')
  let vis_guibg = s:extract_or(s:input_default_hi_group, 'bg', 'gui', '#544a65')
  let fn_ctermfg = s:extract_or('Function', 'fg', 'cterm', '170')
  let fn_guifg = s:extract_or('Function', 'fg', 'gui', '#bc6ec5')

  execute printf(
        \ 'hi ClapSpinner guifg=%s ctermfg=%s ctermbg=%s guibg=%s gui=bold cterm=bold',
        \ fn_guifg,
        \ fn_ctermfg,
        \ vis_ctermbg,
        \ vis_guibg,
        \ )
endfunction

function! s:hi_clap_symbol() abort
  let input_ctermbg = s:extract_or('ClapInput', 'bg', 'cterm', '60')
  let input_guibg = s:extract_or('ClapInput', 'bg', 'gui', '#544a65')
  let normal_ctermfg = s:extract_or('Normal', 'bg', 'cterm', '249')
  let normal_guifg = s:extract_or('Normal', 'bg', 'gui', '#b2b2b2')
  execute printf(
        \ 'hi ClapSymbol guifg=%s ctermfg=%s ctermbg=%s guibg=%s',
        \ input_guibg,
        \ input_ctermbg,
        \ normal_ctermfg,
        \ normal_guifg,
        \ )
endfunction

" Try the palette, otherwise use the built-in material_design_dark theme.
function! s: