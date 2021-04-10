" Author: Mark Wu <markplace@gmail.com>
" Description: List the help tags, ported from https://github.com/zeero/vim-ctrlp-help

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:help_tags = {}

function! s:get_doc_tags() abort
  return ['/doc/tags'] + map(filter(split(&helplang, ','), 'v:val !=? "en"'), '"/doc/tags-".v:val')
endfunction

if clap#maple#is_available()

  " No source attribute as it's implemented on the Rust side directly.

  function! s:help_tags.on_typed() abort
    call clap#client#notify('on_typed')
  endfunction

  function! s:help_tags_sink(line) abort
    let [tag, doc_fname] = split(a:line, "\t")
    if doc_fname =~# '.txt$'
      execute 'help' trim(tag).'@en'
    else
      execute 'help' tag
    endif
  endfunction

  let s:help_tags.source_type = g:__t_rpc
  let s:help_tags.on_move_async = function('clap#impl#on