" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: CursorMoved handler

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:on_move_timer = -1
let s:on_move_delay = get(g:, 'clap_on_move_delay', 300)

function! s:sync_run_with_delay() abort
  if s:on_move_timer != -1
    call timer_stop(s:on_move_timer)
  endif
  let s:on_move_timer = timer_start(s:on_move_delay, { -> g:clap.provider._().on_move() })
endfunction

if clap#maple#is_available()
  " Deprecated.
  function! clap#impl#on_move#handler(result, error) abort
    if a:error is