#!/usr/bin/env python
# -*- coding: utf-8 -*-

from functools import partial

#  Credit: https://github.com/aslpavel/sweep.py/blob/master/sweep.py
#
#  Fuzzy matching for `fzy` utility
#  source: https://github.com/jhawthorn/fzy/blob/master/src/match.c

SCORE_MIN = float("-inf")
SCORE_MAX = float("inf")
SCORE_GAP_LEADING = -0.005
SCORE_GAP_TRAILING = -0.005
SCORE_GAP_INNER = -0.01
SCORE_MATCH_CONSECUTIVE = 1.0


def char_range_with(c_start, c_stop, v, d):
    d = d.copy()
    d.update((chr(c), v) for c in range(ord(c_start), ord(c_stop) + 1))
    return d


lower_with = partial(char_range_with, "a", "z")
upper_with = partial(char_range_with, "A", "Z")
digit_with = partial(char_range_with, "0", "9")

SCORE_MATCH_SLASH = 0.9
SCORE_MATCH_WORD = 0.8
SCORE_MATCH_CAPITAL = 0.7
SCORE_MATCH_DOT = 0.6
BONUS_MAP = {
    "/": SCORE_MATCH_SLASH,
    "-": SCORE_MATCH_WORD,
    "_": SCORE_MATCH_WORD,
    " ": SCORE_MATCH_WORD,
    ".": SCORE_MATCH_DOT,
}
BONUS_STATES = [{}, BONUS_MAP, lower_with(SCORE_MATCH_CAPITAL, BONUS_MAP)]
BONUS_INDEX = digit_with(1, lower_with(1, upper_with(2, {})))


def bonus(haystack):
    """
    Additional bonus based on previous char in haystack
    """
    c_prev = "/"
    bonus = []
    for c in haystack:
        bonus.append(BONUS_STATES[BONUS_INDEX.get(c, 0)].get(c_prev, 0))
        c_prev = c
    return bonus


def subsequence(niddle, haystack):
    """
    Check if niddle is subsequence of haystack
    """
    niddle, haystack = niddle.low