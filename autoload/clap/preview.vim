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
  let fpath = expand(a:fname)
  if filereadable(fpath)
    call s:peek_file(a:fname, fpath)
    return
  elseif exists('g:__clap_provider_cwd')
    let fpath_with_cwd = g:__clap_provider_cwd.s:path_seperator.fpath
    if filereadable(fpath_with_cwd)
      call s:peek_file(a:fname, fpath_with_cwd)
      return
    endif
  endif
  call s:show_file_props(a:fname)
endfunction

function! clap#preview#file_at(fpath, lnum) abort
  let [start, end, hi_lnum] = clap#preview#get_range(a:lnum)
  if filereadable(a:fpath)
    let lines = readfile(a:fpath)[start : end]
  else
    let cwd = clap#rooter#working_dir()
    if filereadable(cwd.s:path_seperator.a:fpath)
      let lines = readfile(cwd.s:path_seperator.a:fpath)[start : end]
    else
      return
    endif
  endif
  call insert(lines, a:fpath)
  call g:clap.preview.show(lines)
  call g:clap.preview.set_syntax(clap#ext#into_filetype(a:fpath))
  call g:clap.preview.add_highlight(hi_lnum+1)
  call clap#preview#highlight_header()
endfunction

" Given the origin lnum and the size of range, return
" [origin_lnum-range_size, origin_lnum+range_size] and the target lnum that
" the origin line should be positioned.
" 0-based
function! clap#preview#get_line_range(origin_lnum, range_size) abort
  if a:origin_lnum - a:range_size > 0
    return [a:origin_lnum - a:range_size, a:origin_lnum + a:range_size, a:range_size]
  else
    return [0, a:origin_lnum + a:range_size, a:origin_lnum]
  endif
endfunction

function! clap#preview#get_range(origin_lnum) abort
  let size = clap#preview#size_of(g:clap.provider.id)
  if clap#preview#direction() ==# 'LR'
    let size = max([size, winheight(g:clap.display.winid) / 2])
  endif
  return clap#preview#get_line_range(a:origin_lnum, size)
endfunction

function! clap#preview#show_lines(lines, syntax, hi_lnum) abort
  call g:clap.preview.show(a:lines)
  call g:clap.preview.set_syntax(a:syntax)
  if a:hi_lnum > 0
    call g:clap.preview.add_highlight(a:hi_lnum)
  endif
endfunction

let s:preview_timer = -1
let s:preview_delay = get(g:, 'clap_preview_delay', 100)

let s:PREVIEW_DISABLED = tolower(g:clap_open_preview) ==# 'never'
let s:ALWAYS_OPEN_PREVIEW = tolower(g:clap_open_preview) ==# 'always'

function! clap#preview#is_enabled() abort
  return !s:PREVIEW_DISABLED
endfunction

function! clap#preview#is_always_open() abort
  return s:ALWAYS_OPEN_PREVIEW
endfunction

function! clap#preview#update_with_delay(