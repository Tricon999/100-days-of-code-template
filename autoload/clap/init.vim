" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: Initialize the plugin, including making a compatible API layer
" and flexiable highlight groups.

let s:save_cpo = &cpoptions
set cpoptions&vim

function! s:hi_fg_only(group_name, cermfg, guifg) abort
  if !hlexists(a:group_name)
    execute printf(
          \ 'hi %s ctermfg=%s guifg=%s ctermbg=%s guibg=%s gui=bold cterm=bold',
          \ a:group_name,
          \ a:cermfg,
          \ a:guifg,
          \ 'NONE',
          \ 'NONE',
          \ )
  endif
endfunction

function! s:init_submatches_hl_group() abort
  let clap_sub_matches = [
        \ [173 , '#e18254'] ,
        \ [196 , '#f2241f'] ,
        \ [184 , '#e5d11c'] ,
        \ [32  , '#4f97d7'] ,
        \ [170 , '#bc6ec5'] ,
        \ [178 , '#ffbb7d'] ,
        \ [136 , '#b1951d'] ,
        \ [29  , '#2d9574'] ,
        \ ]

  " idx from 1
  call map(clap_sub_matches, 's:hi_fg_only("ClapMatches".(v:key