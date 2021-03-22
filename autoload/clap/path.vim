" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: Functions for working with the file path.

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:project_root_markers = get(g:, 'clap_project_root_markers', ['.root', '.git', '.git/'])

function! s:is_dir(pattern) abort
  return a:pattern[-1:] ==# '/'
endfunction

" Credit: vim-rooter
function! s:find_upwards(start_dir, pattern) abort
  let fd_dir = isdirectory(a:start_dir) ? a:start_dir : fnamemodify(a:start_dir, ':h')
  let fd_dir_escaped = escape(fd_dir, ' ')

  if s:is_dir(a:pattern)
    let match = finddir(a:pattern, fd_dir_escaped.';')
  else
    let [_suffixesadd, &suffixesadd] = [&suffixesadd, '']
    let match = findfile(a:pattern, fd_dir_escaped.';')
    let &suffixesadd = _suffixesadd
  endif

  if empty(match)
    return ''
  endif

  if s:is_dir(a:pattern)
    " If the directory we found (`match`) is part of the file's path
    " it is the project root and we return it.
    "
    " Compare with trailing path separators to avoid false positives.
    if stridx(fnamemodify(fd_dir, ':p'), fnamemodify(match, ':p')) == 0
      return fnamemodify(match, ':p:h')
    " Else the directory we found (`match`) is a subdirectory of the
    " pro