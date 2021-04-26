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
let g:clap_forerunner_status_sign = get(g:, 'clap_forerunner_status_sign', {'done': '•', 'running': '!', 'using_cache': '*'})

" Backward compatible
if exists('g:clap_forerunner_status_sign_done')
  let g:clap_forerunner_status_sign.done = g:clap_forerunner_status_sign_done
endif

if exists('g:clap_forerunner_status_sign_running')
  let g:clap_forerunner_status_sign.running = g:clap_forerunner_status_sign_running
endif

let g:clap_no_matches_msg = get(g:, 'clap_no_matches_msg', 'NO MATCHES FOUND')
let g:__clap_no_matches_pattern = '^'.g:clap_no_matches_msg.'$'

let s:default_symbols = {
      \ 'arrow' : ["\ue0b2", "\ue0b0"],
      \ 'curve' : ["\ue0b6", "\ue0b4"],
      \ 'nil'   : ['', ''],
      \ }

let g:clap_search_box_border_symbols = extend(s:default_symbols, get(g:, 'clap_search_box_border_symbols', {}))
let g:clap_search_box_border_style = get(g:, 'clap_search_box_border_style',
      \ exists('g:spacevim_nerd_fonts') || exists('g:airline_powerline_fonts') ? 'curve' : 'nil')
let g:__clap_search_box_border_symbol = {
      \ 'left': get(g:clap_search_box_border_symbols, g:clap_search_box_border_style, '')[0],
      \ 'right': get(g:clap_search_box_border_symbols, g:clap_search_box_border_style, '')[1],
      \ }

let s:default_action = {
  \ 'ctrl-t': 'tab split',
  \ 'ctrl-x': 'split',
  \ 'ctrl-v': 'vsplit',
  \ }

let g:clap_open_preview = get(g:, 'clap_open_preview', 'always')
let g:clap_open_action = get(g:, 'clap_open_action', s:default_action)
let g:clap_enable_icon = get(g:, 'clap_enable_icon', exists('g:loaded_webdevicons') || get(g:, 'spacevim_nerd_fonts', 0))
let g:clap_popup_border = get(g:, 'clap_popup_border', has('nvim') ? 'single' : 'rounded')
let g:clap_preview_size = get(g:, 'clap_preview_size', 5)
let g:clap_preview_direction = get(g:, 'clap_preview_direction', 'AUTO')
let g:clap_insert_mode_only = get(g:, 'clap_insert_mode_only', v:false)
let g:clap_background_shadow_blend = get(g:, 'clap_background_shadow_blend', 50)
let g:clap_providers_relaunch_code = get(g:, 'clap_providers_relaunch_code', '@@')
let g:clap_enable_background_shadow = get(g:, 'clap_enable_background_shadow', v:false)
let g:clap_disable_matches_indicator = get(g:, 'clap_disable_matches_indicator', v:false)
let g:clap_multi_selection_warning_silent = get(g:, 'clap_multi_selection_warning_silent', 0)

function! clap#builtin_providers() abort
  if !exists('s:builtin_providers')
    let s:builtin_provi