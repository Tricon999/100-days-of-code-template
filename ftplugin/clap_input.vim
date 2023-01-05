if exists('b:clap_input_loaded') || !has('nvim')
  finish
endif

let b:clap_input_loaded = 1

setlocal
  \ nonumber
  \ norelativenumber
  \ nopaste
  \ nomodeline
  \ noswapfile
  \ nocursorline
  \ nocursorcolumn
  \ colorcolumn=
  \ nobuflisted
  \ buftype=nofile
  \ bufhidden=hide
  \ signcolumn=no
  \ textwidth=0
  \ nolist
  \ winfixwidth
  \ winfixheight
  \ nospell
  \ nofoldenable
  \ foldcolumn=0
  \ nowrap

augroup ClapOnTyped
  autocmd!
  autocmd CursorMoved,CursorMovedI <buffer> call clap#handler#on_typed()
augroup END

" From vim-rsi
if !exists('g:loaded_rsi')
  inoremap <silent> <buffer> <C-a>  <C-o>0
  inoremap <silent> <buffer> <C-X><C-A> <C-A>

  inoremap <silent> <buffer> <expr> <C-B> getline('.')=~'^\s*$'&&col('.')>strlen(getline('.'))?"0\<Lt>C-D>\<Lt>Esc>kJs":"\<Lt>Left>"
  inoremap <silent> <buffer> <expr> <C-F> col('.')>strlen(getline('.'))?"\<Lt>C-F>":"\<Lt>Right>"

  inoremap <silent> <buffer> <expr> <C-D> col('.')>strlen(getline('.'))?"\<Lt>C-D>":"\<Lt>Del>"
endif

" Use echo to clean the unwanted : at the bottom.
cnoremap <silent> <buffer> q :<c-u>call clap#handl