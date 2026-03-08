"""
Regenerates Temper code found between special markdown comments.

  <!-- TRANSITION_TABLE: Ident; stateVariableFoo, stateVariableBar -->

  | In                    | Regex                | Substitution | Out         |
  | --------------------- | -------------------- | ------------ | ----------- |
  | Foo, _                | `regex`              | "optional"   | Foo2, _     |
  | ...                                                                       |

  <!-- /TRANSITION_TABLE -->

From those Markdown tables it generates an automaton that,
when the input state pattern matches, it applies the regular expression,
and then transitions to the out state.

The markdown may be preceded by one or more grammar abbreviations of the form

    {{name}} = `regex`

Anywhere a left part (name with curlies) occurs in a right hand side or in a back-ticked
Regex column entry, the right side (sans ticks) will be substituted.
This allows sharing verbose grammar definitions like a languages specific set of "space"
characters.

It fills in a ContextPropagator implementation (see core/contextual-autoescaping.temper.md)
inside markers like the below, deleting any content there previously.

  <!-- GENERATED_PROPAGATION_FN: Ident -->

  ...

  <!-- /GENERATED_PROPAGATION_FN -->

Similarly for the below, it will fill in a mermaid diagram from the transitions
in the corresponding table where the given variable names are the nodes, and it ignores
other transitions/variables.

  <!-- GENERATED_TRANSITION_DIAGRAM: Ident; varName -->

  ...

  <!-- /GENERATED_TRANSITION_DIAGRAM -->

"""

import json
import re
from typing import cast, overload

class TransTableRow:
    """
    Each transition table row has an input pattern, a regex, a substitution, and output patterns/actions.
    """
    def __init__(self, input_pattern: str, regex: str, substitution: str, output: str, problem: str):
        self.input_pattern = input_pattern
        self.regex = regex
        self.substitution = substitution
        self.output = output
        self.problem = problem

class TransTable:
    """
    A transition table consisting of rows, each a possible transition to apply, in order.
    """

    ident: str
    var_names: list[str]
    rows: list[TransTableRow]

    def __init__(self, ident: str, var_names: list[str], rows: list[TransTableRow]):
        self.ident = ident
        self.var_names = var_names
        self.rows = rows

class Region:
    """Files are split into regions so we can replace some and not others easily"""
    text: str

class TextRegion(Region):
    """A region of text in a file being processed that doesn't need replacing"""
    def __init__(self, text: str):
        self.text = text

class TableRegion(Region):
    """
    We have to do detailed analysis on this kind of region to figure out replacements,
    but they are not themselves replaced
    """
    table: TransTable
    def __init__(self, table: TransTable, text: str):
        self.table = table
        self.text = text

class SubstRegion(Region):
    """
    A region that gets replaced.
    """
    def __init__(self, kind: str, details: str, text: str):
        self.kind = kind
        self.details = details
        self.text = text

class TemperMdFile:
    """
    Bundles together extracted information and regions to make it easier to generate
    replacements.
    """
    def __init__(self, file_path: str, constants: dict[str, int], regions: list[Region]):
        self.file_path = file_path
        self.constants = constants
        self.regions = regions

    file_path: str
    constants: dict[str, int]
    regions: list[Region]


def var_info_for_table(
    table: TransTable,
    constants: dict[str, int],
    var_names: list[str] | None = None
) -> dict[str, "VarInfo"]:
    """
    A dict mapping each var_name to the VarInfo for it.
    """
    if var_names is None:
        var_names = table.var_names
    var_info = {}
    for var_name in var_names:
        try:
            idx = table.var_names.index(var_name)
        except ValueError:
            raise SyntaxError(f'For table {table.ident}: {var_name} from {var_names} not in {table.var_names}')
        constants_for_name = {}
        for (k, v) in constants.items():
            if k.startswith(var_name):
                if v in constants_for_name:
                    raise SyntaxError(
                        f'variable {var_name} for transition table {table.ident} has duplicate constants with value {v}: {k} and {constants_for_name[v]}'
                    )
                constants_for_name[k] = v
                constants_for_name[v] = k
        var_info[var_name] = VarInfo(var_name, constants_for_name, idx)
    return var_info

class VarInfo:
    """
    For a context state variable, we need its index and its number variable mapping.
    """
    name:str
    constants:dict[str | int, int | str]
    pattern_index:int

    def __init__(self, var_name, constants, pattern_index):
        self.name = var_name
        self.constants = constants
        self.pattern_index = pattern_index
    def __repr__(self):
        return repr({ 'name': self.name, 'idx': self.pattern_index, 'constants': self.constants })

