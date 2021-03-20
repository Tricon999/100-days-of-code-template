" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: Filter out the candidate lines synchorously given the input.
"
" NOTE: Deprecated as now the filtering is entiredly asynchrously done on the
" Rust end.

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:can_use_lua = has('nvim-0.5') || has('lua') ? v:true : v:false

let s:MIDIUM_CAPACITY = 30000

if exists('g:clap_builtin_fuzzy_filter_threshold')
  let s:builtin_filter_capacity = g:clap_builtin_fuzzy_filter_threshold
elseif s:can_use_lua
  let s:builtin_filter_capacity = s:MIDIUM_CAPACITY
else
  let s:builtin_filter_capacity = 10000
endif

let s:related_builtin_providers = ['tags', 'buffers', 'files', 'git_files', 'history', 'filer', 'grep', 'live_grep']

function! s:enable_icon() abort
  if g:clap_enable_icon
        \ && index(s:related_builtin_providers, g:clap.provider.id) > -1
    return v:true
  else
    return v:false
  endif
endfunction

function! clap#legacy#filter#get_bonus_type() abort
  if index(['files', 'git_files', 'filer'], g:clap.provider.id) > -1
    return 'FileName'
  else
    return 'None'
  endif
endfunction

function! clap#legacy#filter#matchfuzzy(query, candidates) abort
  " `result` could be a list of two lists, or a list of three
  " lists(newer vim).
  let result = matchfuzzypos(a:candidates, a:query)
  let filtered = result[0]
  let matched_indices = result[1]
  if s:enable_icon()
    let g:__clap_fuzzy_matched_indices = []
    for indices in matched_indices
      call add(g:__clap_fuzzy_matched_indices, map(indices, 'v:val + 2'))
    endfor
  else
    let g:__clap_fuzzy_matched_indices