;;; xetal-mode.el --- Major mode for X_eTaL  -*- lexical-binding: t; coding: utf-8 -*-

;; Copyright (c) 2026 Michael A Wright
;; SPDX-License-Identifier: MIT

;;; Commentary:

;; X_eTaL source is plain ASCII; `xetal render --color' and the editor
;; draw it decorated.  This mode colours the same classes (system
;; functions blue, symbols light blue, the program's u: functions
;; green, library functions cyan, macros bold yellow, the lambda
;; arguments magenta, numbers yellow) and, with `prettify-symbols-mode'
;; (on by default here), shows := -> _l _r != <= >= as the glyphs the
;; decorated form uses, while the file keeps its ASCII.
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
  `(("\\_<[a-z][A-Za-z0-9]*_[A-Za-z0-9]*<" . 'xetal-macro-face)
    ("\\_<u:[A-Za-z][A-Za-z0-9]*_[A-Za-z0-9]*" . 'xetal-user-face)
    ("\\_<[a-z][a-z0-9]*:[A-Za-z][A-Za-z0-9]*_[A-Za-z0-9]*" . 'xetal-library-face)
    (,(concat "\\_<_[lr]_?\\_>") . 'xetal-argument-face)
    (,(concat "\\_<[A-Za-z][A-Za-z0-9]*_[A-Za-z0-9]*" xetal--mark "?\\(?:_[1-9]+\\)?")
     . 'xetal-builtin-face)
    ("\\^-?[0-9.]+\\|\\_<-?[0-9]+\\(?:\\.[0-9]+\\)?\\(?:[eE]-?[0-9]+\\)?" . 'xetal-number-face)
    ("!=\\|<=\\|>=\\|[-+*/=<>&|^]" . 'xetal-symbol-face))
  "Colours by token class, as in `xetal render --color'.")

(defconst xetal-prettify-symbols
  '((":=" . ?←) ("->" . ?→) ("_l" . ?⍺) ("_r" . ?⍵)
    ("!=" . ?≠) ("<=" . ?≤) (">=" . ?≥))
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

;;;###autoload
(define-derived-mode xetal-mode prog-mode "X_eTaL"
  "Major mode for X_eTaL source (.xtl)."
  (setq-local comment-start "# ")
  (setq-local font-lock-defaults '(xetal-font-lock-keywords))
  (setq-local prettify-symbols-alist xetal-prettify-symbols)
  (prettify-symbols-mode 1))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.xtl\\'" . xetal-mode))

(provide 'xetal-mode)
;;; xetal-mode.el ends here
