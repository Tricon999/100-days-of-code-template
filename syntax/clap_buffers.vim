syntax match ClapBuffersNumberBracket  /\[\|\]/ contained
syntax match ClapBuffersNumber /^\[\d\+\]/ contains=ClapBuffersNumberBracket
syntax match ClapBuffersFsize  /\(\d\|\.\)\+\(K\|B\|G\)/
syntax match ClapBuffersLnum   /line \d\+/ contained
syntax match ClapBuffersExtra   /\[\(+\|#\)\]/ contained
syntax match ClapBuffersFname   /line.*$/ contains=ClapBuffersLnum,ClapBuffersExtra

hi default link ClapBuffersNumbe