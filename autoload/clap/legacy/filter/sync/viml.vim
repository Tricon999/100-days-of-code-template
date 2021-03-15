" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: Native VimL implementation of filter.
" Used when there is no +python3 and external binary.

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:pattern_builder = {}

function! s:pattern_builder._force_case() abort
  " Smart case
  return self.input =~? '\u' ? '\C' : '\c'
endfunction

function! s:pattern_builder.smartcase() abort
  let l:_force_case = self._force_case()
  let s:matchadd_pattern = l:_force_case.self.input
  return l:_force_case.self.input
endfunction

function! s:pattern_builder.substring() abort
  let l:_force_case = self._force_case()
  let l:filter_pattern = ['\V\^', l:_force_case]
  let s:matchadd_pattern = []
  for l:s in split(self.input)
    call add(filter_pattern, printf('\.\*\zs%s\ze', l:s))
    " FIXME can not distinguish `f f` highlight
    " these two f should be highlighed with different colors
    call add