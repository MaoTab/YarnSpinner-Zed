; Highlights for Yarn Spinner (tree-sitter yarn_spinner)

; Comments
(comment) @comment

; File-level hashtags and inline hashtags
(file_hashtag
  (hashtag_marker) @punctuation.special
  (hashtag_text) @tag)

(hashtag
  (hashtag_marker) @punctuation.special
  (hashtag_text) @tag)

; Node headers
(title_header
  (title_kw) @keyword
  title: (rest_of_line) @title)

(when_header
  (when_kw) @keyword
  expr: (_) @expression)

(header
  key: (identifier) @property
  value: (rest_of_line)? @text.literal)

(header_delimiter) @punctuation.delimiter

; Header when expression variants
(header_when_expression
  (always_kw) @keyword)

(header_when_expression
  (once_kw) @keyword
  (if_kw)? @keyword)

; Body delimiters
(body_start) @punctuation.special
(body_end) @punctuation.special

; Statements and structural arrows
(shortcut_arrow) @punctuation.special
(line_group_arrow) @punctuation.special

; Commands
(command_start) @punctuation.special
(command_end) @punctuation.special
(command_text) @preproc

; Keywords
(if_kw) @keyword
(elseif_kw) @keyword
(else_kw) @keyword
(endif_kw) @keyword
(once_kw) @keyword
(endonce_kw) @keyword
(enum_kw) @keyword
(endenum_kw) @keyword
(case_kw) @keyword
(set_kw) @keyword
(call_kw) @keyword
(declare_kw) @keyword
(jump_kw) @keyword
(detour_kw) @keyword
(return_kw) @keyword
(as_kw) @keyword
(always_kw) @keyword

; Expression keywords
(not_kw) @keyword
(and_kw) @keyword
(or_kw) @keyword
(xor_kw) @keyword
(is_kw) @keyword
(eq_kw) @keyword
(neq_kw) @keyword
(lt_kw) @keyword
(gt_kw) @keyword
(lte_kw) @keyword
(gte_kw) @keyword

; Boolean and null literals
(true_kw) @boolean
(false_kw) @boolean
(null_kw) @constant

; If statements
(if_clause
  (if_kw) @keyword)

(else_if_clause
  (elseif_kw) @keyword)

(else_clause
  (else_kw) @keyword)

; Once statements
(once_clause
  (once_kw) @keyword
  (if_kw)? @keyword)

(once_alternate_clause
  (else_kw) @keyword)

; Enum statements
(enum_statement
  (enum_kw) @keyword
  name: (identifier) @type)

(enum_case_statement
  (case_kw) @keyword
  name: (identifier) @constant
  operator: (_)? @operator
  value: (_)? @expression)

; Set statement
(set_statement
  (set_kw) @keyword
  operator: (_) @operator)

; Call statement
(call_statement
  (call_kw) @keyword)

; Declare statement
(declare_statement
  (declare_kw) @keyword
  (as_kw)? @keyword
  type: (identifier)? @type)

; Jump statements
(jump_statement
  (jump_kw) @keyword
  destination: (identifier)? @function)

(jump_statement
  (detour_kw) @keyword
  destination: (identifier)? @function)

; Return statement
(return_statement
  (return_kw) @keyword)

; Line condition
(line_condition
  (if_kw) @keyword)

(line_condition
  (once_kw) @keyword
  (if_kw)? @keyword)

; Inline expressions
(expression_start) @punctuation.bracket
(expression_end) @punctuation.bracket

; Inline markup, for example [wave], [/wave], [/], and [wait=0.6/]
(markup_start) @punctuation.bracket
(markup_end) @punctuation.bracket
(markup_close_marker) @punctuation.bracket
(markup_self_close) @punctuation.bracket
(markup_equals) @operator

(markup_open
  name: (markup_name) @tag)

(markup_close
  name: (markup_name) @tag)

(markup_self_closing
  name: (markup_name) @tag)

(markup_property
  name: (markup_name) @property)

(markup_unquoted_value) @string

; Text content
(text) @text.literal
(text_slash) @text.literal
(text_closing_bracket) @text.literal
(escaped_character) @text.literal

; Variables and identifiers
(variable
  ; "$" @punctuation.delimiter
  (identifier) @variable)

(identifier) @identifier

; Numbers and strings
(number) @number
(string) @string

; Expressions and operators
(binary_expression
  operator: (_) @operator)

(unary_expression
  operator: (_) @operator)

(paren_expression
  "(" @punctuation.bracket
  ")" @punctuation.bracket)

(function_call
  function: (identifier) @function
  "(" @punctuation.bracket
  "," @punctuation.delimiter
  ")" @punctuation.bracket)

(member_expression
  type: (identifier)? @type
  "." @punctuation.delimiter
  member: (identifier) @property)

; Generic command statement highlighting
(command_statement
  (command_start) @punctuation.special
  (command_end) @punctuation.special)

; Newlines (for debugging/visualization)
(newline) @punctuation.whitespace