_constant_decl = re.compile(r'^    +(?:let|const)\s+(\w+)(?:\s*:\s*Int\d*)?\s*=\s*(\d+)\s*;')
_transtbl_start = re.compile(r'<!--\s*TRANSITION_TABLE\s*:\s*(\w+)(?:;(.*?))?-->')
_transtbl_end = re.compile(r'<!--\s*/TRANSITION_TABLE.*?-->')
_gensegment_start = re.compile(r'<!--\s*GENERATED_(PROPAGATION_FN|TRANSITION_DIAGRAM)\s*:\s*(\w+.*?)\s*-->')
_gensegment_end = re.compile(r'<!--\s*/GENERATED_(PROPAGATION_FN|TRANSITION_DIAGRAM).*?-->')

def parse_md(text: str, file_path: str) -> TemperMdFile:
    constants = {}
    regions = []
    region_start = None
    region_start_line_num = 0
    region_lines = []

    markers = (
        (_transtbl_start, 1),
        (_transtbl_end, -1),
        (_gensegment_start, 2),
        (_gensegment_end, -2),
    )

    line_num = 0
    for line in text.splitlines(keepends=True):
        line_num += 1
        const_match = _constant_decl.search(line)
        if const_match:
            constants[const_match[1]] = int(const_match[2])
        # Look for the special region start and end markers
        marker_match = None
        for (marker_pattern, marker_idx) in markers:
            marker_match = marker_pattern.search(line)
            if marker_match: break
        if marker_match:
            if (marker_idx > 0 and region_start is not None) or (marker_idx < 0 and region_start is None):
                raise SyntaxError(f'{file_path}:{line_num}: region marker misplaced')
            if marker_idx > 0: # Region start
                region_lines.append(line)
                text_region = TextRegion(''.join(region_lines))
                regions.append(text_region)
                region_start = line
                del region_lines[:]
            else: # Region end
                assert region_start is not None
                text = ''.join(region_lines)
                del region_lines[:]
                tt_match = _transtbl_start.search(region_start)
                if tt_match:
                    regions.append(parse_table_region(tt_match[1], tt_match[2], text, file_path, region_start_line_num))
                else:
                    s_match = _gensegment_start.search(region_start)
                    assert s_match is not None
                    regions.append(SubstRegion(s_match[1], s_match[2], text))
                region_lines.append(line)
                region_start = None
            region_start_line_num = line_num
        else:
            region_lines.append(line)
    if region_start is not None:
        raise SyntaxError(f'{file_path}:{region_start_line_num}: unclosed region')
    regions.append(TextRegion(''.join(region_lines)))

    return TemperMdFile(
        file_path = file_path,
        constants = constants,
        regions = regions
    )

_grammar_def_pattern = re.compile(r'^(\{\{[^\}]+\}\})\s*=\s*`(.*)`$')
_grammar_left_pattern = re.compile(r'\{\{[^\}]+\}\}')

def parse_table_region(ident: str, state_vars: str, table_text: str, file_path: str, line_num: int) -> TableRegion:
    state_var_names = [x.strip() for x in state_vars.split(',')]

    rows = []
    table_lines = table_text.strip().splitlines()

    # First look for grammar definitions of the form:
    # {{...}} = `...`
    # which are then substituted into any {{...}} around an identifier in the pattern column entries.
    grammar_subs: dict[str, str] = dict()
    while (table_lines):
        line = table_lines[0].strip()
        if not line:
            table_lines = table_lines[1:]
            continue
        grammar_def_match = _grammar_def_pattern.match(line)
        if not grammar_def_match: break
        grammar_subs[grammar_def_match.group(1)] = grammar_def_match.group(2)
        table_lines = table_lines[1:]
    grammar_subs_expanded = set() # We need to do substitution into the right side of a grammar_sub but don't do it more than once

    def grammar_sub(left: str) -> str:
        """The right corresponding to a left like '{{name}}'"""
        if left not in grammar_subs:
            raise SyntaxError(f'{file_path}:{line_num}: no grammar production {left} defined')
        if left not in grammar_subs_expanded:
            # Expand patterns in definition's right hand sides on demand
            grammar_subs_expanded.add(left) # But don't reenter
            grammar_subs[left] = expand_pattern(grammar_subs[left])
        return grammar_subs[left]

    def expand_pattern(s: str) -> str:
        return _grammar_left_pattern.sub(lambda m: grammar_sub(m.group(0)), s)

    if '--' not in table_lines[1]:
        raise SyntaxError(
            f'{file_path}:{line_num}: table does not have --- on second line: {table_lines[1]} after extracting grammar definitions {list(grammar_subs.keys())}'
        )

    row_num = 0
    for table_line in table_lines[2:]:
        row_num += 1
        table_line = table_line.strip()
        table_line = table_line.strip('|') # Left and right bar
        table_line_cells = [x.strip() for x in table_line.split('|')]
        if len(table_line_cells) != 5:
            raise SyntaxError(f'{file_path}:{line_num}: table row #{row_num} does not have 5 cells: {table_line !r}')
        (inp, pattern, sub, out, problem) = table_line_cells
        pattern = expand_pattern(pattern)
        rows.append(TransTableRow(inp, pattern, sub, out, problem))

    table = TransTable(ident, state_var_names, rows)

    return TableRegion(table, table_text)

