;;; xetal-mode.el --- Major mode for X_eTaL  -*- lexical-binding: t; coding: utf-8 -*-

;; Copyright (c) 2026 Michael A Wright
;; SPDX-License-Identifier: MIT

;;; Commentary:

;; X_eTaL source is plain ASCII; `xetal render --color' and the editor
;; draw it decorated.  This mode colors the same classes (system
;; functions blue, symbols light blue, the program's u: functions
;; green, library functions cyan, macros bold yellow, the lambda
;; arguments magenta, numbers yellow) and, with `prettify-symbols-mode'
;; (on by default here), shows := -> _l _r != <= >= |> and a label's $
;; as the glyphs the decorated form uses, while the file keeps its
;; ASCII.  The inside of a code string (c"..." and c#"..."#, ST5) is
;; colored as code; the c, the #s and the quotes as a string.
;;
;; With `xetal-draw-names' (on by default) a function name is drawn as
;; rendered, as `prettify-symbols-mode' draws the arrows: the letter
;; before its `_' underlined and the `_' not shown (r_ev as r̲ev), and an
;; applied group's parentheses underlined with its `_' not shown (rule
;; D-11).  The buffer keeps its ASCII, and the name or group under the
;; cursor is shown as typed, so what you type is what you see there.
;; Rainbow delimiters (D-12): (add-hook 'xetal-mode-hook
;; #'rainbow-delimiters-mode) with the rainbow-delimiters package.
;;
;;   (add-to-list 'load-path "path/to/X_eTaL/docs/emacs")
;;   (require 'xetal-mode)

;;; Code:

(defgroup xetal nil "X_eTaL, the eXperimental Extensible Typed Array Language."
  :group 'languages)

(defface xetal-builtin-face '((t :foreground "#4d7fd6")) "System functions." :group 'xetal)
(defface xetal-symbol-face '((t :foreground "#79b8ff")) "Symbol functions (+ = < ...)." :group 'xetal)
(defface xetal-user-face '((t :foreground "#3fa34d")) "The program's u: functions." :group 'xetal)
(defface xetal-library-face '((t :foreground "#1c9ca8")) "Functions from a library." :group 'xetal)
(defface xetal-macro-face '((t :foreground "#c8a415" :weight bold)) "Macros (u_se<)." :group 'xetal)
(defface xetal-argument-face '((t :foreground "#b04fc4")) "The lambda arguments _l and _r." :group 'xetal)
(defface xetal-number-face '((t :foreground "#b5892a")) "Numbers and exponents." :group 'xetal)

(defface xetal-underline-face '((t :underline t))
  "A drawn name's underlined letter, an applied group's parentheses." :group 'xetal)

(defcustom xetal-draw-names t
  "Draw function names and applied groups as rendered.
The letter before a name's `_' is underlined and the `_' not shown; an
applied group's parentheses are underlined and its `_' not shown.  The
buffer keeps its ASCII; the name under the cursor is shown as typed."
  :type 'boolean :group 'xetal)

(defconst xetal--mark "[|/\\\\+*<>~!?%$&-]"
  "A punctuation character a function name may end in.")

(defconst xetal-font-lock-keywords
  `((xetal--match-code-delimiter . 'font-lock-string-face)
    ("\\_<[a-z][A-Za-z0-9]*_[A-Za-z0-9]*<" . 'xetal-macro-face)
    ("\\_<u:[A-Za-z][A-Za-z0-9]*_[A-Za-z0-9]*" . 'xetal-user-face)
    ("\\_<[a-z][a-z0-9]*:[A-Za-z][A-Za-z0-9]*_[A-Za-z0-9]*" . 'xetal-library-face)
    (,(concat "\\_<_[lr]_?\\_>") . 'xetal-argument-face)
    (,(concat "\\_<[A-Za-z][A-Za-z0-9]*_[A-Za-z0-9]*" xetal--mark "?\\(?:_[1-9]+\\)?")
     . 'xetal-builtin-face)
    ("\\^-?[0-9.]+\\|\\_<-?[0-9]+\\(?:\\.[0-9]+\\)?\\(?:[eE]-?[0-9]+\\)?" . 'xetal-number-face)
    ("!=\\|<=\\|>=\\|[-+*/=<>&|^]" . 'xetal-symbol-face)
    (xetal--match-drawn-name (1 'xetal-underline-face prepend) (2 '(face nil display "")))
    (xetal--match-applied (1 'xetal-underline-face prepend) (2 'xetal-underline-face prepend)
                          (3 '(face nil display ""))))
  "Colors by token class, as in `xetal render --color'.")

(defconst xetal-prettify-symbols
  '((":=" . ?←) ("->" . ?→) ("_l" . ?⍺) ("_r" . ?⍵)
    ("!=" . ?≠) ("<=" . ?≤) (">=" . ?≥) ("|>" . ?▷) ("$" . ?▹))
  "What `prettify-symbols-mode' shows in place of the ASCII.")

(defvar xetal-mode-syntax-table
  (let ((table (make-syntax-table)))
    (modify-syntax-entry ?# "<" table)
    (modify-syntax-entry ?\n ">" table)
    (modify-syntax-entry ?\" "\"" table)
    (modify-syntax-entry ?_ "_" table)
    (modify-syntax-entry ?' "'" table)
    table)
  "Syntax: # comments to the end of the line, \"strings\", _ in names.")

;; Raw strings (ST4): `r"..."` and `r#"..."#`, no escapes, may span
;; lines.  The `r' and the closing `"' are marked as generic string
;; fences and backslashes inside as punctuation, so quotes, `#' and
;; backslashes inside are text.  Code strings (ST5): `c"..."` (the
;; usual escapes) and `c#"..."#` (verbatim) hold X_eTaL: their quotes
;; are punctuation, so the inside is code; in `c"..."` the backslashes
;; are punctuation too, so an escaped inner string opens and closes.
;; A code string not yet closed (being typed) is code to the end.
(defun xetal--syntax-propertize (start end)
  "Mark the raw and code strings that open between START and END."
  (remove-text-properties start end '(xetal-code-delimiter nil))
  (goto-char start)
  (while (re-search-forward "\\(?:^\\|[^[:alnum:]_:]\\)\\([rc]\\)\\(#*\\)\"" end t)
    (let ((open (match-beginning 1))
          (hashes (match-string 2)))
      (unless (nth 8 (save-excursion (syntax-ppss open)))
        (if (eq (char-after open) ?r)
            (xetal--raw-string open hashes)
          (xetal--code-string open (1- (point)) hashes))))))

(defun xetal--raw-string (open hashes)
  "Mark the raw string at OPEN, closed by a quote and HASHES."
  (put-text-property open (1+ open) 'syntax-table (string-to-syntax "|"))
  (xetal--hashes-punctuation (1+ open) (length hashes))
  (when (search-forward (concat "\"" hashes) nil t)
    (xetal--hashes-punctuation (- (point) (length hashes)) (length hashes))
    (put-text-property (- (point) 1 (length hashes)) (- (point) (length hashes))
                       'syntax-table (string-to-syntax "|"))
    (xetal--backslashes-punctuation open (point))))

(defun xetal--code-string (open quote hashes)
  "Mark the code string at OPEN whose quote is at QUOTE, with HASHES."
  (let ((close (xetal--code-close (1+ quote) hashes)))
    (put-text-property quote (1+ quote) 'syntax-table (string-to-syntax "."))
    (put-text-property open (1+ quote) 'xetal-code-delimiter t)
    (xetal--hashes-punctuation (1+ open) (length hashes))
    (when (string= hashes "")
      (xetal--backslashes-punctuation quote (or close (point-max))))
    (when close
      (put-text-property close (1+ close) 'syntax-table (string-to-syntax "."))
      (put-text-property close (+ close 1 (length hashes)) 'xetal-code-delimiter t)
      (xetal--hashes-punctuation (1+ close) (length hashes))
      (goto-char (+ close 1 (length hashes))))))

(defun xetal--code-close (from hashes)
  "Where the code string whose text starts at FROM closes, or nil.
With HASHES, the first quote followed by them; without, the first
quote no backslash escapes."
  (save-excursion
    (goto-char from)
    (if (not (string= hashes ""))
        (and (search-forward (concat "\"" hashes) nil t)
             (- (point) 1 (length hashes)))
      (let (close)
        (while (and (not close) (re-search-forward "\\\\.\\|\"" nil t))
          (when (equal (match-string 0) "\"")
            (setq close (match-beginning 0))))
        close))))

(defun xetal--hashes-punctuation (from count)
  "Mark the COUNT `#'s of a delimiter at FROM as punctuation, not comments."
  (when (> count 0)
    (put-text-property from (+ from count) 'syntax-table (string-to-syntax "."))))

(defun xetal--backslashes-punctuation (from to)
  "Mark every backslash between FROM and TO as punctuation."
  (save-excursion
    (goto-char from)
    (while (search-forward "\\" to t)
      (put-text-property (1- (point)) (point) 'syntax-table (string-to-syntax ".")))))

(defun xetal--match-code-delimiter (limit)
  "Font-lock matcher: the next code string delimiter before LIMIT."
  (let ((pos (text-property-any (point) limit 'xetal-code-delimiter t)))
    (when pos
      (let ((stop (or (text-property-not-all pos limit 'xetal-code-delimiter t) limit)))
        (set-match-data (list pos stop))
        (goto-char stop)
        t))))

;; Drawn names (`xetal-draw-names'): font-lock hides the `_' with a
;; `display' property; the name or group under the cursor is left as
;; typed (`xetal--reveal' notes it after each command and refontifies).
(defvar-local xetal--typed nil
  "The (START . END) of the name or group shown as typed, at the cursor.")

(defun xetal--drawable-p (pos)
  "Whether the `_' at POS may be drawn: in code, not at the cursor."
  (and xetal-draw-names
       (not (nth 8 (save-excursion (syntax-ppss pos))))
       (not (and xetal--typed (<= (car xetal--typed) pos) (< pos (cdr xetal--typed))))))

(defun xetal--match-drawn-name (limit)
  "Font-lock matcher: a function name's underlined letter and `_'."
  (let (found)
    (while (and (not found)
                (re-search-forward
                 "\\_<\\(?:[a-z][a-z0-9]*:\\)?[A-Za-z0-9]*?\\([A-Za-z]\\)\\(_\\)" limit t))
      (setq found (xetal--drawable-p (match-beginning 2))))
    found))

(defun xetal--match-applied (limit)
  "Font-lock matcher: an applied group's `(', `)' and `_' (rule D-11)."
  (let (found)
    (while (and (not found) (re-search-forward ")\\(_\\)\\_>" limit t))
      (let* ((close (1- (match-beginning 1)))
             (open (ignore-errors (scan-lists (1+ close) -1 0))))
        (when (and open (xetal--drawable-p (match-beginning 1)))
          (set-match-data (list open (match-end 1) open (1+ open) close (1+ close)
                                (match-beginning 1) (match-end 1)))
          (setq found t))))
    found))

(defun xetal--reveal ()
  "Show the name or applied group at the cursor as typed, the rest drawn."
  (let ((here (save-excursion
                (skip-chars-backward "A-Za-z0-9_:)")
                (let ((start (point)))
                  (skip-chars-forward "A-Za-z0-9_:)")
                  (and (< start (point)) (cons start (point)))))))
    (unless (equal here xetal--typed)
      (let ((old xetal--typed))
        (setq xetal--typed here)
        (dolist (r (list old here))
          (when r (font-lock-flush (car r) (cdr r))))))))

(defun xetal--compose-p (start end match)
  "Draw a label's $ only where it touches a name or a tuple (S14)."
  (if (equal match "$")
      (let ((next (char-after end)))
        (and next (or (eq next ?\() (and (< next 128) (string-match-p "[A-Za-z]" (string next))))))
    (prettify-symbols-default-compose-p start end match)))

(define-derived-mode xetal-mode prog-mode "X_eTaL"
  "Major mode for X_eTaL source (.xtl) and macro libraries (.xtlm)."
  (setq-local comment-start "# ")
  (setq-local font-lock-defaults '(xetal-font-lock-keywords))
  (setq-local syntax-propertize-function #'xetal--syntax-propertize)
  (add-hook 'syntax-propertize-extend-region-functions
            #'syntax-propertize-wholelines nil t)
  (setq-local prettify-symbols-alist xetal-prettify-symbols)
  (setq-local prettify-symbols-compose-predicate #'xetal--compose-p)
  (setq-local font-lock-extra-managed-props '(display composition))
  (add-hook 'post-command-hook #'xetal--reveal nil t)
  (prettify-symbols-mode 1))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.xtlm?\\'" . xetal-mode))

(provide 'xetal-mode)
;;; xetal-mode.el ends here
