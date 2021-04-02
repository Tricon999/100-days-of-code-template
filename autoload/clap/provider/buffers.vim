" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: List the open buffers.

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:cur_tab_only = get(g:, 'clap_provider_buffers_cur_tab_only', v:false)

function! s:padding(origin, target_width) abort
  let width = strdisplaywidth(a:origin)
  if width < a:target_width
    return a:origin.repeat(' ', a:target_width - width)
  else
    return a:origin
  endif
endfunction

function! s:format_buffer(b) abort
  let buffer_name = bufname(a:b)
  let fullpath = empty(buffer_name) ? '[No Name]' : fnamemodify(buffer_name, ':p:~:.')
  let filename = empty(fullpath) ? '[No Name]' : fnamemodify(fullpath, ':t')
  let flag = a:b == bufnr('')  ? '%' : (a:b == bufnr('#') ? '#' : ' ')
  let modified = getbufvar(a:b, '&modified') ? ' [+]' : ''
  let readonly = getbufvar(a:b, '&modifiable') ? '' : ' [RO]'

  let filename = s:padding(filename, 25)
  let bp = s:padding('['.a: