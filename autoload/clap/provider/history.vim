" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: List the open buffers and oldfiles in order.

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:history = {}

function! s:raw_history() abort
  let history = uniq(map(
    \ filter([expand('%')], 'len(v:val)')
    \   + filter(map(clap#util#buflisted_sorted(v:false), 'bufname(v:val)'), 'len(v:val)')
    \   + filter(copy(v:oldfiles), "filereadable(fnamemodify(v:val, ':p'))"),
    \ 'fnamemodify(v:val, ":~:.")'))
  if exists('*g:ClapProviderHistoryCustomFilter')
    return filter(history, 'g:ClapProviderHistoryCustomFilter(v:val)')
  else
    return history
  endif
endfunction

function! s:history_sink(selected) abort
  let fpath = g:clap_enable_icon ? a:selected[4:] : a:select