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
    let s:builtin_providers = map(
          \ split(globpath(s:cur_dir.'/clap/provider', '*'), '\n'),
          \ 'fnamemodify(v:val, '':t:r'')'
          \ )
  endif
  return s:builtin_providers
endfunction

function! s:inject_default_impl_is_ok(provider_info) abort
  let provider_info = a:provider_info

  " If sync provider
  if has_key(provider_info, 'source')
    if !has_key(provider_info, 'on_typed')
      let provider_info.on_typed = { -> clap#client#notify('on_typed') }
    endif
    if !has_key(provider_info, 'filter')
      let provider_info.filter = function('clap#legacy#filter#sync')
    endif
  else
    if !has_key(provider_info, 'on_typed')
      call clap#helper#echo_error('Provider without source must specify on_moved, but only has: '.keys(provider_info))
      return v:false
    endif
    if !has_key(provider_info, 'jobstop')
      let provider_info.jobstop = function('clap#legacy#dispatcher#jobstop')
    endif
  endif

  return v:true
endfunction

function! s:detect_source_type() abort
  let Source = g:clap.provider._().source
  let source_ty = type(Source)

  if source_ty == v:t_string
    return g:__t_string
  elseif source_ty == v:t_list
    return g:__t_list
  elseif source_ty == v:t_func
    let string_or_list = Source()
    if type(string_or_list) == v:t_string
      return g:__t_func_string
    elseif type(string_or_list) == v:t_list
      return g:__t_func_list
    else
      call g:clap.abort('Must return a String or a List if source is a Funcref')
    endif
  endif
  return v:null
endfunction

function! clap#_init() abort
  call clap#spinner#init()

  call g:clap.provider.init_display_win()

  " Ensure the filetype is empty on init.
  " Each provider can set its own syntax for the highlight purpose.
  call g:clap.display.setbufvar('&filetype', '')
endfunction

function! clap#_exit() abort
  call g:clap.provider.jobstop()
  call clap#maple#clean_up()

  noautocmd call g:clap.close_win()
  call g:clap.preview.clear()
  call g:clap.display.matchdelete()

  let g:clap.display.cache = []
  let g:clap.display.initial_size = -1
  " Reset this for vim issue. Ref #223
  let g:clap.display.winid = -1

  " Remember to get what the sink needs before clearing the buffer.
  call g:clap.input.clear()
  call g:clap.display.clear()

  call clap#sign#reset_all()

  call clap#state#clear_post()
endfunction

function! clap#_for(provider_id_or_alias) abort
  let g:clap.provider.ar