def decompose_regex(regex, file_path):
    """
    We need to split regexen in two so that we
    can separately process the consumed part from the lookahead part.

    This splits a regex with a trailing lookahead in two.

    "x+"        -> ("x+",  None,  None)
    "(?=foo)"   -> (None,  "foo", True)
    "(?!foo)"   -> (None,  "foo", False)
    "foo(?=bar) -> ("foo", "bar", True)
    """
    matcher = []
    lookahead = []

    lookahead_kind = None # True for positive, false for negative

    remainder = regex

    ignore_case = remainder.startswith('(?i)')
    if ignore_case: remainder = remainder[4:]

    while remainder:
        m = regex_atom.match(remainder)
        if not m:
            if lookahead_kind is not None:
                raise SyntaxError(f'{file_path}: cannot process regex `{regex}`, remainder `{remainder}`')
            if remainder.startswith('(?='):
                lookahead_kind = True
            elif remainder.startswith('(?!'):
                lookahead_kind = False
            else:
                raise SyntaxError(f'{file_path}: cannot process regex `{regex}`, remainder `{remainder}`')
            remainder = remainder[3:]
            lookahead.append('(?:')
        else:
            atom = m.group(0)
            remainder = remainder[m.end():]
            if atom == '/':
                # | is not allowed in Markdown tables so we use / instead.
                # Any unescaped / is converted.
                # To include a literal pipe that is not alternation, use a charset like [|]
                atom = '|'
            elif atom in (r'\/', r'\"', r'\''):
                atom = atom[1]
            elif atom == r'\s':
                atom = r'[\t\r\n ]'
            elif atom.startswith(r'\u'):
                try:
                    atom = _decode_unicode_escape(atom)
                except ValueError:
                    raise SyntaxError(f'{file_path}: regex {regex} has malformed \\u escape, {atom}')

            # Fixup charsets and case insensitivity
            if atom.startswith('[') and atom.endswith(']'):
                # expand \s and / as above.
                def fix_part(s):
                    if s == '/':
                        return '|'
                    elif s in (r'\/', r'\"', r'\''):
                        return s[1]
                    elif s == r'\s':
                        return r'\t\r\n '
                    elif s.startswith(r'\u'):
                        try:
                            return _decode_unicode_escape(s)
                        except ValueError:
                            raise SyntaxError(f'{file_path}: regex {regex} has malformed \\u escape, {s}')
                    return s
                ranges = []
                charset_body = atom[1:-1]
                charset_negation = ''
                if charset_body.startswith('^'):
                    charset_negation = '^'
                    charset_body = charset_body[1:]
                for m in _charset_ranges_pattern.finditer(charset_body):
                    left = fix_part(m.group(1))
                    right = m.group(2)
                    if right is not None: right = fix_part(right)
                    ranges.append((left, right))
                if ignore_case:
                    fixed_ranges = []
                    for r in ranges:
                        case_fold_regex_range(r, fixed_ranges)
                else:
                    fixed_ranges = ranges
                fixed_ranges = list(set(fixed_ranges))

                # We should fix the bug that prevents us from having \' and \" in the same
                # regex but in the meantime, work around by treating these two as equivalent
                #    [^\"\'...]
                #    (?:[^!-\(...]|[!\(])
                # This specific pattern of negated charset with both is surprisingly useful.
                do_quote_fixup = None
                if charset_negation:
                    try:
                        dq_index = fixed_ranges.index(('"', None))
                    except ValueError:
                        dq_index = None
                    try:
                        sq_index = fixed_ranges.index(("'", None))
                    except ValueError:
                        sq_index = None
                    if dq_index is not None and sq_index is not None:
                        fixed_ranges.remove(('"', None))
                        fixed_ranges.remove(("'", None))
                        do_quote_fixup = []
                        inclusive = (chr(ord('"') - 1), chr(ord('\'') + 1))
                        for cp in range(ord(inclusive[0]), ord(inclusive[1]) + 1):
                            if cp == 0x22 or cp == 0x27: continue
                            c = chr(cp)
                            found_cp = False
                            for (left, right) in fixed_ranges:
                                if right is None: right = left
                                if left <= c and c <= right:
                                    found_cp = True
                                    break
                            if not found_cp:
                                do_quote_fixup.append(c)
                        fixed_ranges.append(inclusive)
                try:
                    fixed_ranges.remove((r'\-', None))  # \- not recognized.  Just put '-' at the beginning.
                    # Make sure the last one has a right point so the `-` does not attach to it.
                    fixed_ranges[:0] = [('-', None)]
                except ValueError:
                    pass
                if ('\f', None) in fixed_ranges:
                    # HACK: The regex parser in Temper cannot handle \f or \u000c.
                    # Produce approximate ASCII control character handling regex for now.
                    # TODO: Fix the regex parser.
                    fixed_ranges.remove(('\f', None))

                range_strs = []
                fixed_ranges.sort(key=lambda x: (x[0], x[x[1] is not None and 1 or 0]))
                for (left, right) in fixed_ranges:
                    if right is None:
                        range_strs.append(left)
                    else:
                        range_strs.append(f'{left}-{right}')
                atom = f'[{charset_negation}{"".join(range_strs)}]'
                if do_quote_fixup is not None and len(do_quote_fixup):
                    atom = f'(?:{atom}|[{"".join(do_quote_fixup)}])'
            elif ignore_case and (('a' <= atom and atom <= 'z') or ('A' <= atom and atom <= 'Z')):
                atom = f'[{atom.upper()}{atom.lower()}]'

            if lookahead_kind is None:
                matcher.append(atom)
            else:
                lookahead.append(atom)

    def join_and_anchor(parts: list[str]) -> str | None:
        """
        Putting ^ at the front forces matching only at the beginning of strings

        Without that, trying the rules in order picks the first one that matches
        anywhere, not just at the beginning, and leaves leading content matched
        by late rules unrecognized.
        """
        unanchored = ''.join(parts)
        if not unanchored: return None
        return f'^(?:{unanchored})'

    return join_and_anchor(matcher), join_and_anchor(lookahead), lookahead_kind

