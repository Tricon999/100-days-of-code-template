" Author: Jesse Cooke <jesse@relativepath.io>
" Description: Clap theme based on the Solarized Dark theme.

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:base03  = { 'hex': '#002b36', 'xterm': '234', 'xterm_hex': '#1c1c1c' }
let s:base02  = { 'hex': '#073642', 'xterm': '235', 'xterm_hex': '#262626' }
let s:base01  = { 'hex': '#586e75', 'xterm': '240', 'xterm_hex': '#585858' }
let s:base00  = { 'hex': '#657b83', 'xterm': '241', 'xterm_hex': '#626262' }
let s:base0   = { 'hex': '#839496', 'xterm': '244', 'xterm_hex': '#808080' }
let s:base1   = { 'hex': '#93a1a1', 'xterm': '245', 'xterm_hex': '#8a8a8a' }
let s:base2   = { 'hex': '#eee8d5', 'xterm': '254', 'xterm_hex': '#e4e4e4' }
let s:base3   = { 'hex': '#fdf6e3', 'xterm': '23