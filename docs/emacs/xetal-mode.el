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
    ("!=\\|<=\\|>=\\|[-+*/=<>&|^]" . 'xetal-symbol-face))
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

(defun xetal--compose-p (start end match)
  "Draw a label's $ only where it touches a name or a tuple (S14)."
  (if (equal match "$")
      (let ((next (char-after end)))
        (and next (or (eq next ?\() (and (< next 128) (string-match-p "[A-Za-z]" (string next))))))
    (prettify-symbols-default-compose-p start end match)))

(define-derived-mode xetal-mode prog-mode "X_eTaL"
  "Major mode for X_eTaL source (.xtl)."
  (setq-local comment-start "# ")
  (setq-local font-lock-defaults '(xetal-font-lock-keywords))
  (setq-local syntax-propertize-function #'xetal--syntax-propertize)
  (add-hook 'syntax-propertize-extend-region-functions
            #'syntax-propertize-wholelines nil t)
  (setq-local prettify-symbols-alist xetal-prettify-symbols)
  (setq-local prettify-symbols-compose-predicate #'xetal--compose-p)
  (prettify-symbols-mode 1))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.xtl\\'" . xetal-mode))

(provide 'xetal-mode)
;;; xetal-mode.el ends here
