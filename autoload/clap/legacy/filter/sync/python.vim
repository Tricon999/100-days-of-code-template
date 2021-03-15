" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: Python and Rust implementation of fzy filter algorithm.

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:py_exe = has('python3') ? 'python3' : 'python'
let s:pyfile = has('python3') ? 'py3file' : 'pyfile'
let s:plugin_root_dir = fnamemodify(g:clap#autoload_dir, ':h')

if has('win32')
  let s:LIB = '\pythonx\clap\fuzzymatch_rs.pyd'
  let s:SETUP_PY = '\setup_python.py'
else
  let s:LIB = '/pythonx/clap/fuzzymatch_rs.so'
  let s:SET