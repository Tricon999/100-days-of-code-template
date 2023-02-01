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
cnoremap <silent> <buffer> q :<c-u>call clap#handler#exit()<CR>:echo<CR>

inoremap <silent> <buffer> <expr> <C-E> col('.')>strlen(getline('.'))<bar><bar>pumvisible()?"\<Lt>C-E>":"\<Lt>End>"

inoremap <silent> <buffer> <C-l> <Esc>:call clap#handler#relaunch_providers()<CR>

" Use this way when we need stopinsert inside the handler.
inoremap <silent> <buffer> <C-c> <Esc>:<c-u>call clap#handler#exit()<CR>
inoremap <silent> <buffer> <C-g> <Esc>:<c-u>call clap#handler#exit()<CR>
inoremap <silent> <buffer> <CR>  <Esc>:<c-u>call clap#handler#cr_action()<CR>

inoremap <silent> <buffer> <Down> <C-R>=clap#navigation#linewise_scroll('down')<CR>
inoremap <silent> <buffer> <Up>   <C-R>=clap#navigation#linewise_scroll('up')<CR>

inoremap <silent> <buffer> <ScrollWheelDown> <C-R>=clap#navigation#linewise_scroll('down')<CR>
inoremap <silent> <buffer> <ScrollWheelUp>   <C-R>=clap#navigation#linewise_scroll('up')<CR>

inoremap <silent> <buffer> <PageDown> <C-R>=clap#navigation#scroll('down')<CR>
inoremap <silent> <buffer> <PageUp>   <C-R>=clap#navigation#scroll('up')<CR>

inoremap <silent> <buffer> <Tab>       <C-R>=clap#handler#tab_action()<CR>
inoremap <silent> <buffer> <Backspace> <C-R>=clap#handler#bs_action()<CR>
inoremap <silent> <buffer> <A-u>       <C-R>=clap#handler#back_action()<CR>

inoremap <silent> <buffer> <LeftMouse>       <C-R>=clap#handler#tab_action()<CR>
inoremap <silent> <buffer> <RightMouse>      <C-R>=clap#handler#tab_action()<CR>

inoremap <silent> <buffer> <C-j> <C-R>=clap#navigation#linewise_scroll('down')<CR>
inoremap <silent> <buffer> <C-k> <C-R>=clap#navigation#linewise_scroll('up')<CR>

call clap#util#define_open_action_mappings()

if g:clap_insert_mode_only
  inoremap <silent> <buffer> <Esc> <Esc>:<c-u>call clap#handler#exit()<CR>
  finish
endif

nnoremap <silent> <buffer> <Esc>     :<c-u>call clap#handler#exit()<CR>

nnoremap <silent> <buffer> <C-l>     :<c-u>call clap#handler#relaunch_providers()<CR>

nnoremap <silent> <buffer> <C-c>     :<c-u>call clap#handler#exit()<CR>
nnoremap <silent> <buffer> <C-g>     :<c-u>call clap#handler#exit()<CR>
nnoremap <silent> <buffer> <CR>      :<c-u>call clap#handler#sink()<CR>

nnoremap <silent> <buffer> <Down> :<c-u>call clap#navigation#linewise_scroll('down')<CR>
nnoremap <silent> <buffer> <Up>   :<c-u>call clap#navigation#linewise_scroll('up')<CR>

nnoremap <silent> <buffer> <ScrollWheelDown> :<c-u>call clap#navigati