def _decode_unicode_escape(s: str) -> str:
    r"""\uABCD -> U+ABCD, \u{abcd} -> U+ABCD"""
    if s.startswith(r'\u{') and s.endswith(r'}'):
        s = s[3:-1]
    else:
        s = s[2:]
    return chr(int(s, 16))

def case_fold_regex_range(r, out):
    """
    Expands regex character ranges of the form ('A', 'Z') or ('_', None) so that they
    would match as if an ASCII case insensitivity modifier were applied.

    Ranges of that same form are added to out.
    """
    left, right = r
    if right is None:
        # range is a single char specified by left
        if ('A' <= left and left <= 'Z') or ('a' <= left and left <= 'z'):
            out.append((left.upper(), None))
            out.append((left.lower(), None))
        else:
            out.append(r)
    else:
        # Three cases:
        # 1. The range is disjoint from letters. Just emit it.
        # 2. The range includes all letters.  Just emit it.
        # 3. The range is all letters
        #    Copy it.
        # 4. The range overlaps letters and non-letters
        #    Split it into letters and non-letters.  Copy the letters.
        if right < 'A' or left > 'z' or (left > 'Z' and right < 'a'): # 1
            out.append(r)
        elif left <= 'A' and right >= 'z': # 2
            out.append(r)
        elif right < left: # Odd
            out.append(r)
        else:
            letter_ranges = []
            def is_letter(c):
                return ('A' <= c and c <= 'Z') or ('a' <= c and c <= 'z')
            in_letter_range = is_letter(left)
            range_start = left
            for i in range(ord(left) + 1, ord(right)):
                c = chr(i)
                is_c_letter = is_letter(c)
                if is_c_letter != in_letter_range:
                    r = (range_start, chr(i - 1))
                    if in_letter_range:
                        letter_ranges.append(r)
                    else:
                        out.append(r)
                    range_start = c
                    in_letter_range = is_c_letter
            r = (range_start, right)
            if in_letter_range:
                letter_ranges.append(r)
            else:
                out.append(r)
            letter_indices = set([])
            for (l, r) in letter_ranges:  # noqa E741: l for left in context
                for i in range(ord(l), ord(r) + 1):
                    letter_indices.add(i | 32)
                    letter_indices.add(i & ~32)
            letter_indices = list(letter_indices)
            letter_indices.sort()
            range_start = letter_indices[0]
            for i in range(1, len(letter_indices) + 1):
                if i < len(letter_indices):
                    letter_index = letter_indices[i]
                else:
                    letter_index = 0x1_0000_0000
                last = letter_indices[i - 1]
                if letter_index != last + 1:
                    out.append((chr(range_start), chr(last)))
                    range_start = letter_index
    return

