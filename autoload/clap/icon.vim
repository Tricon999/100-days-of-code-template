" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: Icon decorator, derived from vim-devicons.

scriptencoding utf-8

let s:save_cpo = &cpoptions
set cpoptions&vim

let g:clap#icon#default = ''

let s:plugin_root_dir = fnamemodify(g:clap#autoload_dir, ':h')

function! s:deserialize_json(json_file) abort
  return json_de