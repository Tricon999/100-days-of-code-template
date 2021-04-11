" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: List the entries of the location list.

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:loclist = {}

function! s:loclist.source() abort
  let loclist = getloclist(g:clap.start.winid)
  if empty(loclist)
    return ['Location list is empty for window '.g