_unicode_esc = r'\\u(?:[0-9a-fA-F]{4}|\{[^\}]*\}?)?'
regex_atom = re.compile(rf'[^\(\[\\]|\[(?:[^\]\\]|\\.)*\]|\\[^u]|{_unicode_esc}|\((?![?]<?[=!])')
#                          ┗━━━━━━━┛ ┗━━━━━━━━━━━━━━━━━━┛ ┗━━━━┛ ┗━━━━━━━━━━━━┛ ┗━━━━━━━━━━━━━┛
#                Not '(', '[', '\\'         Charset       Esc    Unicode Esc    Paren not lookahead/behind
_regex_part = rf'[^\\]|{_unicode_esc}|\\.'
_charset_ranges_pattern = re.compile(rf'({_regex_part})(?:-({_regex_part}))?')

def regen_region(region: Region, current: TemperMdFile, temper_md_files: list[TemperMdFile]) -> Region:
    if not isinstance(region, SubstRegion): return region

    if region.kind == 'PROPAGATION_FN':
        return regen_context_propagate_function(region, current, temper_md_files)
    elif region.kind == 'TRANSITION_DIAGRAM':
        return regen_transition_table(region, current, temper_md_files)
    else:
        raise SyntaxError(f'kind {region.kind}, detail {region.details}')

@overload
def source_for_ident(ident: str, temper_md_files: list[TemperMdFile]) -> tuple[TableRegion, TemperMdFile]: ...

@overload
def source_for_ident(ident: str, temper_md_files: list[TemperMdFile]) -> tuple[None, None]: ...

def source_for_ident(ident: str, temper_md_files: list[TemperMdFile]) -> tuple[TableRegion, TemperMdFile] | tuple[None, None]:
    for temper_md_file in temper_md_files:
        for x in temper_md_file.regions:
            if isinstance(x, TableRegion) and x.table.ident == ident:
                return x, temper_md_file
    return None, None

