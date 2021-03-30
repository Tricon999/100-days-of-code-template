" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: Various preview support.

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:path_seperator = has('win32') ? '\' : '/'
let s:default_size = 5

function! clap#preview#direction() abort
  if g:clap_preview_direction ==# 'AUTO'
    return &columns < 80 ? 'UD' : 'LR'
  else
    return g:clap_preview_direction
  endif
endfunction

function! s:peek_file(fname, fpath) abort
  if has_key(g:clap.preview, 'winid')
    let size = max([2 * s:default_size, winheight(g:clap.preview.winid)])
  else
    let size = 2 * s:default_size
  endif
  let lines = readfile(a:fpath, '', size)
  call insert(lines, a:fpath)
  call g:clap.preview.show(lines)
  call g:clap.preview.set_syntax(clap#ext#into_filetype(a:fname))
  call clap#preview#highlight_header()
endfunction

function! s:show_file_props(entry) abort
  let props = strftime('%B %d/%m/%Y %H:%M:%S', getftime(a:entry)).'    '.getfperm(a:entry)
  call g:clap.preview.show([a:entry, props])
  call clap#preview#highlight_header()
endfunction

if type(g:clap_preview_size) == v:t_number
  function! clap#preview#size_of(provider_id) abort
    return g:clap_preview_size
  endfunction
elseif type(g:clap_preview_size) == v:t_dict
  function! clap#preview#size_of(provider_id) abort
    if has_key(g:clap_preview_size, a:provider_id)
      return g:clap_preview_size[a:provider_id]
    elseif has_key(g:clap_preview_size, '*')
      return g:clap_preview_size['*']
    else
      return s:default_size
    endif
  endfunction
else
  throw 'g:clap_preview_size has to be a Number or Dict'
endif

" For blines, tags provider
function! clap#preview#buffer(lnum, origin_syntax) abort
  let [start, end, hi_lnum] = clap#preview#get_range(a:lnum)
  let lines = getbufline(g:clap.start.bufnr, start, end)
  call insert(lines, bufname(g:clap.start.bufnr).':'.a:lnum)
  let hi_lnum += 1
  call clap#preview#show_lines(lines, a:origin_syntax, hi_lnum+1)
  call clap#preview#highlight_header()
endfunction

" Preview entry for files,history provider
function! clap#preview#file(fname) abort
  " The preview action can be postponed, user can have closed the main window.
  if !g:clap.display.win_is_valid()
    return
  endif
  let fpath =