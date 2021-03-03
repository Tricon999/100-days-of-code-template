" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: Custom window layout.

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:is_nvim = has('nvim')
let s:layout_keys = ['width', 'height', 'row', 'col', 'relative']

if !clap#preview#is_enabled()
  let s:default_layout = {
            \ 'width': '70%',
            \ 'height': '67%',
            \ 'row': '25%',
            \ 'col': '15%',
            \ }
elseif clap#preview#direction() ==# 'LR'
  let s:defau