def regen_context_propagate_function(region: SubstRegion, current: TemperMdFile, temper_md_files: list[TemperMdFile]) -> Region:
    file_path = current.file_path
    debug_trace = False

    details_parts = re.split(r'\s+', region.details.strip())

    ident = details_parts[0]
    context_name = f'{ident}EscaperContext'
    esc_name = f'{ident}Escaper'

    for part in details_parts[1:]:
        if part in ('+debug', '-debug'):
            debug_trace = (part == '+debug')
        elif part.startswith('esc='):
            esc_name = part[4:]
        else:
            raise SyntaxError(f'{file_path}: unrecognized detail `{part}`')

    source, source_file = source_for_ident(ident, temper_md_files)
    if source is None:
        raise SyntaxError(f'{file_path}: no source table for {ident}')

    table = source.table
    constants = source_file.constants
    var_infos = var_info_for_table(table, constants)

    lcase_ident = f'{ident[0].lower()}{ident[1:]}'

    out_lines = ['']

    # Sometimes we need a helper function.  They're scoped by ident name mangling
    def computation_name(hash_id: str) -> str:
        # "#foo" -> "computeIdentFoo"
        assert hash_id.startswith('#')
        return f'compute{ident}{hash_id[1].upper()}{hash_id[2:]}'

    indent = [0]
    def emit_line(line):
        if not line:
            out_lines.append('')
            return
        if line[0] in '})]': indent[0] -= 1
        out_lines.append(f'{"  " * indent[0]}{line}')
        if line[-1] in '{([': indent[0] += 1

    # For each state variable, output some heplers:
    # - a stringification function for each state enum.
    # - a state equality predicate
    for var_name in table.var_names:
        var_info = var_infos[var_name]

        min_val = None
        max_val = None
        for k, v in var_info.constants.items():
            if isinstance(k, int):
                if min_val is None or k < min_val:
                    min_val = k
                if max_val is None or k > max_val:
                    max_val = k
        if min_val is None:
            min_val, max_val = 0, -1
        assert max_val is not None

        emit_line('let %sStr(x: Int32): String {' % (var_name,))
        emit_line('when (x) {')
        for i in range(min_val, max_val + 1):
            name = var_info.constants.get(i)
            if name is not None:
                assert isinstance(name, str)
                assert name.startswith(var_name)
                suffix = name[len(var_name):]
                emit_line(f'{name} -> {escape_temper_str(suffix)};')
        emit_line('else -> x.toString();')
        emit_line('}')
        emit_line('}')
        emit_line('')

    emit_line(f'let {lcase_ident}StatesEqual(')
    emit_line(f'a: AutoescState<{context_name}, {esc_name}>,')
    emit_line(f'b: AutoescState<{context_name}, {esc_name}>,')
    emit_line('): Boolean {')
    emit_line('if (a.subsidiary != b.subsidiary) { return false; }')
    emit_line(f'let c: {context_name} = a.context;')
    emit_line(f'let d: {context_name} = b.context;')
    for var_name in table.var_names:
        emit_line('if (c.%(var_name)s != d.%(var_name)s) { return false; }' % { 'var_name': var_name })
    emit_line('true')
    emit_line('}')
    emit_line('')

    # Output the context propagation function
    emit_line(f'let {lcase_ident}PropagateContext(')
    emit_line(f'before: AutoescState<{context_name}, {esc_name}>,')
    emit_line(f'literalPart: String?,')
    emit_line(f'callout: DiagnosticCallout?,')
    emit_line(f'): AfterPropagate<{context_name}, {esc_name}> {{')
    emit_line(f'let contextBefore = before.context;')
    if debug_trace:
        emit_line('console.log("propagating %s from ${before} over ${if (literalPart == null) { "null" } else { "`${literalPart}`" } }");' % ident)

    regexs = set(())
    for x in table.rows:
        if x.regex.startswith('`'):
            # regexen are formatted as code using Markdown '`'.
            # You can always \u escape literal backticks
            x = x.regex.strip('`')
            (matched, lookahead, _) = decompose_regex(x, file_path)
            if matched: regexs.add(matched)
            if lookahead: regexs.add(lookahead)
    # Map local variable names to regex values so we can access regexen when we need them.
    regex_names = {}
    for regex in sorted(regexs):
        regex_name = f'pattern{len(regex_names)}'
        if '"' not in regex:
            qt = '"'
        elif '\'' not in regex:
            qt = '\''
        else:
            raise SyntaxError(f'{file_path}: Regex {regex} contains both quotes.  TODO: fix std/regex library to allow escaping')
        emit_line(f'let {regex_name}: Regex = rgx{qt}{regex}{qt};')
        regex_names[regex] = regex_name

    # Some null expectations work better when testing regular
    # expressions against `literalPart` happens inside a high level
    # `if (literalPart != null)` and similarly the ${} pattern handling
    # only works when literalPart is null.
    inside_literal_part_not_null = [False]
    def require_literal_part(wants_literal_part: bool):
        if inside_literal_part_not_null[0] != wants_literal_part:
            if wants_literal_part:
                emit_line('if (literalPart != null) {')
            else:
                emit_line('}')
            inside_literal_part_not_null[0] = wants_literal_part

    row_num = 0
    pattern_columns = table.var_names
    for table_row in table.rows:
        row_num += 1

        input_pattern = table_row.input_pattern
        regex = table_row.regex
        substitution = table_row.substitution
        output = table_row.output
        problem = table_row.problem

        input_parts = [x.strip() for x in input_pattern.split(',')]
        if len(input_parts) != len(pattern_columns):
            raise SyntaxError(f'{file_path}:table {ident}:row {row_num}: input pattern {input_pattern} does not have {len(pattern_columns)} columns')

        # Expect a pattern like
        #
        #    stateVarConstant, _, _, _ ; pop() ; push(otherEscaper)
        #
        # The semicolon separated directives are optional, but we need
        # to separate them from the comma separated pattern parts.
        if ';' in output:
            output_parts, state_adjustments = output.split(';', 1)
        else:
            output_parts = output
            state_adjustments = None

        output_parts = [x.strip() for x in output_parts.split(',')]
        if len(output_parts) != len(pattern_columns):
            raise SyntaxError(f'{file_path}:table {ident}:row {row_num}: output pattern {output} does not have {len(pattern_columns)} columns')

        # Build a predicate for this table row
        predicate_parts = []

        matcher, lookahead, pos_lookahead = None, None, None
        is_interp_prep = regex == '${}'
        if is_interp_prep:  # matching an interpolation start
            require_literal_part(False)
            predicate_parts.append(f'literalPart == null')
        elif regex.startswith('`'):
            require_literal_part(True)
            matcher, lookahead, pos_lookahead = decompose_regex(regex.strip('`'), file_path)
        elif regex:
            raise SyntaxError(f'{file_path}:table {ident}:row {row_num}: unrecognized regex {regex}')

        constant_for_ident_value = {}
        def qname_for_part(pattern_part, var_name):
            if re.fullmatch(r'\d+', pattern_part):
                # Look up a constant.  It's super convenient just to write 0 sometimes.
                const_lookup_key = (pattern_part, var_name)
                if const_lookup_key not in constant_for_ident_value:
                    int_value = int(pattern_part, base = 10)
                    constant_found = next((k for k, v in constants.items()
                                           if k.startswith(var_name) and v == int_value),
                                          f'{var_name}{pattern_part}')
                    constant_for_ident_value[const_lookup_key] = constant_found
                return constant_for_ident_value[const_lookup_key]
            else:
                return f'{var_name}{pattern_part}'

        for pattern_part, var_name in zip(input_parts, pattern_columns):
            if pattern_part == '_': continue # Matches anything
            if pattern_part.startswith('#'): # A computational transform
                predicate_parts.append(f'{computation_name(pattern_part)}(contextBefore.{var_name})')
            else:
                pattern_qname = qname_for_part(pattern_part, var_name)
                predicate_parts.append(f'contextBefore.{var_name} == {pattern_qname}')

        if predicate_parts:
            emit_line( 'if (%s) {' % ' && '.join(predicate_parts))
        else:
            emit_line( 'do {')

        # We add up to two layers of checks for the decomposed regular expression.
        matched = False # Remember so we can compute composed.
        if matcher is not None:
            regex_name = regex_names[matcher]
            emit_line(f'let match: Match? = {regex_name}.find(literalPart) orelse null;')
            emit_line( 'if (match != null) {')
            matched = True
        else:
            emit_line( 'do {')

        looked_ahead = False
        if lookahead is not None:
            regex_name = regex_names[lookahead]
            if matched:
                # TODO: instead of slicing, it'd be nice if we could use the second argument form but
                # that doesn't seem to work with the anchor.
                begin_arg = '.slice(match.full.end, literalPart.end)' # ', match.full.end'
            else:
                begin_arg = ''
            if pos_lookahead:
                op = "!=" # match is not null
            else:
                op = "=="
            emit_line('if ((%s.find(literalPart%s) orelse null) %s null) {' % (regex_name, begin_arg, op))
            looked_ahead = True

        if debug_trace:
            row_str = f'#{row_num} {ident}: {input_pattern} | {regex} | {substitution} | {output}'
            if matched:
                consumed='consuming `${match.full.value}` '
            else:
                consumed=''
            emit_line(f'console.log(". {consumed}chose transition row {escape_temper_str_part(row_str)}");')
        if problem.strip():
            is_error = False
            if problem.startswith('W:'): # A warning
                problem = problem[2:]
            elif problem.startswith('E:'): # An error
                problem = problem[2:]
                is_error = True
            else:
                raise SyntaxError(f'{file_path}:table {ident}:row {row_num}: column problem does not have a clear `W:` or `E:` severity prefix: {problem}')
            emit_line('if (callout != null) {')
            emit_line(f'callout({escape_temper_str(problem.strip())}, {json.dumps(is_error)});')
            emit_line('}')
        emit_line(f'return new AfterPropagate<{context_name}, {esc_name}>(')
        # Output the substitution text
        if substitution:
            emit_line(f"{escape_temper_str(substitution.strip('`'))},")
        elif matched:
            emit_line(f'match.full.value,')
        else:
            emit_line(f'"",')
        # Output the position after the match
        if matched:
            emit_line(f'match.full.end,')
        else:
            emit_line(f'String.begin,')
        # Output the context after.
        if all((x == '_' for x in output_parts)):
            emit_line( 'contextBefore,')
        else:
            emit_line(f'new {context_name}(')
            for output_col, var_name in zip(output_parts, pattern_columns):
                if output_col == '_': # Same as before
                    emit_line(f'contextBefore.{var_name},')
                elif output_col.startswith('#'):
                    # A computational transform computes the next value.
                    emit_line(f'{computation_name(output_col)}(contextBefore.{var_name}),')
                else:
                    emit_line(f'{qname_for_part(output_col, var_name)},')
            emit_line(f'),')
        # Output the subsidiary.
        emit_line(f'before.subsidiary,')
        # Output adjustments to the stack via the result object
        state_adjustment_calls = []
        if state_adjustments:
            state_adjustments = [x.strip() for x in state_adjustments.split(';')]
            for state_adjustment in state_adjustments:
                if state_adjustment.startswith('start('):
                    comma = state_adjustment.index(',')
                    state_adjustment = f'start(new {ident}{state_adjustment[6:comma]}Delegate(){state_adjustment[comma:]}'
                state_adjustment_calls.append(f'.{state_adjustment}') # Call it as a method
        emit_line(f'){"".join(state_adjustment_calls)};')
        if looked_ahead: emit_line( '}')
        emit_line('}') # Matched
        emit_line('}') # Preficate Parts if

    if inside_literal_part_not_null[0]: emit_line('}')
    emit_line('panic()')
    emit_line('}') # Function body
    emit_line('') # Blank line to separate code block from the HTML comment marker
    assert(indent[0] == 0), f'indent={indent}'

    return TextRegion(''.join((f"{f'    {x}'.rstrip()}\n" for x in out_lines)))

