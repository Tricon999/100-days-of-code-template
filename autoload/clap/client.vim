
" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: Vim client talking to the Rust backend.

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:req_id = get(s:, 'req_id', 0)
let s:handlers = get(s:, 'handlers', {})
let s:session_id = get(s:, 'session_id', 0)

let s:last_recent_file = v:null

function! clap#client#handle(msg) abort
  let decoded = json_decode(a:msg)

  if has_key(decoded, 'deprecated_method')
    call call(decoded.deprecated_method, [decoded])
    return
  endif

  " Handle the request from Rust backend.
  if has_key(decoded, 'method')
    let params = get(decoded, 'params', [])
    try
      let result = clap#api#call(decoded.method, params)
      if has_key(decoded, 'id')
        call clap#job#daemon#send_message(json_encode({ 'id': decoded.id, 'result': result }))
      endif