if exists('b:clap_display_loaded') || !has('nvim')
  finish
endif

let b:clap_display_loaded = 1

setlocal
  \ nowrap
  \ nonumber
  \ norelativenumber
  \ nopaste
  \ nocursorline
  \ nocursorcolumn
  \ foldcolumn=0
  \ nomodeline
  \ noswapfile
  \ colorcolumn=
  \ nobuflisted
  \ buftype=nofile
  \ bufhidden=hide
  \ signcolumn=yes
  \ textwidth=0
  \ nolist
  \ winfixwidth
  \ winfixheight
  \ nospell
  \ nofoldenable

in