" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: Primiary code path for the plugin.

scriptencoding utf-8

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:has_features = has('nvim') ? has('nvim-0.4') : has('patch-8.1.2114')

if !s:has_features
  echoerr 'Vim-clap requires NeoVim >= 0.4.0 or Vim 8.1.2114+'
  echoerr 'Please upgrade your editor accordingly.'
  finish
endif

let s:cur_dir = fnamemodify(resolve(expand('<sfile>:p')), ':h')

let g:clap#autoload_dir = s:cur_dir

let g:__t_func = 0
let g:__t_string = 1
let g:__t_list = 2
let g:__t_func_string = 3
let g:__t_func_list = 4
let g:__t_rpc = 5

let s:provider_alias = {
      \ 'hist:': 'command_history',
      \ 'hist/': 'search_history',
      \ 'gfiles': 'git_files',
      \ }

let s:provider_alias = extend(s:provider_alias, get(g:, 'clap_provider_alias', {}))
let g:clap#provider_alias = s:provider_alias
let g:clap_disable_run_rooter = get(g:, 'clap_disable_run_rooter', v:false)
let g:clap_disable_bottom_top = get(g:, 'clap_disable_bottom_top', 0)
let g:clap_enable_debug = get(g:, 'clap_enable_debug', v:false)
let g:clap_forerunner_status