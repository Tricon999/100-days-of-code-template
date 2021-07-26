
#!/usr/bin/env python3
# -*- coding: utf-8 -*-

#  This script is now depreated due to icon/build.rs.
#
#  Whenever you change exactmatch_map.json or extension_map.json,
#  rerun this script to regenerate src/constants.rs:
#    ./update_constants.py
import json
import os

lines = [
    '// This file is generated by ../update_constants.py.',
    '// Do not edit this file manually.',
    '',
]

with open('extension_map.json', 'r') as f:
    disordered = json.load(f)
    sorted_dict = {k: disordered[k] for k in sorted(disordered)}

    joined_tuples = ','.join(
        map(lambda kv: '("%s", \'%s\')' % (kv[0], kv[1]), sorted_dict.items()))
    lines.append('pub static EXTENSION_ICON_TABLE: &[(&str, char)] = &[%s];' %
                 joined_tuples)

with open('exactmatch_map.json', 'r') as f:
    disordered = json.load(f)
    sorted_dict = {k: disordered[k] for k in sorted(disordered)}

    joined_tuples = ','.join(
        map(lambda kv: '("%s", \'%s\')' % (kv[0], kv[1]), sorted_dict.items()))
    lines.append('pub static EXACTMATCH_ICON_TABLE: &[(&str, char)] = &[%s];' %
                 joined_tuples)

with open('tagkind_map.json', 'r') as f:
    disordered = json.load(f)
    sorted_dict = {k: disordered[k] for k in sorted(disordered)}

    joined_tuples = ','.join(
        map(lambda kv: '("%s", \'%s\')' % (kv[0], kv[1]), sorted_dict.items()))
    lines.append('pub static TAGKIND_ICON_TABLE: &[(&str, char)] = &[%s];' %
                 joined_tuples)

lines.append('''
pub fn bsearch_icon_table(c: &str, table: &[(&str, char)]) ->Option<usize> {
    table.binary_search_by(|&(key, _)| key.cmp(&c)).ok()
}
  ''')

with open('src/constants.rs', 'w') as f:
    f.write('\n'.join(lines))

os.system('rustfmt src/constants.rs')