" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: Functions for adding highlights to the display window.

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:default_priority = 10

if has('nvim')
  " lnum and col are 0-based.
  function! s:matchadd_char_at(lnum, col, hl_group) abort
    return matchaddpos(a:hl_group, [[a:lnum+1, a:col+1, 1]])
  endfunction

  " The highlight added by nvim_buf_add_highlight() can be overrided
  " by the sign's highlight, therefore matchaddpos() is used for neovim.
  function! s:add_display_highlights_impl(hl_lines) abort
    " We should not use clearmatches() here.
    call g:clap.display.matchdelete()

    let w:clap_match_ids = []

    let lnum = 0
    for indices in a:hl_lines
      let group_idx = 1
      for idx in indices
        if group_idx < g:__clap_fuzzy_matches_hl_group_cnt + 1
          call add(w:clap_match_ids, s:matchadd_char_at(lnum, idx, 'ClapFuzzyMatches'.group_idx))
          let group_idx += 1
        else
          call add(w:clap_match_ids, s:matchadd_char_at(lnum, idx, g:__clap_fuzzy_last_hl_group))
        endif
      endfor
      let lnum += 1
    endfor
  endfunction

  if exists('*win_execute')
    function! s:add_display_highlights(hl_lines) abort
      call win_execute(g:clap.display.winid, 'call s:add_display_highlights_impl(a:hl_lines)')
    endfunction

    " This is same with g:clap.display.clear_highlight()
    function! clap#highlight#clear() abort
      call win_execute(g:clap.display.winid, 'call g:clap.display.matchdelete()')
    endfunction
  else
    function! s:add_display_highlights(hl_lines) abort
      " Once the default highlight priority of nvim_buf_add_highlight() is
      " higher, we could use the same impl with vim's s:apply_highlight().

      noautocmd call g:clap.display.goto_win()
      call s:add_display_highlights_impl(a:hl_lines)
      noautocmd call g:clap.input.goto_win()
    endfunction

    " This is same with g:clap.display.clear_highlight()
    function! clap#highlight#clear() abort
      noautocmd call g:clap.display.goto_win()
      call g:clap.display.matchdelete()
      noautocmd call g:clap.input.goto_win()
    endfunction
  endif

  function! clap#highlight#add_highlight_at(lnum, col, hl_group) abort
    " 0-based
    call nvim_buf_add_highlight(g:clap.display.bufnr, -1, a:hl_group, a:lnum, a:col, a:col+1)
  endfunction

else
  function! s:add_display_highlights(hl_lines) abort
    " Avoid the error invalid buf
    if !bufexists(g:clap.display.bufnr)
      ret