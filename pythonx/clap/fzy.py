#!/usr/bin/env python
# -*- coding: utf-8 -*-

import vim
from clap.scorer import fzy_scorer, substr_scorer


def str2bool(v):
    #  For neovim, vim.eval("a:enable_icon") is str
    #  For vim, vim.eval("a:enable_icon") is bool
    if isinstance(v, bool):
        return v
    else:
        return v.lower() in ("yes", "true", "t", "1")


def apply_score(scorer, query, candidates, enable_icon):
    scored = []

    for c in candidates:
        #  Skip two chars
        if enable_icon:
            candidate = c[2:]
        else:
            candidate = c
        score, indices = scorer(query, candidate)
        if score != float("-inf"):
            if enable_icon:
                indices = [x + 4 for x in indices]
            scored.append({'score': score, 'indices': indices, 'text': c})

    return scored


def fuzzy_match_py(query, candidates, enable_icon):
    if ' ' in query:
