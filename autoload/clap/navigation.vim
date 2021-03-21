" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: Navigate between the result list.

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:lazy_load_size = 50

function! s:load_cache() abort
  let cache = g:clap.display.cache
  if len(cache) <= s:lazy_load_size
    let to_append = cache
    let g:clap.display.cache = []
  else
    let to_append = cache[:s:lazy_load_size-1]
    let g:clap.display.cache = cache[s:lazy_load_size :]
  endif
  if has_key(g:clap.provider._(), 'converter')
    let to_append = map(to_append, 'g:clap.provider._().converter(v:val)')
  endif
  " The buffer is not empty, qed.
  call g:clap.display.append_lines_unche