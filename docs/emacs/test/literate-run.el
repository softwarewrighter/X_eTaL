;;; literate-run.el --- Run a literate document's blocks, in batch  -*- lexical-binding: t; -*-

;; Copyright (c) 2026 Michael A Wright
;; SPDX-License-Identifier: MIT

;;; Commentary:

;; emacs --batch -L docs/emacs -l docs/emacs/test/literate-run.el FILE.org
;; runs every xetal block of FILE.org (the xetal in XETAL_BIN) and
;; saves the file with each result recorded under its block.

;;; Code:

(require 'org)
(require 'ob-xetal)

(setq org-babel-xetal-command (or (getenv "XETAL_BIN") "xetal"))
(setq org-confirm-babel-evaluate nil)
(org-babel-do-load-languages 'org-babel-load-languages '((xetal . t)))

(dolist (file command-line-args-left)
  (with-current-buffer (find-file-noselect file)
    (org-babel-execute-buffer)
    (save-buffer)))
(setq command-line-args-left nil)

;;; literate-run.el ends here
