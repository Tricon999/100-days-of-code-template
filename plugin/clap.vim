" vim-clap - Modern interactive filter and dispatcher
" Author:    Liu-Cheng Xu <xuliuchengxlc@gmail.com>
" Website:   https://github.com/liuchengxu/vim-clap
" Version:   0.41
" License:   MIT

if exists('g:loaded_clap')
  finish
endif

let g:loaded_clap = 1

command! -bang -nargs=* -bar -range -complete=customlist,clap#helper#complete Clap call clap#(<bang>0, <f-args>)

let g:__clap_buffers = get(g:, '__clap_buffers', {})

let g:__clap_tab_buffers = get(g:, '__clap_tab_buffers', {})

function! s:OnBufEnter(bufnr) abort
  let tabpagenr = tabpagenr()
  if !has_key(g:__clap_tab_buffers, tabpagenr)
    let g:__clap_tab_buffers[tabpagenr] = []
  endif
  if index(g:__clap_tab_buffers[tabpagenr], a:bufnr) == -1 && bufname('') !=# ''
    call add(g:__clap_tab_buffers[tabpagenr], a:bufnr)
  endif
endfunction

function! s:OnBufDelete(bufnr) abort
  if has_key(g:__clap_buffers, a:bufnr)
    call remove(g:__clap_buffers, a:bufnr)
  endif
  let tabpagenr = tabp