def escape_temper_str_part(s: str, metas = r'\"') -> str:
    def escape_temper_char(c: str) -> str:
        if c < ' ':
            return '\\u%04x' % ord(c)
        if c in metas:
            return '\\%s' % c
        return c

    return ''.join((escape_temper_char(x) for x in s))

def escape_temper_str(s: str, metas = r'\"') -> str:
    return f'"{escape_temper_str_part(s, metas)}"'

def regen_transition_table(region: SubstRegion, current: TemperMdFile, temper_md_files: list[TemperMdFile]) -> Region:
    ident, var_names = region.details.split(';', 1)
    ident = ident.strip()
    var_names = [x.strip() for x in var_names.split(',')]

    source, source_file = source_for_ident(ident, temper_md_files)
    if source is None:
        raise SyntaxError(f'{current.file_path}: no source table for {ident}')

    table = source.table

    var_info = var_info_for_table(table, current.constants, var_names = var_names)

    def strip_prefix(pre: str, whole: str) -> str:
        if whole.startswith(pre):
            return whole[len(pre):]
        else:
            return whole

    # We need to assign names to nodes but also present them nicely.
    node_names: dict[tuple[str, ...], str] = {}
    def node_for(parts: list[str], default_parts: list[str] | None = None) -> str | None:
        key: list[str] = []
        for var_name in var_names:
            info = var_info[var_name]
            idx = info.pattern_index
            part = parts[idx]
            if part == '_' and default_parts is not None:
                part = default_parts[idx]
            if part == '_':
                return None
            elif f'{var_name}{part}' in info.constants:
                key.append(f'{var_name}{part}')
            else:
                try:
                    key.append(cast(str, info.constants[int(part)]))
                except ValueError:
                    raise SyntaxError(f'{current.file_path}: in transition table {ident}, {part} not recognized as value for {var_name}')
        key_tuple = tuple(key)
        if key_tuple not in node_names:
            key_text = ','.join(key)
            node_names[key_tuple] = key_text
            display_text = ' '.join([strip_prefix(var_name, x) for var_name, x in zip(var_names, key)])
            out_lines.append(f'  state "{display_text}" as {key_text}')
        return node_names[key_tuple]

    # Build the output lines
    out_lines = ['stateDiagram-v2']
    for row in table.rows:
        input_pattern = row.input_pattern
        substitution = row.substitution
        output_pattern = row.output
        actions = None
        if ';' in output_pattern:
            output_pattern, actions = output_pattern.split(';', 1)
            actions = actions.strip()

        input_parts = [x.strip() for x in input_pattern.split(',')]
        output_parts = [x.strip() for x in output_pattern.split(',')]

        input_node = node_for(input_parts)
        if input_node is None:
            # An any transition which doesn't fit well in the model of
            # transitions on a subset of inputs.
            continue

        output_node = node_for(output_parts, input_parts)
        message = f"{row.regex}"
        if substitution:
            message = f"{message or '``'}\u2192{substitution}"
        if actions:
            message = f"{message}\n{actions}"
        def esc_mermaid(m):
            c = m.group(0)
            return '#%d;' % ord(c)
        message = re.sub(r'[:`#*_\'"<>(){}\[\]&]', esc_mermaid, message)
        message = message.replace('\n', '<br>')
        if message: message = f": {message}"
        out_lines.append(f'  {input_node} --> {output_node}{message}')

    # Pick an unambiguous Markdown fenced block delimiter
    txt = '\n'.join(out_lines)
    delim = '```'
    while delim in txt:
        delim = f'{delim}`'

    # Insert code fence at beginning and end with blank lines that
    # separate the markdown from the HTML comment markers.
    out_lines[0:0] = ['', f'{delim}mermaid']
    out_lines[len(out_lines):len(out_lines)] = [delim, '', '']

    return TextRegion('\n'.join(out_lines))

def regen_temper(file_text: str, temper_md_file: TemperMdFile, temper_md_files: list[TemperMdFile]) -> str:
    """generate replacement text for a file"""
    new_regions = [
        regen_region(region, temper_md_file, temper_md_files)
        for region in temper_md_file.regions
    ]
    return ''.join([new_region.text for new_region in new_regions])

def main(argv):
    dry_run = False
    if '--dry-run' in argv:
        dry_run = True
        argv = [x for x in argv if x != '--dry-run']
    file_paths = argv
    temper_md_files_and_texts = []
    for file_path in file_paths:
        with open(file_path, 'r', encoding="utf-8") as file_input:
            file_text = file_input.read()
            temper_md_files_and_texts.append((parse_md(file_text, file_path), file_text))
    temper_md_files = [tmf for (tmf, _) in temper_md_files_and_texts]
    for (temper_md_file, file_text) in temper_md_files_and_texts:
        got = regen_temper(file_text, temper_md_file, temper_md_files)
        if got != file_text:
            file_path = temper_md_file.file_path
            if dry_run:
                print(f'Dry run so did not write changes to {file_path}.')
            else:
                with open(file_path, 'w', encoding="utf-8") as file_output:
                    file_output.write(got)

if __name__ == '__main__':
    import sys
    main(sys.argv[1:])
